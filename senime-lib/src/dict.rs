use bincode::{Decode, Encode, config};
use serde::Deserialize;
use std::{
    fs::File,
    io::{Error, ErrorKind, Read, Write},
    path::{Path, PathBuf},
    time::{Duration, UNIX_EPOCH},
};

/// 码表类型：前缀精确匹配或模糊查询。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DictKindName {
    /// 基于 Prism 前缀索引的精确前缀匹配（默认）
    #[default]
    Prefix,
    /// 基于 nucleo 的模糊查询（用于 emoji 等场景）
    Fuzzy,
}

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
    /// 码表类型，默认为 prefix（前缀匹配）
    #[serde(default)]
    pub kind: DictKindName,
}

/// 连续内存字符串池，所有字符串的 UTF-8 字节存放在一块连续的 Vec<u8> 中，
/// 通过 (offset, len) 索引访问，避免大量小 String 的堆分配开销。
#[derive(Debug)]
pub struct StringArena {
    data: Vec<u8>,
}

impl StringArena {
    pub(crate) fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub(crate) fn push(&mut self, s: &str) -> (u32, u16) {
        let offset = self.data.len() as u32;
        let len = s.len() as u16;
        self.data.extend_from_slice(s.as_bytes());
        (offset, len)
    }

    pub(crate) fn get(&self, offset: u32, len: u16) -> &str {
        let start = offset as usize;
        let end = start + len as usize;
        // SAFETY: 存入的都是合法的 UTF-8 字符串
        unsafe { str::from_utf8_unchecked(&self.data[start..end]) }
    }
}

impl Encode for StringArena {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.data, encoder)
    }
}

impl<Context> Decode<Context> for StringArena {
    fn decode<D: bincode::de::Decoder<Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let data: Vec<u8> = bincode::Decode::decode(decoder)?;
        Ok(Self { data })
    }
}

/// 二进制文件头，用于检测码表是否发生了变动
#[derive(Debug, Decode, Encode)]
struct BinHeader {
    head: [char; 6],
    ver: i64,
    mtime: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Column {
    Code,
    Text,
    Weight,
    Other,
}

/// 临时解析结果，持有借用的字符串切片，后续会 push 到 StringArena
struct ParsedCandidate<'a> {
    code: &'a str,
    text: &'a str,
    weight: i32,
}

/// Arena 索引化的候选，不持有 String，只存 (offset, len) 索引。
/// 每个 Candidate 仅占 16 字节（vs 原来 ~120 字节），零堆分配。
#[derive(Debug, Clone)]
pub struct Candidate {
    pub code: (u32, u16),
    pub text: (u32, u16),
    pub weight: i32,
}

impl Encode for Candidate {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.code.0, encoder)?;
        bincode::Encode::encode(&self.code.1, encoder)?;
        bincode::Encode::encode(&self.text.0, encoder)?;
        bincode::Encode::encode(&self.text.1, encoder)?;
        bincode::Encode::encode(&self.weight, encoder)?;
        Ok(())
    }
}

impl<Context> Decode<Context> for Candidate {
    fn decode<D: bincode::de::Decoder<Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let code_offset: u32 = bincode::Decode::decode(decoder)?;
        let code_len: u16 = bincode::Decode::decode(decoder)?;
        let text_offset: u32 = bincode::Decode::decode(decoder)?;
        let text_len: u16 = bincode::Decode::decode(decoder)?;
        let weight: i32 = bincode::Decode::decode(decoder)?;
        Ok(Self {
            code: (code_offset, code_len),
            text: (text_offset, text_len),
            weight,
        })
    }
}

/// 字典类型：前缀精确匹配或模糊查询。
#[derive(Debug)]
pub enum DictKind {
    /// 基于 Prism 前缀索引的精确前缀匹配字典（现有码表）
    Prefix(PrefixDict),
    /// 基于 nucleo 的模糊查询字典（emoji 等场景）
    Fuzzy(FuzzDict),
}

impl Default for DictKind {
    fn default() -> Self {
        Self::Prefix(PrefixDict::default())
    }
}

impl DictKind {
    /// 判断给定编码是否可达（有候选结果）。
    pub fn reachable(&self, codes: &[char]) -> bool {
        match self {
            DictKind::Prefix(dict) => dict.reachable(codes),
            DictKind::Fuzzy(fuzz) => fuzz.reachable(codes),
        }
    }

    /// 前缀字典的条目数。模糊字典无此概念，返回 0。
    pub fn count(&self) -> usize {
        match self {
            DictKind::Prefix(dict) => dict.count(),
            DictKind::Fuzzy(fuzz) => fuzz.count(),
        }
    }
}

