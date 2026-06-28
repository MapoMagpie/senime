use std::{collections::HashMap, fs::File, io::Read, iter::Peekable, path::PathBuf, slice::Iter};

use ahash::AHashMap;
use serde::Deserialize;

use crate::dict::{Candidate, Dict};
use crate::util::resolve_relative_path;

/// 码表元信息，描述一个码表的触发字符、提示文字和路径。
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DictMeta {
    /// 触发该码表的前缀字符。主码表为 '\0'，反查码表通常为 '@'。
    #[serde(default)]
    pub trigger: char,
    /// 提示文字，如 "反"。主码表通常为空。
    #[serde(default)]
    pub hint: String,
    /// 码表文件路径（相对于配置文件）
    pub path: String,
}

/// 输入法配置。可从 TOML 或 JSON 反序列化。
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    /// 码表列表。第一个元素为主码表，其余为反查码表等。
    pub dicts: Vec<DictMeta>,
    /// 选重键列表
    pub selection_keys: [char; 9],
    /// 标点映射
    pub punctuations: HashMap<char, Vec<String>>,
    /// 逃逸符对（开闭字符）
    pub escape_pair: Option<[char; 2]>,
    /// 是否去除两端的逃逸字符
    pub trim_escape_pair: bool,
    /// 每页的数量
    pub page_count: usize,
}

impl Config {
    pub(crate) fn patch_punctuations(&mut self, patch: HashMap<char, Vec<String>>) {
        let mut patch = patch;
        std::mem::swap(&mut self.punctuations, &mut patch);
        self.punctuations.extend(patch);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dicts: vec![],
            selection_keys: default_selection_keys(),
            punctuations: default_punctuations(),
            escape_pair: default_escape_pair(),
            trim_escape_pair: true,
            page_count: 9,
        }
    }
}

fn default_selection_keys() -> [char; 9] {
    ['1', '2', '3', '4', '5', '6', '7', '8', '9']
}

// ',' : { commit: ， }
// '.' : { commit: 。 }
// '<' : [ 《, 〈, «, ‹, ˂, ˱ ]
// '>' : [ 》, 〉, », ›, ˃, ˲ ]
// '?' : { commit: ？ }
// ';' : { commit: ； }
// ';' : [ ；, ：, ":" ]
// ':' : { commit: ： }
// "'": { pair: [ '‘', '’' ] }
// '"' : { pair: [ '“', '”' ] }
// '\' : [ 、, '\', ＼ ]
// '/' : [ ？, 、, '/', ／, ÷ ]
// '|' : [ '|', ·, '·' , ｜, '§', '¦', '‖', ︴ ]
// '`' : [ '`', ‵, ‶, ‷, ′, ″, ‴, ⁗ ]
// '~' : [ '~', ～, ~~~, ˜, ˷, ⸯ, ≈, ≋, ≃, ≅, ≇, ∽, ⋍, ≌, ﹏, ﹋, ﹌, ︴ ]
// '!' : { commit: ！ }
// # '@' : [ '@', ©, ®, ℗ ]
// '#' : [ '#', № ]
// '%' : [ '%', ％, '°', '℃', ‰, ‱, ℉, ℅, ℆, ℀, ℁, ⅍ ]
// '$' : [ ￥, '$', '€', '£', '¥', '¢', '¤', ₩ ]
// '^' : { commit: …… }
// '&' : '&'
// '*' : [ '*', ＊, ·, ‧, ・, ･, ×, ※, ❂, ⁂, ☮, ☯, ☣ ]
// '(' : （
// ')' : ）
// '-' : '-'
// '_' : ——
// '+' : '+'
// '=' : [ '=', 々, 〃 ]
// '[' : [ 「, '“', 【, 〔, ［, 〚, 〘 ]
// ']' : [ 」, '”', 】, 〕, ］, 〛, 〙 ]
// '{' : [ "{", 〖, 『 , ｛ ]
// '}' : [ "}", 〗, 』 , ｝ ]
fn default_punctuations() -> HashMap<char, Vec<String>> {
    let punctuations = vec![
        (',', vec!["，", ",", "……"]),
        ('.', vec!["。", ".", "……"]),
        ('!', vec!["！", "!"]),
        ('/', vec!["？", "/"]),
        (';', vec!["；", "：", ";"]),
        ('[', vec!["「", "“", "[", "【"]),
        (']', vec!["」", "”", "]", "】"]),
        ('\\', vec!["、", "\\"]),
        ('|', vec!["·", "|"]),
        ('_', vec!["——", "_"]),
        ('<', vec!["《", "<"]),
        ('>', vec!["》", ">"]),
        ('\'', vec!["‘", "’"]),
        ('~', vec!["~", "～", "~~~"]),
        ('(', vec!["（", "(", "『"]),
        (')', vec!["）", ")", "』"]),
    ];
    let mut map = HashMap::new();
    punctuations.into_iter().for_each(|(ch, puncs)| {
        map.insert(ch, puncs.iter().map(|s| s.to_string()).collect());
    });
    map
}

