// 根据汉字词反查编码及所在的候选位置
// 同时使用编码表检查是否能顶字，
// 如果一个汉字词的编码是下一个汉字词的编码的前缀，说明不能顶字。
// 如
// aam>你好
// aamc>世界
// cee>后面
// 输入"你好后面" aam接c时触发世界，表示不能顶字需要空格
use std::{ops::Range, ptr};

use ahash::AHashMap;

use crate::dict::Candidate;

#[derive(Debug, Clone)]
struct CodePos(String, usize);

pub struct Looker {
    map: AHashMap<String, Vec<CodePos>>,
    code_trie: Trie,
}

const INFINITY: usize = 10000000;
impl Looker {
    pub fn new(candidates: &[Candidate]) -> Self {
        let mut map: AHashMap<String, Vec<CodePos>> = AHashMap::new();
        let mut code_trie = Trie::new();
        let mut code = "";
        let mut pos = 0;

        for i in 0..candidates.len() {
            let cand = &candidates[i];
            if code == cand.code {
                pos += 1;
            } else {
                code_trie.insert(cand.code.chars());
                code = &cand.code;
                pos = 0;
            }
            match map.get_mut(cand.text.as_str()) {
                Some(v) => {
                    v.push(CodePos(cand.code.clone(), pos));
                }
                None => {
                    map.insert(cand.text.clone(), vec![CodePos(cand.code.clone(), pos)]);
                }
            }
        }
        Self { map, code_trie }
    }

    fn get<'a>(&'a self, text: &str) -> Option<&'a Vec<CodePos>> {
        let codes = self.map.get(text);
        codes
    }
    fn get_single<'a>(&'a self, text: &str) -> Option<&'a CodePos> {
        let codes = self.map.get(text);
        codes.map(|c| &c[0])
    }

    pub fn analyze(&self, chars: &[char]) -> Vec<Segment> {
        let char_len = chars.len();
        // dp[i]表示前i个字符的最小编码长度
        let mut dp: Vec<Segment> =
            vec![
                Segment::new(0..1, "".to_string(), "".to_string(), 0, false, INFINITY);
                char_len + 1
            ];
        dp[0].cost = 0; // 空字符串的编码长度为0
        // 记录分词路径
        let mut path: Vec<Option<usize>> = vec![None; char_len + 1];
        let mut j_cursor = 0;
        for i in 1..=char_len {
            // 不存在于码表中的字符单独作为一段，如标点符号
            // 另外，如果一个字不在码表里，虽然也可以进行分割，但是单字虽然不在码表，却可能包含在某个词里，因此不以无编码的字来分割
            // is_alphanumeric 指的是汉字与字母数字类的char，不包括标点符号emoji等
            let cha = &chars[i - 1];
            if !cha.is_alphanumeric() {
                dp[i] = Segment::new((i - 1)..i, cha.to_string(), cha.to_string(), 0, true, 0);
                path[i] = Some(i - 1);
                j_cursor = i;
                continue;
            }
            let mut j = j_cursor;
            while j < i {
                let word = chars[j..i].iter().collect::<String>();
                if let Some(code_pos) = self.get(word.as_str()) {
                    // 找出下一单字对应的编码的首个字
                    let next_code = (i < char_len)
                        .then(|| {
                            self.get_single(chars[i].to_string().as_str())
                                .map(|cp| cp.0.chars().next().map(|c| c.to_string()))
                        })
                        .flatten()
                        .flatten()
                        .unwrap_or("".to_string());
                    let (code_cost, no_space, CodePos(code, pos)) =
                        self.code_cost(code_pos, next_code.as_str());
                    let curr_cost = dp[j].cost + code_cost;
                    if curr_cost < dp[i].cost {
                        dp[i] = Segment::new(j..i, word, code, pos, no_space, curr_cost);
                        path[i] = Some(j);
                    }
                }
                j += 1;
            }
        }
        // 回溯找出分词方案
        let mut segments = vec![];
        let mut i = char_len;
        while i > 0 {
            let j: usize = path[i].expect("分词时出现错误");
            segments.push(dp.swap_remove(i));
            i = j;
        }
        segments.reverse();
        segments
    }

    /// 计算编码消耗
    /// 初始消耗: 编码长度code.len()
    /// 自动顶+0
    /// 空格  +1
    /// 次选  +2
    /// 最后选取消耗最小的CodePos.
    fn code_cost(&self, code_pos: &Vec<CodePos>, next: &str) -> (usize, bool, CodePos) {
        let costs: Vec<(usize, bool, &CodePos)> = code_pos
            .iter()
            .map(|cp| {
                let cost = cp.0.len();
                // 需要选重
                if cp.1 > 0 {
                    return (cost + 2, true, cp);
                }
                // 需要空格
                if !next.is_empty() && self.code_trie.reachable(cp.0.chars().chain(next.chars())) {
                    return (cost + 1, false, cp);
                }
                // 自动顶
                (cost, true, cp)
            })
            .collect();
        let (cost, no_space, code_pos) = costs.iter().min_by_key(|x| x.0).unwrap();
        (*cost, *no_space, (*code_pos).clone())
    }
}

