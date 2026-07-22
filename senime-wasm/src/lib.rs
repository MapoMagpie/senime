use std::sync::Mutex;

use senime_lib::{
    AnalysisResult, CandidateRich, Config, DictKind, DictKindName, DictMeta, InputAnalyzer,
};
use wasm_bindgen::prelude::*;

static IME: Mutex<Option<InputAnalyzer>> = Mutex::new(None);

/// 接受 txt 码表内容和 JSON 配置，解析为 Dict，序列化为二进制并存入 IME，返回二进制码表
#[wasm_bindgen]
pub fn init_ime(content: &str, config: &str) -> Result<Vec<u8>, JsValue> {
    let cfg: Config =
        serde_json::from_str(config).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let dict =
        DictKind::from_str(content, DictKindName::Prefix).map_err(|e| JsValue::from_str(&e))?;
    let bin = dict.to_bin(0);
    let mut ime = IME.lock().unwrap();
    ime.replace(InputAnalyzer::new(cfg, vec![(DictMeta::default(), dict)]));
    Ok(bin)
}

/// 从二进制码表加载 IME 实例（用于从 IndexedDB 缓存恢复）
#[wasm_bindgen]
pub fn load_bin(bs: &[u8], config: &str) -> Result<(), JsValue> {
    let dict = DictKind::try_from((0i64, DictKindName::Prefix, bs))
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let cfg: Config =
        serde_json::from_str(config).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let mut ime = IME.lock().unwrap();
    ime.replace(InputAnalyzer::new(cfg, vec![(DictMeta::default(), dict)]));
    Ok(())
}

#[wasm_bindgen]
pub fn completion(input: &str) -> JsAnalysisResult {
    let ime = IME.lock().unwrap();
    if let Some(an) = ime.as_ref() {
        let chars: Vec<char> = input.chars().collect();
        let an_ret = an.analyze(chars.as_slice());
        JsAnalysisResult::from(an_ret)
    } else {
        JsAnalysisResult {
            segments: vec![],
            pending: false,
            candidates: None,
        }
    }
}

// ---- WASM 包装类型 ----

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct JsSegment {
    text: String,
    origin: String,
    tag_name: String,
}

#[wasm_bindgen]
impl JsSegment {
    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn origin(&self) -> String {
        self.origin.clone()
    }
    /// 标签名: "Code" | "Punctuation" | "Escape" | "Unknown"
    #[wasm_bindgen(getter)]
    pub fn tag_name(&self) -> String {
        self.tag_name.clone()
    }
}

impl From<(String, Vec<char>, senime_lib::input_analyzer::Tag)> for JsSegment {
    fn from((text, origin, tag): (String, Vec<char>, senime_lib::input_analyzer::Tag)) -> Self {
        let tag_name = match tag {
            senime_lib::input_analyzer::Tag::Code(_) => "Code",
            senime_lib::input_analyzer::Tag::Punctuation(_) => "Punctuation",
            senime_lib::input_analyzer::Tag::Escape(_) => "Escape",
            senime_lib::input_analyzer::Tag::Unknown => "Unknown",
        };
        Self {
            text,
            origin: origin.into_iter().collect(),
            tag_name: tag_name.to_string(),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct JsCandidate {
    code: String,
    text: String,
    weight: i32,
    origin: String,
    order: usize,
    select_key: char,
}

#[wasm_bindgen]
impl JsCandidate {
    #[wasm_bindgen(getter)]
    pub fn code(&self) -> String {
        self.code.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn weight(&self) -> i32 {
        self.weight
    }
    #[wasm_bindgen(getter)]
    pub fn origin(&self) -> String {
        self.origin.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn order(&self) -> usize {
        self.order
    }
    #[wasm_bindgen(getter)]
    pub fn select_key(&self) -> char {
        self.select_key
    }
}

impl From<CandidateRich> for JsCandidate {
    fn from(c: CandidateRich) -> Self {
        Self {
            code: c.code,
            text: c.text,
            weight: c.weight,
            origin: c.origin.into_iter().collect(),
            order: c.order,
            select_key: c.select_key,
        }
    }
}

#[wasm_bindgen]
pub struct JsAnalysisResult {
    segments: Vec<JsSegment>,
    pending: bool,
    candidates: Option<Vec<JsCandidate>>,
}

#[wasm_bindgen]
impl JsAnalysisResult {
    /// 返回分段列表的长度
    #[wasm_bindgen(getter)]
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }
    /// 获取第 i 个分段
    pub fn segment(&self, i: usize) -> JsSegment {
        self.segments[i].clone()
    }
    /// 是否未决
    #[wasm_bindgen(getter)]
    pub fn pending(&self) -> bool {
        self.pending
    }
    /// 是否有候选列表
    #[wasm_bindgen(getter)]
    pub fn has_candidates(&self) -> bool {
        self.candidates.is_some()
    }
    /// 候选数量（无候选时返回 0）
    #[wasm_bindgen(getter)]
    pub fn candidate_count(&self) -> usize {
        self.candidates.as_ref().map_or(0, |v| v.len())
    }
    /// 获取第 i 个候选
    pub fn candidate(&self, i: usize) -> JsCandidate {
        self.candidates.as_ref().unwrap()[i].clone()
    }
}

impl From<AnalysisResult> for JsAnalysisResult {
    fn from(r: AnalysisResult) -> Self {
        Self {
            segments: r.segments.into_iter().map(JsSegment::from).collect(),
            pending: r.pending,
            candidates: r
                .candidates
                .map(|v| v.into_iter().map(JsCandidate::from).collect()),
        }
    }
}
