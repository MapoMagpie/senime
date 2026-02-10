use std::collections::HashMap;

use crate::dict::{Candidate, Config, Dict};

#[derive(Debug)]
enum InputType {
    Selection(usize),
    Punctuation(Vec<String>),
    Secondary,
    EscapePair(char),
    Unknown,
}

#[derive(Debug)]
pub struct InputAnalyzer {
    dict: Dict,
    secondary_dict: Option<Dict>,
    secondary_hint: Option<Vec<Candidate>>,
    key_map: HashMap<char, InputType>,
    selection_keys: [char; 9],
}

impl InputAnalyzer {
    pub fn new(dict: Dict, secondary: Option<(Dict, String)>) -> Self {
        let Config {
            selection_keys,
            punctuations,
            escape_pair,
            reverse_key,
            reverse_dict: _,
        } = dict.config().clone();
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
        if let Some(char) = reverse_key {
            key_map.insert(char, InputType::Secondary);
        }
        let (secondary_dict, secondary_hint) = match secondary {
            Some((sec_dict, hint)) => {
                let hint = vec![Candidate {
                    code: "r".to_string(),
                    text: format!(":({hint})"),
                    weight: 0,
                }];
                (Some(sec_dict), Some(hint))
            }
            _ => (None, None),
        };
        Self {
            dict,
            secondary_dict,
            secondary_hint,
            key_map,
            selection_keys,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Tag {
    Normal,
    Selection(usize),
    Punctuation,
    SelectionForPunc(usize),
    Escape((char, char)),
    Secondary,
    SelectionForSecondary(usize),
    Unknown,
}

impl InputAnalyzer {
    pub fn analyze(&self, input: &[char]) -> AnalysisResult {
        if input.is_empty() {
            return AnalysisResult {
                segments: vec![],
                candidates: None,
            };
        }
        let segments = self.segments(input);
        let segment_len = segments.len();
        let mut reduce_space = false;
        let mut segments_ret: Vec<(String, Vec<char>)> = vec![];
        let mut candidates: Option<Vec<CandidateRich>> = None;
        for (i, (codes, tag)) in segments.into_iter().enumerate() {
            let at_last = i == segment_len - 1;
            let get_count = if at_last { 9 } else { 1 };
            let use_secondary_dict = matches!(tag, Tag::Secondary | Tag::SelectionForSecondary(_));
            match tag {
                Tag::Normal | Tag::Secondary => {
                    if let Some((cands, unique)) =
                        self.search_candidates(&codes, 0, get_count, use_secondary_dict)
                    {
                        reduce_space = !unique;
                        segments_ret.push((cands[0].text.clone(), codes.clone()));
                        // candidates
                        if at_last && !unique {
                            let to_rich = |(i, cand): (usize, &Candidate)| -> CandidateRich {
                                let select_key = self.selection_keys.get(i).copied().unwrap_or(' ');
                                CandidateRich::new(
                                    cand.clone(),
                                    codes.to_vec(),
                                    i,
                                    select_key,
                                    false,
                                )
                            };
                            candidates = Some(cands.iter().enumerate().map(to_rich).collect());
                        }
                    } else {
                        segments_ret.push((codes.iter().collect(), codes));
                    }
                }
                Tag::Selection(i_cand) | Tag::SelectionForSecondary(i_cand) => {
                    if let Some((cands, _unique)) = self.search_candidates(
                        &codes[..codes.len() - 1],
                        i_cand,
                        get_count,
                        use_secondary_dict,
                    ) {
                        segments_ret.push((cands[0].text.clone(), codes));
                    }
                }
                Tag::Punctuation | Tag::SelectionForPunc(_) => {
                    let mut compacted = compact_vec(&codes);
                    let selections = if let Tag::SelectionForPunc(i_cand) = tag {
                        // 如果tag是SelectionForPunc，select_key必然处于最后
                        compacted.pop().map(|c| (i_cand, c.0))
                    } else {
                        reduce_space = true;
                        None
                    };
                    let last_i = compacted.len() - 1;
                    for (i, (punc, repeat)) in compacted.into_iter().enumerate() {
                        let select = if i == last_i { selections } else { None };
                        let mut origin = [punc].repeat(repeat);
                        if let Some((_, k)) = select {
                            origin.push(k);
                        }
                        match self.get_punctuation(&punc, repeat, select.map(|s| s.0)) {
                            Some((punc_text, cands)) => {
                                segments_ret.push((punc_text, origin));
                                if at_last {
                                    candidates = cands;
                                }
                            }
                            _ => {
                                segments_ret.push((origin.iter().collect(), origin));
                            }
                        }
                    }
                }
                Tag::Escape((_, end)) => {
                    let last_i = codes.len() - 1;
                    // 当escape闭合时，移除escape_key本身，如果escape_key前后相连(其中为空, last_i < 1)出现，保持其原样
                    let text = if last_i > 1 && codes[last_i] == end {
                        codes[1..last_i].iter().collect()
                    } else {
                        codes.iter().collect()
                    };
                    segments_ret.push((text, codes));
                }
                _ => {
                    let start = (reduce_space && codes[0] == ' ') as usize;
                    if reduce_space {
                        reduce_space = false;
                    }
                    let text = &codes[start..];
                    segments_ret.push((text.iter().collect(), codes.to_vec()));
                }
            };
        }
        AnalysisResult {
            segments: segments_ret,
            candidates,
        }
    }

    fn search_candidates(
        &self,
        code: &[char],
        index: usize,
        count: usize,
        use_secondary_dict: bool,
    ) -> Option<(&[Candidate], bool)> {
        let (dict, codes) = if use_secondary_dict {
            (self.secondary_dict.as_ref()?, &code[1..])
        } else {
            (&self.dict, code)
        };
        if use_secondary_dict && codes.is_empty() {
            return self
                .secondary_hint
                .as_ref()
                .map(|hint| (hint.as_slice(), false));
        }
        dict.search(codes).map(|c| {
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
        let mut escape: Option<(char, char)> = None;
        let push_before = |c: &mut Vec<char>, seg: &mut Vec<(Vec<char>, Tag)>, t: Tag| {
            if c.len() > 1 {
                let before = c[0..c.len() - 1].to_vec();
                seg.push((before, t));
            }
            *c = c.split_off(c.len() - 1);
        };
        for c in input.iter().chain(std::iter::once(&'\n')) {
            codes.push(*c);

            // 进入escape状态
            if let Some((left, right)) = escape {
                let is_lf = *c == '\n';
                if *c == right || is_lf {
                    if is_lf {
                        codes.pop();
                    }
                    segments.push((codes.clone(), Tag::Escape((left, right))));
                    codes.clear();
                    escape = None;
                }
                last_tag = None;
                continue;
            }

            // (reachable = false) codes到此处时已无法到达
            match self.key_map.get(c).unwrap_or(&InputType::Unknown) {
                InputType::EscapePair(char) => {
                    push_before(&mut codes, &mut segments, last_tag.unwrap_or(Tag::Normal));
                    escape = Some((*c, *char));
                }
                InputType::Selection(index) => {
                    match last_tag {
                        // 当前char是Selection，当last_tag是Normal或Punctuation或Secondary，则将codes作为segment，并清空codes
                        Some(Tag::Normal) | Some(Tag::Punctuation) | Some(Tag::Secondary) => {
                            let tag = match last_tag {
                                Some(Tag::Normal) => Tag::Selection(*index),
                                Some(Tag::Punctuation) => Tag::SelectionForPunc(*index),
                                Some(Tag::Secondary) => Tag::SelectionForSecondary(*index),
                                _ => Tag::Unknown,
                            };
                            segments.push((codes.clone(), tag));
                            codes.clear();
                            last_tag = None;
                        }
                        _ => {
                            // last_tag为其他，将当前char认为unknown
                            last_tag = Some(Tag::Unknown);
                        }
                    }
                }
                InputType::Punctuation(_) => {
                    match last_tag {
                        // 当last_tag为punctuation时，继续向codes追加
                        Some(Tag::Punctuation) => {}
                        // 当last_tag是其他时，将之前的codes加入segments
                        _ => {
                            push_before(
                                &mut codes,
                                &mut segments,
                                last_tag.unwrap_or(Tag::Unknown),
                            );
                        }
                    }
                    last_tag = Some(Tag::Punctuation);
                }
                InputType::Secondary => {
                    // 反查标记
                    push_before(&mut codes, &mut segments, last_tag.unwrap_or(Tag::Unknown));
                    last_tag = Some(Tag::Secondary);
                }
                _ => {
                    let is_code = c.is_ascii_lowercase();
                    let reachable = match last_tag {
                        Some(Tag::Unknown) => false,
                        Some(Tag::Secondary) => match self.secondary_dict.as_ref() {
                            Some(sec_dict) if codes.len() > 1 => sec_dict.reachable(&codes[1..]),
                            // 当secondary_dict(反查)不存在时，保持codes(是连续的字母)继续追加
                            _ => is_code,
                        },
                        _ => self.dict.reachable(&codes),
                    };
                    // 仍是有效code
                    if reachable {
                        last_tag = if last_tag == Some(Tag::Secondary) {
                            Some(Tag::Secondary)
                        } else {
                            Some(Tag::Normal)
                        };
                        continue;
                    }
                    // 不可到达(码表中无此码)
                    if is_code {
                        // 不可到达，但当前字符仍是code，截取之前的codes为segment
                        push_before(&mut codes, &mut segments, last_tag.unwrap_or(Tag::Normal));
                        // 是否允许Secondary(反查)能连续输入(组句)？目前设计不允许，使下一段codes成为Normal
                        last_tag = Some(Tag::Normal);
                    } else {
                        // 当前char，不是code是unknown，如果last_tag是normal或none，则截取之前的codes为segment
                        match last_tag {
                            Some(Tag::Normal)
                            | Some(Tag::Punctuation)
                            | Some(Tag::Secondary)
                            | None => {
                                push_before(
                                    &mut codes,
                                    &mut segments,
                                    last_tag.unwrap_or(Tag::Normal),
                                );
                            }
                            _ => {
                                if *c == '\n' && codes.len() > 1 {
                                    push_before(
                                        &mut codes,
                                        &mut segments,
                                        last_tag.unwrap_or(Tag::Unknown),
                                    );
                                    codes.clear();
                                }
                            }
                        }
                        last_tag = Some(Tag::Unknown);
                    }
                }
            }
        }
        segments
    }

    fn get_punctuation(
        &self,
        punc: &char,
        repeat: usize,
        select: Option<usize>,
    ) -> Option<(String, Option<Vec<CandidateRich>>)> {
        self.key_map.get(punc).map(|t| match t {
            InputType::Punctuation(ps) => {
                // 如果ps["a", "b"]的长度为2，而repeat为5，最终result将变成bba
                // 另外如果有select，则在最后一轮时直接从cands中选择对应的punc
                let mut result: Vec<&str> = vec![];
                let mut cands: &[String] = &ps[..];
                let mut c = repeat;
                while c > 0 {
                    let index = if (c - 1) >= ps.len() {
                        ps.len() - 1
                    } else {
                        c - 1
                    };
                    result.push(&ps[index]);
                    cands = &ps[index..];
                    c = c - ps.len().min(c);
                }
                let cands: Option<Vec<CandidateRich>> = if cands.len() > 1 {
                    if let Some(i_cand) = select {
                        // 将result最后一个元素修改为cands[i_cand]对应的内容
                        if let (Some(punc), Some(last)) = (cands.get(i_cand), result.last_mut()) {
                            *last = punc;
                        }
                        None
                    } else {
                        let cands = cands
                            .iter()
                            .enumerate()
                            .map(|(i, pu)| CandidateRich {
                                code: String::new(),
                                text: pu.clone(),
                                weight: 0,
                                origin: [*punc].repeat(repeat),
                                order: i,
                                select_key: self.selection_keys.get(i).copied().unwrap_or('_'),
                                unique: false,
                            })
                            .collect();
                        Some(cands)
                    }
                } else {
                    None
                };
                Some((result.join(""), cands))
            }
            _ => None,
        })?
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
    pub origin: Vec<char>,
    pub order: usize,
    pub select_key: char,
    pub unique: bool,
}

impl CandidateRich {
    pub fn new(
        cand: Candidate,
        origin: Vec<char>,
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
    pub segments: Vec<(String, Vec<char>)>,
    pub candidates: Option<Vec<CandidateRich>>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn gen_table() -> String {
        let raw = r#"
```toml
selection_keys = ["U","I","O","H","J","K","B","N","M"]
[punctuations]
',' = ["，", ",", "……"]
'.' = ["。", ".", "……"]
'!' = ["！", "!"]
'/' = ["？", "/"]
';' = ["：", "；", ";"]
'[' = ["「", "“", "[", "【"]
']' = ["」", "”", "]", "】"]
```
ahb 来 1
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
        let dict = Dict::from_str(&gen_table()).unwrap();
        let analyzer = InputAnalyzer::new(dict, None);
        let input = "a cIzk";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        let (texts, _): (Vec<String>, Vec<Vec<char>>) = result.segments.into_iter().unzip();
        assert_eq!(texts, vec!["来", "", "不是", "可能",]);

        let input = "a cIzk@abc";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        let (texts, _): (Vec<String>, Vec<Vec<char>>) = result.segments.into_iter().unzip();
        assert_eq!(texts, vec!["来", "", "不是", "可能", "@abc"]);

        let input = "a cIzk,,,[]I]]";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        let (texts, _): (Vec<String>, Vec<Vec<char>>) = result.segments.into_iter().unzip();
        assert_eq!(texts, vec!["来", "", "不是", "可能", "……", "「", "”", "”"]);

        let input = "acIzk";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        let (texts, _): (Vec<String>, Vec<Vec<char>>) = result.segments.into_iter().unzip();
        assert_eq!(texts, vec!["来", "不是", "可能"]);

        let input = "zk  cuahcI";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        let (texts, _): (Vec<String>, Vec<Vec<char>>) = result.segments.into_iter().unzip();
        assert_eq!(texts, vec!["可能", " ", "还", "疲惫不堪"]);

        let input = "zk ,cuahcI";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        let (texts, _): (Vec<String>, Vec<Vec<char>>) = result.segments.into_iter().unzip();
        assert_eq!(texts, vec!["可能", "", "，", "还", "疲惫不堪"]);

        let input = "zk  c,cua.hcI";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        let (texts, _): (Vec<String>, Vec<Vec<char>>) = result.segments.into_iter().unzip();
        assert_eq!(
            texts,
            vec!["可能", " ", "不", "，", "还", "来", "。", "h", "不是",]
        );

        let input = "zk`zk`c,cua.hcI";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        let (texts, _): (Vec<String>, Vec<Vec<char>>) = result.segments.into_iter().unzip();
        assert_eq!(
            texts,
            vec!["可能", "zk", "不", "，", "还", "来", "。", "h", "不是",]
        );

        let input = "zk `zk` c,cua.hcI`hhh";
        let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
        let (texts, _): (Vec<String>, Vec<Vec<char>>) = result.segments.into_iter().unzip();
        assert_eq!(
            texts,
            vec![
                "可能", "", "zk", " ", "不", "，", "还", "来", "。", "h", "不是", "`hhh"
            ]
        );
    }

    #[test]
    fn test_segments() {
        let trie = Dict::from_str(&gen_table()).unwrap();
        let analyzer = InputAnalyzer::new(trie, None);
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
                "c   A  zk,,zkcI",
                vec!["c", "   A  ", "zk", ",,", "zkcI"],
                vec![
                    Tag::Normal,
                    Tag::Unknown,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Selection(1),
                ],
            ),
            (
                "IOczk,,zkcI",
                vec!["IO", "c", "zk", ",,", "zkcI"],
                vec![
                    Tag::Unknown,
                    Tag::Normal,
                    Tag::Normal,
                    Tag::Punctuation,
                    Tag::Selection(1),
                ],
            ),
            (
                "IOczk,I,,O.. zkcI",
                vec!["IO", "c", "zk", ",I", ",,O", "..", " ", "zkcI"],
                vec![
                    Tag::Unknown,
                    Tag::Normal,
                    Tag::Normal,
                    Tag::SelectionForPunc(1),
                    Tag::SelectionForPunc(2),
                    Tag::Punctuation,
                    Tag::Unknown,
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
                    Tag::Escape(('`', '`')),
                    Tag::Escape(('`', '`')),
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
                    Tag::Escape(('`', '`')),
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
                    Tag::Escape(('`', '`')),
                ],
            ),
            (
                "zk  c,cua.hcI@abc@abcI cu",
                vec![
                    "zk", "  ", "c", ",", "cu", "a", ".", "h", "cI", "@abc", "@abcI", " ", "cu",
                ],
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
                    Tag::Secondary,
                    Tag::SelectionForSecondary(1),
                    Tag::Unknown,
                    Tag::Normal,
                ],
            ),
        ];
        for (i, (input, expected, expected_tags)) in samples.into_iter().enumerate() {
            let segments = analyzer.segments(input.chars().collect::<Vec<_>>().as_slice());
            // println!("Segments: {:?}", segments);
            let (segments, tags): (Vec<String>, Vec<Tag>) = segments
                .into_iter()
                .map(|seg| (seg.0.iter().collect::<String>(), seg.1))
                .unzip();
            println!("sample: [{i}] {input}");
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