#[derive(Clone, Debug)]
pub struct Segment {
    pub range: Range<usize>,
    pub text: String,
    pub code: String,
    pub pos: usize,
    pub auto_select: bool,
    pub cost: usize,
}

impl Segment {
    fn new(
        range: Range<usize>,
        text: String,
        code: String,
        pos: usize,
        auto_select: bool,
        length: usize,
    ) -> Self {
        Self {
            range,
            text,
            code,
            pos,
            auto_select,
            cost: length,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Dict, dict::Candidate, lookup_code::Looker};

    fn create_candidates() -> Vec<Candidate> {
        let data = vec![
            ("d", "中", 0),
            ("jv", "华", 0),
            ("j", "人", 0),
            ("dm", "民", 0),
            ("lhb", "共", 0),
            ("xd", "和", 0),
            ("rn", "国", 0),
            ("dgjv", "中华", 0),
            ("jrdm", "人民", 0),
            ("lhxd", "共和", 0),
            ("lxrn", "共和国", 0),
            ("djjr", "中华人民共和国", 0),
            ("o", "是", 0),
            ("u", "的", 0),
            ("hkdm", "公民", 0),
            ("djv", "喻", 0),
            ("jdm", "人民网", 0),
        ];
        let mut cands: Vec<_> = data
            .iter()
            .map(|d| Candidate {
                code: d.0.to_string(),
                text: d.1.to_string(),
                weight: d.2,
            })
            .collect();
        cands.sort();
        cands
    }

    #[test]
    fn test_analyze_segments() {
        let cands = create_candidates();
        let looker = Looker::new(&cands);
        let text = "中华人民是中华人民共和国的公民，共和国"
            .chars()
            .collect::<Vec<_>>();
        let segments = looker.analyze(text.as_slice());
        let ranges = segments.into_iter().map(|seg| seg.text).collect::<Vec<_>>();
        let expected = vec![
            "中华",
            "人民",
            "是",
            "中华人民共和国",
            "的",
            "公民",
            "，",
            "共和国",
        ];
        assert_eq!(ranges, expected);
    }
    #[test]
    fn test_with_file_analyze_segments() {
        let dict = Dict::load("../test/虎码码表.txt");
        let looker = Looker::new(&dict.candidates);
        let text = "当第一片梧桐叶染上金黄，当清晨的风带上丝丝凉意，当枝头的果实缀满枝头，我们便知道，秋，悄无声息地来了。秋天没有春天的姹紫嫣红，没有夏天的热烈奔放，没有冬天的银装素裹，却有着独属于自己的温柔与厚重，像一杯陈年的酒，越品越有滋味，像一首悠远的诗，越读越有深意。秋日的清晨，薄雾缭绕，天地间仿佛蒙上了一层轻纱，朦胧而美好。路边的草木褪去了盛夏的翠绿，换上了五彩的盛装，金黄的银杏、火红的枫叶、深褐的松柏，交织在一起，构成了一幅绚丽多彩的画卷。踩在铺满落叶的小路上，脚下发出“沙沙”的声响，那是秋天的私语，是岁月的回响。午后的阳光变得格外温柔，不再像盛夏那样刺眼，透过枝叶的缝隙洒下来，在地上投下斑驳的光影。坐在院子里，晒着太阳，看着落叶随风飘落，心中没有丝毫的伤感，反而多了一份从容与淡然。秋天是收获的季节，田埂上，金黄的稻田随风起伏，像一片金色的海洋；果园里，红彤彤的苹果、黄澄澄的梨子、沉甸甸的葡萄，挂满了枝头，散发着诱人的香气，那是汗水浇灌的成果，是岁月馈赠的惊喜。秋天也是沉淀的季节，褪去了盛夏的浮躁，人心也变得沉静下来。我们开始复盘过往，整理心情，放下不必要的执念，珍藏那些温暖的回忆，为接下来的日子积蓄力量。秋意渐浓，岁月沉香，愿我们能在这温柔的秋日里，收获成长，珍藏美好，不负时光，不负自己。"
            .chars()
            .collect::<Vec<_>>();
        let segments = looker.analyze(text.as_slice());
        let ranges = segments
            .iter()
            .map(|seg| {
                format!(
                    "{}{}{}",
                    seg.text,
                    seg.code,
                    if seg.pos > 0 {
                        seg.pos.to_string()
                    } else if seg.auto_select {
                        "".to_string()
                    } else {
                        "_".to_string()
                    }
                )
            })
            .collect::<Vec<_>>();
        println!("{ranges:?}");
    }
}
// Trie 节点结构
struct TrieNode {
    children: [*mut TrieNode; 26], // 26个字母的子节点
}

impl TrieNode {
    // 创建新节点
    unsafe fn new() -> *mut Self {
        let node = Box::into_raw(Box::new(TrieNode {
            children: [ptr::null_mut(); 26],
        }));
        node
    }
}

// Trie 结构
#[derive(Debug)]
pub struct Trie {
    root: *mut TrieNode,
}

impl Trie {
    // 创建新 Trie
    pub fn new() -> Self {
        unsafe {
            Trie {
                root: TrieNode::new(),
            }
        }
    }

