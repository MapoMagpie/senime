// 根据汉字词反查编码及所在的候选位置
// 同时使用编码表检查是否能顶字，
// 如果一个汉字词的编码是下一个汉字词的编码的前缀，说明不能顶字。
// 如
// aam>你好
// aamc>世界
// cee>后面
// 输入"你好后面" aam接c时触发世界，表示不能顶字需要空格
use std::{cmp::Ordering, ops::Range};
use trie::Trie;

use ahash::AHashMap;

use crate::dict::Candidate;

#[derive(Debug, Clone)]
struct CodePos(Vec<char>, u16);

pub struct Looker {
    map: AHashMap<Vec<char>, Vec<CodePos>>,
    code_trie: Trie,
    // 码表中最长的字词，将用于动态规划时的长度
    max_text_len: usize,
}

const INFINITY: usize = 100000000;
impl Looker {
    pub fn new(candidates: &[Candidate]) -> Self {
        let mut map: AHashMap<Vec<char>, Vec<CodePos>> = AHashMap::new();
        let mut code_trie = Trie::new();
        let mut code = "";
        let mut pos = 0;
        let mut max_text_len = 0;

        for cand in candidates {
            // let cand = &candidates[i];
            if code == cand.code {
                pos += 1;
            } else {
                code_trie.insert(cand.code.chars());
                code = &cand.code;
                pos = 0;
            }
            let chars = cand.text.chars().collect::<Vec<_>>();
            let codes = cand.code.chars().collect::<Vec<_>>();
            max_text_len = max_text_len.max(chars.len());
            match map.get_mut(&chars) {
                Some(v) => {
                    v.push(CodePos(codes, pos));
                }
                None => {
                    map.insert(chars, vec![CodePos(codes, pos)]);
                }
            }
        }
        Self {
            map,
            code_trie,
            max_text_len,
        }
    }