fn default_escape_pair() -> Option<[char; 2]> {
    Some(['`', '`'])
}

/// 从配置文件路径构建完整的 `InputAnalyzer`。
///
/// 支持 `.toml` 配置文件（自动解析码表路径、标点覆盖、反查字典），
/// 或直接传入 `.txt` / `.bin` 码表文件路径（使用默认配置）。
pub fn load_input_analyzer<P: Into<PathBuf>>(path: P) -> Result<InputAnalyzer, String> {
    let path = path.into();
    match path.extension().and_then(|e| e.to_str()) {
        Some("toml") => {
            let mut content = String::new();
            File::open(&path)
                .map_err(|e| format!("无法读取配置文件 {:?}: {e}", path))?
                .read_to_string(&mut content)
                .map_err(|e| format!("无法读取配置文件内容: {e}"))?;
            let mut config: Config =
                toml::from_str(&content).map_err(|e| format!("无法解析配置文件: {e}"))?;
            if config.dicts.is_empty() {
                return Err("配置文件中 dicts 为空，请指定至少一个码表".to_string());
            }
            config.patch_punctuations(default_punctuations());
            // 第一个元素的 trigger_char 设为空字符
            config.dicts[0].trigger = '\0';
            // 加载所有码表
            let mut dicts: Vec<(DictMeta, Dict)> = Vec::with_capacity(config.dicts.len());
            for meta in &config.dicts {
                let dict_path = resolve_relative_path(&path, &meta.path);
                let dict = Dict::try_load(dict_path)?;
                dicts.push((meta.clone(), dict));
            }
            Ok(InputAnalyzer::new(config, dicts))
        }
        _ => {
            let dict = Dict::try_load(&path)?;
            Ok(InputAnalyzer::new(
                Config::default(),
                vec![(
                    DictMeta {
                        trigger: '\0',
                        hint: String::new(),
                        path: path.to_str().unwrap_or("").to_string(),
                    },
                    dict,
                )],
            ))
        }
    }
}

// ⇞ (U+21DE) 和 ⇟ (U+21DF)
pub const PAGE_UP: char = '⇞';
pub const PAGE_DOWN: char = '⇟';

#[derive(Debug)]
pub struct InputAnalyzer {
    dicts: Vec<(DictMeta, Dict)>,
    main_dict_code_map: AHashMap<char, (u32, u16)>,
    escape_pair: Option<[char; 2]>,
    trim_escape_pair: bool,
    selection_keys: [char; 9],
    punctuations: HashMap<char, Vec<String>>,
    page_count: usize,
}

impl InputAnalyzer {
    /// 创建 InputAnalyzer。
    ///
    /// - `config`: 输入法配置（标点、选重键等）
    /// - `dicts`: 码表数组，与 config.dicts 一一对应
    pub fn new(config: Config, dicts: Vec<(DictMeta, Dict)>) -> Self {
        let Config {
            dicts: _,
            selection_keys,
            punctuations,
            escape_pair,
            trim_escape_pair,
            page_count,
        } = config;
        let mut main_dict_code_map = AHashMap::<char, (u32, u16)>::new();
        // 如果有副码表（非主码表），从主码表中构建单字→最长编码的映射
        if dicts.len() > 1 {
            let main_dict = &dicts[0].1;
            for cand in main_dict.candidates.iter() {
                let text_chars = main_dict.get_str(cand.text).chars().collect::<Vec<_>>();
                if text_chars.len() == 1 {
                    let ch = text_chars.first().unwrap();
                    if let Some(existing) = main_dict_code_map.get_mut(ch) {
                        if cand.code.1 > existing.1 {
                            *existing = cand.code;
                        }
                    } else {
                        main_dict_code_map.insert(*ch, cand.code);
                    }
                }
            }
        }
        // 确保主码表在第一个位置（trigger_char == '\0'）
        Self {
            dicts,
            selection_keys,
            main_dict_code_map,
            escape_pair,
            trim_escape_pair,
            punctuations,
            page_count,
        }
    }