impl DictKind {
    pub fn from_str(raw: &str, kind: DictKindName) -> Result<Self, String> {
        let (arena, candidates) = parse_dict_txt(raw)?;
        match kind {
            DictKindName::Prefix => {
                let prism = Prism::new_with_arena(&candidates, &arena);
                Ok(Self::Prefix(PrefixDict {
                    arena,
                    prism,
                    candidates,
                }))
            }
            DictKindName::Fuzzy => Ok(Self::Fuzzy(FuzzDict { arena, candidates })),
        }
    }

    /// 从文件路径加载纯码表（.txt 或 .bin），不处理 .toml 配置。
    /// 如需加载含配置的完整引擎，请使用 `InputAnalyzer::load_analyzer`。
    pub fn try_load(meta: &DictMeta) -> Result<Self, String> {
        let path = PathBuf::from(&meta.path);
        match path.extension().and_then(|e| e.to_str()) {
            Some("txt") => Self::load_from_txt(&path, meta.kind),
            Some("bin") => Self::load_from_bin(&path, meta.kind),
            _ => Err(format!("不支持的文件类型: {:?}", path)),
        }
    }

    /// 从纯码表 .txt 文件加载
    pub fn load_from_txt(txt_path: &Path, kind: DictKindName) -> Result<Self, String> {
        let mtime = get_mtime_or(txt_path, 0);
        let bin_path = txt_path.with_extension("txt.bin");
        // 尝试加载 bin 缓存
        if let Ok(mut file) = File::open(&bin_path) {
            let mut buf = Vec::new();
            if file.read_to_end(&mut buf).is_ok()
                && let Ok(dict) = Self::try_from((mtime, kind, buf.as_slice()))
            {
                return Ok(dict);
            }
            println!("二进制缓存无效，重新构建: {:?}", bin_path);
        }
        // 读取 txt 并构建
        let mut raw = String::new();
        File::open(txt_path)
            .map_err(|e| format!("无法读取码表文件 {:?}: {e}", txt_path))?
            .read_to_string(&mut raw)
            .map_err(|e| format!("无法读取码表内容: {e}"))?;
        let dict = Self::from_str(&raw, kind).map_err(|e| format!("解析码表失败: {e}"))?;
        // 写入 bin 缓存
        if let Ok(mut bin_file) = File::create(&bin_path) {
            let _ = bin_file.write_all(&dict.to_bin(mtime));
        }
        Ok(dict)
    }

    /// 直接加载 .bin 二进制文件
    fn load_from_bin(bin_path: &Path, kind: DictKindName) -> Result<Self, String> {
        let mut buf = Vec::new();
        File::open(bin_path)
            .map_err(|e| format!("无法读取二进制文件 {:?}: {e}", bin_path))?
            .read_to_end(&mut buf)
            .map_err(|e| format!("无法读取二进制文件内容: {e}"))?;
        Self::try_from((0, kind, buf.as_slice()))
            .map_err(|e| format!("无效的二进制文件 {:?}: {e}", bin_path))
    }

    /// 将 Dict 序列化为二进制格式（包含 BinHeader 头部）
    pub fn to_bin(&self, mtime: i64) -> Vec<u8> {
        let meta = BinHeader {
            head: ['s', 'e', 'n', 'i', 'm', 'e'],
            ver: VERSION,
            mtime,
        };
        let cfg = config::standard();
        let mut buf = Vec::new();
        // 30 字节头部
        let mut head = [0u8; 30];
        bincode::encode_into_slice(meta, &mut head, cfg).unwrap();
        buf.extend_from_slice(&head);
        match self {
            Self::Prefix(dict) => {
                // 按顺序编码 arena, candidates, prism
                buf.extend(bincode::encode_to_vec(&dict.arena, cfg).unwrap());
                buf.extend(bincode::encode_to_vec(&dict.candidates, cfg).unwrap());
                buf.extend(bincode::encode_to_vec(&dict.prism, cfg).unwrap());
            }
            Self::Fuzzy(dict) => {
                // 按顺序编码 arena, candidates, prism
                buf.extend(bincode::encode_to_vec(&dict.arena, cfg).unwrap());
                buf.extend(bincode::encode_to_vec(&dict.candidates, cfg).unwrap());
            }
        }
        buf
    }

    pub fn as_prefix(&self) -> &PrefixDict {
        match self {
            Self::Prefix(dict) => dict,
            _ => panic!("不是前缀码表"),
        }
    }

