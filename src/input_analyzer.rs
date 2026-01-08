use std::collections::HashMap;

use crate::dict::{Candidate, Config, Dict};

#[derive(Debug)]
enum InputType {
    Selection(usize),
    Punctuation(Vec<char>),
    EscapePair(char),
    Unknown,
}

#[derive(Debug)]
pub struct InputAnalyzer {
    dict: Dict,
    key_map: HashMap<char, InputType>,
    selection_keys: [char; 9],
}

impl InputAnalyzer {
    pub fn new(dict: Dict) -> Self {
        let Config {
            selection_keys,
            punctuations,
            escape_pair,
        } = dict.config();
        let mut key_map = HashMap::new();
        for (i, key) in selection_keys.iter().enumerate() {
            key_map.insert(*key, InputType::Selection(i));
        }
        for (key, values) in punctuations.into_iter() {
            key_map.insert(key, InputType::Punctuation(values));
        }
        if let Some([left, right]) = escape_pair {
            key_map.insert(left, InputType::EscapePair(right));
        }
        Self {
            dict,
            key_map,
            selection_keys,
        }
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Tag {
    Normal,
    Selection(usize),
    Punctuation,
    Escape,
    Unknown,
}

impl InputAnalyzer {
    pub fn analyze(&self, input: &[char]) -> AnalysisResult {
        let segments = self.segments(input);
        let segment_len = segments.len();
        let mut reduce_space = false;
        let mut sentence: Vec<String> = vec![];
        let mut candidates: Vec<CandidateRich> = vec![];
        for (i, (codes, tag)) in segments.into_iter().enumerate() {
            let at_last = i == segment_len - 1;
            let get_count = if at_last { 9 } else { 1 };
            match tag {
                Tag::Normal => {
                    reduce_space = true;
                    if let Some((cands, _unique)) = self.search_candidates(&codes, 0, get_count) {
                        sentence.push(cands[0].text.clone());
                        if at_last {
                            let to_rich = |(i, cand): (usize, &Candidate)| -> CandidateRich {
                                let select_key =
                                    self.selection_keys.get(i).map(|c| *c).unwrap_or(' ');
                                CandidateRich::new(
                                    cand.clone(),
                                    codes.iter().collect(),
                                    i,
                                    select_key,
                                    false,
                                )
                            };
                            candidates = cands.iter().enumerate().map(to_rich).collect();
                        }
                    } else {
                        sentence.push(codes.iter().collect());
                    }
                }
                Tag::Selection(i_cand) => {
                    if let Some((cands, _unique)) =
                        self.search_candidates(&codes[..codes.len() - 1], i_cand, get_count)
                    {
                        sentence.push(cands[0].text.clone());
                    }
                }
                Tag::Punctuation => {
                    sentence.push(self.punctuation_solve(&codes));
                }
                Tag::Escape => {
                    sentence.push(codes.iter().collect());
                }
                _ => {
                    let start = (reduce_space && codes[0] == ' ') as usize;
                    if reduce_space {
                        reduce_space = false;
                    }
                    sentence.push(codes[start..].iter().collect());
                }
            };
        }
        AnalysisResult {
            sentence,
            candidates,
        }
    }

    fn search_candidates(
        &self,
        code: &[char],
        index: usize,
        count: usize,
    ) -> Option<(&[Candidate], bool)> {
        self.dict.search(code).map(|c| {
            if index == 0 {
                (&c[0..c.len().min(count)], c.len() == 1)
            } else {
                let index = if index >= c.len() { 0 } else { index };
                (&c[index..index + 1], true)
            }
        })
    }

    fn segments(&self, input: &[char]) -> Vec<(Vec<char>, Tag)> {
        // (候选位置，要显示的字符<可以是正常被选的text，可以是标点，也可以是原始的输入字符>)
        let mut segments: Vec<(Vec<char>, Tag)> = vec![];
        let mut codes = vec![];
        let mut last_tag = None;
        let mut escape_end: Option<char> = None;
        for c in input.iter().chain(std::iter::once(&'\n')) {
            codes.push(*c);

            // 字面量不参与解析
            if let Some(escape_end_char) = escape_end {
                let is_lf = *c == '\n';
                if *c == escape_end_char || is_lf {
                    if is_lf {
                        codes.pop();
                    }
                    segments.push((codes.clone(), Tag::Escape));
                    codes.clear();
                    escape_end = None;
                }
                last_tag = None;
                continue;
            }

            let is_code = c.is_ascii_lowercase();
            let reachable = self.dict.reachable(&codes);
            // 仍是有效code
            if reachable && is_code {
                last_tag = None;
                continue;
            }
            // (reachable && !is_code) 此条件不可能

            // 由于codes加上当前字符后，已无法到达，表示此字符之前的codes是有效的
            // 因此这是顶字上屏
            if is_code {
                if codes.len() > 1 {
                    let before = codes[0..codes.len() - 1].to_vec();
                    segments.push((before, last_tag.unwrap_or(Tag::Normal)));
                }
                codes = vec![*c];
                last_tag = None;
                continue;
            }

            // (reachable = false) codes到此处时已无法到达
            match self.key_map.get(&c).unwrap_or(&InputType::Unknown) {
                InputType::EscapePair(char) => {
                    if codes.len() > 1 {
                        let before = codes[0..codes.len() - 1].to_vec();
                        segments.push((before, last_tag.clone().unwrap_or(Tag::Normal)));
                    }
                    codes = vec![*c];
                    escape_end = Some(*char);
                }
                InputType::Selection(index) if last_tag.is_none() => {
                    if codes.len() > 1 {
                        segments.push((codes.clone(), Tag::Selection(*index)));
                    }
                    codes.clear();
                    last_tag = Some(Tag::Unknown);
                }
                InputType::Punctuation(_) => {
                    if matches!(last_tag, Some(Tag::Punctuation)) {
                        if let Some(last_segment) = segments.last_mut() {
                            last_segment.0.push(*c);
                        }
                    } else {
                        if codes.len() > 1 {
                            let before = codes[0..codes.len() - 1].to_vec();
                            segments.push((before, last_tag.unwrap_or(Tag::Normal)));
                        }
                        segments.push((vec![*c], Tag::Punctuation));
                    }
                    codes.clear();
                    last_tag = Some(Tag::Punctuation);
                }
                _ => {
                    if last_tag.is_none() && codes.len() > 1 {
                        let before = codes[0..codes.len() - 1].to_vec();
                        segments.push((before, Tag::Normal));
                        codes = vec![*c];
                    }
                    // 未知字符不进行解析，且每一个字符单独一段
                    if *c == '\n' && codes.len() > 1 {
                        let before = codes[0..codes.len() - 1].to_vec();
                        segments.push((before, Tag::Unknown));
                        codes.clear();
                    }
                    last_tag = Some(Tag::Unknown);
                }
            }
        }
        segments
    }

    fn punctuation_solve(&self, puncs: &[char]) -> String {
        compact_vec(puncs)
            .iter()
            .map(|(p, c)| {
                self.key_map.get(p).map_or(String::new(), |t| match t {
                    InputType::Punctuation(ps) => {
                        let mut result = vec![];
                        let mut c = *c;
                        while c > 0 {
                            let index = (c - 1) % ps.len();
                            result.push(ps[index]);
                            c = c - ps.len().min(c);
                        }
                        result.iter().collect::<String>()
                    }
                    _ => String::new(),
                })
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

// code = ['.', '.', '!', '!', ';', '!']
// want convert to [('.', 2), ('!', 2), (';', 1), ('!', 1)]
fn compact_vec(v: &[char]) -> Vec<(char, usize)> {
    let mut result = Vec::new();
    let mut count = 1;
    let mut last_char = v[0];
    for c in v.iter().skip(1) {
        if *c == last_char {
            count += 1;
        } else {
            result.push((last_char, count));
            last_char = *c;
            count = 1;
        }
    }
    result.push((last_char, count));
    result
}

#[derive(Debug)]
pub struct CandidateRich {
    pub code: String,
    pub text: String,
    pub weight: i32,
    pub origin: String,
    pub order: usize,
    pub select_key: char,
    pub unique: bool,
}

impl CandidateRich {
    pub fn new(
        cand: Candidate,
        origin: String,
        order: usize,
        select_key: char,
        unique: bool,
    ) -> Self {
        let Candidate { code, text, weight } = cand;
        Self {
            code,
            text,
            weight,
            origin,
            order,
            select_key,
            unique,
        }
    }
}

pub struct AnalysisResult {
    pub sentence: Vec<String>,
    pub candidates: Vec<CandidateRich>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_table() -> String {
        let raw = r#"ahb 来 1
ahc 麦克 1
ahcg 疲惫不堪 1
c 不 10
c 不是 1
cu 还 1
cb 层 1
cb 不 1
z 可 1
z 可以 1
zk 可能 1
zkc 射 1"#;
        raw.replace(" ", "\t")
    }
    #[test]
    fn test_analyzer() {
        let trie = Dict::from_str(gen_table());
        let analyzer = InputAnalyzer::new(trie);
        let input = "a cIzk";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        assert_eq!(result.sentence, vec!["来", "", "不是", "可能"]);
        let input = "acIzk";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        assert_eq!(result.sentence, vec!["来", "不是", "可能"]);
        let input = "zk  cuahcI";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        assert_eq!(result.sentence, vec!["可能", " ", "还", "疲惫不堪"]);
        let input = "zk ,cuahcI";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        assert_eq!(result.sentence, vec!["可能", "", "，", "还", "疲惫不堪"]);
        let input = "zk  c,cua.hcI";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        assert_eq!(
            result.sentence,
            vec!["可能", " ", "不", "，", "还", "来", "。", "h", "不是",]
        );
        let input = "zk`zk`c,cua.hcI";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        assert_eq!(
            result.sentence,
            vec!["可能", "`zk`", "不", "，", "还", "来", "。", "h", "不是",]
        );
        let input = "zk `zk` c,cua.hcI";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        assert_eq!(
            result.sentence,
            vec![
                "可能", "", "`zk`", " ", "不", "，", "还", "来", "。", "h", "不是",
            ]
        );
    }

    #[test]
    fn test_segments() {
        let trie = Dict::from_str(gen_table());
        let analyzer = InputAnalyzer::new(trie);
        let samples: Vec<(&str, Vec<&str>, Vec<Tag>)> = vec![
            (
                "c zk,,zkcI",
                vec!["c", " ", "zk", ",,", "zkcI"],
                vec![
                    Tag::Normal,
                    Tag::Unknown,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Selection(1),
                ],
            ),
            (
                "ahcgahccb cIII;...",
                vec!["ahcg", "ahc", "cb", " ", "cI", "II", ";..."],
                vec![
                    Tag::Normal,
                    Tag::Normal,
                    Tag::Normal,
                    Tag::Unknown,
                    Tag::Selection(1),
                    Tag::Unknown,
                    Tag::Punctuation,
                ],
            ),
            (
                "8  8ahcgahccb  cIII;...",
                vec!["8  8", "ahcg", "ahc", "cb", "  ", "cI", "II", ";..."],
                vec![
                    Tag::Unknown,
                    Tag::Normal,
                    Tag::Normal,
                    Tag::Normal,
                    Tag::Unknown,
                    Tag::Selection(1),
                    Tag::Unknown,
                    Tag::Punctuation,
                ],
            ),
            (
                "zk  c,cua.hcI",
                vec!["zk", "  ", "c", ",", "cu", "a", ".", "h", "cI"],
                vec![
                    Tag::Normal,
                    Tag::Unknown,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Normal,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Normal,
                    Tag::Selection(1),
                ],
            ),
            (
                "zk  ,c,cua.hcI",
                vec!["zk", "  ", ",", "c", ",", "cu", "a", ".", "h", "cI"],
                vec![
                    Tag::Normal,
                    Tag::Unknown,
                    Tag::Punctuation,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Normal,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Normal,
                    Tag::Selection(1),
                ],
            ),
            (
                "zk ,c,cua.hcI",
                vec!["zk", " ", ",", "c", ",", "cu", "a", ".", "h", "cI"],
                vec![
                    Tag::Normal,
                    Tag::Unknown,
                    Tag::Punctuation,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Normal,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Normal,
                    Tag::Selection(1),
                ],
            ),
            (
                "zk  `hello``world`c,cua.hcI",
                vec![
                    "zk", "  ", "`hello`", "`world`", "c", ",", "cu", "a", ".", "h", "cI",
                ],
                vec![
                    Tag::Normal,
                    Tag::Unknown,
                    Tag::Escape,
                    Tag::Escape,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Normal,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Normal,
                    Tag::Selection(1),
                ],
            ),
            (
                "zk  c,cua.hcI`hello",
                vec!["zk", "  ", "c", ",", "cu", "a", ".", "h", "cI", "`hello"],
                vec![
                    Tag::Normal,
                    Tag::Unknown,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Normal,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Normal,
                    Tag::Selection(1),
                    Tag::Escape,
                ],
            ),
            (
                "zk  c,cua.hcI`hello`",
                vec!["zk", "  ", "c", ",", "cu", "a", ".", "h", "cI", "`hello`"],
                vec![
                    Tag::Normal,
                    Tag::Unknown,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Normal,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Normal,
                    Tag::Selection(1),
                    Tag::Escape,
                ],
            ),
        ];
        for (input, expected, expected_tags) in samples {
            let segments = analyzer.segments(input.chars().collect::<Vec<_>>().as_slice());
            // println!("Segments: {:?}", segments);
            let (segments, tags): (Vec<String>, Vec<Tag>) = segments
                .into_iter()
                .map(|seg| (seg.0.iter().collect::<String>(), seg.1))
                .unzip();
            assert_eq!(segments, expected);
            assert_eq!(tags, expected_tags);
        }
    }

    #[test]
    fn test_compact_vec() {
        let input = vec!['.', '.', '!', '!', ';', '!'];
        let expected = vec![('.', 2), ('!', 2), (';', 1), ('!', 1)];
        let result = compact_vec(&input);
        assert_eq!(result, expected);
    }
}
