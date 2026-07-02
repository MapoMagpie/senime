use arc_swap::ArcSwap;
use notify::{RecursiveMode, Watcher};
use senime_lib::{
    AnalysisResult, InputAnalyzer, PAGE_DOWN, PAGE_UP, input_analyzer::load_input_analyzer,
    resolve_relative_path,
};
use std::{
    cell::RefCell,
    ffi::{CStr, CString, c_char},
    panic::{AssertUnwindSafe, catch_unwind},
    path::Path,
    ptr,
    sync::{Arc, mpsc},
    time::Duration,
};

mod keysym;
use keysym::*;

// Android logcat support
#[cfg(target_os = "android")]
unsafe extern "C" {
    fn __android_log_print(prio: i32, tag: *const c_char, fmt: *const c_char, ...) -> i32;
}

#[cfg(target_os = "android")]
macro_rules! android_log {
    ($prio:expr, $($arg:tt)*) => {{
        let msg = std::ffi::CString::new(format!($($arg)*)).unwrap_or_default();
        let tag = std::ffi::CString::new("senime").unwrap_or_default();
        unsafe {
            __android_log_print($prio, tag.as_ptr(), msg.as_ptr());
        }
    }};
}

#[cfg(target_os = "android")]
macro_rules! log_warn {
    ($($arg:tt)*) => { android_log!(5i32, $($arg)*) };  // ANDROID_LOG_WARN = 5
}

