use std::sync::Mutex;

use senime_lib::{Dict, InputAnalyzer};
use wasm_bindgen::prelude::*;

static IME: Mutex<Option<InputAnalyzer>> = Mutex::new(None);

#[wasm_bindgen]
pub fn init_ime(bs: &[u8]) -> Result<(), JsValue> {
    let mut ime = IME.lock().unwrap();
    let dict = Dict::try_from((0, 0, bs)).map_err(|e| JsValue::from_str(&e.to_string()))?;
    ime.replace(InputAnalyzer::new(dict, None));
    Ok(())
}

#[wasm_bindgen]
pub fn completion(input: &str) -> String {
    let ime = IME.lock().unwrap();
    if let Some(an) = ime.as_ref() {
        let chars: Vec<char> = input.chars().collect();
        let an_ret = an.analyze(chars.as_slice());
        an_ret.segments.into_iter().map(|s| s.0).collect()
    } else {
        "input method engine not yet ready".to_string()
    }
}
