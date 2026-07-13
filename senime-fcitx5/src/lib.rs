use senime_lib::{
    AnalysisResult, CandidateRich, InputAnalyzer, PAGE_DOWN, PAGE_UP, RecommendedWatcher,
    input_analyzer::{Tag, load_input_analyzer},
    resolve_relative_path, spawn_watcher,
};
use std::{
    ffi::{CStr, CString, c_char},
    panic::{AssertUnwindSafe, catch_unwind},
    path::Path,
    ptr,
    sync::{Arc, RwLock},
};

mod keysym;
use keysym::*;

// ── Fcitx5 日志桥接 ──────────────────────────────────────────────────────

/// Fcitx5 日志回调函数类型: (level, message)
/// level: 0=INFO, 1=WARN, 2=ERROR
type FcitxLogFn = unsafe extern "C" fn(i32, *const c_char);

/// 全局 Fcitx5 日志回调，由 C++ 端在初始化时设置
static FCITX_LOG: std::sync::OnceLock<FcitxLogFn> = std::sync::OnceLock::new();

/// 日志级别常量
const FCITX_LOG_INFO: i32 = 0;
const FCITX_LOG_WARN: i32 = 1;
const FCITX_LOG_ERROR: i32 = 2;

/// 通过 fcitx5 日志系统输出日志。
/// 用法: `fcitx_log!(FCITX_LOG_WARN, "message: {}", value);`
macro_rules! fcitx_log {
    ($level:expr, $($arg:tt)*) => {{
        let msg = format!($($arg)*);
        if let Some(log_fn) = FCITX_LOG.get() {
            if let Ok(c_msg) = std::ffi::CString::new(msg) {
                unsafe { log_fn($level, c_msg.as_ptr()); }
            }
        } else {
            // 回调未设置时回退到 stderr
            eprintln!($($arg)*);
        }
    }};
}

/// C++ 调用此函数设置日志回调。
///
/// 同时安装一个 `log::Log` 适配器，使 `senime_lib::watcher` 等使用 `log` facade
/// 的代码也能通过 fcitx5 日志回调输出。
///
/// # Safety
///
/// `callback` 必须是一个有效的、在插件生命周期内始终可用的函数指针。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_set_log_callback(callback: FcitxLogFn) {
    let _ = FCITX_LOG.set(callback);
    // 安装 log facade → fcitx 日志回调 的桥接，仅安装一次。
    let _ = log::set_logger(&FcitxLogger);
    log::set_max_level(log::LevelFilter::Info);
}

/// 将 `log` facade 调用桥接到 fcitx5 日志回调。
struct FcitxLogger;

impl log::Log for FcitxLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let level = match record.level() {
            log::Level::Error => FCITX_LOG_ERROR,
            log::Level::Warn => FCITX_LOG_WARN,
            _ => FCITX_LOG_INFO,
        };
        fcitx_log!(level, "{}", record.args());
    }

    fn flush(&self) {}
}

// Fcitx5 modifier masks (from KeyState enum)
#[allow(unused)]
const FCITX_MOD_SHIFT: u32 = 0x01;
#[allow(unused)]
const FCITX_MOD_CAPSLOCK: u32 = 0x02;
#[allow(unused)]
const FCITX_MOD_CTRL: u32 = 0x04;
#[allow(unused)]
const FCITX_MOD_ALT: u32 = 0x08;
#[allow(unused)]
const FCITX_MOD_NUMLOCK: u32 = 0x10;
#[allow(unused)]
const FCITX_MOD_SUPER: u32 = 0x40;

// ── FFI types for state-based key event processing ───────────────────────
//
// 每轮 key_event 后，Rust 侧在 SenimeState.inner_state 中填充期望的输入面板状态。
// C++ 侧按固定顺序读取并 apply，无需关心指令顺序。

#[repr(C)]
pub struct SenimeInnerState {
    pub commit_str: *mut c_char,
    pub preedit_str: *mut c_char,
    pub aux_up_str: *mut c_char,
    pub aux_down_str: *mut c_char,
    pub candidates: *mut SenimeCandidateData,
    pub candidate_count: usize,
    pub update_status_area: bool,
}

impl Default for SenimeInnerState {
    fn default() -> Self {
        Self {
            commit_str: ptr::null_mut(),
            preedit_str: ptr::null_mut(),
            aux_up_str: ptr::null_mut(),
            aux_down_str: ptr::null_mut(),
            candidates: ptr::null_mut(),
            candidate_count: 0,
            update_status_area: false,
        }
    }
}

#[repr(C)]
pub struct SenimeCandidateData {
    pub text: *mut c_char,
    pub code: *mut c_char,
    pub select_key: u32,
}

#[repr(C)]
pub struct SenimeKeyEvent {
    pub sym: u32,
    pub states: u32,
    pub is_release: bool,
}

#[repr(C)]
pub struct SenimeConfig {
    pub toggle_sym: u32,
    pub toggle_states: u32,
    pub trigger_start_char: u32,
    pub trigger_end_char: u32,
    pub table_path: *mut c_char,
    pub default_chinese_mode: bool,
    pub sentence_flow: bool,
    pub enable_text_preedit: bool,
    pub enable_input_preedit: bool,
}