    // 插入单词
    pub fn insert(&mut self, chars: impl IntoIterator<Item = char>) {
        unsafe {
            let mut current = self.root;

            for c in chars.into_iter() {
                let index = (c as usize) - ('a' as usize);
                if index >= 26 {
                    break;
                }
                if (*current).children[index].is_null() {
                    (*current).children[index] = TrieNode::new();
                }
                current = (*current).children[index];
            }
        }
    }

    // 搜索单词 dmt
    pub fn reachable(&self, chars: impl IntoIterator<Item = char>) -> bool {
        unsafe {
            let mut current = self.root;

            let mut hit = false;
            for c in chars {
                let index = (c as usize) - ('a' as usize);
                if index >= 26 {
                    return false;
                }
                hit = true;
                if (*current).children[index].is_null() {
                    return false;
                }
                current = (*current).children[index];
            }
            hit
        }
    }
}

impl Drop for Trie {
    fn drop(&mut self) {
        // 递归释放所有节点
        fn free_node(node: *mut TrieNode) {
            if node.is_null() {
                return;
            }

            for i in 0..26 {
                unsafe {
                    if !(*node).children[i].is_null() {
                        free_node((*node).children[i]);
                    }
                }
            }
            unsafe {
                let _ = Box::from_raw(node);
            }
        }
        free_node(self.root);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        let word_1 = vec!['d', 'm', 'r', 'l'];
        let word_2 = vec!['d', 'm'];
        let word_3 = vec!['r', 'p', 'p'];
        let word_4 = vec!['t', 'c'];
        let word_5 = vec!['x', 'c', 'd', 'z'];
        let word_6 = vec!['t', 'c', 'x', 'c'];
        let word_7 = vec!['b', 'w'];
        let word_8 = vec!['m', 'l', 'w', 'g'];
        let word_9 = vec!['b', 'w', 'm', 'l'];
        trie.insert(word_1);
        trie.insert(word_2);
        trie.insert(word_3);
        trie.insert(word_4);
        trie.insert(word_5);
        trie.insert(word_6);
        trie.insert(word_7);
        trie.insert(word_8);
        trie.insert(word_9);
        let search_1 = vec!['t', 'c', 'x'];
        let search_2 = vec!['b', 'w', 'm'];
        let search_3 = vec!['d', 'm', 't'];
        let search_4 = vec!['x', 'c', 'd', 'z'];
        assert!(trie.reachable(search_1.into_iter()));
        assert!(trie.reachable(search_2.into_iter()));
        assert!(!trie.reachable(search_3.into_iter()));
        assert!(trie.reachable(search_4.into_iter()));
        // println!("{trie:?}");
    }
}
