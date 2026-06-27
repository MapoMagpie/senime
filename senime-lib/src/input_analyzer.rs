use std::{collections::HashMap, iter::Peekable, slice::Iter};

use ahash::AHashMap;

use crate::dict::{CandidateView, Config, Dict};

#[derive(Debug)]
pub struct InputAnalyzer {
    dicts: Vec<(char, String, Dict)>,
    main_dict_code_map: AHashMap<char, String>,
    escape_pair: Option<[char; 2]>,
    selection_keys: [char; 9],
    punctuations: HashMap<char, Vec<String>>,
    page_count: usize,
}

// ⇞ (U+21DE) 和 ⇟ (U+21DF)
const PAGE_UP: char = '⇞';
const PAGE_DOWN: char = '⇟';

impl InputAnalyzer {
    pub fn new(dict: Dict, secondary: Option<(Dict, String)>) -> Self {
        let Config {
            dict: _,
            selection_keys,
            punctuations,
            escape_pair,
            reverse_key: _,
            reverse_dict: _,
        } = dict.config().clone();
        let mut dicts = vec![];
        let mut main_dict_code_map = AHashMap::<char, String>::new();
        if let Some((sec_dict, hint)) = secondary {
            // 从dict.candidates中筛选单字，存入该单字的最长编码
            for cand in dict.candidates_iter() {
                if cand.text.chars().count() == 1 {
                    let ch = cand.text.chars().next().unwrap();
                    let code = cand.code.to_string();
                    if let Some(existing) = main_dict_code_map.get_mut(&ch) {
                        if code.len() > existing.len() {
                            *existing = code;
                        }
                    } else {
                        main_dict_code_map.insert(ch, code);
                    }
                }
            }
            dicts.push(('@', hint, sec_dict));
        };
        dicts.push(('\0', "".to_string(), dict));
        dicts.reverse();
        Self {
            dicts,
            selection_keys,
            main_dict_code_map,
            escape_pair,
            punctuations,
            page_count: 9,
        }
    }

    pub fn get_dict(&self) -> &Dict {
        &self.dicts[0].2
    }

    pub fn get_sec_dict(&self) -> Option<&Dict> {
        self.dicts.get(1).as_ref().map(|d| &d.2)
    }
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
        let mut segments_ret: Vec<(String, Vec<char>, Tag)> = vec![];
        let mut candidates: Option<Vec<CandidateRich>> = None;
        for (i, (codes, tag)) in segments.into_iter().enumerate() {
            let at_last = i == segment_len - 1;
            match tag {
                Tag::Code(selection) => {
                    if let Some((cands, unique)) =
                        self.search_candidates(&codes, &selection, !at_last)
                    {
                        reduce_space = !unique;
                        segments_ret.push((cands[0].text.to_string(), codes.clone(), tag));
                        // candidates
                        if at_last && !unique {
                            let to_rich = |(i, cand): (usize, &CandidateView)| -> CandidateRich {
                                let select_key = self.selection_keys.get(i).copied().unwrap_or(' ');
                                CandidateRich::new(
                                    cand.code.to_string(),
                                    cand.text.to_string(),
                                    cand.weight,
                                    codes.to_vec(),
                                    i,
                                    select_key,
                                    false,
                                )
                            };
                            candidates = Some(cands.iter().enumerate().map(to_rich).collect());
                        }
                    } else {
                        // Dict中无结果，直接返回
                        segments_ret.push((codes.iter().collect(), codes, tag));
                    }
                }
                Tag::Punctuation((idx, has_selection)) => {
                    match self.get_punctuation(&codes, idx, has_selection, !at_last) {
                        Some((punc_text, cands)) => {
                            reduce_space = !cands.is_empty();
                            segments_ret.push((punc_text, codes, tag));
                            if at_last && !cands.is_empty() {
                                candidates = Some(cands);
                            }
                        }
                        _ => {
                            segments_ret.push((codes.iter().collect(), codes, tag));
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
                    segments_ret.push((text, codes, tag));
                }
                _ => {
                    // 如果unknow段
                    let start = (reduce_space && codes[0] == ' ') as usize;
                    if reduce_space {
                        reduce_space = false;
                    }
                    let text = &codes[start..];
                    segments_ret.push((text.iter().collect(), codes.to_vec(), tag));
                }
            };
        }
        AnalysisResult {
            segments: segments_ret,
            candidates,
        }
    }

    /// 搜索候选。普通模式返回 CandidateView 切片（借用 arena），反查模式返回 owned 的 CandidateRich。
    fn search_candidates<'a>(
        &'a self,
        code: &[char],
        selection: &CodeSelection,
        no_cands: bool,
    ) -> Option<(Vec<CandidateView<'a>>, bool)> {
        let dict = &self.dicts[selection.dict_idx].2;
        let mut code = code;
        if selection.dict_idx > 0 {
            code = &code[1..];
        }
        if selection.has_selection {
            code = &code[..code.len() - 1];
        }
        let slice = if selection.has_pagination {
            // 过滤PAGE_UP和PAGE_DOWN
            let code = code
                .iter()
                .filter(|&&c| c != PAGE_UP && c != PAGE_DOWN)
                .copied()
                .collect::<Vec<_>>();
            dict.search(&code)?
        } else {
            dict.search(code)?
        };
        // 翻页后的窗口
        let start = (selection.page_no * self.page_count).min(slice.len() % self.page_count);
        let slice = &slice[start..(start + self.page_count).min(slice.len())];
        // 是否唯一只针对实际的查询结果，与selection无关。
        let unique = slice.len() <= 1;
        let cands = if selection.sel_idx > 0 || no_cands {
            let index = if selection.sel_idx >= slice.len() {
                0
            } else {
                selection.sel_idx
            };
            &slice[index..index + 1]
        } else {
            slice
        };
        if selection.dict_idx > 0 {
            // 反查时需要从 main_dict_code_map 构建新的 code
            self.candidates_remap_code(cands)
                .map(|cands| (cands, unique))
        } else {
            Some((cands.to_vec(), unique))
        }
    }