#[derive(Clone)]
struct SenimeResolvedConfig {
    toggle_key: SenimeKeyBinding,
    /// 当为 None 时，禁用临时中文模式。`(start, end)` 中 `end` 可能与 `start` 相同。
    trigger_chars: Option<(char, char)>,
    default_chinese_mode: bool,
    sentence_flow: bool,
    enable_text_preedit: bool,
    enable_input_preedit: bool,
}

impl Default for SenimeResolvedConfig {
    fn default() -> Self {
        Self {
            toggle_key: SenimeKeyBinding::from((FCITX_KEY_Shift_L, FCITX_MOD_SHIFT)),
            trigger_chars: None,
            default_chinese_mode: false,
            sentence_flow: false,
            enable_text_preedit: true,
            enable_input_preedit: false,
        }
    }
}

impl From<&SenimeConfig> for SenimeResolvedConfig {
    fn from(value: &SenimeConfig) -> Self {
        let trigger_chars = keysym_to_char(value.trigger_start_char).map(|start| {
            let end = keysym_to_char(value.trigger_end_char).unwrap_or(start);
            (start, end)
        });
        Self {
            toggle_key: (value.toggle_sym, value.toggle_states).into(),
            trigger_chars,
            default_chinese_mode: value.default_chinese_mode,
            sentence_flow: value.sentence_flow,
            enable_text_preedit: value.enable_text_preedit,
            enable_input_preedit: value.enable_input_preedit,
        }
    }
}

#[repr(C)]
pub struct SenimeKeyEventResult {
    pub accepted: bool,
    /// 指向 SenimeState.inner_state 的指针，C++ 侧只读。
    pub inner_state: *const SenimeInnerState,
}

// ── Rust-side state ──────────────────────────────────────────────────────

pub struct SenimeState {
    input: Vec<char>,
    chinese_mode: bool,
    config: SenimeResolvedConfig,
    /// 语句流/临时中文模式下，保存最近一次分析的分段结果，用于按段回退。
    segments: Vec<(String, Vec<char>, Tag)>,
    /// 预设模式：Alt+C 加载后，用户输入自动提交预设内容。
    /// Vec 已反转，末尾元素为当前待输入项 (text, code_len)。
    preset: Option<Vec<(String, usize)>>,
    /// 每轮 key_event 填充的显示状态，C++ 侧通过指针读取。
    inner_state: SenimeInnerState,
}

impl SenimeState {
    fn new(config: SenimeResolvedConfig) -> Self {
        Self {
            input: Vec::new(),
            chinese_mode: config.default_chinese_mode,
            config,
            segments: Vec::new(),
            preset: None,
            inner_state: SenimeInnerState::default(),
        }
    }

    /// 重置状态：清空输入缓冲，重置中英模式标记。
    fn reset(&mut self) {
        self.reset_input();
        self.chinese_mode = self.config.default_chinese_mode;
    }

    /// 重置状态：清空输入缓冲，但不重置中英文模式。
    fn reset_input(&mut self) {
        self.input.clear();
        self.segments.clear();
        self.preset = None;
        self.clear_inner_state();
    }

    // ── InnerState 管理 ─────────────────────────────────────────────────