    pub fn as_fuzzy(&self) -> &FuzzDict {
        match self {
            Self::Fuzzy(dict) => dict,
            _ => panic!("不是模糊码表"),
        }
    }

    pub fn into_prefix(self) -> PrefixDict {
        match self {
            Self::Prefix(dict) => dict,
            _ => panic!("不是前缀码表"),
        }
    }

    pub fn into_fuzzy(self) -> FuzzDict {
        match self {
            Self::Fuzzy(dict) => dict,
            _ => panic!("不是模糊码表"),
        }
    }
}

pub(crate) const VERSION: i64 = 14;

impl TryFrom<(i64, DictKindName, &[u8])> for DictKind {
    type Error = std::io::Error;
    fn try_from((txt_mtime, kind, bs): (i64, DictKindName, &[u8])) -> Result<Self, Error> {
        let map_err = |reason: &str| Error::new(ErrorKind::InvalidData, reason);
        let metadata = &bs[..30];
        let (
            BinHeader {
                head,
                ver,
                mtime: cached_txt,
            },
            _size,
        ): (BinHeader, usize) = bincode::decode_from_slice(metadata, config::standard())
            .map_err(|_| map_err("无效的二进制数据[HEAD]"))?;
        let c_head = ['s', 'e', 'n', 'i', 'm', 'e'];
        let mtime_ok = txt_mtime == 0 || txt_mtime == cached_txt;
        if !(head == c_head && ver == VERSION && mtime_ok) {
            return Err(Error::other("码表已更新，重新构建二进制文件"));
        }
        let buf = &bs[30..];
        let cfg = config::standard();
        let (arena, n1): (StringArena, usize) =
            bincode::decode_from_slice(buf, cfg).map_err(|_| map_err("无效的二进制数据[ARENA]"))?;
        let (candidates, n2): (Vec<Candidate>, usize) = bincode::decode_from_slice(&buf[n1..], cfg)
            .map_err(|_| map_err("无效的二进制数据[CANDIDATES]"))?;
        match kind {
            DictKindName::Prefix => {
                let (prism, _): (Prism, usize) = bincode::decode_from_slice(&buf[n1 + n2..], cfg)
                    .map_err(|_| map_err("无效的二进制数据[PRISM]"))?;
                Ok(Self::Prefix(PrefixDict {
                    arena,
                    prism,
                    candidates,
                }))
            }
            DictKindName::Fuzzy => Ok(Self::Fuzzy(FuzzDict { arena, candidates })),
        }
    }
}

/// 从码表文本解析出 StringArena 和 Candidate 列表（供 Dict 和 FuzzDict 共用）。
pub(crate) fn parse_dict_txt(raw: &str) -> Result<(StringArena, Vec<Candidate>), String> {
    let mut parsed = Vec::new();
    let mut columns: Option<Vec<Column>> = None;
    let mut parse_columns_times = 0;
    for line in raw.lines() {
        if columns.is_none() {
            match try_parse_columns(line) {
                Ok(cols) => columns = Some(cols),
                Err(_) => {
                    parse_columns_times += 1;
                    if parse_columns_times > 100 {
                        return Err(
                            "码表中的第一个有效项(非第一行)未满足码表格式:必须要有英文和汉字，以制表符隔开，顺序随意，数字(权重)可选，以供程序自动且正确解析码表排列顺序。".to_string()
                        );
                    }
                    continue;
                }
            }
        }
        if let Ok(cand) = parse_candidate(line, columns.as_ref().unwrap()) {
            parsed.push(cand);
        }
    }
    // 排序：按 code 字典序，code 相同时按 weight 降序
    parsed.sort_by(|a, b| match a.code.cmp(b.code) {
        std::cmp::Ordering::Equal => b.weight.cmp(&a.weight),
        ord => ord,
    });
    // 将所有字符串一次性 push 到 arena
    let mut arena = StringArena::new();
    let candidates: Vec<Candidate> = parsed
        .into_iter()
        .map(|p| Candidate {
            code: arena.push(p.code),
            text: arena.push(p.text),
            weight: p.weight,
        })
        .collect();
    Ok((arena, candidates))
}

