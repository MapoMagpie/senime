use std::{
    fs::File,
    io::{self, Error, Read},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct Candidate {
    pub code: Vec<char>,
    pub text: Vec<char>,
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
        match other.weight.cmp(&self.weight) {
            std::cmp::Ordering::Equal => self.code.cmp(&other.text),
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
    #[allow(dead_code)]
    pub fn new(code: String, text: String, weight: i32) -> Self {
        Candidate {
            code: code.chars().collect(),
            text: text.chars().collect(),
            weight,
        }
    }

    fn parse(raw: &str) -> Result<Self, Error> {
        let split = raw.split("\t").collect::<Vec<_>>();
        if split.len() < 2 {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("无效行: {raw}"),
            ))
        } else {
            let code = split[0].trim().chars().collect::<Vec<_>>();
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
            Ok(Self { code, text, weight })
        }
    }
}

// Trie 节点结构
#[derive(Debug)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    indices: Vec<usize>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: [const { None }; 26],
            indices: Vec::new(),
        }
    }

    fn insert(&mut self, chars: &[char], i_cand: usize) {
        if chars.len() > 0 {
            let index = (chars[0] as usize) - 0x61;
            if let Some(child) = self.children[index].as_mut() {
                child.insert(&chars[1..], i_cand);
            } else {
                let mut new_child = TrieNode::new();
                new_child.insert(&chars[1..], i_cand);
                self.children[index] = Some(Box::new(new_child));
            }
        } else {
            self.indices.push(i_cand);
        }
    }

    fn all_candidates(&self) -> Vec<usize> {
        let max_count = 9;
        let mut result = vec![];
        result.extend(&self.indices);
        if result.len() < max_count {
            let mut children = self
                .children
                .iter()
                .filter_map(|o| o.as_ref())
                .collect::<Vec<_>>();
            'outer: while children.len() > 0 {
                let mut new_children = Vec::new();
                for child in children.iter() {
                    if result.len() >= max_count {
                        break 'outer;
                    }
                    result.extend(&child.indices);
                    new_children.extend(child.children.iter().filter_map(|o| o.as_ref()));
                }
                children = new_children;
            }
        }
        result
    }

    fn search(&self, chars: &[char]) -> Option<&TrieNode> {
        if chars.len() > 0 {
            let index = (chars[0] as usize) - 0x61;
            if let Some(child) = self.children[index].as_ref() {
                child.search(&chars[1..])
            } else {
                None
            }
        } else {
            Some(self)
        }
    }
}

// Trie 结构
#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
    candidates: Vec<Candidate>,
}

impl Trie {
    pub fn load<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let mut file = File::open(path.into()).expect("无法读取码表文件");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("无法从码表中读取内容");
        Self::from_str(content)
    }

    pub fn from_str(raw: String) -> Self {
        let mut trie = Trie::new();
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
        candidates.iter().enumerate().for_each(|(i, c)| {
            trie.insert(&c.code, i);
        });
        trie.candidates = candidates;
        trie
    }

    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
            candidates: Vec::new(),
        }
    }

    pub fn insert(&mut self, chars: &[char], index: usize) {
        if chars.len() > 0 {
            self.root.insert(chars, index);
        }
    }

    pub fn reachable(&self, chars: &[char]) -> bool {
        self.root.search(chars).is_some()
    }

    pub fn search(&self, chars: &[char]) -> Option<Vec<&Candidate>> {
        let node = self.root.search(chars);
        if let Some(node) = node {
            let candidates = node
                .all_candidates()
                .iter()
                .map(|i| &self.candidates[*i])
                .collect::<Vec<_>>();
            Some(candidates)
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
        let trie = Trie::from_str(gen_table());
        println!("trie loaded: {}", trie.count());
        let result = trie.search("ah".chars().collect::<Vec<char>>().as_slice());
        assert_eq!(3, result.map_or(0, |candidates| candidates.len()));
        let result = trie.search("ahb".chars().collect::<Vec<char>>().as_slice());
        assert_eq!(1, result.map_or(0, |candidates| candidates.len()));
        let result = trie.search("aha".chars().collect::<Vec<char>>().as_slice());
        assert_eq!(0, result.map_or(0, |candidates| candidates.len()));
        let result = trie.search("c".chars().collect::<Vec<char>>().as_slice());
        assert_eq!(5, result.map_or(0, |candidates| candidates.len()));
        let result = trie.search("cb".chars().collect::<Vec<char>>().as_slice());
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
    #[test]
    fn test_load() {
        let path = "./test/虎码码表.txt";
        let time_start = Instant::now();
        let trie = Trie::load(path);
        let time_trie_loaded = Instant::now();
        println!("loaded {} from {}", trie.count(), path);
        let candidates = trie.search("i".chars().collect::<Vec<_>>().as_slice());
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
}