    /// 释放上一轮的 InnerState 数据，归零所有字段。
    fn clear_inner_state(&mut self) {
        // 释放字符串
        unsafe {
            for &ptr in &[
                self.inner_state.commit_str,
                self.inner_state.preedit_str,
                self.inner_state.aux_up_str,
                self.inner_state.aux_down_str,
            ] {
                if !ptr.is_null() {
                    drop(CString::from_raw(ptr));
                }
            }
        }
        // 释放候选数据
        if !self.inner_state.candidates.is_null() && self.inner_state.candidate_count > 0 {
            unsafe {
                let cands = Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    self.inner_state.candidates,
                    self.inner_state.candidate_count,
                ));
                for cand in cands.iter() {
                    if !cand.text.is_null() {
                        drop(CString::from_raw(cand.text));
                    }
                    if !cand.code.is_null() {
                        drop(CString::from_raw(cand.code));
                    }
                }
            }
        }
        self.inner_state = SenimeInnerState::default();
    }

    fn clear_commit(&mut self) {
        if !self.inner_state.commit_str.is_null() {
            unsafe { drop(CString::from_raw(self.inner_state.commit_str)) };
            self.inner_state.commit_str = ptr::null_mut();
        }
    }

    /// 获取 InnerState 的 FFI 指针（用于返回给 C++ 侧）。
    fn inner_state_ptr(&self) -> *const SenimeInnerState {
        &self.inner_state as *const SenimeInnerState
    }

    // ── 便捷方法：填充 InnerState ──────────────────────────────────────

    fn set_commit(&mut self, text: String) {
        if text.is_empty() {
            return;
        }
        if self.inner_state.commit_str.is_null() {
            self.inner_state.commit_str = into_c_string(text);
        } else {
            // 已存在待提交文本，追加到尾部
            let existing = unsafe { CStr::from_ptr(self.inner_state.commit_str) }
                .to_str()
                .unwrap_or("");
            let combined = format!("{existing}{text}");
            unsafe { drop(CString::from_raw(self.inner_state.commit_str)) };
            self.inner_state.commit_str = into_c_string(combined);
        }
    }

    fn make_preedit(&mut self, text: String, input: Option<Vec<char>>) -> String {
        match (
            self.config.enable_text_preedit,
            self.config.enable_input_preedit,
        ) {
            (true, true) => format!(
                "{}{}",
                text,
                input.unwrap_or(vec![]).iter().collect::<String>()
            ),
            (false, true) if let Some(input) = input => input.iter().collect(),
            (false, false) => String::new(),
            _ => text,
        }
    }
    /// 设置 preedit，根据 config 组合文本和输入编码。
    fn set_preedit(&mut self, text: String) {
        // let preedit = match (
        //     self.config.enable_text_preedit,
        //     self.config.enable_input_preedit,
        // ) {
        //     (true, true) => {
        //         if let Some(input) = input {
        //             format!("{text}{}", input.iter().collect::<String>())
        //         } else {
        //             text
        //         }
        //     }
        //     (true, false) => text,
        //     (false, true) => input.unwrap_or(vec![]).iter().collect::<String>(),
        //     (false, false) => String::new(),
        // };
        if !text.is_empty() {
            self.inner_state.preedit_str = into_c_string(text);
        }
    }

    fn set_candidates(&mut self, cands: Vec<CandidateRich>) {
        let data: Vec<SenimeCandidateData> = cands
            .into_iter()
            .map(|c| {
                let origin: String = c.origin.iter().collect();
                let remaining = c.code.strip_prefix(&origin).unwrap_or(&c.code);
                SenimeCandidateData {
                    text: into_c_string(c.text),
                    code: into_c_string(remaining.to_owned()),
                    select_key: c.select_key as u32,
                }
            })
            .collect();
        let boxed = data.into_boxed_slice();
        self.inner_state.candidate_count = boxed.len();
        self.inner_state.candidates = Box::into_raw(boxed) as *mut SenimeCandidateData;
    }

    fn set_aux_up(&mut self, text: String) {
        if !text.is_empty() {
            self.inner_state.aux_up_str = into_c_string(text);
        }
    }

    #[allow(unused)]
    fn set_aux_down(&mut self, text: String) {
        if !text.is_empty() {
            self.inner_state.aux_down_str = into_c_string(text);
        }
    }

    /// Process a key event. Returns acceptance flag; display state is in self.inner_state.
    fn key_event(&mut self, engine: &Arc<RwLock<InputAnalyzer>>, key: &SenimeKeyEvent) -> bool {
        let toggle_key = &self.config.toggle_key;
        // 是否切换中文模式
        if key.sym == toggle_key.sym && key.states == toggle_key.modifier {
            self.clear_inner_state();
            if self.chinese_mode {
                // 语句流模式下，提交分析后的中文文本而非原始编码
                if self.config.sentence_flow && !self.segments.is_empty() {
                    let text: String = self.segments.iter().map(|(t, _, _)| t.as_str()).collect();
                    self.set_commit(text);
                } else {
                    self.set_commit(self.input.iter().collect());
                }
                self.chinese_mode = false;
                self.input.clear();
                self.segments.clear();
                self.preset = None;
                self.set_aux_up("En".to_string());
            } else {
                self.chinese_mode = true;
                self.set_aux_up("中".to_string());
            }
            self.inner_state.update_status_area = true;
            // !key.is_release防止下级应用接收到此key的按下事件，但释放事件却被fcitx5拦截，导致该key一直repeat。
            return !key.is_release;
            // return true;
        }
        // 英文模式处理
        if !self.chinese_mode {
            if let Some((start, _)) = self.config.trigger_chars {
                // 已进入临时中文模式（输入缓冲以触发字符开头）
                if self.input.first() == Some(&start) {
                    return self.chinese_mode(engine, key.sym, key.states, true);
                // 按下触发键，进入临时中文模式
                } else if let Some(ch) = keysym_to_char(key.sym)
                    && ch == start
                {
                    self.clear_inner_state();
                    self.input.push(ch);
                    self.set_aux_up("中".to_string());
                    return true;
                };
            }
            self.clear_inner_state();
            return false;
        }

        // Chinese mode handling
        self.chinese_mode(engine, key.sym, key.states, false)
    }

    fn chinese_mode(
        &mut self,
        engine: &Arc<RwLock<InputAnalyzer>>,
        sym: u32,
        states: u32,
        temp_chinese_mode: bool,
    ) -> bool {
        // Alt+C → 加载预设
        if sym == FCITX_KEY_C && states == FCITX_MOD_ALT {
            self.clear_inner_state();
            self.load_preset();
            if self.preset.is_some() {
                self.set_aux_up("[预设启用]".to_string());
            }
            return true;
        }

        // Non-shift modifiers: commit pending, forward key
        let non_shift_mods = states & !(FCITX_MOD_SHIFT | FCITX_MOD_CAPSLOCK | FCITX_MOD_NUMLOCK);
        if non_shift_mods != 0 {
            self.clear_inner_state();
            if !self.input.is_empty() {
                self.set_commit(self.input.iter().collect());
            }
            return false;
        }

        // Escape
        if sym == FCITX_KEY_Escape {
            self.clear_inner_state();
            self.chinese_mode = false;
            self.reset_input();
            self.inner_state.update_status_area = true;
            return false;
        }

        // Return → 分析后直接提交中文
        if sym == FCITX_KEY_Return {
            self.clear_inner_state();
            self.do_update(engine, temp_chinese_mode, true);
            return false;
        }

        // Backspace
        if sym == FCITX_KEY_BackSpace {
            self.clear_inner_state();
            let mut accept = true;
            // 语句流/临时中文模式下，按段回退
            if (temp_chinese_mode || self.config.sentence_flow)
                && !self.input.is_empty()
                && !self.segments.is_empty()
            {
                // 找到最后一个text非空的segment，回退到它之前（移除该segment及其后的空segment）
                let trim_idx = self
                    .segments
                    .iter()
                    .rposition(|(text, _, _)| !text.is_empty())
                    .unwrap_or(0);
                let prev_input: Vec<char> = self.segments[..trim_idx]
                    .iter()
                    .flat_map(|(_, origin, _)| origin.iter().copied())
                    .collect();
                self.input = prev_input;
                // 临时中文模式需要重新包裹触发字符
                if temp_chinese_mode && let Some((start, _)) = self.config.trigger_chars {
                    self.input.insert(0, start);
                }
            } else if !self.input.is_empty() {
                // 普通逐字符回退
                while let Some(ch) = self.input.pop() {
                    if ch != PAGE_UP && ch != PAGE_DOWN {
                        break;
                    }
                }
            } else {
                accept = false;
            }
            self.do_update(engine, temp_chinese_mode, false);
            return accept;
        }

        // PageUp / PageDown → 翻页（仅在有输入时生效）
        if (sym == FCITX_KEY_Page_Up || sym == FCITX_KEY_KP_Page_Up) && !self.input.is_empty() {
            self.clear_inner_state();
            self.input.push(PAGE_UP); // ⇞
            self.do_update(engine, temp_chinese_mode, false);
            return true;
        }
        if (sym == FCITX_KEY_Page_Down || sym == FCITX_KEY_KP_Page_Down) && !self.input.is_empty() {
            self.clear_inner_state();
            self.input.push(PAGE_DOWN); // ⇟
            self.do_update(engine, temp_chinese_mode, false);
            return true;
        }

        // All other keys → append and analyze
        if let Some(ch) = keysym_to_char(sym) {
            self.clear_inner_state();
            self.input.push(ch);
            self.do_update(engine, temp_chinese_mode, false);
            return true;
        } else {
            self.clear_commit();
        }

        false
    }

    /// Core update: analyze input and fill inner_state.
    fn do_update(
        &mut self,
        engine: &Arc<RwLock<InputAnalyzer>>,
        temp_chinese_mode: bool,
        just_commit: bool,
    ) {
        // ── 预设模式：输入长度匹配时自动提交预设文本 ──────────────
        if self.do_preset() {
            return;
        }

        let chars: Vec<char> = if temp_chinese_mode {
            if let Some((start, end)) = self.config.trigger_chars {
                let s = self.input.as_slice();
                let s = s.strip_prefix(&[start]).unwrap_or(s);
                let s = s.strip_suffix(&[end]).unwrap_or(s);
                s.to_vec()
            } else {
                self.input.clone()
            }
        } else {
            self.input.clone()
        };
        if chars.is_empty() {
            // 临时中文模式，但只有首尾两个触发符，直接提交首部的触发符
            if self.input.len() == 2 {
                self.set_commit(self.input[0].to_string());
                self.input.clear();
                self.segments.clear();
            } else {
                self.set_preedit(self.input.iter().collect());
            }
            return;
        }
        // 先 read() 获取引用，分析后 drop guard，再调用 &self 方法
        let result = { engine.read().unwrap().analyze(&chars) };
        if result.segments.is_empty() {
            self.input.clear();
            self.segments.clear();
            self.set_preedit(String::new());
            return;
        }
        // 保存分段结果，用于语句流/临时中文模式下的按段回退
        if just_commit {
            self.update_mode_just_commit(result);
        } else if temp_chinese_mode {
            self.update_mode_temp_chinese(result);
        } else if self.config.sentence_flow {
            self.update_mode_sentence_flow(result);
        } else {
            self.update_mode_normal(result);
        };
    }

    /// 仅提交模式：所有文本立即提交
    fn update_mode_just_commit(&mut self, result: AnalysisResult) {
        self.input.clear();
        self.segments.clear();
        let text = result.segments.into_iter().map(|seg| seg.0).collect();
        self.set_commit(text);
    }

    /// 临时中文模式：trigger char范围内，所有文本保持在preedit
    fn update_mode_temp_chinese(&mut self, mut result: AnalysisResult) {
        if self
            .config
            .trigger_chars
            .is_some_and(|(_start, end)| self.input.last() == Some(&end))
        {
            // 临时中文模式结束
            let text = result.segments.into_iter().map(|seg| seg.0).collect();
            self.set_commit(text);
            self.input.clear();
            self.segments.clear();
        } else {
            // 临时中文模式未决
            self.segments = result.segments.clone();
            let last_seg = result.segments.pop().unwrap();
            let pre_text: String = result
                .segments
                .into_iter()
                .map(|(text, _, _)| text)
                .collect();
            let input = match &last_seg.2 {
                Tag::Code(selection) => {
                    if selection.dict_idx > 0 {
                        self.set_aux_up("查".to_string());
                    }
                    Some(last_seg.1.to_vec())
                }
                _ => None,
            };
            let text = self.make_preedit(last_seg.0, input);
            self.set_preedit(pre_text + &text);
            if result.pending {
                if let Some(cands) = result.candidates {
                    self.set_candidates(cands);
                }
            }
        }
    }

    /// 语句流模式：文本保持在preedit，仅在标点或unknown(text非空)时提交
    fn update_mode_sentence_flow(&mut self, result: AnalysisResult) {
        self.segments = result.segments.clone();
        let mut pre_segments = result.segments;
        let last_seg = pre_segments.pop().unwrap();
        let is_code_tag = |seg: &(String, Vec<char>, Tag)| {
            matches!(seg.2, Tag::Code(_)) || (matches!(seg.2, Tag::Unknown) && seg.0.is_empty())
        };
        let pre_text = pre_segments
            .iter()
            .map(|(text, _, _)| text.as_str())
            .collect::<String>();
        let last_input = match &last_seg.2 {
            Tag::Code(selection) => {
                if selection.dict_idx > 0 {
                    self.set_aux_up("查".to_string());
                }
                Some(last_seg.1.to_vec())
            }
            _ => None,
        };
        // 当前段是code，如果上一段也是code或没有上一段，则继续语句流
        if is_code_tag(&last_seg)
            && (pre_segments.is_empty() || is_code_tag(pre_segments.last().unwrap()))
        {
            let text = self.make_preedit(last_seg.0, last_input);
            self.set_preedit(pre_text + &text);
            if let Some(cands) = result.candidates {
                self.set_candidates(cands);
            }
        } else {
            self.segments.clear();
            // 当前段不是code ，则先将之前的段上屏，再根据pending走正常的流程。
            self.set_commit(pre_text);
            if result.pending {
                self.input = last_seg.1;
                let preedit = self.make_preedit(last_seg.0, last_input);
                self.set_preedit(preedit);
            } else {
                self.input.clear();
                self.set_commit(last_seg.0);
            }
            if let Some(cands) = result.candidates {
                self.set_candidates(cands);
            }
        }
    }

    /// 正常中文模式：中间段提交，最后一段preedit
    fn update_mode_normal(&mut self, mut result: AnalysisResult) {
        let last_seg = result.segments.pop().unwrap();
        let pre_text: String = result.segments.into_iter().map(|seg| seg.0).collect();
        if !pre_text.is_empty() {
            self.set_commit(pre_text);
        }
        if result.pending {
            let input = match &last_seg.2 {
                Tag::Code(selection) => {
                    if selection.dict_idx > 0 {
                        self.set_aux_up("查".to_string());
                    }
                    Some(last_seg.1.to_vec())
                }
                _ => None,
            };
            self.input = last_seg.1;
            let preedit = self.make_preedit(last_seg.0, input);
            self.set_preedit(preedit);
            if let Some(cands) = result.candidates {
                self.set_candidates(cands);
            }
        } else {
            self.set_commit(last_seg.0);
            self.input.clear();
        }
    }
}