    fn get<'a>(&'a self, text: &[char]) -> Option<&'a Vec<CodePos>> {
        self.map.get(text)
    }
    fn get_single<'a>(&'a self, text: &[char]) -> Option<&'a CodePos> {
        self.map.get(text).map(|c| &c[0])
    }

    pub fn analyze<'a>(&'a self, chars: &'a [char]) -> Vec<Segment<'a>> {
        let char_len = chars.len();
        // dp[i]表示前i个字符的最小编码长度
        let mut dp: Vec<Segment> = vec![Segment::default(); char_len + 1];
        dp[0].cost = 0; // 空字符串的编码长度为0
        // 记录分词路径
        let mut path: Vec<Option<usize>> = vec![None; char_len + 1];
        let mut j_cursor = 0;
        for i in 1..=char_len {
            // 不存在于码表中的字符单独作为一段，如标点符号
            // 另外，如果一个单字不在码表里，却可能包含在某个词里，因此不以无编码的字来分割
            // is_alphanumeric 指的是汉字与字母数字类的char，不包括标点符号emoji等
            let cha = &chars[i - 1];
            if !cha.is_alphanumeric() {
                dp[i] = Segment::new((i - 1)..i, &chars[i - 1..i], &chars[i - 1..i], 0, true, 0);
                path[i] = Some(i - 1);
                j_cursor = i;
                continue;
            }
            if i - j_cursor > self.max_text_len {
                j_cursor += 1;
            }
            let mut j = j_cursor;
            while j < i {
                let word = &chars[j..i];
                // println!("[{j}..{i}] {word:?}, j [{j}] cost: {}", dp[j].cost);
                if let Some(code_pos) = self.get(word) {
                    // 找出下一单字对应的编码的首个字
                    let next_code = (i < char_len)
                        .then(|| self.get_single(&chars[i..i + 1]).map(|cp| &cp.0[0]))
                        .flatten();
                    let (code_cost, no_space, CodePos(code, pos)) =
                        self.code_cost(code_pos, next_code);
                    let j_cost = if i - j == 1 && dp[j].cost == INFINITY {
                        if dp[i].cost < INFINITY {
                            j += 1;
                            continue;
                        }
                        0
                    } else {
                        dp[j].cost
                    };
                    let curr_cost = j_cost + code_cost;
                    // println!(
                    //     "word: [{}] code cost: [{}], befo cost: [{}], curr cost: [{}]",
                    //     word, code_cost, dp[j].cost, curr_cost
                    // );
                    if curr_cost < dp[i].cost {
                        // println!("set [{i}]> {j} cost: {}, word: {word}", curr_cost);
                        dp[i] = Segment::new(j..i, word, code, *pos, no_space, curr_cost);
                        path[i] = Some(j);
                    }
                } else if i - j == 1 && path[i].is_none() {
                    // 单字不存在于码表，也可能是英文字母
                    // 进入下一轮时，可能整个词里包含这个字，这种情况下 path[i] 表现为Some
                    path[i] = Some(j);
                    dp[i] = Segment::new(j..i, word, word, 0, true, INFINITY);
                }
                j += 1;
            }
        }
        // dp.iter()
        //     .zip(path.iter())
        //     .enumerate()
        //     .for_each(|(i, (seg, path))| {
        //         println!(
        //             "[{i}] path:{path:?} range: {:?} text: {} code: {} cost: {}",
        //             seg.range, seg.text, seg.code, seg.cost
        //         );
        //     });
        // 回溯找出分词方案
        let mut segments = vec![];
        let mut i = char_len;
        while i > 0 {
            let j: usize = path[i].unwrap_or_else(|| {
                panic!(
                    "分词时出现错误: [{}] in [{}]",
                    chars[i.min(char_len - 1)],
                    chars[(i - 10).max(0)..(i + 10).min(char_len - 1)]
                        .iter()
                        .collect::<String>()
                )
            });
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
    fn code_cost<'a>(
        &'a self,
        code_pos: &'a [CodePos],
        next: Option<&char>,
    ) -> (usize, bool, &'a CodePos) {
        code_pos
            .iter()
            .map(|cp| {
                let cost = cp.0.len();
                // 需要选重
                if cp.1 > 0 {
                    return (cost + 2, false, cp);
                }
                // 需要空格
                if next.is_some()
                    && self
                        .code_trie
                        .reachable(cp.0.iter().chain(std::iter::once(next.unwrap())))
                {
                    return (cost + 1, false, cp);
                }
                // 自动顶
                (cost, true, cp)
            })
            .min_by(|a, b| match a.0.cmp(&(b.0)) {
                Ordering::Equal => a.2.0.len().cmp(&b.2.0.len()),
                other => other,
            })
            .unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct Segment<'a> {
    pub range: Range<usize>,
    pub text: &'a [char],
    pub code: &'a [char],
    pub pos: u16,
    pub auto_select: bool,
    pub cost: usize,
}

impl<'a> Segment<'a> {
    pub fn simple(&self) -> (Range<usize>, u16, bool) {
        (self.range.clone(), self.pos, self.auto_select)
    }
}

impl<'a> Default for Segment<'a> {
    fn default() -> Self {
        Self {
            range: Default::default(),
            text: Default::default(),
            code: Default::default(),
            pos: Default::default(),
            auto_select: true,
            cost: INFINITY,
        }
    }
}

impl<'a> Segment<'a> {
    fn new(
        range: Range<usize>,
        text: &'a [char],
        code: &'a [char],
        pos: u16,
        auto_select: bool,
        cost: usize,
    ) -> Self {
        Self {
            range,
            text,
            code,
            pos,
            auto_select,
            cost,
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
            ("jdma", "人民网world", 0),
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
        let text = "中华人民是中华人民共和国的公民中华人民是中华人民共和国的公民，共和hello国人民网world。"
            .chars()
            .collect::<Vec<_>>();
        let segments = looker.analyze(text.as_slice());
        let ranges = segments
            .into_iter()
            .map(|seg| seg.text.iter().collect::<String>())
            .collect::<Vec<_>>();
        let expected = vec![
            "中华",
            "人民",
            "是",
            "中华人民共和国",
            "的",
            "公民",
            "中华",
            "人民",
            "是",
            "中华人民共和国",
            "的",
            "公民",
            "，",
            "共和",
            "h",
            "e",
            "l",
            "l",
            "o",
            "国",
            "人民网world",
            "。",
        ];
        assert_eq!(ranges, expected);
    }
    #[test]
    fn test_with_file_analyze_segments() {
        let dict = Dict::load("../test/虎码码表.txt");
        let looker = Looker::new(&dict.candidates);
        let text = r#"奶煺妫约词鼓阋丫龉攀笫伲灰褂行俺嘧又摹保部梢愿吒咝诵说目吹"#;
        // let text = r#"噼里啪啦噼里啪啦噼里啪啦噼里啪啦噼里啪啦噼里啪啦噼里啪啦噼里啪啦"#;
        // let text = r#"Ｋ纳贸ぶΓ匀皇窃谑闱榈氖坏庖黄窦涔适率渌凳录＜蚱樱闯渎哦"#;
        // let text = "当第一片梧桐叶染上金黄，当清晨的风带上丝丝凉意，当枝头的果实缀满枝头，我们便知道，秋，悄无声息地来了。秋天没有春天的姹紫嫣红，没有夏天的热烈奔放，没有冬天的银装素裹，却有着独属于自己的温柔与厚重，像一杯陈年的酒，越品越有滋味，像一首悠远的诗，越读越有深意。秋日的清晨，薄雾缭绕，天地间仿佛蒙上了一层轻纱，朦胧而美好。路边的草木褪去了盛夏的翠绿，换上了五彩的盛装，金黄的银杏、火红的枫叶、深褐的松柏，交织在一起，构成了一幅绚丽多彩的画卷。踩在铺满落叶的小路上，脚下发出“沙沙”的声响，那是秋天的私语，是岁月的回响。午后的阳光变得格外温柔，不再像盛夏那样刺眼，透过枝叶的缝隙洒下来，在地上投下斑驳的光影。坐在院子里，晒着太阳，看着落叶随风飘落，心中没有丝毫的伤感，反而多了一份从容与淡然。秋天是收获的季节，田埂上，金黄的稻田随风起伏，像一片金色的海洋；果园里，红彤彤的苹果、黄澄澄的梨子、沉甸甸的葡萄，挂满了枝头，散发着诱人的香气，那是汗水浇灌的成果，是岁月馈赠的惊喜。秋天也是沉淀的季节，褪去了盛夏的浮躁，人心也变得沉静下来。我们开始复盘过往，整理心情，放下不必要的执念，珍藏那些温暖的回忆，为接下来的日子积蓄力量。秋意渐浓，岁月沉香，愿我们能在这温柔的秋日里，收获成长，珍藏美好，不负时光，不负自己。"
        let text = text.chars().collect::<Vec<_>>();
        let segments = looker.analyze(text.as_slice());
        let ranges = segments
            .iter()
            .map(|seg| {
                format!(
                    "{}{}{}",
                    seg.text.iter().collect::<String>(),
                    seg.code.iter().collect::<String>(),
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

mod trie {

    #[derive(Debug)]
    struct TrieNode {
        // 使用 usize 存储子节点在 Vec 中的索引，None 表示没有子节点
        children: [Option<usize>; 26],
    }

    impl TrieNode {
        fn new() -> Self {
            Self {
                children: [None; 26],
            }
        }
    }

    #[derive(Debug)]
    pub struct Trie {
        // 所有节点存储在连续的内存中
        nodes: Vec<TrieNode>,
    }

    impl Default for Trie {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Trie {
        pub fn new() -> Self {
            // 初始化时放入根节点（索引为 0）
            Self {
                nodes: vec![TrieNode::new()],
            }
        }

        pub fn insert(&mut self, chars: impl IntoIterator<Item = char>) {
            let mut current_idx = 0;

            for c in chars.into_iter() {
                // 只处理 a-z
                let index = (c as usize).wrapping_sub('a' as usize);
                if index >= 26 {
                    break;
                }

                // 如果当前字符对应的子节点不存在，则创建一个新节点并推入 Vec
                if self.nodes[current_idx].children[index].is_none() {
                    let next_idx = self.nodes.len();
                    self.nodes.push(TrieNode::new());
                    self.nodes[current_idx].children[index] = Some(next_idx);
                }

                // 移动到子节点
                current_idx = self.nodes[current_idx].children[index].unwrap();
            }
        }

        pub fn reachable<I, C>(&self, chars: I) -> bool
        where
            I: IntoIterator<Item = C>,
            C: std::borrow::Borrow<char>,
        {
            let mut current_idx = 0;
            let mut hit = false;

            for c in chars {
                let index = (*c.borrow() as usize).wrapping_sub('a' as usize);
                if index >= 26 {
                    return false;
                }

                hit = true;
                match self.nodes[current_idx].children[index] {
                    Some(next_idx) => current_idx = next_idx,
                    None => return false,
                }
            }
            hit
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
}
