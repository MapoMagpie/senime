use bincode::{Decode, Encode, config};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{Error, ErrorKind, Read, Write},
    mem,
    path::{Path, PathBuf},
    str::FromStr,
    time::{Duration, UNIX_EPOCH},
};

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

#[derive(Debug, Clone, Decode, Encode)]
pub struct Candidate {
    pub code: String,
    pub text: String,
    pub weight: i32,
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Eq for Candidate {}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.code.cmp(&other.code) {
            std::cmp::Ordering::Equal => other.weight.cmp(&self.weight),
            ord => ord,
        }
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Candidate {
    fn parse(raw: &str, columns: &[Column]) -> Result<Self, Error> {
        let split = raw.split('\t').collect::<Vec<_>>();
        if split.len() < 2 {
            Err(Error::new(ErrorKind::InvalidData, format!("无效行: {raw}")))
        } else {
            let mut code = "";
            let mut text = Vec::with_capacity(0);
            let mut weight = 0;
            for (i, col) in columns.iter().enumerate() {
                if let Some(v) = split.get(i) {
                    match *col {
                        Column::Code => code = v.trim(),
                        Column::Text => text = v.trim().chars().collect::<Vec<_>>(),
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
            if text.len() == 1 && is_extended_cjk(text[0]) {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("拓展字符: {raw}"),
                ));
            }
            Ok(Self {
                code: code.to_string(),
                text: text.into_iter().collect(),
                weight,
            })
        }
    }
}

// Dict 节点结构
#[derive(Debug, Decode, Encode)]
struct Prism {
    keys: Vec<Vec<char>>,
    indices: Vec<(usize, usize)>,
}

impl Prism {
    fn new(candidates: &[Candidate]) -> Self {
        let mut map: BTreeMap<Vec<char>, (usize, usize)> = BTreeMap::new();
        for (i, cand) in candidates.iter().enumerate() {
            let code = cand.code.chars().collect::<Vec<_>>();
            for len in 1..code.len() + 1 {
                let prefix = &code[..len];
                map.entry(prefix.to_vec())
                    .and_modify(|r| r.1 = i + 1)
                    .or_insert((i, i + 1));
            }
        }
        let (keys, indices) = map.into_iter().unzip();
        Self { keys, indices }
    }

    fn lookup(&self, code: &[char]) -> Option<&(usize, usize)> {
        self.indices.get(
            self.keys
                .binary_search_by(|k| k.as_slice().cmp(code))
                .ok()?,
        )
    }
}

const VERSION: i64 = 10;

// Dict 结构
#[derive(Debug, Decode, Encode)]
pub struct Dict {
    prism: Prism,
    pub candidates: Vec<Candidate>,
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
        bincode::decode_from_slice::<Dict, _>(buf, config::standard())
            .map_err(|_| map_err("无效的二进制数据[DICT]"))
            .map(|a| a.0)
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
        let mut candidates = Vec::new();
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
            if let Ok(candidate) = Candidate::parse(line, columns.as_ref().unwrap()) {
                candidates.push(candidate);
            }
        }
        candidates.sort();
        let prism = Prism::new(&candidates);
        Ok(Self {
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
        let meta = DictMeta {
            head: ['s', 'e', 'n', 'i', 'm', 'e'],
            ver: VERSION,
            txt_mtime,
            config_mtime,
        };
        if let Ok(mut bin_file) = File::create(&bin_path) {
            let mut head = [0u8; 30];
            if bincode::encode_into_slice(meta, &mut head, config::standard()).is_ok() {
                let _ = bin_file.write_all(&head);
                if let Ok(encoded) = bincode::encode_to_vec(&dict, config::standard()) {
                    let _ = bin_file.write_all(&encoded);
                }
            }
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

    pub fn reachable(&self, chars: &[char]) -> bool {
        self.prism.lookup(chars).is_some()
    }

    pub fn search(&self, chars: &[char]) -> Option<&[Candidate]> {
        if let Some(range) = self.prism.lookup(chars) {
            Some(&self.candidates[range.0..range.1.min(range.0 + 9)])
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
            candidates.map_or(0, |candidates| candidates.len())
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