    pub fn main_dict(&self) -> &Dict {
        &self.dicts[0].1
    }

    /// 获取码表元信息列表
    pub fn dict_metas(&self) -> Vec<&DictMeta> {
        self.dicts.iter().map(|(m, _)| m).collect()
    }
}

impl InputAnalyzer {
    pub fn analyze(&self, input: &[char]) -> AnalysisResult {
        if input.is_empty() {
            return AnalysisResult {
                segments: vec![],
                pending: false,
                candidates: None,
            };
        }
        let segments_raw = self.segments(input);
        let segment_len = segments_raw.len();
        let mut reduce_space = false;
        let mut segments: Vec<(String, Vec<char>, Tag)> = vec![];
        let mut candidates: Option<Vec<CandidateRich>> = None;
        // 是否未决，在这些条件下会被设为`false`，表示已决: `unknown | escape finished(完成的逃逸块) | dict search unique(码表的查询结果是唯一) | punctuation with selection(标点符号带候选选择)`
        // 其他条件下会保持`true`:  `punctuation no selection | escape unfinished | dict search none or not unique`
        let mut pending = true;
        for (i, (codes, tag)) in segments_raw.into_iter().enumerate() {
            let at_last = i == segment_len - 1;
            match tag {
                Tag::Code(selection) => {
                    match self.search_candidates(&codes, &selection, !at_last) {
                        Some((cands, unique, rel_page_no)) => {
                            reduce_space = !unique;
                            let mut codes = codes;
                            if selection.has_pagination {
                                codes = codes
                                    .into_iter()
                                    .filter(|c| c != &PAGE_UP && c != &PAGE_DOWN)
                                    .collect::<Vec<_>>();
                                if selection.page_no > 0 {
                                    (0..rel_page_no).for_each(|_| codes.push(PAGE_DOWN));
                                }
                            }
                            // 当前是最后一段时，若当前所查询的码表不是主码表，则在text前面加上`hint`
                            if at_last && selection.dict_idx > 0 {
                                let hint = self.dicts[selection.dict_idx].0.hint.clone();
                                segments.push((hint + "|" + &cands[0].text, codes, tag));
                                pending = true;
                            } else {
                                segments.push((cands[0].text.to_string(), codes, tag));
                                if unique {
                                    pending = false;
                                }
                            }
                            // candidates
                            if at_last && !unique {
                                candidates = Some(cands);
                            }
                        }
                        None => {
                            // 当前是最后一段时，`codes` 其实是反复触发符，则展示反查的 `hint`
                            if at_last && selection.dict_idx > 0 && codes.len() == 1 {
                                segments.push((
                                    self.dicts[selection.dict_idx].0.hint.clone(),
                                    codes,
                                    tag,
                                ));
                            } else {
                                segments.push((codes.iter().collect(), codes, tag));
                            }
                        }
                    }
                }
                Tag::Punctuation((idx, has_selection)) => {
                    match self.get_punctuation(&codes, idx, has_selection, !at_last) {
                        Some((punc_text, cands)) => {
                            segments.push((punc_text, codes, tag));
                            if at_last && !cands.is_empty() {
                                candidates = Some(cands);
                            }
                            if has_selection {
                                pending = false;
                            } else {
                                reduce_space = true;
                            }
                        }
                        _ => {
                            if at_last {
                                pending = false;
                            }
                            segments.push((codes.iter().collect(), codes, tag));
                        }
                    }
                }
                Tag::Escape((_, end)) => {
                    let text =
                        if self.trim_escape_pair && codes.len() > 2 && codes.last() == Some(&end) {
                            // 当escape闭合时，移除两端的逃逸字符，
                            // 如果escape_key前后相连(其中为空, last_i < 1)出现，保持其原样
                            if at_last {
                                pending = false;
                            }
                            codes[1..codes.len() - 1].iter().collect()
                        } else {
                            codes.iter().collect()
                        };
                    segments.push((text, codes, tag));
                }
                _ => {
                    // 如果unknow段
                    let start = (reduce_space && codes[0] == ' ') as usize;
                    if reduce_space {
                        reduce_space = false;
                    }
                    if at_last {
                        pending = false;
                    }
                    let text = &codes[start..];
                    segments.push((text.iter().collect(), codes, tag));
                }
            };
        }
        AnalysisResult {
            segments,
            pending,
            candidates,
        }
    }