impl SenimeState {
    /// 预设模式：检查输入长度是否匹配当前预设项的编码长度。
    /// 返回 true 表示已处理（调用方应直接 return）。
    fn do_preset(&mut self) -> bool {
        let Some(ref mut items) = self.preset else {
            return false;
        };
        let Some((text, code_len)) = items.last() else {
            self.preset = None;
            return false;
        };
        if self.input.len() < *code_len + 1 {
            let preedit: String = self.input.iter().collect();
            self.set_preedit(preedit);
            true
        } else {
            let commit_text = text.clone();
            items.pop();
            self.input.clear();
            self.segments.clear();
            if items.is_empty() {
                self.preset = None;
            }
            self.set_commit(commit_text);
            true
        }
    }

    /// 从 ~/.cache/senime/preset.json 加载预设。
    /// JSON 格式: { "codes": [{ "text": "...", "code": "..." }, ...] }
    /// 加载后反转 Vec，便于 pop 取当前项。
    fn load_preset(&mut self) {
        let path = match dirs::cache_dir() {
            Some(dir) => dir.join("senime").join("preset.json"),
            None => {
                fcitx_log!(FCITX_LOG_WARN, "cannot determine cache directory");
                return;
            }
        };
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                fcitx_log!(FCITX_LOG_WARN, "failed to read {}: {}", path.display(), e);
                return;
            }
        };
        let json: serde_json::Value = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                fcitx_log!(FCITX_LOG_WARN, "failed to parse preset JSON: {}", e);
                return;
            }
        };
        let codes = match json.get("codes").and_then(|v| v.as_array()) {
            Some(arr) => arr,
            None => {
                fcitx_log!(
                    FCITX_LOG_WARN,
                    "preset.json: missing or invalid 'codes' array"
                );
                return;
            }
        };
        let mut items: Vec<(String, usize)> = Vec::with_capacity(codes.len());
        for entry in codes {
            let text = match entry.get("text").and_then(|v| v.as_str()) {
                Some(t) => t.to_string(),
                None => continue,
            };
            let code = match entry.get("code").and_then(|v| v.as_str()) {
                Some(c) => c,
                None => continue,
            };
            items.push((text, code.chars().count()));
        }
        if items.is_empty() {
            fcitx_log!(FCITX_LOG_WARN, "preset.json: no valid entries found");
            return;
        }
        items.reverse();
        self.preset = Some(items);
    }
}

