use arc_swap::ArcSwap;
use notify::{RecursiveMode, Watcher};
use senime_lib::{AnalysisResult, Dict, InputAnalyzer, input_analyzer::Tag, secondary_dict_path};
use std::{
    cell::RefCell,
    ffi::{CStr, CString, c_char},
    panic::{AssertUnwindSafe, catch_unwind},
    path::{Path, PathBuf},
    ptr,
    sync::{Arc, mpsc},
    time::Duration,
};

mod keysym;
use keysym::*;

// Fcitx5 modifier masks (from KeyState enum)
const FCITX_MOD_SHIFT: u32 = 0x01;
const FCITX_MOD_CAPSLOCK: u32 = 0x02;
const FCITX_MOD_ALT: u32 = 0x08;
const FCITX_MOD_NUMLOCK: u32 = 0x10;

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
            .map(|c| SenimeCandidateData {
                text: into_c_string(c.text),
                code: into_c_string(c.code),
                select_key: c.select_key as u32,
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
pub struct SenimeKeyEventResult {
    pub accepted: bool,
    pub chinese_mode: bool,
    pub commands: *mut SenimeCommand,
    pub command_count: usize,
}

// ── Rust-side state ──────────────────────────────────────────────────────

pub struct SenimeState {
    engine: Arc<ArcSwap<InputAnalyzer>>,
    input: String,
    chinese_mode: bool,
}

impl SenimeState {
    fn new(engine: Arc<ArcSwap<InputAnalyzer>>) -> Self {
        Self {
            engine,
            input: String::new(),
            chinese_mode: false,
        }
    }

    /// Process a key event. Returns (accepted, commands).
    fn key_event(&mut self, key: &SenimeKeyEvent) -> (bool, Vec<SenimeCommand>) {
        if key.is_release {
            return (false, Vec::new());
        }

        let sym = key.sym;
        let states = key.states;

        // Alt+J: toggle Chinese mode
        if sym == FCITX_KEY_J && (states & FCITX_MOD_ALT) != 0 {
            let mut cmds = Vec::new();
            if self.chinese_mode {
                cmds.push(SenimeCommand::with_commit_text(self.input.clone()));
                cmds.push(SenimeCommand::with_type(SenimeCommandType::ResetInputPanel));
                self.chinese_mode = false;
                self.input.clear();
            } else {
                cmds.push(SenimeCommand::with_preedit_text(":(中)".to_string()));
                self.chinese_mode = true;
            }
            cmds.push(SenimeCommand::with_type(SenimeCommandType::UpdateUI));
            cmds.push(SenimeCommand::with_type(
                SenimeCommandType::UpdateStatusArea,
            ));
            return (true, cmds);
        }

        // English mode handling
        if !self.chinese_mode {
            if self.input.starts_with('`') {
                return self.chinese_mode(sym, states, true);
            } else if sym == FCITX_KEY_quoteleft {
                self.input.push('`');
                let cmds = vec![
                    SenimeCommand::with_preedit_text(":(中)".to_string()),
                    SenimeCommand::with_type(SenimeCommandType::UpdateUI),
                ];
                return (true, cmds);
            };
            return (false, vec![]);
        }

        // Chinese mode handling
        self.chinese_mode(sym, states, false)
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

        // Escape / Return
        if sym == FCITX_KEY_Escape || sym == FCITX_KEY_Return {
            // 空输入时提交空字符串，非空时提交当前输入
            let cmds = vec![
                SenimeCommand::with_commit_text(self.input.clone()),
                SenimeCommand::with_type(SenimeCommandType::ResetInputPanel),
                SenimeCommand::with_type(SenimeCommandType::UpdateUI),
                SenimeCommand::with_type(SenimeCommandType::UpdateStatusArea),
            ];
            if sym == FCITX_KEY_Escape {
                self.chinese_mode = false;
            }
            self.input.clear();
            return (sym != FCITX_KEY_Return, cmds);
        }

        // Backspace
        if sym == FCITX_KEY_BackSpace {
            let mut accept = false;
            if !self.input.is_empty() {
                self.input.pop();
                accept = true;
            }
            let mut cmds = Vec::new();
            self.do_update(temp_chinese_mode, &mut cmds);
            return (accept, cmds);
        }

        // All other keys → append and analyze
        if let Some(ch) = keysym_to_char(sym) {
            self.input.push(ch);
            let mut cmds = Vec::new();
            self.do_update(temp_chinese_mode, &mut cmds);
            return (true, cmds);
        }

        (false, Vec::new())
    }

    /// Core update: analyze input and produce commands.
    fn do_update(&mut self, temp_chinese_mode: bool, cmds: &mut Vec<SenimeCommand>) {
        let chars: Vec<char> = if temp_chinese_mode {
            self.input.chars().filter(|&c| c != '`').collect()
        } else {
            self.input.chars().collect()
        };
        if chars.is_empty() {
            if self.input.len() == 2 {
                cmds.push(SenimeCommand::with_commit_text("`".to_string()));
                self.input.clear();
            } else {
                cmds.push(SenimeCommand::with_preedit_text("".to_string()));
            }
            cmds.push(SenimeCommand::with_type(SenimeCommandType::ResetInputPanel));
            cmds.push(SenimeCommand::with_type(SenimeCommandType::UpdateUI));
            return;
        }
        // 先 load() 获取 guard，分析后 drop guard，再调用 &self 方法
        let (pre_text, last_text, last_input, last_tag, candidates) = {
            let guard = self.engine.load();
            let AnalysisResult {
                mut segments,
                candidates,
            } = guard.analyze(&chars);
            let (last_text, last_input, last_tag) = segments.pop().map_or(
                ("".to_string(), "".to_string(), Tag::Unknown),
                |(text, origin, tag)| (text, origin.into_iter().collect(), tag),
            );
            let pre_text: String = segments.into_iter().map(|seg| seg.0).collect();
            (pre_text, last_text, last_input, last_tag, candidates)
        };
        // println!(
        //     "pre_text: [{pre_text}] last_text: [{last_text}] last_input: [{last_input}] last_tag: [{last_tag:?}] candidates: {}",
        //     candidates.as_ref().map_or(0, |cands| cands.len())
        // );
        if !temp_chinese_mode {
            // 正常中文模式
            // 如果senime输出了多segment，则将之前的segments的文本作为commit
            if !pre_text.is_empty() {
                self.input = last_input.clone();
                cmds.push(SenimeCommand::with_commit_text(pre_text));
            }
            if let Some(cands) = candidates
                && !cands.is_empty()
            {
                cmds.push(SenimeCommand::with_preedit_text(last_text));
                cmds.push(SenimeCommand::with_candidates(cands));
            } else {
                cmds.push(SenimeCommand::with_type(SenimeCommandType::ResetInputPanel));
                // 无候选，且Escape还未结束
                if let Tag::Escape((_, escape_end)) = last_tag
                    && (last_input.len() < 2 || !last_input.ends_with(escape_end))
                {
                    cmds.push(SenimeCommand::with_preedit_text(last_text));
                } else {
                    self.input.clear();
                    cmds.push(SenimeCommand::with_commit_text(last_text));
                }
            }
            cmds.push(SenimeCommand::with_type(SenimeCommandType::UpdateUI));
        } else {
            // 临时中文模式
            if self.input.ends_with('`') {
                // 临时中文模式结束
                self.input.clear();
                cmds.push(SenimeCommand::with_commit_text(
                    pre_text + last_text.as_str(),
                ));
                cmds.push(SenimeCommand::with_type(SenimeCommandType::ResetInputPanel));
            } else {
                if let Some(cands) = candidates {
                    cmds.push(SenimeCommand::with_candidates(cands));
                } else {
                    cmds.push(SenimeCommand::with_type(SenimeCommandType::ResetInputPanel));
                }
                let text = pre_text + last_text.as_str();
                cmds.push(SenimeCommand::with_preedit_text(text));
            }
            cmds.push(SenimeCommand::with_type(SenimeCommandType::UpdateUI));
        }
    }
}

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

pub struct SenimeEngine {
    inner: Arc<ArcSwap<InputAnalyzer>>,
    _watcher: Option<notify::RecommendedWatcher>,
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

/// Build a new engine inner from the given table path.
fn build_engine(table_path: &str) -> Result<InputAnalyzer, String> {
    let dict = Dict::try_load(table_path)?;
    let reverse_dict = dict.config().reverse_dict.as_ref().map(|sec_table_path| {
        let hint = PathBuf::from(sec_table_path)
            .file_name()
            .and_then(|name| name.to_str().map(|n| n.chars().take(1).collect::<String>()))
            .unwrap_or("反".to_string());
        (
            Dict::load(secondary_dict_path(table_path, sec_table_path)),
            hint,
        )
    });
    Ok(InputAnalyzer::new(dict, reverse_dict))
}

/// 查找默认码表路径: XDG_CONFIG_HOME/senime/config.toml
fn get_default_table() -> Result<String, String> {
    use std::io::{Error, ErrorKind};
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
            match build_engine(&main_path) {
                Ok(new_inner) => {
                    inner.swap(Arc::new(new_inner));
                }
                Err(e) => {
                    eprintln!("[senime] hot-reload failed: {e}");
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
pub unsafe extern "C" fn senime_engine_new(table_path: *const c_char) -> *mut SenimeEngine {
    clear_last_error();
    let Some(table_path) = cstr_to_str(table_path, "table_path") else {
        return ptr::null_mut();
    };
    // 空字符串时尝试默认路径
    let table_path = if table_path.is_empty() {
        match get_default_table() {
            Ok(p) => p,
            Err(msg) => {
                set_last_error(msg);
                return ptr::null_mut();
            }
        }
    } else {
        table_path.to_string()
    };
    let result: Result<Box<SenimeEngine>, String> = (|| {
        let engine = build_engine(&table_path)?;
        let mut watch_paths = vec![table_path];
        if let Some(dict_path) = engine.get_dict().config().dict.as_ref() {
            watch_paths.push(dict_path.to_owned());
        }
        if let Some(sec_dict_path) = engine.get_dict().config().reverse_dict.as_ref() {
            watch_paths.push(sec_dict_path.to_owned());
        }
        watch_paths.dedup();
        let engine = Arc::new(ArcSwap::from_pointee(engine));

        // Spawn file watcher — failure is non-fatal (engine works without hot-reload).
        let watcher = spawn_watcher(engine.clone(), watch_paths)
            .map_err(|e| {
                eprintln!("[senime] file watcher init failed: {e}");
                e
            })
            .ok();

        Ok(Box::new(SenimeEngine {
            inner: engine,
            _watcher: watcher,
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn senime_state_new(engine: *const SenimeEngine) -> *mut SenimeState {
    clear_last_error();
    if engine.is_null() {
        set_last_error("engine is null");
        return ptr::null_mut();
    }
    let engine = unsafe { &*engine };
    Box::into_raw(Box::new(SenimeState::new(engine.inner.clone())))
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
        let chinese_mode = state.chinese_mode;
        // 使用 into_boxed_slice 确保 capacity == len，避免 Vec::from_raw_parts 的安全隐患
        let boxed = commands.into_boxed_slice();
        let count = boxed.len();
        let cmd_ptr = if count > 0 {
            Box::into_raw(boxed) as *mut SenimeCommand
        } else {
            drop(boxed);
            ptr::null_mut()
        };
        Box::new(SenimeKeyEventResult {
            accepted,
            chinese_mode,
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