/// 从这一行中解析 列 的顺序
/// 有效行是指：非空的、以 `\t` 分隔后，最少有两列的行
/// 满足条件后，对每一列进行分析，
///   全英文则为 code(英文码)
///   全数字则为 weight
///   包含ascii之外字符的则为 text(通常为汉字，不限制字符)
///   如果有两列都是code或text，则无法确认列顺序，抛出错误
/// 简而言之：必须要有一列是 code ，一列 text
fn try_parse_columns(line: &str) -> Result<Vec<Column>, std::io::Error> {
    let split = line
        .split('\t')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    if split.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidData, "非有效行"));
    }
    let mut code_count = 0;
    let mut text_count = 0;
    let mut columns = Vec::with_capacity(split.len());
    for sp in split {
        let col = if sp.is_ascii()
            && sp.chars().any(|c| c.is_ascii_alphabetic())
            && !sp.chars().all(|c| c.is_ascii_digit())
        {
            // ASCII 且包含字母 -> Code（允许 `,` `|` 等分隔符，用于模糊查询标签）
            code_count += 1;
            Column::Code
        } else if sp.chars().all(|c| c.is_ascii_digit()) {
            Column::Weight
        } else if !sp.is_ascii() {
            text_count += 1;
            Column::Text
        } else {
            Column::Other
        };
        columns.push(col);
    }
    if code_count == 1 && text_count == 1 {
        return Ok(columns);
    }
    Err(Error::new(ErrorKind::InvalidData, "无法确认[列] 顺序"))
}

/// 从码表文本行解析出临时的 ParsedCandidate
fn parse_candidate<'a>(raw: &'a str, columns: &[Column]) -> Result<ParsedCandidate<'a>, Error> {
    let split = raw.split('\t').collect::<Vec<_>>();
    if split.len() < 2 {
        Err(Error::new(ErrorKind::InvalidData, format!("无效行: {raw}")))
    } else {
        let mut code = "";
        let mut text = "";
        let mut weight = 0;
        for (i, col) in columns.iter().enumerate() {
            if let Some(v) = split.get(i) {
                match *col {
                    Column::Code => code = v.trim(),
                    Column::Text => text = v.trim(),
                    Column::Weight => {
                        weight = v.trim().parse().unwrap_or_default();
                    }
                    _ => {}
                }
            }
        }
        if code.is_empty() || text.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("code为空: {raw}"),
            ));
        }
        // 检查拓展 CJK 字符（仅单字时检查）
        let text_len = text.chars().count();
        if text_len == 1 {
            let c = text.chars().next().unwrap();
            if is_extended_cjk(c) {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("拓展字符: {raw}"),
                ));
            }
        }
        Ok(ParsedCandidate { code, text, weight })
    }
}

// https://github.com/rime/librime/blob/47033202f986f4dced82eceb90440285fcb9501e/src/rime/gear/charset_filter.cc#L18
fn is_extended_cjk(c: char) -> bool {
    let c = c as u32;
    (0x3400..=0x4DBF).contains(&c)   ||  // CJK Unified Ideographs Extension A
    (0x20000..=0x2A6DF).contains(&c) ||  // CJK Unified Ideographs Extension B
    (0x2A700..=0x2B73F).contains(&c) ||  // CJK Unified Ideographs Extension C
    (0x2B740..=0x2B81F).contains(&c) ||  // CJK Unified Ideographs Extension D
    (0x2B820..=0x2CEAF).contains(&c) ||  // CJK Unified Ideographs Extension E
    (0x2CEB0..=0x2EBEF).contains(&c) ||  // CJK Unified Ideographs Extension F
    (0x30000..=0x3134F).contains(&c) ||  // CJK Unified Ideographs Extension G
    (0x31350..=0x323AF).contains(&c) ||  // CJK Unified Ideographs Extension H
    (0x2EBF0..=0x2EE5F).contains(&c) ||  // CJK Unified Ideographs Extension I
    (0x323B0..=0x3347F).contains(&c) ||  // CJK Unified Ideographs Extension J
    (0x3300..=0x33FF).contains(&c)   ||  // CJK Compatibility
    (0xFE30..=0xFE4F).contains(&c)   ||  // CJK Compatibility Forms
    (0xF900..=0xFAFF).contains(&c)   ||  // CJK Compatibility Ideographs
    (0x2F800..=0x2FA1F).contains(&c) // CJK Compatibility Ideographs Supplement
}

pub(crate) fn get_mtime_or(path: &Path, default: i64) -> i64 {
    let Ok(metadata) = path.metadata() else {
        return default;
    };
    if let Ok(modi) = metadata.modified() {
        let duration = modi.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO);
        duration.as_millis() as i64
    } else {
        0
    }
}