    fn candidates_remap_code<'a>(
        &self,
        cands: &[CandidateView<'a>],
    ) -> Option<Vec<CandidateView<'a>>> {
        let code_map = &self.main_dict_code_map;
        let cands: Vec<CandidateView<'a>> = cands
            .iter()
            .map(|cand| {
                let mut code = String::new();
                for (i, ch) in cand.text.chars().enumerate() {
                    if i > 0 {
                        code.push(' ');
                    }
                    let part = code_map.get(&ch).map(String::as_str).unwrap_or("_");
                    code.push_str(part);
                }
                // 由于 code 是动态生成的，需要泄漏字符串以获得 'a 生命周期
                // 这里只在反查时发生，数量很少，可以接受
                let code: &'a str = Box::leak(code.into_boxed_str());
                CandidateView {
                    code,
                    text: cand.text,
                    weight: cand.weight,
                }
            })
            .collect();
        Some(cands)
    }

    fn segments(&self, input: &[char]) -> Vec<(Vec<char>, Tag)> {
        let mut segments: Vec<(Vec<char>, Tag)> = vec![];
        let mut iter = input.iter().peekable();
        let mut unknown_chars = vec![];
        while (&iter.peek()).is_some() {
            let seg = if let Some(seg) = self.match_seg_escape(&mut iter) {
                // println!("matched escape");
                seg
            } else if let Some(seg) = self.match_seg_puncs(&mut iter) {
                // println!("matched puncs");
                seg
            } else if let Some(seg) = self.match_seg_code(&mut iter) {
                // println!("matched code: {:?}", seg.0);
                seg
            } else {
                let ch = iter.next().unwrap();
                unknown_chars.push(*ch);
                // println!("matched unknown: {:?}", unknown_chars);
                // 使连续的unknown字符为一段，而不是每个字符单独分段
                continue;
            };
            if !unknown_chars.is_empty() {
                segments.push((unknown_chars.to_vec(), Tag::Unknown));
                unknown_chars.clear();
            }
            segments.push(seg);
        }
        if !unknown_chars.is_empty() {
            segments.push((unknown_chars.to_vec(), Tag::Unknown));
        }
        segments
    }

    fn get_punctuation(
        &self,
        chars: &[char],
        sel_idx: usize,
        has_selection: bool,
        no_cands: bool,
    ) -> Option<(String, Vec<CandidateRich>)> {
        self.punctuations.get(&chars[0]).map(|ps| {
            // 如果ps["a", "b", "c"]的长度为3，而chars.len()为7，最终result将变成cca
            // 如果ps["a", "b", "c"]的长度为3，而chars.len()为2，最终result将变成b
            // 另外如果有select，则在最后一轮时直接从cands中选择对应的punc
            let mut result: Vec<&str> = vec![];
            let mut cands: &[String] = &ps[..];
            let mut c = if has_selection {
                chars.len() - 1
            } else {
                chars.len()
            };
            while c > 0 {
                // 如果当前c小于等于ps.len，这是最后一轮，选择c - 1的元素
                // 如果当前c大于ps.len，此轮从ps中选择最后一个元素(ps.len() - 1)。
                let i = if c <= ps.len() { c - 1 } else { ps.len() - 1 };
                result.push(&ps[i]);
                cands = &ps[i..];
                c = c - ps.len().min(c);
            }
            let cands: Vec<CandidateRich> = if has_selection || no_cands {
                // 将result最后一个元素修改为cands[i_cand]对应的内容
                if let (Some(punc), Some(last)) =
                    (cands.get(sel_idx.min(cands.len() - 1)), result.last_mut())
                {
                    *last = punc;
                }
                vec![]
            } else {
                let cands = cands
                    .iter()
                    .enumerate()
                    .map(|(i, pu)| CandidateRich {
                        code: String::new(),
                        text: pu.clone(),
                        weight: 0,
                        origin: chars.to_vec(),
                        order: i,
                        select_key: self.selection_keys.get(i).copied().unwrap_or('_'),
                        unique: false,
                    })
                    .collect();
                cands
            };
            Some((result.join(""), cands))
        })?
    }

    fn match_seg_escape(&self, chars: &mut Peekable<Iter<'_, char>>) -> Option<(Vec<char>, Tag)> {
        if let (Some(first), Some(pair)) = (chars.peek(), self.escape_pair)
            && **first == pair[0]
        {
            let mut result = vec![**first];
            chars.next();
            while let Some(ch) = chars.next() {
                result.push(*ch);
                if *ch == pair[1] {
                    break;
                }
            }
            Some((result, Tag::Escape((pair[0], pair[1]))))
        } else {
            None
        }
    }

    fn match_seg_puncs(&self, chars: &mut Peekable<Iter<'_, char>>) -> Option<(Vec<char>, Tag)> {
        if let Some(first) = chars.peek()
            && self.punctuations.contains_key(*first)
        {
            let first = **first;
            let mut result = vec![first];
            chars.next();
            while let Some(ch) = chars.peek() {
                if **ch == first {
                    // 只追加相同的标点符号映射字符
                    result.push(**ch);
                    chars.next();
                } else if let Some(i) = self.selection_keys.iter().position(|&k| k == **ch) {
                    // 如果与首个不同，检查是否是selection_key
                    result.push(**ch);
                    chars.next();
                    return Some((result, Tag::Punctuation((i, true))));
                } else {
                    break;
                }
            }
            Some((result, Tag::Punctuation((0, false))))
        } else {
            None
        }
    }

    fn match_seg_code(&self, chars: &mut Peekable<Iter<'_, char>>) -> Option<(Vec<char>, Tag)> {
        if let Some(first) = chars.peek() {
            let first = **first;
            let (dict_idx, has_prefix) = self
                .dicts
                .iter()
                .enumerate()
                .find_map(|(i, d)| (d.0 == first).then(|| (i, true)))
                .unwrap_or((0, false));
            let dict = &self.dicts[dict_idx].2;
            let mut codes = vec![];
            if has_prefix {
                chars.next();
            }
            let mut page_no = 0;
            let mut sel_idx = 0;
            let mut has_selection = false;
            let mut has_pagination = false;
            while let Some(ch) = chars.peek() {
                codes.push(**ch);
                if dict.reachable(&codes) {
                    chars.next();
                    continue;
                }
                // 只有存在一个有效的code时，才可以继续判断后面是否跟着翻页或选择的字符
                if codes.len() > 1 {
                    if **ch == PAGE_DOWN {
                        // 检查当前字符是否是翻页键
                        page_no += 1;
                        has_pagination = true;
                        chars.next();
                    } else if **ch == PAGE_UP {
                        // 检查当前字符是否是翻页键
                        page_no -= 1;
                        has_pagination = true;
                        chars.next();
                    } else if let Some(i) = self.selection_keys.iter().position(|&k| k == **ch) {
                        // 检查当前字符是否是selection_key
                        sel_idx = i;
                        has_selection = true;
                        chars.next();
                        break;
                    } else {
                        // codes在Dict中已无结果，将当前字符吐出，但当前字符还未取出，因此下一轮仍将计算当前字符
                        codes.pop();
                        break;
                    }
                } else {
                    if has_prefix {
                        // 除prefix外，首字符在Dict中无结果，下一阶段，当前未被取出的字符会被当作unknown
                        // 由于prefix被取出，因此将其当作unknown
                        return Some((vec![first], Tag::Unknown));
                    } else {
                        // 下一阶段，当前未被取出的字符会被当作unknown
                        return None;
                    }
                }
            }
            // 添加first到result的首位
            if has_prefix {
                codes.insert(0, first);
            }
            Some((
                codes,
                Tag::Code(CodeSelection {
                    page_no,
                    sel_idx,
                    dict_idx,
                    has_selection,
                    has_pagination,
                }),
            ))
        } else {
            None
        }
    }
}

