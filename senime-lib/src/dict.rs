use bincode::{Decode, Encode, config};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{Error, ErrorKind, Read, Write},
    mem,
    path::{Path, PathBuf},
    str::{self, FromStr},
    time::{Duration, UNIX_EPOCH},
};

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
struct DictMeta {
    head: [char; 6],
    ver: i64,
    txt_mtime: i64,
    config_mtime: i64,
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
    code: (u32, u16),
    text: (u32, u16),
    pub weight: i32,
}

impl Candidate {
    /// 从 arena 获取 code
    pub fn code<'a>(&self, arena: &'a StringArena) -> &'a str {
        arena.get(self.code.0, self.code.1)
    }

    /// 从 arena 获取 text
    pub fn text<'a>(&self, arena: &'a StringArena) -> &'a str {
        arena.get(self.text.0, self.text.1)
    }
}

/// 借用视图，持有 arena 中的 &str 引用，用于对外 API 返回。
#[derive(Debug, Clone)]
pub struct CandidateView<'a> {
    pub code: &'a str,
    pub text: &'a str,
    pub weight: i32,
}

impl CandidateView<'_> {
    pub fn to_owned_candidate(&self, arena: &mut StringArena) -> Candidate {
        Candidate {
            code: arena.push(self.code),
            text: arena.push(self.text),
            weight: self.weight,
        }
    }
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
        Ok(ParsedCandidate {
            code,
            text,
            weight,
        })
    }
}

// Dict 节点结构 — 使用连续字节存储所有前缀，避免每个前缀独立 Vec<char> 的堆分配开销。
#[derive(Debug)]
struct Prism {
    // 所有前缀的字节连续存放（code 都是 ASCII，每个字符 1 字节 vs char 的 4 字节）
    keys: Vec<u8>,
    // 每个前缀在 keys 中的 (offset, len)
    key_meta: Vec<(u32, u16)>,
    // candidates 的索引范围，与 key_meta 一一对应: [start, end)
    indices: Vec<(usize, usize)>,
}

impl Prism {
    fn new_with_arena(candidates: &[Candidate], arena: &StringArena) -> Self {
        let mut map: BTreeMap<Vec<u8>, (usize, usize)> = BTreeMap::new();
        for (i, cand) in candidates.iter().enumerate() {
            let code_bytes = cand.code(arena).as_bytes();
            for len in 1..=code_bytes.len() {
                let prefix = &code_bytes[..len];
                map.entry(prefix.to_vec())
                    .and_modify(|r| r.1 = i + 1)
                    .or_insert((i, i + 1));
            }
        }
        let mut keys = Vec::new();
        let mut key_meta = Vec::with_capacity(map.len());
        let mut indices = Vec::with_capacity(map.len());
        for (prefix, range) in map {
            let offset = keys.len() as u32;
            let len = prefix.len() as u16;
            keys.extend_from_slice(&prefix);
            key_meta.push((offset, len));
            indices.push(range);
        }
        keys.shrink_to_fit();
        Self {
            keys,
            key_meta,
            indices,
        }
    }

    fn lookup(&self, code: &[char]) -> Option<&(usize, usize)> {
        // code 都是 ASCII lowercase，直接转为字节比较
        let code_bytes: Vec<u8> = code.iter().map(|c| *c as u8).collect();
        let idx = self
            .key_meta
            .binary_search_by(|&(offset, len)| {
                let start = offset as usize;
                let end = start + len as usize;
                self.keys[start..end].cmp(&code_bytes)
            })
            .ok()?;
        self.indices.get(idx)
    }
}

impl Encode for Prism {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.keys, encoder)?;
        bincode::Encode::encode(&self.key_meta, encoder)?;
        bincode::Encode::encode(&self.indices, encoder)?;
        Ok(())
    }
}

impl<Context> Decode<Context> for Prism {
    fn decode<D: bincode::de::Decoder<Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let keys: Vec<u8> = bincode::Decode::decode(decoder)?;
        let key_meta: Vec<(u32, u16)> = bincode::Decode::decode(decoder)?;
        let indices: Vec<(usize, usize)> = bincode::Decode::decode(decoder)?;
        Ok(Self {
            keys,
            key_meta,
            indices,
        })
    }
}