use crate::prefix_dict::PrefixDict;
use crate::{fuzz_dict::FuzzDict, prefix_dict::Prism};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{gen_test_dict_files, remove_test_dict_files};
    use std::time::Instant;

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

    #[test]
    fn test_prefix_dict() {
        let dict = DictKind::from_str(&gen_entries(), DictKindName::Prefix)
            .unwrap()
            .into_prefix();
        println!("dict loaded: {}", dict.count());
        let result = dict.search("ah".chars().collect::<Vec<_>>().as_slice());
        assert_eq!(0, result.map_or(0, |candidates| candidates.len()));
        let result = dict.search("a".chars().collect::<Vec<_>>().as_slice());
        assert_eq!(8, result.map_or(0, |candidates| candidates.len()));
        let result = dict.search("abd".chars().collect::<Vec<_>>().as_slice());
        assert_eq!(0, result.map_or(0, |candidates| candidates.len()));
        let result = dict.search("da".chars().collect::<Vec<_>>().as_slice());
        assert_eq!(1, result.map_or(0, |candidates| candidates.len()));
        let result = dict.search("@".chars().collect::<Vec<_>>().as_slice());
        assert_eq!(0, result.map_or(0, |candidates| candidates.len()));
    }

    // 初版
    // loaded 162982 from ./test/虎码码表.txt
    // searched: 6099
    // dict loaded time: 392.5498ms
    // search time: 2.929678ms

    // 非递归后
    // loaded 162982 from ./test/虎码码表.txt
    // searched: 6099
    // dict loaded time: 295.150222ms
    // search time: 1.874144ms

    // 限制最大候选后，数量为9
    // loaded 162982 from ./test/虎码码表.txt
    // searched: 12
    // dict loaded time: 301.187025ms
    // search time: 21.641µs

    // 最大候选1000，无--release参数
    // loaded 162982 from ./test/虎码码表.txt
    // searched: 1000
    // dict loaded time: 460.254761ms
    // search time: 481.759µs
    //
    // 最大候选1000，无--release参数，使用prism
    // loaded 162982 from ./test/虎码码表.txt
    // searched: 1000
    // dict loaded time: 1.128577844s
    // search time: 11.27µs
    #[test]
    fn test_load() {
        let (_config_path, dict_path) = gen_test_dict_files();
        let time_start = Instant::now();
        let dict = DictKind::load_from_txt(&dict_path, DictKindName::Prefix)
            .unwrap()
            .into_prefix();
        let time_dict_loaded = Instant::now();
        println!("loaded {} from {:?}", dict.count(), dict_path);
        let candidates = dict.search("a".chars().collect::<Vec<_>>().as_slice());
        let time_searched = Instant::now();
        println!("searched: {}", candidates.as_ref().map_or(0, |c| c.len()));
        assert!(candidates.is_some_and(|cands| cands.len() > 0));
        println!(
            "dict loaded time: {:?}",
            time_dict_loaded.duration_since(time_start)
        );
        println!(
            "search time: {:?}",
            time_searched.duration_since(time_dict_loaded)
        );
        remove_test_dict_files();
    }

    #[test]
    fn test_try_parse_column_head() {
        let line = "abcd 你好 10".replace(' ', "\t");
        let ret = try_parse_columns(&line).expect("解析失败");
        assert_eq!(vec![Column::Code, Column::Text, Column::Weight], ret);
        let line = "你好 10 abcd".replace(' ', "\t");
        let ret = try_parse_columns(&line).expect("解析失败");
        assert_eq!(vec![Column::Text, Column::Weight, Column::Code], ret);
        let line = "你好 10 abcd efgh".replace(' ', "\t");
        try_parse_columns(&line).expect_err("解析应该失败");
        let line = "你好hello 10 abcd".replace(' ', "\t");
        let ret = try_parse_columns(&line).expect("解析失败");
        assert_eq!(vec![Column::Text, Column::Weight, Column::Code], ret);
    }
    // 导出棱镜和码表文件
    // #[test]
    // fn test_dict_output() {
    //     let path = "../test/虎码码表.txt";
    //     let dict = Dict::load(path);
    //     let candidates = dict
    //         .candidates
    //         .iter()
    //         .enumerate()
    //         .map(|(i, c)| format!("{}\t{}\t{}\t{}", i, c.code, c.text, c.weight))
    //         .collect::<Vec<String>>()
    //         .join("\n");
    //     let prims = dict
    //         .prism
    //         .keys
    //         .iter()
    //         .zip(dict.prism.indices)
    //         .map(|(k, r)| format!("{}\t{}\t{}", k.iter().collect::<String>(), r.0, r.1));
    //     let can_path = "../test/candidates.txt";
    //     let prim_path = "../test/prism.txt";
    //     std::fs::write(can_path, candidates).unwrap();
    //     std::fs::write(prim_path, prims.collect::<Vec<String>>().join("\n")).unwrap();
    // }
}