// TODO: fcitx5是否有一种通知功能，可以更好地提示错误信息？

#[derive(Clone)]
struct SenimeKeyBinding {
    sym: u32,
    modifier: u32,
    #[allow(unused)]
    modifier_only: bool,
}

impl From<(u32, u32)> for SenimeKeyBinding {
    fn from((sym, modifier): (u32, u32)) -> Self {
        Self {
            sym,
            modifier,
            modifier_only: keysym_to_states(sym) == modifier,
        }
    }
}

pub struct SenimeEngine {
    inner: Arc<RwLock<InputAnalyzer>>,
    _watcher: Option<RecommendedWatcher>,
    config: SenimeResolvedConfig,
}

fn cstr_to_str<'a>(value: *const c_char, name: &str) -> Option<&'a str> {
    if value.is_null() {
        fcitx_log!(FCITX_LOG_ERROR, "{name} is null");
        return None;
    }
    unsafe { CStr::from_ptr(value) }.to_str().map_or_else(
        |err| {
            fcitx_log!(FCITX_LOG_ERROR, "{name} is not valid utf-8: {err}");
            None
        },
        Some,
    )
}

fn into_c_string(value: String) -> *mut c_char {
    CString::new(value.replace('\0', " "))
        .expect("nul byte was removed")
        .into_raw()
}

