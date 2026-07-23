use nucleo_matcher::Matcher;
use nucleo_matcher::Utf32String;
use nucleo_matcher::pattern::{Atom, AtomKind, CaseMatching, Normalization};

use crate::dict::{Candidate, StringArena};

/// 基于 nucleo 的模糊查询字典，用于 emoji 等场景。
///
/// 内部使用 `StringArena` + `Vec<Candidate>` 存储条目（与 Dict 共享格式），
/// `Candidate.code` 对应标签字符串，`Candidate.text` 对应展示文本（如 emoji）。
///
/// 查询时用 `,` 分隔多个关键词，每个关键词独立做模糊匹配，
/// 最终结果按得分降序排列。
#[derive(Debug)]
pub struct FuzzDict {
    pub arena: StringArena,
    pub candidates: Vec<Candidate>,
}

impl FuzzDict {
    pub fn count(&self) -> usize {
        self.candidates.len()
    }

    /// 从 arena 中解析 code（标签）字符串。
    pub fn get_code(&self, cand: &Candidate) -> &str {
        self.arena.get(cand.code.0, cand.code.1)
    }

    /// 从 arena 中解析 text（展示文本）字符串。
    pub fn get_text(&self, cand: &Candidate) -> &str {
        self.arena.get(cand.text.0, cand.text.1)
    }

    pub fn get_str(&self, range: (u32, u16)) -> &str {
        self.arena.get(range.0, range.1)
    }

    /// 判断输入是否满足模糊查询的基本格式：
    /// 仅包含 ASCII 字母和分隔符 `,` `|`，且至少有一个字母。
    pub fn reachable(&self, input: &[char]) -> bool {
        if input.is_empty() {
            return false;
        }
        if input[0] == ',' && input.len() == 1 {
            return false;
        }
        input.iter().all(|c| c.is_ascii_lowercase() || *c == ',')
    }

    /// 模糊搜索，返回 `(candidate_index, score)` 列表，按得分降序排列。
    pub fn search(&self, query: &str) -> Vec<(usize, u16)> {
        let tokens: Vec<&str> = query.split(',').filter(|t| !t.is_empty()).collect();

        if tokens.is_empty() {
            return Vec::new();
        }

        let mut matcher = Matcher::new(nucleo_matcher::Config::DEFAULT);
        let atoms: Vec<Atom> = tokens
            .iter()
            .map(|t| {
                Atom::new(
                    t,
                    CaseMatching::Ignore,
                    Normalization::Never,
                    AtomKind::Fuzzy,
                    false,
                )
            })
            .collect();

        let mut results: Vec<(usize, u16)> = self
            .candidates
            .iter()
            .enumerate()
            .filter_map(|(i, cand)| {
                let tag_str = self.get_code(cand);
                let tag_utf32 = Utf32String::from(tag_str);
                let haystack = tag_utf32.slice(..);
                let mut score = 0;
                for atom in atoms.iter() {
                    match atom.score(haystack, &mut matcher) {
                        Some(s) => score += s,
                        None => return None,
                    }
                }
                Some((i, score))
            })
            .collect();

        // 按得分降序，得分相同时保持元素在文件中的位置
        results.sort_by(|le, ri| ri.1.cmp(&le.1));
        // println!("search: {}", query);
        // results.iter().for_each(|re| {
        //     println!(
        //         "candidate: {}, text: {}, code: {}, score: {}",
        //         re.0,
        //         self.get_text(&self.candidates[re.0]),
        //         self.get_code(&self.candidates[re.0]),
        //         re.1
        //     );
        // });
        results
    }
}

#[cfg(test)]
mod test {
    use nucleo_matcher::{
        Matcher, Utf32String,
        pattern::{Atom, AtomKind, CaseMatching, Normalization},
    };

    fn gen_fuzz_entries() -> String {
        r#"
你好	nihao,hello,oi
世界	shijie,world,sikei
心	heart,beat
苹果	apple,fruit,food
西瓜	watermelon,fruit,food
梨子	pear,fruit,food
芒果	mango,fruit,food
菠萝	pineapple,fruit,food
♡	White Heart Suit
➡️	Right Arrow
⬅️	Left Arrow
⬇️	Down Arrow
⬆️	Up Arrow
💘	Heart With Arrow
"#
        .to_string()
    }
    #[test]
    fn test_nucleo_base() {
        let raw = gen_fuzz_entries();
        let lines = raw.lines();
        let mut matcher = Matcher::new(nucleo_matcher::Config::DEFAULT);
        let patt = Atom::new(
            "hear arrow",
            CaseMatching::Smart,
            Normalization::Never,
            AtomKind::Fuzzy,
            false,
        );
        let ret = lines
            .filter_map(|line| {
                let tag_utf32 = Utf32String::from(line);
                let haystack = tag_utf32.slice(..);
                patt.score(haystack, &mut matcher).map(|s| (s, line))
            })
            .collect::<Vec<_>>();
        println!("ret:");
        if ret.is_empty() {
            println!("ret is empty");
        }
        for ele in ret {
            println!("{:?}", ele);
        }
    }
}
