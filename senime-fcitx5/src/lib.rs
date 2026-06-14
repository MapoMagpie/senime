use senime_lib::{AnalysisResult, Dict, InputAnalyzer, secondary_dict_path};
use std::{
    cell::RefCell,
    ffi::{CStr, CString, c_char},
    panic::{AssertUnwindSafe, catch_unwind},
    path::PathBuf,
    ptr,
};

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

pub struct SenimeEngine {
    analyzer: InputAnalyzer,
    selection_keys: CString,
}

#[repr(C)]
pub struct SenimeCandidate {
    text: *mut c_char,
    code: *mut c_char,
    select_key: u32,
    order: usize,
    unique: bool,
}

#[repr(C)]
pub struct SenimeAnalysis {
    text: *mut c_char,
    candidates: *mut SenimeCandidate,
    candidate_count: usize,
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

#[unsafe(no_mangle)]
pub extern "C" fn senime_engine_new(table_path: *const c_char) -> *mut SenimeEngine {
    clear_last_error();
    let Some(table_path) = cstr_to_str(table_path, "table_path") else {
        return ptr::null_mut();
    };
    let result: Result<Box<SenimeEngine>, String> = (|| {
        let dict = Dict::try_load(table_path)?;
        let reverse_dict = dict.config().reverse_dict.as_ref().map(|path| {
            let hint = PathBuf::from(path)
                .file_name()
                .and_then(|name| name.to_str().map(|n| n.chars().take(1).collect::<String>()))
                .unwrap_or("反".to_string());
            (Dict::load(secondary_dict_path(table_path, path)), hint)
        });
        let selection_keys = CString::new(
            dict.config()
                .selection_keys
                .iter()
                .collect::<String>(),
        )
        .unwrap_or_default();
        Ok(Box::new(SenimeEngine {
            analyzer: InputAnalyzer::new(dict, reverse_dict),
            selection_keys,
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

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn senime_engine_analyze(
    engine: *const SenimeEngine,
    input: *const c_char,
) -> *mut SenimeAnalysis {
    clear_last_error();
    if engine.is_null() {
        set_last_error("engine is null");
        return ptr::null_mut();
    }
    let Some(input) = cstr_to_str(input, "input") else {
        return ptr::null_mut();
    };
    let result = catch_unwind(AssertUnwindSafe(|| {
        let chars = input.chars().collect::<Vec<_>>();
        let AnalysisResult {
            segments,
            candidates,
        } = unsafe { &*engine }.analyzer.analyze(chars.as_slice());
        let text = segments
            .into_iter()
            .map(|(text, _)| text)
            .collect::<Vec<_>>()
            .join("");
        let mut candidates = candidates
            .unwrap_or_default()
            .into_iter()
            .map(|cand| SenimeCandidate {
                text: into_c_string(cand.text),
                code: into_c_string(cand.code),
                select_key: cand.select_key as u32,
                order: cand.order,
                unique: cand.unique,
            })
            .collect::<Vec<_>>();
        let (candidate_ptr, candidate_count) = if candidates.is_empty() {
            (ptr::null_mut(), 0)
        } else {
            let count = candidates.len();
            let ptr = candidates.as_mut_ptr();
            std::mem::forget(candidates);
            (ptr, count)
        };
        Box::new(SenimeAnalysis {
            text: into_c_string(text),
            candidates: candidate_ptr,
            candidate_count,
        })
    }));
    match result {
        Ok(analysis) => Box::into_raw(analysis),
        Err(_) => {
            set_last_error("failed to analyze input");
            ptr::null_mut()
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn senime_analysis_free(analysis: *mut SenimeAnalysis) {
    if analysis.is_null() {
        return;
    }
    unsafe {
        let analysis = Box::from_raw(analysis);
        senime_string_free(analysis.text);
        if !analysis.candidates.is_null() && analysis.candidate_count > 0 {
            let candidates = Vec::from_raw_parts(
                analysis.candidates,
                analysis.candidate_count,
                analysis.candidate_count,
            );
            for candidate in candidates {
                senime_string_free(candidate.text);
                senime_string_free(candidate.code);
            }
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

/// Returns the selection keys string from the engine's dictionary config.
/// The returned pointer is valid for the lifetime of the engine.
/// `len` is set to the number of selection keys.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn senime_engine_selection_keys(
    engine: *const SenimeEngine,
    len: *mut usize,
) -> *const c_char {
    if engine.is_null() {
        if !len.is_null() {
            unsafe { *len = 0 };
        }
        return ptr::null();
    }
    let engine = unsafe { &*engine };
    if !len.is_null() {
        unsafe { *len = engine.selection_keys.as_bytes().len() };
    }
    engine.selection_keys.as_ptr()
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