/// 查找默认码表路径: XDG_CONFIG_HOME/senime/config.toml
fn get_default_table() -> Result<String, String> {
    use std::io::{Error, ErrorKind};

    #[cfg(target_os = "android")]
    {
        use std::path::PathBuf;
        // fcitx5-android 设置的 XDG 环境变量:
        //   XDG_DATA_HOME   = <externalFilesDir>/data   (外部存储用户数据)
        //   XDG_CONFIG_HOME = <externalFilesDir>/config  (外部存储用户配置)
        //   XDG_DATA_DIRS   = <internalDataDir>/usr/share (内部存储打包资源)
        // 其中 externalFilesDir = /storage/emulated/0/Android/data/org.fcitx.fcitx5.android/files
        // 因此 XDG_DATA_HOME 本身已包含 Android 包路径，无需再拼接。

        // 1. 优先查找外部存储的用户配置 (config.toml)
        for xdg_var in ["XDG_CONFIG_HOME", "XDG_DATA_HOME"] {
            if let Ok(dir) = std::env::var(xdg_var) {
                let path = PathBuf::from(&dir).join("senime").join("config.toml");
                if path.is_file() {
                    return Ok(path.to_str().unwrap().to_owned());
                }
            }
        }

        // 2. 查找外部存储的用户码表 (默认码表.txt)
        for xdg_var in ["XDG_DATA_HOME", "XDG_CONFIG_HOME"] {
            if let Ok(dir) = std::env::var(xdg_var) {
                let path = PathBuf::from(&dir).join("senime").join("默认码表.txt");
                if path.is_file() {
                    return Ok(path.to_str().unwrap().to_owned());
                }
            }
        }

        // 3. 查找内部存储的打包资源 (由 fcitx5-android 从 plugin assets 同步)
        if let Ok(data_dirs) = std::env::var("XDG_DATA_DIRS") {
            for dir in data_dirs.split(':') {
                if dir.is_empty() {
                    continue;
                }
                let path = PathBuf::from(dir)
                    .join("fcitx5")
                    .join("data")
                    .join("senime")
                    .join("默认码表.txt");
                if path.is_file() {
                    return Ok(path.to_str().unwrap().to_owned());
                }
            }
        }

        Err(Error::new(
            ErrorKind::NotFound,
            "未找到默认配置或码表: 请在外部存储 data/senime/ 放置 config.toml 或 默认码表.txt",
        )
        .to_string())
    }

    #[cfg(not(target_os = "android"))]
    {
        dirs::config_dir()
            .map(|dir| dir.join("senime").join("config.toml"))
            .filter(|path| path.is_file())
            .map(|path| path.to_str().unwrap().to_owned())
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::NotFound,
                    "未指定配置或码表路径，且无法找到默认配置文件路径",
                )
                .to_string()
            })
    }
}