const VERSION: i64 = 12;

// Dict 结构
#[derive(Debug)]
pub struct Dict {
    arena: StringArena,
    prism: Prism,
    candidates: Vec<Candidate>,
    config: Config,
}

impl TryFrom<(i64, i64, &[u8])> for Dict {
    type Error = std::io::Error;
    fn try_from((txt_mtime, config_mtime, bs): (i64, i64, &[u8])) -> Result<Dict, Error> {
        let map_err = |reason: &str| Error::new(ErrorKind::InvalidData, reason);
        let metadata = &bs[..30];
        let (
            DictMeta {
                head,
                ver,
                txt_mtime: cached_txt,
                config_mtime: cached_cfg,
            },
            _size,
        ): (DictMeta, usize) = bincode::decode_from_slice(metadata, config::standard())
            .map_err(|_| map_err("无效的二进制数据[HEAD]"))?;
        let c_head = ['s', 'e', 'n', 'i', 'm', 'e'];
        let mtime_ok = (txt_mtime == 0 || txt_mtime == cached_txt)
            && (config_mtime == 0 || config_mtime == cached_cfg);
        if !(head == c_head && ver == VERSION && mtime_ok) {
            return Err(Error::other("码表已更新，重新构建二进制文件"));
        }
        let buf = &bs[30..];
        let cfg = config::standard();
        let (arena, n1): (StringArena, usize) =
            bincode::decode_from_slice(buf, cfg).map_err(|_| map_err("无效的二进制数据[ARENA]"))?;
        let (candidates, n2): (Vec<Candidate>, usize) =
            bincode::decode_from_slice(&buf[n1..], cfg).map_err(|_| map_err("无效的二进制数据[CANDIDATES]"))?;
        let (prism, n3): (Prism, usize) =
            bincode::decode_from_slice(&buf[n1 + n2..], cfg).map_err(|_| map_err("无效的二进制数据[PRISM]"))?;
        let (config, _): (Config, usize) =
            bincode::decode_from_slice(&buf[n1 + n2 + n3..], cfg).map_err(|_| map_err("无效的二进制数据[CONFIG]"))?;
        Ok(Self {
            arena,
            prism,
            candidates,
            config,
        })
    }
}

impl FromStr for Dict {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Self::from_str_with_config(raw, Config::default())
    }
}

impl Dict {
    /// 使用指定配置解析码表文本
    pub fn from_str_with_config(raw: &str, config: Config) -> Result<Self, String> {
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
        parsed.sort_by(|a, b| {
            match a.code.cmp(b.code) {
                std::cmp::Ordering::Equal => b.weight.cmp(&a.weight),
                ord => ord,
            }
        });
        // 将所有字符串一次性 push 到 arena（自动去重）
        let mut arena = StringArena::new();
        let candidates: Vec<Candidate> = parsed
            .into_iter()
            .map(|p| Candidate {
                code: arena.push(p.code),
                text: arena.push(p.text),
                weight: p.weight,
            })
            .collect();
        let prism = Prism::new_with_arena(&candidates, &arena);
        Ok(Self {
            arena,
            candidates,
            prism,
            config,
        })
    }
}

/// 解析相对于基准文件所在目录的路径
fn resolve_relative_path(base_file: &Path, relative: &str) -> PathBuf {
    let path: PathBuf = relative.into();
    if path.is_absolute() {
        path
    } else {
        base_file.parent().map(|p| p.join(&path)).unwrap_or(path)
    }
}