macro_rules! senime_warn {
    ($($arg:tt)*) => {{
        #[cfg(target_os = "android")]
        { log_warn!($($arg)*); }
        #[cfg(not(target_os = "android"))]
        { eprintln!($($arg)*); }
    }};
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

// ── FFI types for command-based key event processing ──────────────────────

#[repr(C)]
pub enum SenimeCommandType {
    CommitText,
    SetPreedit,
    SetCandidates,
    /// 重置InputPanel，但仍需要更新`UI`
    ResetInputPanel,
    /// 更新`UI InputPanel`
    UpdateUI,
    UpdateStatusArea,
}

#[repr(C)]
pub struct SenimeCommand {
    pub type_: SenimeCommandType,
    pub text: *mut c_char,
    pub candidates: *mut SenimeCandidateData,
    pub candidate_count: usize,
}

impl SenimeCommand {
    fn with_type(type_: SenimeCommandType) -> Self {
        Self {
            type_,
            text: ptr::null_mut(),
            candidates: ptr::null_mut(),
            candidate_count: 0,
        }
    }
    fn with_commit_text(text: String) -> Self {
        Self {
            type_: SenimeCommandType::CommitText,
            text: into_c_string(text),
            candidates: ptr::null_mut(),
            candidate_count: 0,
        }
    }
    fn with_preedit_text(text: String) -> Self {
        Self {
            type_: SenimeCommandType::SetPreedit,
            text: into_c_string(text),
            candidates: ptr::null_mut(),
            candidate_count: 0,
        }
    }

    fn with_candidates(cands: Vec<senime_lib::CandidateRich>) -> SenimeCommand {
        let data: Vec<SenimeCandidateData> = cands
            .into_iter()
            .map(|c| {
                // 只展示用户尚未输入的编码部分
                let origin: String = c.origin.iter().collect();
                let remaining = c.code.strip_prefix(&origin).unwrap_or(&c.code);
                SenimeCandidateData {
                    text: into_c_string(c.text),
                    code: into_c_string(remaining.to_owned()),
                    select_key: c.select_key as u32,
                }
            })
            .collect();
        // 使用 into_boxed_slice 确保 capacity == len，避免 Vec::from_raw_parts 的安全隐患
        let boxed = data.into_boxed_slice();
        let count = boxed.len();
        let ptr = Box::into_raw(boxed) as *mut SenimeCandidateData;
        SenimeCommand {
            type_: SenimeCommandType::SetCandidates,
            text: ptr::null_mut(),
            candidates: ptr,
            candidate_count: count,
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
    pub trigger_sym: u32,
    pub trigger_states: u32,
    pub table_path: *mut c_char,
    pub default_chinese_mode: bool,
}

#[derive(Clone)]
struct SenimeResolvedConfig {
    toggle_key: SenimeKeyBinding,
    /// 当为 None 时，禁用临时中文模式
    trigger_char: Option<char>,
    default_chinese_mode: bool,
}

impl Default for SenimeResolvedConfig {
    fn default() -> Self {
        Self {
            toggle_key: SenimeKeyBinding::from((FCITX_KEY_Shift_L, FCITX_MOD_SHIFT)),
            trigger_char: None,
            default_chinese_mode: false,
        }
    }
}

impl From<&SenimeConfig> for SenimeResolvedConfig {
    fn from(value: &SenimeConfig) -> Self {
        Self {
            toggle_key: (value.toggle_sym, value.toggle_states).into(),
            trigger_char: keysym_to_char(value.trigger_sym),
            default_chinese_mode: value.default_chinese_mode,
        }
    }
}

#[repr(C)]
pub struct SenimeKeyEventResult {
    pub accepted: bool,
    pub commands: *mut SenimeCommand,
    pub command_count: usize,
}

// ── Rust-side state ──────────────────────────────────────────────────────

pub struct SenimeState {
    engine: Arc<ArcSwap<InputAnalyzer>>,
    input: String,
    chinese_mode: bool,
    last_unrelease_key: u32,
    config: SenimeResolvedConfig,
}

impl SenimeState {
    fn new(engine: Arc<ArcSwap<InputAnalyzer>>, config: SenimeResolvedConfig) -> Self {
        let chinese_mode = config.default_chinese_mode;
        Self {
            engine,
            input: String::new(),
            chinese_mode,
            last_unrelease_key: 0,
            config,
        }
    }

    /// 重置状态：清空输入缓冲，重置中英模式标记。
    fn reset(&mut self) {
        self.input.clear();
        self.chinese_mode = self.config.default_chinese_mode;
    }

    /// 重置输入缓冲，但保留中英模式标记。
    /// 用于手动选择候选项后清空输入状态。
    fn reset_input(&mut self) {
        self.input.clear();
    }

    /// Process a key event. Returns (accepted, commands).
    fn key_event(&mut self, key: &SenimeKeyEvent) -> (bool, Vec<SenimeCommand>) {
        let toggle_key = &self.config.toggle_key;
        let mut toggle_chinese_mode =
            key.sym == toggle_key.sym && key.states == toggle_key.modifier;
        if key.is_release {
            toggle_chinese_mode = toggle_key.modifier_only
                && self.last_unrelease_key == key.sym
                && toggle_chinese_mode;
            if !toggle_chinese_mode {
                return (false, vec![]);
            }
        }
        self.last_unrelease_key = key.sym;
        if toggle_chinese_mode {
            let mut cmds = Vec::new();
            if self.chinese_mode {
                cmds.push(SenimeCommand::with_commit_text(self.input.clone()));
                self.chinese_mode = false;
                self.input.clear();
            } else {
                cmds.push(SenimeCommand::with_preedit_text(":中>".to_string()));
                self.chinese_mode = true;
            }
            cmds.push(SenimeCommand::with_type(SenimeCommandType::ResetInputPanel));
            cmds.push(SenimeCommand::with_type(
                SenimeCommandType::UpdateStatusArea,
            ));
            cmds.push(SenimeCommand::with_type(SenimeCommandType::UpdateUI));
            // !key.is_release防止下级应用接收到此key的按下事件，但释放事件却被fcitx5拦截，导致该key一直repeat。
            return (!key.is_release, cmds);
        }
        // 英文模式处理
        if !self.chinese_mode {
            if let Some(trigger_char) = self.config.trigger_char {
                // 已进入临时中文模式（输入缓冲以触发字符开头）
                if self.input.starts_with(trigger_char) {
                    return self.chinese_mode(key.sym, key.states, true);
                // 按下触发键，进入临时中文模式
                } else if let Some(ch) = keysym_to_char(key.sym)
                    && ch == trigger_char
                {
                    self.input.push(ch);
                    let cmds = vec![
                        SenimeCommand::with_preedit_text(":(中)".to_string()),
                        SenimeCommand::with_type(SenimeCommandType::UpdateUI),
                    ];
                    return (true, cmds);
                };
            }
            return (false, vec![]);
        }

        // Chinese mode handling
        self.chinese_mode(key.sym, key.states, false)
    }

    fn chinese_mode(
        &mut self,
        sym: u32,
        states: u32,
        temp_chinese_mode: bool,
    ) -> (bool, Vec<SenimeCommand>) {
        // Non-shift modifiers: commit pending, forward key
        let non_shift_mods = states & !(FCITX_MOD_SHIFT | FCITX_MOD_CAPSLOCK | FCITX_MOD_NUMLOCK);
        if non_shift_mods != 0 {
            let mut cmds = Vec::new();
            if !self.input.is_empty() {
                cmds.push(SenimeCommand::with_commit_text(self.input.clone()));
                cmds.push(SenimeCommand::with_type(SenimeCommandType::ResetInputPanel));
            }
            return (false, cmds);
        }

        // Escape
        if sym == FCITX_KEY_Escape {
            let cmds = vec![
                SenimeCommand::with_commit_text(self.input.clone()),
                SenimeCommand::with_type(SenimeCommandType::ResetInputPanel),
                SenimeCommand::with_type(SenimeCommandType::UpdateUI),
                SenimeCommand::with_type(SenimeCommandType::UpdateStatusArea),
            ];
            self.chinese_mode = false;
            self.input.clear();
            return (true, cmds);
        }

        // Return → 分析后直接提交中文
        if sym == FCITX_KEY_Return {
            let mut cmds = Vec::new();
            self.do_update(temp_chinese_mode, true, &mut cmds);
            return (false, cmds);
        }

        // Backspace
        if sym == FCITX_KEY_BackSpace {
            let mut accept = false;
            // remove ⇞ and ⇟ from self.input first
            if !self.input.is_empty() {
                while let Some(ch) = self.input.pop() {
                    if ch != PAGE_UP && ch != PAGE_DOWN {
                        break;
                    }
                }
                accept = true;
            }
            let mut cmds = Vec::new();
            self.do_update(temp_chinese_mode, false, &mut cmds);
            return (accept, cmds);
        }

        // PageUp / PageDown → 翻页（仅在有输入时生效）
        if (sym == FCITX_KEY_Page_Up || sym == FCITX_KEY_KP_Page_Up) && !self.input.is_empty() {
            self.input.push(PAGE_UP); // ⇞
            let mut cmds = Vec::new();
            self.do_update(temp_chinese_mode, false, &mut cmds);
            return (true, cmds);
        }
        if (sym == FCITX_KEY_Page_Down || sym == FCITX_KEY_KP_Page_Down) && !self.input.is_empty() {
            self.input.push(PAGE_DOWN); // ⇟
            let mut cmds = Vec::new();
            self.do_update(temp_chinese_mode, false, &mut cmds);
            return (true, cmds);
        }

        // All other keys → append and analyze
        if let Some(ch) = keysym_to_char(sym) {
            self.input.push(ch);
            let mut cmds = Vec::new();
            self.do_update(temp_chinese_mode, false, &mut cmds);
            return (true, cmds);
        }

        (false, Vec::new())
    }

    /// Core update: analyze input and produce commands.
    fn do_update(
        &mut self,
        temp_chinese_mode: bool,
        just_commit: bool,
        cmds: &mut Vec<SenimeCommand>,
    ) {
        let chars: Vec<char> = if temp_chinese_mode {
            self.input
                .chars()
                .filter(|&c| self.config.trigger_char.is_none_or(|tc| c != tc))
                .collect()
        } else {
            self.input.chars().collect()
        };
        if chars.is_empty() {
            if self.input.len() == 2 {
                cmds.push(SenimeCommand::with_commit_text(
                    self.input.split_at(1).0.to_string(),
                ));
                self.input.clear();
            } else {
                cmds.push(SenimeCommand::with_preedit_text("".to_string()));
            }
            cmds.push(SenimeCommand::with_type(SenimeCommandType::ResetInputPanel));
            cmds.push(SenimeCommand::with_type(SenimeCommandType::UpdateUI));
            return;
        }
        // 先 load() 获取 guard，分析后 drop guard，再调用 &self 方法
        let (pre_text, last_text, last_input, candidates, pending) = {
            let guard = self.engine.load();
            let AnalysisResult {
                mut segments,
                pending,
                candidates,
            } = guard.analyze(&chars);
            let (last_text, last_input) = segments
                .pop()
                .map_or(("".to_string(), "".to_string()), |(text, origin, _)| {
                    (text, origin.into_iter().collect())
                });
            let pre_text: String = segments.into_iter().map(|seg| seg.0).collect();
            (pre_text, last_text, last_input, candidates, pending)
        };
        // println!(
        //     "pre_text: [{pre_text}] last_text: [{last_text}] last_input: [{last_input}] last_tag: [{last_tag:?}] candidates: {}",
        //     candidates.as_ref().map_or(0, |cands| cands.len())
        // );
        if just_commit {
            self.input.clear();
            cmds.push(SenimeCommand::with_commit_text(
                pre_text + last_text.as_str(),
            ));
            cmds.push(SenimeCommand::with_type(SenimeCommandType::ResetInputPanel));
            cmds.push(SenimeCommand::with_type(SenimeCommandType::UpdateUI));
            return;
        }
        if !temp_chinese_mode {
            // 正常中文模式
            // 如果senime输出了多segment，则将之前的segments的文本作为commit
            if !pre_text.is_empty() {
                cmds.push(SenimeCommand::with_commit_text(pre_text));
            }
            if pending {
                cmds.push(SenimeCommand::with_preedit_text(last_text));
                if let Some(cands) = candidates {
                    cmds.push(SenimeCommand::with_candidates(cands));
                } else {
                    cmds.push(SenimeCommand::with_type(SenimeCommandType::ResetInputPanel));
                }
                self.input = last_input.clone();
            } else {
                cmds.push(SenimeCommand::with_commit_text(last_text));
                cmds.push(SenimeCommand::with_type(SenimeCommandType::ResetInputPanel));
                self.input.clear();
            }
        } else {
            // 临时中文模式
            if self
                .config
                .trigger_char
                .is_some_and(|tc| self.input.ends_with(tc))
            {
                // 临时中文模式结束
                cmds.push(SenimeCommand::with_commit_text(
                    pre_text + last_text.as_str(),
                ));
                cmds.push(SenimeCommand::with_type(SenimeCommandType::ResetInputPanel));
                self.input.clear();
            } else {
                // 临时中文模式未决
                let text = pre_text + last_text.as_str();
                cmds.push(SenimeCommand::with_preedit_text(text));
                if pending {
                    if let Some(cands) = candidates {
                        cmds.push(SenimeCommand::with_candidates(cands));
                    } else {
                        cmds.push(SenimeCommand::with_type(SenimeCommandType::ResetInputPanel));
                    }
                } else {
                    cmds.push(SenimeCommand::with_type(SenimeCommandType::ResetInputPanel));
                }
            }
        }
        cmds.push(SenimeCommand::with_type(SenimeCommandType::UpdateUI));
    }
}

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

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
    inner: Arc<ArcSwap<InputAnalyzer>>,
    _watcher: Option<notify::RecommendedWatcher>,
    config: SenimeResolvedConfig,
}

fn set_last_error(err: impl ToString) {
    let sanitized = err.to_string().replace('\0', " ");
    LAST_ERROR.with(|last| {
        *last.borrow_mut() = CString::new(sanitized).ok();
    });
}

fn clear_last_error() {
    LAST_ERROR.with(|last| {
        *last.borrow_mut() = None;
    });
}

fn cstr_to_str<'a>(value: *const c_char, name: &str) -> Option<&'a str> {
    if value.is_null() {
        set_last_error(format!("{name} is null"));
        return None;
    }
    unsafe { CStr::from_ptr(value) }.to_str().map_or_else(
        |err| {
            set_last_error(format!("{name} is not valid utf-8: {err}"));
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

/// Spawn a background file watcher that rebuilds the engine on changes.
fn spawn_watcher(
    inner: Arc<ArcSwap<InputAnalyzer>>,
    paths: Vec<String>,
) -> notify::Result<notify::RecommendedWatcher> {
    let (tx, rx) = mpsc::channel();

    // Create the filesystem watcher — events go through the channel.
    let mut watcher = notify::recommended_watcher(tx)?;
    let main_path = paths[0].clone();
    for path in paths {
        watcher.watch(Path::new(&path), RecursiveMode::NonRecursive)?;
    }

    // Debounce thread: drain events, wait, then rebuild.
    std::thread::spawn(move || {
        while rx.recv().is_ok() {
            // Drain any queued events (batch rapid-fire notifications).
            while rx.try_recv().is_ok() {}

            // Check if any event touches a file we care about.
            // (We drain above without inspecting — just rebuild on any event
            //  in the watched directories. The directories are chosen to be
            //  the parent dirs of our target files, so this is precise enough.)
            std::thread::sleep(Duration::from_millis(200));

            // Re-read the filter: events may have been for unrelated files.
            // Since we watch narrow directories (parents of our files),
            // just rebuild unconditionally — it's fast enough.
            match load_input_analyzer(&main_path) {
                Ok(new_inner) => {
                    inner.swap(Arc::new(new_inner));
                }
                Err(e) => {
                    senime_warn!("[senime] hot-reload failed: {e}");
                }
            }
        }
    });

    Ok(watcher)
}

/// 创建一个新的输入法引擎实例。
///
/// # Safety
///
/// `table_path` 必须是一个有效的、以 NUL 结尾的 C 字符串指针。
/// 如果 `table_path` 为空字符串，则尝试查找默认配置文件。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_engine_new(config: *const SenimeConfig) -> *mut SenimeEngine {
    clear_last_error();
    if config.is_null() {
        set_last_error("初始化引擎失败，传入的配置为空。");
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
                    set_last_error(msg);
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
        let engine = Arc::new(ArcSwap::from_pointee(engine));

        // Spawn file watcher — failure is non-fatal (engine works without hot-reload).
        let watcher = spawn_watcher(engine.clone(), watch_paths)
            .map_err(|e| {
                senime_warn!("[senime] file watcher init failed: {e}");
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
            set_last_error(msg);
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

/// 获取最后一次操作的错误信息。
///
/// # Safety
///
/// 返回的指针在线程局部存储中有效，直到下一次调用 senime API。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_last_error() -> *const c_char {
    LAST_ERROR.with(|last| {
        last.borrow()
            .as_ref()
            .map(|err| err.as_ptr())
            .unwrap_or(ptr::null())
    })
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
    clear_last_error();
    if engine.is_null() {
        set_last_error("engine is null");
        return ptr::null_mut();
    }
    let engine = unsafe { &*engine };
    Box::into_raw(Box::new(SenimeState::new(
        engine.inner.clone(),
        engine.config.clone(),
    )))
}

/// 释放输入状态实例。
///
/// # Safety
///
/// `state` 必须是由 `senime_state_new` 返回的有效指针，且只能释放一次。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_state_free(state: *mut SenimeState) {
    if !state.is_null() {
        unsafe { drop(Box::from_raw(state)) };
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
    unsafe { (*state).chinese_mode = chinese };
}

/// 重置输入状态：清空输入缓冲并重置中英模式。
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

/// 重置输入缓冲，但保留中英模式标记。
/// 用于手动选择候选项后清空输入状态。
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

/// 处理键盘事件，返回操作结果和命令列表。
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
    clear_last_error();
    if engine.is_null() {
        set_last_error("engine is null");
        return ptr::null_mut();
    }
    if state.is_null() {
        set_last_error("state is null");
        return ptr::null_mut();
    }
    if key.is_null() {
        set_last_error("key is null");
        return ptr::null_mut();
    }
    let result = catch_unwind(AssertUnwindSafe(|| {
        let state = unsafe { &mut *state };
        let key = unsafe { &*key };
        let (accepted, commands) = state.key_event(key);
        // 使用 into_boxed_slice 确保 capacity == len，避免 Vec::from_raw_parts 的安全隐患
        let (cmd_ptr, count) = if commands.is_empty() {
            (ptr::null_mut(), 0)
        } else {
            let boxed = commands.into_boxed_slice();
            let count = boxed.len();
            (Box::into_raw(boxed) as *mut SenimeCommand, count)
        };
        Box::new(SenimeKeyEventResult {
            accepted,
            commands: cmd_ptr,
            command_count: count,
        })
    }));
    match result {
        Ok(result) => Box::into_raw(result),
        Err(_) => {
            set_last_error("failed to process key");
            ptr::null_mut()
        }
    }
}

// ── FFI: Result cleanup ──────────────────────────────────────────────────

/// 释放键盘事件结果及其中的所有命令和候选数据。
///
/// # Safety
///
/// `result` 必须是由 `senime_engine_key_event` 返回的有效指针，且只能释放一次。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_key_event_result_free(result: *mut SenimeKeyEventResult) {
    if result.is_null() {
        return;
    }
    unsafe {
        let result = Box::from_raw(result);
        if !result.commands.is_null() && result.command_count > 0 {
            // 安全: commands 由 into_boxed_slice + Box::into_raw 产生
            let commands = Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                result.commands,
                result.command_count,
            ));
            for cmd in commands.iter() {
                senime_string_free(cmd.text);
                if !cmd.candidates.is_null() && cmd.candidate_count > 0 {
                    // 安全: candidates 由 into_boxed_slice + Box::into_raw 产生
                    let candidates = Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                        cmd.candidates,
                        cmd.candidate_count,
                    ));
                    for cand in candidates.iter() {
                        senime_string_free(cand.text);
                        senime_string_free(cand.code);
                    }
                }
            }
        }
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
