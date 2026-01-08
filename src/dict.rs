use bincode::{Decode, Encode, config};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{self, Error, Read, Write},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

/// 二进制文件头，用于检测码表是否发生了变动
#[derive(Debug, Decode, Encode)]
struct DictMeta {
    head: [char; 6],
    ver: usize,
    mtime: i64,
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
    fn parse(raw: &str) -> Result<Self, Error> {
        let split = raw.split("\t").collect::<Vec<_>>();
        if split.len() < 2 {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("无效行: {raw}"),
            ))
        } else {
            let code = split[0].trim().to_string();
            let text = split[1].trim().chars().collect::<Vec<_>>();
            if text.len() == 1 && is_extended_cjk(text[0]) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("拓展字符: {raw}"),
                ));
            }
            if code.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("code为空: {raw}"),
                ));
            }
            let weight = if split.len() > 2 {
                split[2].parse().unwrap_or_default()
            } else {
                0
            };
            Ok(Self {
                code,
                text: text.into_iter().collect(),
                weight,
            })
        }
    }
}

// Trie 节点结构
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

// Trie 结构
#[derive(Debug, Decode, Encode)]
pub struct Dict {
    prism: Prism,
    candidates: Vec<Candidate>,
    config: Config,
}

impl Dict {
    pub fn load<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        let filename = path
            .file_name()
            .expect("无效的码表文件路径")
            .to_str()
            .unwrap();
        let t_mtime = path.metadata().expect("无法读取码表文件元信息").mtime();
        let file_dir = path
            .parent()
            .expect("码表文件需要在某个文件夹内")
            .to_str()
            .unwrap();
        let bin_path = format!("{}/{}.bin", file_dir, filename);
        File::open(&bin_path)
            .and_then(|mut file| {
                let map_err = |reason: &str| io::Error::new(io::ErrorKind::InvalidData, reason);
                let mut metadata = [0; 20];
                file.read(&mut metadata)?;
                let (DictMeta { head, ver, mtime }, _size): (DictMeta, usize) =
                    bincode::decode_from_slice(&metadata, config::standard())
                        .map_err(|_| map_err("无效的二进制数据[HEAD]"))?;
                let c_head = ['s', 'e', 'n', 'i', 'm', 'e'];
                let c_ver = 1;
                if !(head == c_head && ver == c_ver && mtime == t_mtime) {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "码表已更新，重新构建二进制文件",
                    ));
                }
                let mut buf: Vec<u8> = vec![];
                file.read_to_end(&mut buf)?;
                // 前二十个字节
                bincode::decode_from_slice::<Dict, _>(&buf, config::standard())
                    .map_err(|_| map_err("无效的二进制数据[DICT]"))
                    .map(|a| a.0)
            })
            .unwrap_or_else(|err| {
                println!("打开码表二进制文件: {}", err);
                let mut file = File::open(path).expect("无法读取码表文件");
                let mut content = String::new();
                file.read_to_string(&mut content)
                    .expect("无法从码表中读取内容");
                let mut dict_bin = File::create(bin_path).unwrap();
                let meta = DictMeta {
                    head: ['s', 'e', 'n', 'i', 'm', 'e'],
                    ver: 1,
                    mtime: t_mtime,
                };
                let mut head = [0; 20];
                bincode::encode_into_slice(meta, &mut head, config::standard()).unwrap();
                dict_bin.write(&head).unwrap();
                let dict = Self::from_str(content);
                let encoded = bincode::encode_to_vec(&dict, config::standard()).unwrap();
                dict_bin.write_all(&encoded).unwrap();
                dict
            })
    }

    pub fn from_str(raw: String) -> Self {
        let mut candidates = Vec::new();
        let mut enter_toml = false;
        let mut toml_content = String::new();
        for line in raw.lines() {
            if enter_toml {
                if line.starts_with("```") {
                    enter_toml = false;
                } else {
                    toml_content.push_str(line);
                }
                continue;
            }
            if line.starts_with("```") {
                enter_toml = true;
                continue;
            }
            match Candidate::parse(line) {
                Ok(candidate) => {
                    candidates.push(candidate);
                }
                Err(_err) => {
                    println!("{:?}", _err.to_string())
                }
            }
        }
        println!("toml_content {}", toml_content);
        let config = if toml_content.is_empty() {
            Config::default()
        } else {
            toml::from_str(&toml_content).expect("无法从码表中解析配置")
        };
        candidates.sort();
        let prism = Prism::new(&candidates);
        Self {
            candidates,
            prism,
            config,
        }
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

    // unused
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.candidates.len()
    }
    pub fn config(&self) -> Config {
        self.config.clone()
    }
}

#[derive(Debug, Clone, Decode, Encode, Deserialize)]
pub struct Config {
    #[serde(default = "default_selection_keys")]
    pub selection_keys: [char; 9],
    #[serde(default = "default_punctuations")]
    pub punctuations: HashMap<char, Vec<char>>,
    #[serde(default = "default_escape_pair")]
    pub escape_pair: Option<[char; 2]>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            selection_keys: default_selection_keys(),
            punctuations: default_punctuations(),
            escape_pair: Some(['`', '`']),
        }
    }
}