/// 创建一个新的输入法引擎实例。
///
/// # Safety
///
/// `table_path` 必须是一个有效的、以 NUL 结尾的 C 字符串指针。
/// 如果 `table_path` 为空字符串，则尝试查找默认配置文件。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_engine_new(config: *const SenimeConfig) -> *mut SenimeEngine {
    if config.is_null() {
        fcitx_log!(FCITX_LOG_ERROR, "engine init failed: config is null");
        return ptr::null_mut();
    }
    let config = unsafe { &*config };
    let table_path = match cstr_to_str(config.table_path, "table_path") {
        Some(table_path) if !table_path.is_empty() => table_path.to_owned(),
        _ => {
            // 空字符串时尝试默认路径
            match get_default_table() {
                Ok(p) => p,
                Err(msg) => {
                    fcitx_log!(FCITX_LOG_ERROR, "failed to find table: {msg}");
                    return ptr::null_mut();
                }
            }
        }
    };
    let config: SenimeResolvedConfig = config.into();

    let result: Result<Box<SenimeEngine>, String> = (|| {
        let engine = load_input_analyzer(&table_path)?;
        let mut watch_paths = vec![table_path.clone()];
        for dict_meta in engine.dict_metas() {
            watch_paths.push(resolve_relative_path(
                Path::new(&table_path),
                &dict_meta.path,
            ));
        }
        watch_paths.dedup();
        let engine = Arc::new(RwLock::new(engine));

        let watcher_engine = engine.clone();
        let main_path = table_path.clone();
        let watcher = spawn_watcher(
            move || {
                // 1. 先从锁中取出旧引擎，用 Default 占位，释放内存
                let old = {
                    match watcher_engine.write() {
                        Ok(mut guard) => std::mem::take(&mut *guard),
                        Err(e) => {
                            fcitx_log!(FCITX_LOG_ERROR, "hot-reload lock poisoned: {e}");
                            return;
                        }
                    }
                };
                drop(old);
                // 2. 加载新引擎（旧引擎已释放，内存峰值可控）
                match load_input_analyzer(&main_path) {
                    Ok(new_ia) => {
                        match watcher_engine.write() {
                            Ok(mut guard) => {
                                *guard = new_ia;
                                fcitx_log!(FCITX_LOG_INFO, "hot-reload succeeded")
                            }
                            Err(e) => fcitx_log!(FCITX_LOG_ERROR, "hot-reload lock poisoned: {e}"),
                        };
                    }
                    Err(e) => fcitx_log!(FCITX_LOG_ERROR, "hot-reload failed: {e}"),
                }
            },
            watch_paths,
        )
        .map_err(|e| {
            fcitx_log!(FCITX_LOG_WARN, "file watcher init failed: {e}");
            e
        })
        .ok();

        Ok(Box::new(SenimeEngine {
            inner: engine,
            _watcher: watcher,
            config,
        }))
    })();
    match result {
        Ok(engine) => Box::into_raw(engine),
        Err(msg) => {
            fcitx_log!(FCITX_LOG_ERROR, "engine creation failed: {msg}");
            ptr::null_mut()
        }
    }
}

/// 释放输入法引擎实例。
///
/// # Safety
///
/// `engine` 必须是由 `senime_engine_new` 返回的有效指针，且只能释放一次。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_engine_free(engine: *mut SenimeEngine) {
    if !engine.is_null() {
        unsafe { drop(Box::from_raw(engine)) };
    }
}

/// 释放由 senime API 返回的 C 字符串。
///
/// # Safety
///
/// `value` 必须是由 senime API 分配的有效 `CString` 指针，且只能释放一次。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_string_free(value: *mut c_char) {
    if !value.is_null() {
        unsafe { drop(CString::from_raw(value)) };
    }
}

// ── Helper functions ─────────────────────────────────────────────────────

/// Convert an X11 keysym to a char. Returns None for non-printable keys.
fn keysym_to_char(sym: u32) -> Option<char> {
    // ASCII printable 或 Latin-1 supplement
    if (FCITX_KEY_space..=FCITX_KEY_asciitilde).contains(&sym)
        || (FCITX_KEY_nobreakspace..=FCITX_KEY_ydiaeresis).contains(&sym)
    {
        char::from_u32(sym)
    } else {
        None
    }
}

// ── FFI: State management ────────────────────────────────────────────────

/// 创建一个新的输入状态实例。
///
/// # Safety
///
/// `engine` 必须是由 `senime_engine_new` 返回的有效指针。
/// `key_config` 可以为 null（使用默认值）。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_state_new(engine: *const SenimeEngine) -> *mut SenimeState {
    if engine.is_null() {
        fcitx_log!(FCITX_LOG_ERROR, "senime_state_new: engine is null");
        return ptr::null_mut();
    }
    let engine = unsafe { &*engine };
    Box::into_raw(Box::new(SenimeState::new(engine.config.clone())))
}

/// 释放输入状态实例。
///
/// # Safety
///
/// `state` 必须是由 `senime_state_new` 返回的有效指针，且只能释放一次。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_state_free(state: *mut SenimeState) {
    if !state.is_null() {
        unsafe {
            (*state).clear_inner_state();
            drop(Box::from_raw(state));
        };
    }
}

/// 查询当前是否处于中文模式。
///
/// # Safety
///
/// `state` 必须是由 `senime_state_new` 返回的有效指针。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_state_chinese_mode(state: *const SenimeState) -> bool {
    if state.is_null() {
        return false;
    }
    unsafe { (*state).chinese_mode }
}

/// 设置中英文模式。
///
/// # Safety
///
/// `state` 必须是由 `senime_state_new` 返回的有效指针。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_state_set_chinese_mode(state: *mut SenimeState, chinese: bool) {
    if state.is_null() {
        return;
    }
    unsafe {
        (*state).input.clear();
        (*state).segments.clear();
        (*state).preset = None;
        (*state).chinese_mode = chinese
    };
}