    /// 搜索候选。普通模式返回 CandidateView 切片（借用 arena），反查模式返回 owned 的 CandidateRich。
    fn search_candidates<'a>(
        &'a self,
        codes: &[char],
        selection: &CodeSelection,
        no_cands: bool,
    ) -> Option<(Vec<CandidateRich>, bool, usize)> {
        let dict = &self.dicts[selection.dict_idx].1;
        let mut s_codes = codes;
        if selection.dict_idx > 0 {
            s_codes = &s_codes[1..];
        }
        if selection.has_selection {
            s_codes = &s_codes[..s_codes.len() - 1];
        }
        if s_codes.is_empty() {
            return None;
        }
        let slice = if selection.has_pagination {
            // 过滤PAGE_UP和PAGE_DOWN
            dict.search(
                &s_codes
                    .iter()
                    .filter(|&&c| c != PAGE_UP && c != PAGE_DOWN)
                    .copied()
                    .collect::<Vec<_>>(),
            )?
        } else {
            dict.search(s_codes)?
        };
        // 翻页后的窗口
        let mut start = selection.page_no * self.page_count;
        if start >= slice.len() {
            let m0d = slice.len() % self.page_count;
            if m0d == 0 {
                start = slice.len() - self.page_count;
            } else {
                start = slice.len() - m0d;
            }
        }
        let rel_page_no = start / self.page_count;
        let slice = &slice[start..(start + self.page_count).min(slice.len())];
        // 是否唯一，如果实际的查询结果只有一个，或者直接存在selection_key
        // let unique = slice.len() <= 1 || selection.has_selection;
        let unique = selection.has_selection;
        let cands = if selection.sel_idx > 0 || no_cands {
            let index = if selection.sel_idx >= slice.len() {
                slice.len() - 1
            } else {
                selection.sel_idx
            };
            &slice[index..index + 1]
        } else {
            slice
        };
        if selection.dict_idx > 0 {
            let code_map = &self.main_dict_code_map;
            let main_dict = &self.dicts[0].1;
            // 反查时需要从 main_dict_code_map 构建新的 code
            // self.candidates_remap_code(cands)
            //     .map(|cands| (cands, unique))
            let enrich = |(i, cand): (usize, &Candidate)| -> CandidateRich {
                let select_key = self.selection_keys.get(i).copied().unwrap_or(' ');
                let text = dict.get_str(cand.text);
                let mut re_code = String::new();
                for (i, ch) in text.chars().enumerate() {
                    if i > 0 {
                        re_code.push(' ');
                    }
                    let part = code_map
                        .get(&ch)
                        .map(|range| main_dict.get_str(*range))
                        .unwrap_or("_");
                    re_code.push_str(part);
                }
                CandidateRich::new(
                    re_code,
                    text.to_owned(),
                    cand.weight,
                    codes.to_vec(),
                    i,
                    select_key,
                )
            };
            Some((
                cands.iter().enumerate().map(enrich).collect(),
                unique,
                rel_page_no,
            ))
        } else {
            let enrich = |(i, cand): (usize, &Candidate)| -> CandidateRich {
                let select_key = self.selection_keys.get(i).copied().unwrap_or(' ');
                CandidateRich::new(
                    dict.get_str(cand.code).to_owned(),
                    dict.get_str(cand.text).to_owned(),
                    cand.weight,
                    codes.to_vec(),
                    i,
                    select_key,
                )
            };
            Some((
                cands.iter().enumerate().map(enrich).collect(),
                unique,
                rel_page_no,
            ))
        }
    }

    fn segments(&self, input: &[char]) -> Vec<(Vec<char>, Tag)> {
        let mut segments: Vec<(Vec<char>, Tag)> = vec![];
        let mut iter = input.iter().peekable();
        let mut unknown_chars = vec![];
        while (&iter.peek()).is_some() {
            let seg = if let Some(seg) = self.match_seg_escape(&mut iter) {
                seg
            } else if let Some(seg) = self.match_seg_puncs(&mut iter) {
                seg
            } else if let Some(seg) = self.match_seg_code(&mut iter) {
                seg
            } else {
                let ch = iter.next().unwrap();
                unknown_chars.push(*ch);
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
                .find_map(|(i, d)| (d.0.trigger == first).then(|| (i, true)))
                .unwrap_or((0, false));
            let dict = &self.dicts[dict_idx].1;
            let mut codes = vec![];
            if has_prefix {
                chars.next();
            }
            let mut page_no: usize = 0;
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
                        page_no = page_no.saturating_add(1);
                        has_pagination = true;
                        chars.next();
                    } else if **ch == PAGE_UP {
                        // 检查当前字符是否是翻页键
                        page_no = page_no.saturating_sub(1);
                        has_pagination = true;
                        chars.next();
                    } else if let Some(i) = self.selection_keys.iter().position(|&k| k == **ch) {
                        // 检查当前字符是否是selection_key
                        sel_idx = i;
                        has_selection = true;
                        chars.next();
                        break;
                    } else {
                        // codes在Dict中已无结果，但当前字符还未取出，因此下一轮仍将计算当前字符
                        codes.pop();
                        break;
                    }
                } else {
                    if has_prefix {
                        // 除prefix外，首字符在Dict中无结果，下一阶段，当前未被取出的字符会被当作unknown
                        // 将单独的prefix作为Tag::Code段返回
                        return Some((
                            vec![first],
                            Tag::Code(CodeSelection {
                                dict_idx,
                                ..Default::default()
                            }),
                        ));
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
            if codes.is_empty() {
                None
            } else {
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
            }
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

/// 候选
/// `code`         编码
/// `text`         字词(码表查询结果)
/// `origin`       绝对完整的原始的输入，
///                与 `code` 区别是，当有反查时，`origin`  包含了触发反查的前缀如 `@` ，
///                以及用于选择候选的`selection_key` 如 `1, 2, 3` `U, I, O`
/// `select_key`   当前候选可用哪个按键进行选择，如 `1, 2, 3` `U, I, O`
#[derive(Debug)]
pub struct CandidateRich {
    pub code: String,
    pub text: String,
    pub weight: i32,
    pub origin: Vec<char>,
    pub order: usize,
    pub select_key: char,
}

impl CandidateRich {
    pub fn new(
        code: String,
        text: String,
        weight: i32,
        origin: Vec<char>,
        order: usize,
        select_key: char,
    ) -> Self {
        Self {
            code,
            text,
            weight,
            origin,
            order,
            select_key,
        }
    }
}

/// InputAnalyzer::analyze的结果，
/// `segments`   从码表中查询的结果是分段的形式，以便前端灵活使用。
///              分段的形式，典型的应用方式就是自动上屏，保留最后一段为未决的preedit，而前面的分段直接上屏。
///              每个分段中的元素为 `(text, origin, tag)`  `(字词, 原始输入, 分段的标签)`
/// `candidates` 仅对应最后一个分段
/// `pending`    最后一段的结果是否未决，前端可根据此项判断时是否要清空`input`(前端维护的当前输入状态)
///              比如按下了反查键`@`但是后面没其他输入，此时无候选，但前端应保持`@`在`input`中
pub struct AnalysisResult {
    pub segments: Vec<(String, Vec<char>, Tag)>,
    pub pending: bool,
    pub candidates: Option<Vec<CandidateRich>>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;

    fn gen_entries() -> String {
        r#"
a 啊 1
aa 啊 1
aaa 啊 1
ab 啊波 1
abc 啊波此 1
abcd 啊波此的 1
ac 啊此 1
ad 啊的 1
b 波 1
ba 波啊 1
bb 波 1
bc 波此 1
bd 波的 1
c 此 1
ca 此啊 1
cb 此波 1
cc 此 1
cd 此的 1
d 的 1
da 的啊 1
db 的波 1
dc 的此 1
dd 的 1
q 其0 1
q 其1 1
q 其2 1
q 其3 1
q 其4 1
q 其5 1
q 其6 1
q 其7 1
q 其8 1
q 其9 1
q 其10 1
q 其11 1
q 其12 1
q 其13 1
y 伊0 1
y 伊1 1
y 伊2 1
y 伊3 1
y 伊4 1
y 伊5 1
"#
        .replace(' ', "\t")
    }

    fn gen_test_config() -> Config {
        let raw = r#"
selection_keys = ["U","I","O","P","5","6","7","8","9"]
[punctuations]
',' = ["，", ",", "……"]
'.' = ["。", ".", "……"]
'!' = ["！", "!"]
'/' = ["？", "/"]
';' = ["：", "；", ";"]
'[' = ["「", "“", "[", "【"]
']' = ["」", "”", "]", "】"]
"#;
        toml::from_str(raw).unwrap()
    }

    #[test]
    fn test_analyze() {
        let dict = Dict::from_str(&gen_entries()).unwrap();
        let analyzer = InputAnalyzer::new(gen_test_config(), vec![(DictMeta::default(), dict)]);
        let inputs = vec![
            ("a cIzk", vec!["啊", "", "此啊", "zk"]),
            ("a cI@abc", vec!["啊", "", "此啊", "@", "啊波此"]),
            (
                "a  cIzk,,,[]I]]",
                vec!["啊", " ", "此啊", "zk", "……", "「", "”", "”"],
            ),
            (
                "zk`zk`c,cua.hcP",
                vec!["zk", "zk", "此", "，", "此", "u", "啊", "。", "h", "此"],
            ),
        ];
        for (i, (input, expect)) in inputs.into_iter().enumerate() {
            let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
            let texts: Vec<String> = result.segments.into_iter().map(|seg| seg.0).collect();
            assert_eq!(texts, expect, "> No.{} input: {}", i, input);
        }
    }

    #[test]
    fn test_analyze_with_sec_dict() {
        let dict = Dict::from_str(&gen_entries()).unwrap();
        let dict_sec = Dict::from_str(&gen_entries()).unwrap();
        let analyzer = InputAnalyzer::new(
            gen_test_config(),
            vec![
                (
                    DictMeta {
                        trigger: '\0',
                        ..Default::default()
                    },
                    dict,
                ),
                (
                    DictMeta {
                        trigger: '@',
                        hint: "反".to_string(),
                        ..Default::default()
                    },
                    dict_sec,
                ),
            ],
        );
        let inputs = vec![
            ("aaaaaa aaaaa", vec!["啊", "啊", "", "啊", "啊"]),
            ("aaa8 ", vec!["啊", " "]),
            ("a cI@abc", vec!["啊", "", "此啊", "反|啊波此"]),
            ("a cI@abcaaa", vec!["啊", "", "此啊", "啊波此", "啊"]),
            ("@aaac@cP@@@", vec!["啊", "此", "此", "@", "@", "反"]),
            ("@aaax@xxx", vec!["啊", "x", "@", "xxx"]),
            ("@aaax@cc@", vec!["啊", "x", "此", "反"]),
        ];
        for (i, (input, expect)) in inputs.into_iter().enumerate() {
            let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
            let texts: Vec<String> = result.segments.into_iter().map(|seg| seg.0).collect();
            assert_eq!(texts, expect, "> No.{} input: {}", i, input);
        }
    }

    #[test]
    fn test_segments_with_sec_dict() {
        let dict = Dict::from_str(&gen_entries()).unwrap();
        let dict_sec = Dict::from_str(&gen_entries()).unwrap();
        let analyzer = InputAnalyzer::new(
            gen_test_config(),
            vec![
                (
                    DictMeta {
                        trigger: '\0',
                        ..Default::default()
                    },
                    dict,
                ),
                (
                    DictMeta {
                        trigger: '@',
                        ..Default::default()
                    },
                    dict_sec,
                ),
            ],
        );
        let samples: Vec<(usize, &str, Vec<&str>, Vec<Tag>)> = vec![
            (
                1,
                "IIII@abc@ahI cu",
                vec!["IIII", "@abc", "@a", "hI ", "c", "u"],
                vec![
                    Tag::Unknown,
                    Tag::Code(CodeSelection::n().w_dict(1)),
                    Tag::Code(CodeSelection::n().w_dict(1)),
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
                    Tag::Code(CodeSelection::n()),
                    Tag::Code(CodeSelection::n().w_dict(1)),
                    Tag::Code(CodeSelection::n().w_dict(1)),
                    Tag::Unknown,
                    Tag::Code(CodeSelection::default()),
                    Tag::Unknown,
                    Tag::Code(CodeSelection::n().w_dict(1)),
                ],
            ),
            (
                3,
                "aaxx@abca@xx@aPPP@P",
                vec!["aa", "xx", "@abc", "a", "@", "xx", "@aP", "PP", "@", "P"],
                vec![
                    Tag::Code(CodeSelection::default()),
                    Tag::Unknown,
                    Tag::Code(CodeSelection::n().w_dict(1)),
                    Tag::Code(CodeSelection::n()),
                    Tag::Code(CodeSelection::n().w_dict(1)),
                    Tag::Unknown,
                    Tag::Code(CodeSelection::n().w_dict(1).w_sel(3).hs()),
                    Tag::Unknown,
                    Tag::Code(CodeSelection::n().w_dict(1)),
                    Tag::Unknown,
                ],
            ),
        ];
        for (no, input, expected, expected_tags) in samples.into_iter() {
            let segments = analyzer.segments(input.chars().collect::<Vec<_>>().as_slice());
            let (segments, tags): (Vec<String>, Vec<Tag>) = segments
                .into_iter()
                .map(|seg| (seg.0.iter().collect::<String>(), seg.1))
                .unzip();
            assert_eq!(segments, expected, "> No.{} sample: {}", no, input);
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
                "> No.{} sample: {}",
                no,
                input
            );
        }
    }

    #[test]
    fn test_segments() {
        let dict = Dict::from_str(&gen_entries()).unwrap();
        let analyzer = InputAnalyzer::new(gen_test_config(), vec![(DictMeta::default(), dict)]);
        let samples: Vec<(usize, &str, Vec<&str>, Vec<Tag>)> = vec![
            (
                0,
                "a zk,,cO",
                vec!["a", " zk", ",,", "cO"],
                vec![
                    Tag::Code(CodeSelection::default()),
                    Tag::Unknown,
                    Tag::Punctuation((0, false)),
                    Tag::Code(CodeSelection::n().w_sel(2).hs()),
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
                    Tag::Code(CodeSelection::n().w_sel(1).hs()),
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
                    Tag::Code(CodeSelection::n().w_sel(3).hs()),
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
                    Tag::Code(CodeSelection::n().w_sel(3).hs()),
                    Tag::Unknown,
                ],
            ),
        ];
        for (no, input, expected, expected_tags) in samples.into_iter() {
            let segments = analyzer.segments(input.chars().collect::<Vec<_>>().as_slice());
            let (segments, tags): (Vec<String>, Vec<Tag>) = segments
                .into_iter()
                .map(|seg| (seg.0.iter().collect::<String>(), seg.1))
                .unzip();
            assert_eq!(segments, expected, "> No.{} sample: {}", no, input);
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
                "> No.{} sample: {}",
                no,
                input
            );
        }
    }

    #[test]
    fn test_analyze_pending() {
        let dict = Dict::from_str(&gen_entries()).unwrap();
        let analyzer = InputAnalyzer::new(gen_test_config(), vec![(DictMeta::default(), dict)]);
        let samples = vec![("a", true), ("abcd", true), ("abI", false)];
        for (i, (input, expected)) in samples.into_iter().enumerate() {
            let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
            assert_eq!(expected, result.pending, "> No.{} input: {}", i, input);
        }
    }

    #[test]
    fn test_segments_pagination() {
        let dict = Dict::from_str(&gen_entries()).unwrap();
        let analyzer = InputAnalyzer::new(
            gen_test_config_page_count(2),
            vec![(DictMeta::default(), dict)],
        );
        // ⇞ (U+21DE) 和 ⇟ (U+21DF)
        let samples = vec![
            ("a", Tag::Code(CodeSelection::n().w_page_no(0))),
            ("a⇟⇟", Tag::Code(CodeSelection::n().w_page_no(2).hp())),
            (
                "a⇟2",
                Tag::Code(CodeSelection::n().w_page_no(1).hp().w_sel(1).hs()),
            ),
            (
                "a⇟⇟⇞⇞⇞⇞⇞⇞⇞⇞⇞⇞⇞⇟",
                Tag::Code(CodeSelection::n().w_page_no(1).hp()),
            ),
        ];
        for (i, (input, expected)) in samples.into_iter().enumerate() {
            let mut segments = analyzer.segments(input.chars().collect::<Vec<_>>().as_slice());
            assert_eq!(
                Some(expected),
                segments.pop().map(|seg| seg.1),
                "> No.{} input: {}",
                i,
                input
            );
        }
    }

    fn gen_test_config_page_count(page_count: usize) -> Config {
        Config {
            page_count,
            ..Default::default()
        }
    }
    #[test]
    fn test_analyze_pagination() {
        let dict = Dict::from_str(&gen_entries()).unwrap();
        let analyzer = InputAnalyzer::new(
            gen_test_config_page_count(2),
            vec![(DictMeta::default(), dict)],
        );
        // ⇞ (U+21DE) 和 ⇟ (U+21DF)
        let samples = vec![("a", "啊"), ("a⇟⇟", "啊波此"), ("a⇟2", "啊波")];
        for (i, (input, expected)) in samples.into_iter().enumerate() {
            let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
            assert_eq!(
                expected,
                result.segments.last().map(|seg| seg.0.clone()).unwrap(),
                "> No.{} input: {}",
                i,
                input
            );
        }
        let dict = Dict::from_str(&gen_entries()).unwrap();
        let analyzer = InputAnalyzer::new(
            gen_test_config_page_count(3),
            vec![(DictMeta::default(), dict)],
        );
        // ⇞ (U+21DE) 和 ⇟ (U+21DF)
        let samples = vec![
            ("q", "其0"),
            ("q⇟⇟", "其6"),
            ("q⇟2", "其4"),
            ("q⇟⇟⇟⇟⇟⇟⇟⇟⇟", "其12"),
            ("qqqqq⇟⇟⇟⇟⇟⇟⇟⇟⇟", "其12"),
            ("y⇟", "伊3"),
            ("y⇟⇟", "伊3"),
        ];
        for (i, (input, expected)) in samples.into_iter().enumerate() {
            let result = analyzer.analyze(input.chars().collect::<Vec<_>>().as_slice());
            assert_eq!(
                expected,
                result.segments.last().map(|seg| seg.0.clone()).unwrap(),
                "> No.{} input: {}",
                i,
                input
            );
        }
    }

    fn gen_test_config_escape_no_trim() -> Config {
        Config {
            escape_pair: Some(['E', 'D']),
            trim_escape_pair: false,
            ..Default::default()
        }
    }
    fn gen_test_config_escape_trim() -> Config {
        Config {
            escape_pair: Some(['E', 'D']),
            trim_escape_pair: true,
            ..Default::default()
        }
    }
    #[test]
    fn test_analyze_trim_escape() {
        let dict = Dict::from_str(&gen_entries()).unwrap();
        let analyzer_trim = InputAnalyzer::new(
            gen_test_config_escape_trim(),
            vec![(DictMeta::default(), dict)],
        );
        let dict = Dict::from_str(&gen_entries()).unwrap();
        let analyzer_no_trim = InputAnalyzer::new(
            gen_test_config_escape_no_trim(),
            vec![(DictMeta::default(), dict)],
        );
        let samples = vec![
            ("xxx`aa`", "xxx`啊`", "xxx`啊`"),
            ("xxxEaa`", "xxxEaa`", "xxxEaa`"),
            ("xxxEaaD`", "xxxaa`", "xxxEaaD`"),
        ];
        for (i, (input, expected_trim, expected_no_trim)) in samples.into_iter().enumerate() {
            let result = analyzer_trim.analyze(input.chars().collect::<Vec<_>>().as_slice());
            assert_eq!(
                expected_trim,
                result
                    .segments
                    .into_iter()
                    .map(|seg| seg.0)
                    .collect::<String>(),
                "> Trim No.{} input: {}",
                i,
                input
            );
            let result = analyzer_no_trim.analyze(input.chars().collect::<Vec<_>>().as_slice());
            assert_eq!(
                expected_no_trim,
                result
                    .segments
                    .into_iter()
                    .map(|seg| seg.0)
                    .collect::<String>(),
                "> No Trim No.{} input: {}",
                i,
                input
            );
        }
    }

    #[allow(unused)]
    impl CodeSelection {
        fn n() -> Self {
            Self::default()
        }
        fn w_dict(self, dict_idx: usize) -> Self {
            Self { dict_idx, ..self }
        }
        fn w_sel(self, sel_idx: usize) -> Self {
            Self { sel_idx, ..self }
        }
        fn hs(self) -> Self {
            Self {
                has_selection: true,
                ..self
            }
        }
        fn w_page_no(self, page_no: usize) -> Self {
            Self { page_no, ..self }
        }
        fn hp(self) -> Self {
            Self {
                has_pagination: true,
                ..self
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
}