fn get_mtime_or(path: &Path, default: i64) -> i64 {
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

// #[cfg(not(unix))]
// fn get_mtime(path: &PathBuf) -> i64 {
//     0
// }

impl Dict {
    pub fn load<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self::try_load(path).unwrap_or_else(|e| panic!("{e}"))
    }

    pub fn try_load<P>(path: P) -> Result<Self, String>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        match path.extension().and_then(|e| e.to_str()) {
            Some("toml") => Self::load_from_toml(&path),
            Some("txt") => Self::load_from_txt_and_config(&path, Config::default(), 0),
            Some("bin") => Self::load_from_bin(&path),
            _ => Err(format!("不支持的文件类型: {:?}", path)),
        }
    }

    /// 从 .toml 配置文件加载
    fn load_from_toml(toml_path: &Path) -> Result<Self, String> {
        let toml_mtime = get_mtime_or(toml_path, 0);
        let mut content = String::new();
        File::open(toml_path)
            .map_err(|e| format!("无法读取配置文件 {:?}: {e}", toml_path))?
            .read_to_string(&mut content)
            .map_err(|e| format!("无法读取配置文件内容: {e}"))?;
        let mut config: Config =
            toml::from_str(&content).map_err(|e| format!("无法解析配置文件: {e}"))?;
        let dict_name = config.dict.as_ref().ok_or("配置文件中缺少 dict 字段")?;
        let dict_path = resolve_relative_path(toml_path, dict_name);
        config.patch_punctuations(default_punctuations());
        match dict_path.extension().and_then(|e| e.to_str()) {
            Some("bin") => Self::load_from_bin(&dict_path),
            Some("txt") => Self::load_from_txt_and_config(&dict_path, config, toml_mtime),
            _ => Err(format!("不支持的 dict 文件类型: {:?}", dict_path)),
        }
    }

    /// 从纯码表 .txt 文件加载，使用指定配置
    fn load_from_txt_and_config(
        txt_path: &Path,
        config: Config,
        config_mtime: i64,
    ) -> Result<Self, String> {
        let txt_mtime = get_mtime_or(txt_path, 0);
        let bin_path = txt_path.with_extension("txt.bin");
        // 尝试加载 bin 缓存
        if let Ok(mut file) = File::open(&bin_path) {
            let mut buf = Vec::new();
            if file.read_to_end(&mut buf).is_ok()
                && let Ok(dict) = Self::try_from((txt_mtime, config_mtime, buf.as_slice()))
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
        let dict =
            Self::from_str_with_config(&raw, config).map_err(|e| format!("解析码表失败: {e}"))?;
        // 写入 bin 缓存
        if let Ok(mut bin_file) = File::create(&bin_path) {
            let _ = bin_file.write_all(&dict.to_bin(txt_mtime, config_mtime));
        }
        Ok(dict)
    }

    /// 直接加载 .bin 二进制文件
    fn load_from_bin(bin_path: &Path) -> Result<Self, String> {
        let mut buf = Vec::new();
        File::open(bin_path)
            .map_err(|e| format!("无法读取二进制文件 {:?}: {e}", bin_path))?
            .read_to_end(&mut buf)
            .map_err(|e| format!("无法读取二进制文件内容: {e}"))?;
        Self::try_from((0, 0, buf.as_slice()))
            .map_err(|e| format!("无效的二进制文件 {:?}: {e}", bin_path))
    }

    /// 将 Dict 序列化为二进制格式（包含 DictMeta 头部）
    pub fn to_bin(&self, txt_mtime: i64, config_mtime: i64) -> Vec<u8> {
        let meta = DictMeta {
            head: ['s', 'e', 'n', 'i', 'm', 'e'],
            ver: VERSION,
            txt_mtime,
            config_mtime,
        };
        let cfg = config::standard();
        let mut buf = Vec::new();
        // 30 字节头部
        let mut head = [0u8; 30];
        bincode::encode_into_slice(meta, &mut head, cfg).unwrap();
        buf.extend_from_slice(&head);
        // 按顺序编码 arena, candidates, prism, config
        buf.extend(bincode::encode_to_vec(&self.arena, cfg).unwrap());
        buf.extend(bincode::encode_to_vec(&self.candidates, cfg).unwrap());
        buf.extend(bincode::encode_to_vec(&self.prism, cfg).unwrap());
        buf.extend(bincode::encode_to_vec(&self.config, cfg).unwrap());
        buf
    }

    pub fn reachable(&self, chars: &[char]) -> bool {
        self.prism.lookup(chars).is_some()
    }

    pub fn search(&self, chars: &[char]) -> Option<Vec<CandidateView<'_>>> {
        if let Some(range) = self.prism.lookup(chars) {
            let end = range.1.min(range.0 + 9);
            Some(
                self.candidates[range.0..end]
                    .iter()
                    .map(|c| self.candidate_view(c))
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn count(&self) -> usize {
        self.candidates.len()
    }
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取某个 Candidate 的借用视图
    pub fn candidate_view<'a>(&'a self, cand: &'a Candidate) -> CandidateView<'a> {
        CandidateView {
            code: cand.code(&self.arena),
            text: cand.text(&self.arena),
            weight: cand.weight,
        }
    }

    /// 迭代所有 candidates 的借用视图
    pub fn candidates_iter(&self) -> impl Iterator<Item=CandidateView<'_>> {
        self.candidates.iter().map(move |c| self.candidate_view(c))
    }

    /// 获取 arena 的引用（用于外部直接构造 Candidate 等场景）
    pub fn arena(&self) -> &StringArena {
        &self.arena
    }

    /// 获取可变 arena 的引用（用于外部向 arena 添加字符串）
    pub fn arena_mut(&mut self) -> &mut StringArena {
        &mut self.arena
    }
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
        let col = if sp.chars().all(|c| c.is_ascii_alphabetic()) {
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

#[derive(Debug, Clone, Decode, Encode, Deserialize)]
#[serde(default)]
pub struct Config {
    pub dict: Option<String>,
    pub selection_keys: [char; 9],
    pub punctuations: HashMap<char, Vec<String>>,
    pub escape_pair: Option<[char; 2]>,
    pub reverse_key: Option<char>,
    pub reverse_dict: Option<String>,
}

impl Config {
    fn patch_punctuations(&mut self, patch: HashMap<char, Vec<String>>) {
        let mut patch = patch;
        mem::swap(&mut self.punctuations, &mut patch);
        self.punctuations.extend(patch);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dict: None,
            selection_keys: default_selection_keys(),
            punctuations: default_punctuations(),
            escape_pair: default_escape_pair(),
            reverse_key: default_reverse_key(),
            reverse_dict: None,
        }
    }
}

fn default_selection_keys() -> [char; 9] {
    // ['U', 'I', 'O', 'H', 'J', 'K', 'B', 'N', 'M']
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
fn default_reverse_key() -> Option<char> {
    Some('@')
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{gen_test_dict_files, remove_test_dict_files};
    use std::time::Instant;

    fn gen_entries() -> String {
        r#"
ahb 来 1
ahc 麦克 1
ahcg 疲惫不堪 1
c 不 1
c 不是 1
cu 还 1
cb 层 1
cb 不 1
z 可 1
z 可以 1
zk 可能 1
zkc 射 1
"#
        .replace(" ", "\t")
    }

    #[test]
    fn test_dict() {
        let dict = Dict::from_str_with_config(&gen_entries(), Config::default()).unwrap();
        println!("dict loaded: {}", dict.count());
        let result = dict.search("ah".chars().collect::<Vec<_>>().as_slice());
        assert_eq!(3, result.map_or(0, |candidates| candidates.len()));
        let result = dict.search("ahb".chars().collect::<Vec<_>>().as_slice());
        assert_eq!(1, result.map_or(0, |candidates| candidates.len()));
        let result = dict.search("aha".chars().collect::<Vec<_>>().as_slice());
        assert_eq!(0, result.map_or(0, |candidates| candidates.len()));
        let result = dict.search("c".chars().collect::<Vec<_>>().as_slice());
        assert_eq!(5, result.map_or(0, |candidates| candidates.len()));
        let result = dict.search("cb".chars().collect::<Vec<_>>().as_slice());
        assert_eq!(2, result.map_or(0, |candidates| candidates.len()));
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
        let (config_path, _dict_path) = gen_test_dict_files();
        let time_start = Instant::now();
        let dict = Dict::load(config_path.clone());
        let time_dict_loaded = Instant::now();
        println!("loaded {} from {:?}", dict.count(), config_path);
        let candidates = dict.search("a".chars().collect::<Vec<_>>().as_slice());
        let time_searched = Instant::now();
        println!(
            "searched: {}",
            candidates.as_ref().map_or(0, |c| c.len())
        );
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
