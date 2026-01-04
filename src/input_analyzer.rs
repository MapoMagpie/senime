use std::collections::HashMap;

use crate::trie::{Candidate, Dict};

enum InputType {
    Selection(usize),
    Punctuation(Vec<char>),
    Unknown,
}

pub struct InputAnalyzer {
    dict: Dict,
    key_map: HashMap<char, InputType>,
}

impl InputAnalyzer {
    pub fn new(dict: Dict) -> Self {
        let s_keys = ['U', 'I', 'O', 'H', 'J', 'K', 'B', 'N', 'M'];
        let p_keys = [
            (',', vec!['，', ',']),
            ('.', vec!['。', '.']),
            ('!', vec!['！', '!']),
            ('/', vec!['？', '?']),
            (';', vec!['：', ';']),
            ('[', vec!['「', '“', '[']),
            (']', vec!['」', '”', ']']),
        ];
        let mut key_map = HashMap::new();
        for (i, key) in s_keys.into_iter().enumerate() {
            key_map.insert(key, InputType::Selection(i));
        }
        for (key, values) in p_keys.into_iter() {
            key_map.insert(key, InputType::Punctuation(values));
        }
        Self { dict, key_map }
    }
}
#[derive(Debug)]
enum Tag {
    Normal,
    Selection(usize),
    Punctuation,
    Unknown,
}

impl InputAnalyzer {
    pub fn analyze<I>(&self, input: I) -> AnalysisResult
    where
        I: IntoIterator<Item = char>,
    {
        let segments = self.segments(input);
        let segment_len = segments.len();
        let mut reduce_space = false;
        let text_with_last_candidates: Vec<(String, Option<&[Candidate]>)> = segments
            .into_iter()
            .enumerate()
            .map(|(i, seg)| match seg.1 {
                Tag::Normal => {
                    reduce_space = true;
                    self.search_candidates(&seg.0, 0, if i == segment_len - 1 { 9 } else { 1 })
                        .map_or((seg.0.iter().collect(), None), |candidates| {
                            (candidates[0].text.clone(), Some(candidates))
                        })
                }
                Tag::Selection(i_cand) => self
                    .search_candidates(
                        &seg.0[0..seg.0.len() - 1],
                        i_cand,
                        if i == segment_len - 1 { 9 } else { 1 },
                    )
                    .map_or((String::new(), None), |candidates| {
                        (candidates[0].text.clone(), None)
                    }),
                Tag::Punctuation => (self.punctuation_solve(&seg.0), None),
                _ => {
                    let start = (reduce_space && seg.0[0] == ' ') as usize;
                    if reduce_space {
                        reduce_space = false;
                    }
                    (seg.0[start..].iter().collect(), None)
                }
            })
            .collect();
        let candidates: Vec<Candidate> = text_with_last_candidates
            .last()
            .map(|e| e.1.map_or(vec![], |c| c.to_vec()))
            .unwrap_or(vec![]);
        let sentence = text_with_last_candidates
            .into_iter()
            .map(|e| e.0)
            .collect::<Vec<_>>();
        AnalysisResult {
            sentence,
            candidates,
        }
    }

    fn search_candidates(&self, code: &[char], index: usize, count: usize) -> Option<&[Candidate]> {
        self.dict.search(code).map(|c| {
            if index == 0 {
                &c[0..c.len().min(count)]
            } else {
                let index = if index >= c.len() { 0 } else { index };
                &c[index..index + 1]
            }
        })
    }

    fn segments<I>(&self, input: I) -> Vec<(Vec<char>, Tag)>
    where
        I: IntoIterator<Item = char>,
    {
        // (候选位置，要显示的字符<可以是正常被选的text，可以是标点，也可以是原始的输入字符>)
        let mut segments: Vec<(Vec<char>, Tag)> = vec![];
        let mut codes = vec![];
        let mut last_tag = None;
        for c in input.into_iter().chain(std::iter::once('\n')) {
            codes.push(c);
            let is_code = c >= 'a' && c <= 'z';
            let reachable = self.dict.reachable(&codes);
            // 仍是有效code
            if reachable && is_code && c != '+' {
                last_tag = None;
                continue;
            }
            // if reachable && !is_code // not possible
            // 无效code，但之前是有效code，选择之前的code，留下当前code
            if is_code {
                if codes.len() > 1 {
                    let before = codes[0..codes.len() - 1].to_vec();
                    segments.push((before, Tag::Normal));
                }
                codes = vec![c];
                last_tag = None;
                continue;
            }
            match self.key_map.get(&c).unwrap_or(&InputType::Unknown) {
                InputType::Selection(index) => {
                    // 如果上一个字符也是Selection，直接抛弃当前
                    if !matches!(last_tag, Some(Tag::Selection(_))) {
                        if codes.len() > 1 {
                            segments.push((codes.clone(), Tag::Selection(*index)));
                        }
                    }
                    last_tag = Some(Tag::Selection(*index));
                    codes.clear();
                }
                InputType::Punctuation(_) => {
                    if matches!(last_tag, Some(Tag::Punctuation)) {
                        if let Some(last_segment) = segments.last_mut() {
                            last_segment.0.push(c);
                        }
                    } else {
                        if codes.len() > 1 {
                            let before = codes[0..codes.len() - 1].to_vec();
                            segments.push((before, Tag::Normal));
                        }
                        segments.push((vec![c], Tag::Punctuation));
                    }
                    codes.clear();
                    last_tag = Some(Tag::Punctuation);
                }
                InputType::Unknown => {
                    if codes.len() > 1 {
                        let before = codes[0..codes.len() - 1].to_vec();
                        segments.push((before, Tag::Normal));
                    }
                    if c != '\n' {
                        segments.push((vec![c], Tag::Unknown));
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

pub struct AnalysisResult {
    pub sentence: Vec<String>,
    pub candidates: Vec<Candidate>,
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
        let result = analyzer.analyze(input.chars());
        assert_eq!(result.sentence, vec!["来", "", "不是", "可能"]);
        let input = "acIzk";
        let result = analyzer.analyze(input.chars());
        assert_eq!(result.sentence, vec!["来", "不是", "可能"]);
        let input = "zk  cuahcI";
        let result = analyzer.analyze(input.chars());
        assert_eq!(result.sentence, vec!["可能", "", " ", "还", "疲惫不堪"]);
        let input = "zk  c,cua.hcI";
        let result = analyzer.analyze(input.chars());
        assert_eq!(
            result.sentence,
            vec!["可能", "", " ", "不", "，", "还", "来", "。", "h", "不是",]
        );
    }

    #[test]
    fn test_segments() {
        let trie = Dict::from_str(gen_table());
        let analyzer = InputAnalyzer::new(trie);
        let samples = vec![
            ("c zk,,zkcI", vec!["c", " ", "zk", ",,", "zkcI"]),
            (
                "ahcgahccb cIII;...",
                vec!["ahcg", "ahc", "cb", " ", "cI", ";..."],
            ),
            (
                "  ahcgahccb  cIII;...",
                vec![" ", " ", "ahcg", "ahc", "cb", " ", " ", "cI", ";..."],
            ),
            (
                "zk  c,cua.hcI",
                vec!["zk", " ", " ", "c", ",", "cu", "a", ".", "h", "cI"],
            ),
        ];
        for (input, expected) in samples {
            let segments = analyzer.segments(input.chars());
            println!("Segments: {:?}", segments);
            let segments = segments
                .into_iter()
                .map(|seg| seg.0.iter().collect::<String>())
                .collect::<Vec<_>>();
            assert_eq!(segments, expected);
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