// code = ['.', '.', '!', '!', ';', '!']
// want convert to [('.', 2), ('!', 2), (';', 1), ('!', 1)]
// fn compact_vec(v: &[char]) -> Vec<(char, usize)> {
//     let mut result = Vec::new();
//     let mut count = 1;
//     let mut last_char = v[0];
//     for c in v.iter().skip(1) {
//         if *c == last_char {
//             count += 1;
//         } else {
//             result.push((last_char, count));
//             last_char = *c;
//             count = 1;
//         }
//     }
//     result.push((last_char, count));
//     result
// }

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct CodeSelection {
    pub page_no: usize,
    pub sel_idx: usize,
    pub dict_idx: usize,
    pub has_selection: bool,
    pub has_pagination: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Tag {
    Code(CodeSelection),
    Punctuation((usize, bool)),
    Escape((char, char)),
    Unknown,
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
        code: String,
        text: String,
        weight: i32,
        origin: Vec<char>,
        order: usize,
        select_key: char,
        unique: bool,
    ) -> Self {
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
    /// (text, origin_input, tag)
    pub segments: Vec<(String, Vec<char>, Tag)>,
    pub candidates: Option<Vec<CandidateRich>>,
}

#[cfg(test)]
mod tests {

    use crate::test_utils::gen_test_config;

    use super::*;

    fn gen_entries() -> String {
        r#"
a 嗯 1
aa 嗯嗯 1
ab 嗯毕 1
ac 嗯渗 1
ad 嗯弟 1
aaa 嗯嗯嗯 1
abc 嗯毕渗 1
abcd 嗯毕渗弟 1
b 毕 1
ba 毕嗯 1
bb 毕毕 1
bc 毕渗 1
bd 毕弟 1
c 渗 1
ca 渗嗯 1
cb 渗毕 1
cc 渗渗 1
cd 渗弟 1
d 弟 1
da 弟嗯 1
db 弟毕 1
dc 弟渗 1
dd 弟弟 1"#
            .replace(' ', "\t")
    }

    fn test_config() -> Config {
        let raw = &gen_test_config();
        toml::from_str(raw).unwrap()
    }

    #[test]
    fn test_analyzer() {
        let dict = Dict::from_str_with_config(&gen_entries(), test_config()).unwrap();
        let analyzer = InputAnalyzer::new(dict, None);
        let inputs = vec![
            ("a cIzk", vec!["嗯", "", "渗嗯", "zk"]),
            ("a cI@abc", vec!["嗯", "", "渗嗯", "@", "嗯毕渗"]),
            (
                "a  cIzk,,,[]I]]",
                vec!["嗯", " ", "渗嗯", "zk", "……", "「", "”", "”"],
            ),
            (
                "zk`zk`c,cua.hcP",
                vec!["zk", "zk", "渗", "，", "渗", "u", "嗯", "。", "h", "渗渗"],
            ),
        ];
        for (input, expect) in inputs.into_iter() {
            let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
            let texts: Vec<String> = result.segments.into_iter().map(|seg| seg.0).collect();
            assert_eq!(texts, expect);
        }
    }

    #[test]
    fn test_analyzer_with_sec_dict() {
        let dict = Dict::from_str_with_config(&gen_entries(), test_config()).unwrap();
        let dict_sec = Dict::from_str_with_config(&gen_entries(), test_config()).unwrap();
        let analyzer = InputAnalyzer::new(dict, Some((dict_sec, "R".to_string())));
        let inputs = vec![
            (
                "aaaaaa aaaaa",
                vec!["嗯嗯嗯", "嗯嗯嗯", " ", "嗯嗯嗯", "嗯嗯"],
            ),
            ("aaa8 ", vec!["嗯嗯嗯", " "]),
            ("a cI@abc", vec!["嗯", "", "渗嗯", "嗯毕渗"]),
            ("@aaac@cP@@@", vec!["嗯嗯嗯", "渗", "渗渗", "@", "@", "@"]),
        ];
        for (input, expect) in inputs.into_iter() {
            let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
            let texts: Vec<String> = result.segments.into_iter().map(|seg| seg.0).collect();
            assert_eq!(texts, expect);
        }
    }

    impl CodeSelection {
        fn with_sel(sel_idx: usize) -> Self {
            Self {
                sel_idx,
                has_selection: true,
                ..Default::default()
            }
        }
        fn with_sec_dict(dict_idx: usize) -> Self {
            Self {
                dict_idx,
                ..Default::default()
            }
        }
        fn with_sec_dict_and_sel(dict_idx: usize, sel_idx: usize) -> Self {
            Self {
                dict_idx,
                sel_idx,
                has_selection: true,
                ..Default::default()
            }
        }
    }

    impl ToString for Tag {
        fn to_string(&self) -> String {
            match self {
                Tag::Code(code_selection) => format!(
                    "C_di={}_si={}_pn={}_hs={}_hp={}",
                    code_selection.dict_idx,
                    code_selection.sel_idx,
                    code_selection.page_no,
                    code_selection.has_selection,
                    code_selection.has_pagination
                ),
                Tag::Punctuation((idx, has_selection)) => {
                    format!("P_i={}_hs={}", idx, has_selection)
                }
                Tag::Escape(_) => "escape".to_string(),
                Tag::Unknown => "unknown".to_string(),
            }
        }
    }

    #[test]
    fn test_segments_with_sec_dict() {
        let dict = Dict::from_str_with_config(&gen_entries(), test_config()).unwrap();
        let dict_sec = Dict::from_str_with_config(&gen_entries(), test_config()).unwrap();
        let analyzer = InputAnalyzer::new(dict, Some((dict_sec, "R".to_string())));
        let samples: Vec<(usize, &str, Vec<&str>, Vec<Tag>)> = vec![
            (
                1,
                "IIII@abc@ahI cu",
                vec!["IIII", "@abc", "@a", "hI ", "c", "u"],
                vec![
                    Tag::Unknown,
                    Tag::Code(CodeSelection::with_sec_dict(1)),
                    Tag::Code(CodeSelection::with_sec_dict(1)),
                    Tag::Unknown,
                    Tag::Code(CodeSelection::default()),
                    Tag::Unknown,
                ],
            ),
            (
                2,
                "a@a@xa @a",
                vec!["a", "@a", "@", "x", "a", " ", "@a"],
                vec![
                    Tag::Code(CodeSelection::default()),
                    Tag::Code(CodeSelection::with_sec_dict(1)),
                    Tag::Unknown,
                    Tag::Unknown,
                    Tag::Code(CodeSelection::default()),
                    Tag::Unknown,
                    Tag::Code(CodeSelection::with_sec_dict(1)),
                ],
            ),
            (
                3,
                "aaxx@abca@xx@aPPP@P",
                vec!["aa", "xx", "@abc", "a", "@", "xx", "@aP", "PP", "@", "P"],
                vec![
                    Tag::Code(CodeSelection::default()),
                    Tag::Unknown,
                    Tag::Code(CodeSelection::with_sec_dict(1)),
                    Tag::Code(CodeSelection::default()),
                    Tag::Unknown,
                    Tag::Unknown,
                    Tag::Code(CodeSelection::with_sec_dict_and_sel(1, 3)),
                    Tag::Unknown,
                    Tag::Unknown,
                    Tag::Unknown,
                ],
            ),
        ];
        for (no, input, expected, expected_tags) in samples.into_iter() {
            let segments = analyzer.segments(input.chars().collect::<Vec<_>>().as_slice());
            // println!("Segments: {:?}", segments);
            println!("sample: [{no}] {input}");
            let (segments, tags): (Vec<String>, Vec<Tag>) = segments
                .into_iter()
                .map(|seg| (seg.0.iter().collect::<String>(), seg.1))
                .unzip();
            assert_eq!(segments, expected);
            assert_eq!(
                tags.into_iter()
                    .enumerate()
                    .map(|(i, tag)| format!("[{i}]<{}>", tag.to_string()))
                    .collect::<Vec<_>>(),
                expected_tags
                    .into_iter()
                    .enumerate()
                    .map(|(i, tag)| format!("[{i}]<{}>", tag.to_string()))
                    .collect::<Vec<_>>(),
            );
        }
    }

    #[test]
    fn test_segments() {
        let dict = Dict::from_str_with_config(&gen_entries(), test_config()).unwrap();
        let analyzer = InputAnalyzer::new(dict, None);
        let samples: Vec<(usize, &str, Vec<&str>, Vec<Tag>)> = vec![
            (
                0,
                "a zk,,cO",
                vec!["a", " zk", ",,", "cO"],
                vec![
                    Tag::Code(CodeSelection::default()),
                    Tag::Unknown,
                    Tag::Punctuation((0, false)),
                    Tag::Code(CodeSelection::with_sel(2)),
                ],
            ),
            (
                1,
                "   A  zk,,;IaII",
                vec!["   A  zk", ",,", ";I", "aI", "I"],
                vec![
                    Tag::Unknown,
                    Tag::Punctuation((0, false)),
                    Tag::Punctuation((1, true)),
                    Tag::Code(CodeSelection::with_sel(1)),
                    Tag::Unknown,
                ],
            ),
            (
                2,
                "IOcaK  ",
                vec!["IO", "ca", "K  "],
                vec![
                    Tag::Unknown,
                    Tag::Code(CodeSelection::default()),
                    Tag::Unknown,
                ],
            ),
            (
                3,
                "8  8ahcgccbPPP;8...8",
                vec!["8  8", "a", "h", "c", "g", "cc", "bP", "PP", ";8", "...8"],
                vec![
                    Tag::Unknown,
                    Tag::Code(CodeSelection::default()),
                    Tag::Unknown,
                    Tag::Code(CodeSelection::default()),
                    Tag::Unknown,
                    Tag::Code(CodeSelection::default()),
                    Tag::Code(CodeSelection::with_sel(3)),
                    Tag::Unknown,
                    Tag::Punctuation((7, true)),
                    Tag::Punctuation((7, true)),
                ],
            ),
            (
                4,
                "aaxx  `hello`world`@a,cua.hcI",
                vec!["aa", "xx  ", "`hello`", "worl", "d", "`@a,cua.hcI"],
                vec![
                    Tag::Code(CodeSelection::default()),
                    Tag::Unknown,
                    Tag::Escape(('`', '`')),
                    Tag::Unknown,
                    Tag::Code(CodeSelection::default()),
                    Tag::Escape(('`', '`')),
                ],
            ),
            (
                5,
                "aaxx@abca@xx@aPPP@P",
                vec!["aa", "xx@", "abc", "a", "@xx@", "aP", "PP@P"],
                vec![
                    Tag::Code(CodeSelection::default()),
                    Tag::Unknown,
                    Tag::Code(CodeSelection::default()),
                    Tag::Code(CodeSelection::default()),
                    Tag::Unknown,
                    Tag::Code(CodeSelection::with_sel(3)),
                    Tag::Unknown,
                ],
            ),
        ];
        for (no, input, expected, expected_tags) in samples.into_iter() {
            let segments = analyzer.segments(input.chars().collect::<Vec<_>>().as_slice());
            // println!("Segments: {:?}", segments);
            println!("sample: [{no}] {input}");
            let (segments, tags): (Vec<String>, Vec<Tag>) = segments
                .into_iter()
                .map(|seg| (seg.0.iter().collect::<String>(), seg.1))
                .unzip();
            assert_eq!(segments, expected);
            assert_eq!(
                tags.into_iter()
                    .enumerate()
                    .map(|(i, tag)| format!("[{i}]<{}>", tag.to_string()))
                    .collect::<Vec<_>>(),
                expected_tags
                    .into_iter()
                    .enumerate()
                    .map(|(i, tag)| format!("[{i}]<{}>", tag.to_string()))
                    .collect::<Vec<_>>(),
            );
        }
    }

    // #[test]
    // fn test_compact_vec() {
    //     let input = vec!['.', '.', '!', '!', ';', '!'];
    //     let expected = vec![('.', 2), ('!', 2), (';', 1), ('!', 1)];
    //     let result = compact_vec(&input);
    //     assert_eq!(result, expected);
    // }
}
