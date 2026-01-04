use bincode::{Decode, Encode, config};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, BufReader, Error, Read, Write},
    path::PathBuf,
};

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
            if text.len() == 1 {
                if is_extended_cjk(text[0]) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("拓展字符: {raw}"),
                    ));
                }
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
}

impl Dict {
    pub fn load<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        let filename = path.file_name().unwrap().to_str().unwrap();
        let file_dir = path.parent().unwrap().to_str().unwrap();
        let bin_path = format!("{}/{}.bin", file_dir, filename);
        match File::open(&bin_path) {
            Ok(mut file) => {
                let reader = BufReader::new(&mut file);
                let trie: Dict = bincode::decode_from_reader(reader, config::standard()).unwrap();
                // println!("Loaded trie from file: {}", trie.count());
                trie
            }
            _ => {
                let mut file = File::open(path).expect("无法读取码表文件");
                let mut content = String::new();
                file.read_to_string(&mut content)
                    .expect("无法从码表中读取内容");
                let dict = Self::from_str(content);
                let encoded = bincode::encode_to_vec(&dict, config::standard()).unwrap();
                let mut dict_bin = File::create(bin_path).unwrap();
                dict_bin.write_all(&encoded).unwrap();
                dict
            }
        }
    }

    pub fn from_str(raw: String) -> Self {
        let mut candidates = Vec::new();
        for line in raw.lines() {
            match Candidate::parse(line) {
                Ok(candidate) => {
                    candidates.push(candidate);
                }
                Err(_err) => {
                    println!("{:?}", _err.to_string())
                }
            }
        }
        candidates.sort();
        let prism = Prism::new(&candidates);
        Self { candidates, prism }
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
}

// https://github.com/rime/librime/blob/47033202f986f4dced82eceb90440285fcb9501e/src/rime/gear/charset_filter.cc#L18
fn is_extended_cjk(c: char) -> bool {
    let c = c as u32;
    c >= 0x3400  && c <= 0x4DBF  ||  // CJK Unified Ideographs Extension A
    c >= 0x20000 && c <= 0x2A6DF ||  // CJK Unified Ideographs Extension B
    c >= 0x2A700 && c <= 0x2B73F ||  // CJK Unified Ideographs Extension C
    c >= 0x2B740 && c <= 0x2B81F ||  // CJK Unified Ideographs Extension D
    c >= 0x2B820 && c <= 0x2CEAF ||  // CJK Unified Ideographs Extension E
    c >= 0x2CEB0 && c <= 0x2EBEF ||  // CJK Unified Ideographs Extension F
    c >= 0x30000 && c <= 0x3134F ||  // CJK Unified Ideographs Extension G
    c >= 0x31350 && c <= 0x323AF ||  // CJK Unified Ideographs Extension H
    c >= 0x2EBF0 && c <= 0x2EE5F ||  // CJK Unified Ideographs Extension I
    c >= 0x323B0 && c <= 0x3347F ||  // CJK Unified Ideographs Extension J
    c >= 0x3300  && c <= 0x33FF  ||  // CJK Compatibility
    c >= 0xFE30  && c <= 0xFE4F  ||  // CJK Compatibility Forms
    c >= 0xF900  && c <= 0xFAFF  ||  // CJK Compatibility Ideographs
    c >= 0x2F800 && c <= 0x2FA1F // CJK Compatibility Ideographs Supplement
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn gen_table() -> String {
        let raw = r#"ahb 来 1
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
        // calculate Trie candidates memory usage
        // let trie_size = trie.get_heap_size();
        // let candidates_size = trie.candidates.get_heap_size();
        // let node_size = trie.root.get_heap_size();
        // println!("Trie size: {} bytes", trie_size);
        // println!("Candidates size: {} bytes", candidates_size);
        // println!("Node size: {} bytes", node_size);
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
}
