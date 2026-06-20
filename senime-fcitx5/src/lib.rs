use arc_swap::ArcSwap;
use notify::{RecursiveMode, Watcher};
use senime_lib::{AnalysisResult, Dict, InputAnalyzer, input_analyzer::Tag, secondary_dict_path};
use std::{
    cell::RefCell,
    collections::HashSet,
    ffi::{CStr, CString, c_char},
    panic::{AssertUnwindSafe, catch_unwind},
    path::PathBuf,
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
        let mut data: Vec<SenimeCandidateData> = cands
            .into_iter()
            .map(|c| SenimeCandidateData {
                text: into_c_string(c.text),
                code: into_c_string(c.code),
                select_key: c.select_key as u32,
            })
            .collect();
        let count = data.len();
        let ptr = data.as_mut_ptr();
        std::mem::forget(data);
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

        // Escape
        if sym == FCITX_KEY_Escape || sym == FCITX_KEY_Return {
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
                remove_last_utf8_char(&mut self.input);
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
        // Drop guard before calling &self methods
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
    let reverse_dict = dict.config().reverse_dict.as_ref().map(|path| {
        let hint = PathBuf::from(path)
            .file_name()
            .and_then(|name| name.to_str().map(|n| n.chars().take(1).collect::<String>()))
            .unwrap_or("反".to_string());
        (Dict::load(secondary_dict_path(table_path, path)), hint)
    });
    Ok(InputAnalyzer::new(dict, reverse_dict))
}

/// Collect all file paths that should be watched for changes.
fn collect_watch_paths(table_path: &str, dict: &Dict) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let table = PathBuf::from(table_path);
    paths.push(table.clone());

    // If loaded from .toml, also watch the resolved .txt path
    if let Some(dict_name) = &dict.config().dict {
        let resolved = secondary_dict_path(table_path, dict_name);
        if resolved != table {
            paths.push(resolved);
        }
    }

    // Watch reverse dict if configured
    if let Some(sec_name) = &dict.config().reverse_dict {
        let resolved = secondary_dict_path(table_path, sec_name);
        if resolved != table {
            paths.push(resolved);
        }
    }

    paths.sort();
    paths.dedup();
    paths
}

/// Spawn a background file watcher that rebuilds the engine on changes.
fn spawn_watcher(
    inner: Arc<ArcSwap<InputAnalyzer>>,
    table_path: String,
    watch_paths: Vec<PathBuf>,
) -> notify::Result<notify::RecommendedWatcher> {
    // Collect the parent directories to watch (handles vim-style atomic replace via rename).
    let watch_dirs: HashSet<PathBuf> = watch_paths
        .iter()
        .filter_map(|p| p.parent().map(PathBuf::from))
        .collect();

    let (tx, rx) = mpsc::channel();

    // Create the filesystem watcher — events go through the channel.
    let mut watcher = notify::recommended_watcher(tx)?;

    // Watch parent directories (handles vim-style atomic replace via rename).
    for dir in &watch_dirs {
        watcher.watch(dir, RecursiveMode::NonRecursive)?;
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
            match build_engine(&table_path) {
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

#[unsafe(no_mangle)]
pub extern "C" fn senime_engine_new(table_path: *const c_char) -> *mut SenimeEngine {
    clear_last_error();
    let Some(table_path) = cstr_to_str(table_path, "table_path") else {
        return ptr::null_mut();
    };
    let result: Result<Box<SenimeEngine>, String> = (|| {
        let engine = build_engine(table_path)?;
        let watch_paths = collect_watch_paths(table_path, engine.get_dict());
        let engine = Arc::new(ArcSwap::from_pointee(engine));

        // Spawn file watcher — failure is non-fatal (engine works without hot-reload).
        let watcher = spawn_watcher(engine.clone(), table_path.to_string(), watch_paths)
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

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn senime_engine_free(engine: *mut SenimeEngine) {
    if !engine.is_null() {
        unsafe {
            drop(Box::from_raw(engine));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn senime_last_error() -> *const c_char {
    LAST_ERROR.with(|last| {
        last.borrow()
            .as_ref()
            .map(|err| err.as_ptr())
            .unwrap_or(ptr::null())
    })
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn senime_string_free(value: *mut c_char) {
    if !value.is_null() {
        unsafe {
            drop(CString::from_raw(value));
        }
    }
}

// ── Helper functions ─────────────────────────────────────────────────────

/// Convert an X11 keysym to a char. Returns None for non-printable keys.
fn keysym_to_char(sym: u32) -> Option<char> {
    // ASCII printable range
    if (FCITX_KEY_space..=FCITX_KEY_asciitilde).contains(&sym) {
        char::from_u32(sym)
    } else {
        // Latin-1 supplement
        if (FCITX_KEY_nobreakspace..=FCITX_KEY_ydiaeresis).contains(&sym) {
            char::from_u32(sym)
        } else {
            None
        }
    }
}

/// Remove the last UTF-8 character from a string.
fn remove_last_utf8_char(s: &mut String) {
    if let Some(last_byte_pos) = s.char_indices().next_back().map(|(pos, _)| pos) {
        s.truncate(last_byte_pos);
    }
}

// ── FFI: State management ────────────────────────────────────────────────

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn senime_state_new(engine: *const SenimeEngine) -> *mut SenimeState {
    clear_last_error();
    if engine.is_null() {
        set_last_error("engine is null");
        return ptr::null_mut();
    }
    let engine = unsafe { &*engine };
    Box::into_raw(Box::new(SenimeState::new(engine.inner.clone())))
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn senime_state_free(state: *mut SenimeState) {
    if !state.is_null() {
        unsafe {
            drop(Box::from_raw(state));
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn senime_state_chinese_mode(state: *const SenimeState) -> bool {
    if state.is_null() {
        return false;
    }
    unsafe { &*state }.chinese_mode
}

// ── FFI: Key event processing ────────────────────────────────────────────

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn senime_engine_key_event(
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
        let mut commands = commands;
        let count = commands.len();
        let cmd_ptr = if count > 0 {
            let ptr = commands.as_mut_ptr();
            std::mem::forget(commands);
            ptr
        } else {
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

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn senime_key_event_result_free(result: *mut SenimeKeyEventResult) {
    if result.is_null() {
        return;
    }
    unsafe {
        let result = Box::from_raw(result);
        if !result.commands.is_null() && result.command_count > 0 {
            let commands =
                Vec::from_raw_parts(result.commands, result.command_count, result.command_count);
            for cmd in commands {
                senime_string_free(cmd.text);
                if !cmd.candidates.is_null() && cmd.candidate_count > 0 {
                    let candidates = Vec::from_raw_parts(
                        cmd.candidates,
                        cmd.candidate_count,
                        cmd.candidate_count,
                    );
                    for cand in candidates {
                        senime_string_free(cand.text);
                        senime_string_free(cand.code);
                    }
                }
            }
        }
    }
}