fn default_selection_keys() -> [char; 9] {
    // ['U', 'I', 'O', 'H', 'J', 'K', 'B', 'N', 'M']
    ['1', '2', '3', '4', '5', '6', '7', '8', '9']
}
fn default_punctuations() -> HashMap<char, Vec<char>> {
    let punctuations = vec![
        (',', vec!['，', ',']),
        ('.', vec!['。', '.']),
        ('!', vec!['！', '!']),
        ('/', vec!['？', '?']),
        (';', vec!['：', ';']),
        ('[', vec!['「', '“', '[']),
        (']', vec!['」', '”', ']']),
        ('\\', vec!['、', '\\']),
        ('|', vec!['·', '|']),
    ];
    let mut map = HashMap::new();
    punctuations.into_iter().for_each(|(ch, puncs)| {
        map.insert(ch, puncs);
    });
    map
}

fn default_escape_pair() -> Option<[char; 2]> {
    Some(['`', '`'])
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
    use std::time::Instant;

    fn gen_table() -> String {
        let raw = r#"
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
zkc 射 1"#;
        raw.replace(" ", "\t")
    }

    #[test]
    fn test_trie() {
        let trie = Dict::from_str(gen_table());
        println!("trie loaded: {}", trie.count());
        let result = trie.search("ah".chars().collect::<Vec<_>>().as_slice());
        assert_eq!(3, result.map_or(0, |candidates| candidates.len()));
        let result = trie.search("ahb".chars().collect::<Vec<_>>().as_slice());
        assert_eq!(1, result.map_or(0, |candidates| candidates.len()));
        let result = trie.search("aha".chars().collect::<Vec<_>>().as_slice());
        assert_eq!(0, result.map_or(0, |candidates| candidates.len()));
        let result = trie.search("c".chars().collect::<Vec<_>>().as_slice());
        assert_eq!(5, result.map_or(0, |candidates| candidates.len()));
        let result = trie.search("cb".chars().collect::<Vec<_>>().as_slice());
        // println!("result: {:?}", result);
        assert_eq!(2, result.map_or(0, |candidates| candidates.len()));
    }

    // 初版
    // loaded 162982 from ./test/虎码码表.txt
    // searched: 6099
    // trie loaded time: 392.5498ms
    // search time: 2.929678ms

    // 非递归后
    // loaded 162982 from ./test/虎码码表.txt
    // searched: 6099
    // trie loaded time: 295.150222ms
    // search time: 1.874144ms

    // 限制最大候选后，数量为9
    // loaded 162982 from ./test/虎码码表.txt
    // searched: 12
    // trie loaded time: 301.187025ms
    // search time: 21.641µs

    // 最大候选1000，无--release参数
    // loaded 162982 from ./test/虎码码表.txt
    // searched: 1000
    // trie loaded time: 460.254761ms
    // search time: 481.759µs
    //
    // 最大候选1000，无--release参数，使用prism
    // loaded 162982 from ./test/虎码码表.txt
    // searched: 1000
    // trie loaded time: 1.128577844s
    // search time: 11.27µs
    #[test]
    fn test_load() {
        let path = "./test/虎码码表.txt";
        let time_start = Instant::now();
        let dict = Dict::load(path);
        let time_trie_loaded = Instant::now();
        println!("loaded {} from {}", dict.count(), path);
        let candidates = dict.search("i".chars().collect::<Vec<_>>().as_slice());
        let time_searched = Instant::now();
        println!(
            "searched: {}",
            candidates.map_or(0, |candidates| candidates.len())
        );
        println!(
            "trie loaded time: {:?}",
            time_trie_loaded.duration_since(time_start)
        );
        println!(
            "search time: {:?}",
            time_searched.duration_since(time_trie_loaded)
        );
    }

    // #[test]
    // fn test_other() {
    //     let text = "码表输入法";
    //     println!("text len: {}", text.len());
    //     println!("char count: {}", text.chars().count());
    //     text.chars().for_each(|c| {
    //         println!("char: {}", c);
    //         println!("unicode: U+{:X}", c as u32);
    //     });
    // }

    #[test]
    fn test_dict_by_output() {
        let path = "./test/虎码码表.txt";
        let dict = Dict::load(path);
        let candidates = dict
            .candidates
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{}\t{}\t{}\t{}", i, c.code, c.text, c.weight))
            .collect::<Vec<String>>()
            .join("\n");
        let prims = dict
            .prism
            .keys
            .iter()
            .zip(dict.prism.indices)
            .map(|(k, r)| format!("{}\t{}\t{}", k.iter().collect::<String>(), r.0, r.1));
        let can_path = "./test/candidates.txt";
        let prim_path = "./test/prism.txt";
        std::fs::write(can_path, candidates).unwrap();
        std::fs::write(prim_path, prims.collect::<Vec<String>>().join("\n")).unwrap();
    }

    #[test]
    fn test_parse_config() {
        let raw = r#"
selection_keys = ["U","I","O","H","J","K","B","N","M"]
[punctuations]
"," = [",", "，"]
"." = ["。", "."]
";" = ["；", ";"]

"#;
        let config: Config = toml::from_str(raw).unwrap();
        println!("config 1: {:?}", config);
        let raw = r#"
[punctuations]
"," = [",", "，"]
"." = ["。", "."]
";" = ["；", ";"]
"#;
        let config: Config = toml::from_str(raw).unwrap();
        println!("config 2: {:?}", config);
        let raw = r#""#;
        let config: Config = toml::from_str(raw).unwrap();
        println!("config 3: {:?}", config);
    }
}