/// 重置状态：清空输入缓冲，重置中英模式标记。
///
/// # Safety
///
/// `state` 必须是由 `senime_state_new` 返回的有效指针。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_state_reset(state: *mut SenimeState) {
    if state.is_null() {
        return;
    }
    unsafe { (*state).reset() };
}

/// 重置状态：清空输入缓冲，但不重置中英文模式。
///
/// # Safety
///
/// `state` 必须是由 `senime_state_new` 返回的有效指针。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_state_reset_input(state: *mut SenimeState) {
    if state.is_null() {
        return;
    }
    unsafe { (*state).reset_input() };
}

// ── FFI: Key event processing ────────────────────────────────────────────

/// 处理键盘事件，返回结果结构体（含指向 state 内部 InnerState 的指针）。
///
/// # Safety
///
/// - `engine` 必须是由 `senime_engine_new` 返回的有效指针
/// - `state` 必须是由 `senime_state_new` 返回的有效指针
/// - `key` 必须是指向有效 `SenimeKeyEvent` 的指针
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_engine_key_event(
    engine: *const SenimeEngine,
    state: *mut SenimeState,
    key: *const SenimeKeyEvent,
) -> *mut SenimeKeyEventResult {
    if engine.is_null() {
        fcitx_log!(FCITX_LOG_ERROR, "senime_engine_key_event: engine is null");
        return ptr::null_mut();
    }
    if state.is_null() {
        fcitx_log!(FCITX_LOG_ERROR, "senime_engine_key_event: state is null");
        return ptr::null_mut();
    }
    if key.is_null() {
        fcitx_log!(FCITX_LOG_ERROR, "senime_engine_key_event: key is null");
        return ptr::null_mut();
    }
    let result = catch_unwind(AssertUnwindSafe(|| {
        let engine_ref = unsafe { &*engine };
        let state = unsafe { &mut *state };
        let key = unsafe { &*key };
        let accepted = state.key_event(&engine_ref.inner, key);
        let inner_state = state.inner_state_ptr();
        Box::new(SenimeKeyEventResult {
            accepted,
            inner_state,
        })
    }));
    match result {
        Ok(result) => Box::into_raw(result),
        Err(_) => {
            fcitx_log!(
                FCITX_LOG_ERROR,
                "senime_engine_key_event: failed to process key"
            );
            ptr::null_mut()
        }
    }
}

// ── FFI: Result cleanup ──────────────────────────────────────────────────

/// 释放键盘事件结果结构体。
/// 注意：InnerState 及其字符串归 SenimeState 所有，不在此释放。
///
/// # Safety
///
/// `result` 必须是由 `senime_engine_key_event` 返回的有效指针，且只能释放一次。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_key_event_result_free(result: *mut SenimeKeyEventResult) {
    if !result.is_null() {
        unsafe { drop(Box::from_raw(result)) };
    }
}

/// 手动清空 InnerState（通常在 reset 时调用，释放上一轮分配的字符串和候选数据）。
///
/// # Safety
///
/// `state` 必须是由 `senime_state_new` 返回的有效指针。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_inner_state_clear(state: *mut SenimeState) {
    if !state.is_null() {
        unsafe { (*state).clear_inner_state() };
    }
}
#[allow(non_upper_case_globals)]
fn keysym_to_states(toggle_key: u32) -> u32 {
    match toggle_key {
        FCITX_KEY_Control_L | FCITX_KEY_Control_R => FCITX_MOD_CTRL,
        FCITX_KEY_Alt_L | FCITX_KEY_Alt_R | FCITX_KEY_Meta_L | FCITX_KEY_Meta_R => FCITX_MOD_ALT,
        FCITX_KEY_Shift_L | FCITX_KEY_Shift_R => FCITX_MOD_SHIFT,
        FCITX_KEY_Super_L | FCITX_KEY_Super_R | FCITX_KEY_Hyper_L | FCITX_KEY_Hyper_R => {
            FCITX_MOD_SUPER
        }
        _ => 0,
    }
}
// println!(
//     "key event: sym: [{}], state: [{}], is release: [{}], toggle_key: sym [{}] mod [{}] mod only [{}]",
//     key.sym,
//     key.states,
//     key.is_release,
//     toggle_key.sym,
//     toggle_key.modifier,
//     toggle_key.modifier_only
// );
// 假设shift_l是切换键，
// 当按下大写J时到结束，接收的事件为
// sym: [65505], state: [0], release: [false], toggle_key: sym [65505] mod [1] mod only [false]
// sym: [74],    state: [0], release: [false], toggle_key: sym [65505] mod [1] mod only [false]
// sym: [74],    state: [0], release: [true],  toggle_key: sym [65505] mod [1] mod only [false]
// sym: [65505], state: [1], release: [true],  toggle_key: sym [65505] mod [1] mod only [false]
// 如果在key.is_release时判断是否是trigger_key，那么本意是输出大写字母J的操作就会触发中文模式切换，这是不对的。
// 所以，当key.is_release为false时，要记录当前key的状态，用于key.is_release为true时的判断
