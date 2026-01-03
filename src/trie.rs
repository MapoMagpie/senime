use std::{
    fs::File,
    io::{self, Error, Read},
    path::PathBuf,
};

#[derive(Debug, Clone)]
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
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Candidate {
    fn new(code: String, text: String, weight: i32) -> Self {
        Candidate { code, text, weight }
    }
}

// Trie 节点结构
#[derive(Debug)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    candidates: Vec<Candidate>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: [const { None }; 26],
            candidates: Vec::new(),
        }
    }

    fn insert(&mut self, chars: &[char], candidate: Candidate) {
        if chars.len() > 0 {
            let index = (chars[0] as usize) - 0x61;
            if let Some(child) = self.children[index].as_mut() {
                child.insert(&chars[1..], candidate);
            } else {
                let mut new_child = TrieNode::new();
                new_child.insert(&chars[1..], candidate);
                self.children[index] = Some(Box::new(new_child));
            }
        } else {
            self.candidates.push(candidate);
        }
    }

    fn all_candidates(&self) -> Vec<&Candidate> {
        let mut result = vec![];
        let a = self
            .candidates
            .iter()
            .map(|c| c)
            .collect::<Vec<&Candidate>>();
        result.extend(a);
        result.sort();
        for child in self.children.iter() {
            if let Some(child) = child {
                result.extend(child.all_candidates());
            }
        }
        // result.dedup();
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
    pub count: usize,
}

impl Trie {
    pub fn load<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let mut trie = Trie::new();
        let mut count = 0;
        let mut file = File::open(path.into()).expect("无法读取码表文件");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("无法从码表中读取内容");
        for line in content.lines() {
            match Pair::new(line) {
                Ok(pair) => {
                    count += 1;
                    let code = pair.code.as_slice();
                    let text = pair.text.clone();
                    trie.insert(code, text, pair.weight);
                }
                Err(_err) => {
                    // println!("{:?}", err.to_string())
                }
            }
        }
        trie.count = count;
        trie
    }

    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
            count: 0,
        }
    }

    pub fn insert(&mut self, chars: &[char], text: String, weight: i32) {
        if chars.len() > 0 {
            self.root.insert(
                chars,
                Candidate::new(chars.iter().collect::<String>(), text, weight),
            );
        }
    }

    pub fn reachable(&self, chars: &[char]) -> bool {
        self.root.search(chars).is_some()
    }

    pub fn search(&self, chars: &[char]) -> Option<Vec<&Candidate>> {
        let node = self.root.search(chars);
        if let Some(node) = node {
            Some(node.all_candidates())
        } else {
            None
        }
    }
}
struct Pair {
    code: Vec<char>,
    text: String,
    weight: i32,
}

impl Pair {
    fn new(raw: &str) -> Result<Self, Error> {
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
            Ok(Self {
                code,
                text: text.into_iter().collect(),
                weight,
            })
        }
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
    use std::time::Instant;

    use super::*;

    fn create_candidate() -> Vec<(&'static str, &'static str, i32)> {
        vec![
            ("ahb", "来", 1),
            ("ahc", "麦克", 1),
            ("ahcg", "疲惫不堪", 1),
            ("c", "不", 1),
            ("c", "不是", 1),
            ("cu", "还", 1),
            ("cb", "层", 1),
            ("cb", "不", 1),
            ("z", "可", 1),
            ("z", "可以", 1),
            ("zk", "可能", 1),
            ("zkc", "射", 1),
        ]
    }

    #[test]
    fn test_trie() {
        let candidates = create_candidate();
        let mut trie = Trie::new();
        for (code, text, weight) in candidates {
            trie.insert(
                code.chars().collect::<Vec<char>>().as_slice(),
                text.to_string(),
                weight,
            );
        }
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

    #[test]
    fn test_load() {
        let path = "./test/虎码码表.txt";
        let time_start = Instant::now();
        let trie = Trie::load(path);
        let time_trie_loaded = Instant::now();
        println!("loaded {} from {}", trie.count, path);
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
    }

    #[test]
    fn test_other() {
        let text = "码表输入法";
        println!("text len: {}", text.len());
        println!("char count: {}", text.chars().count());
        text.chars().for_each(|c| {
            println!("char: {}", c);
            println!("unicode: U+{:X}", c as u32);
        });
    }
}
