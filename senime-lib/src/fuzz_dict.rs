use nucleo_matcher::Matcher;
use nucleo_matcher::Utf32String;
use nucleo_matcher::pattern::{Atom, AtomKind, CaseMatching, Normalization};

use crate::dict::{Candidate, StringArena};

/// 基于 nucleo 的模糊查询字典，用于 emoji 等场景。
///
/// 内部使用 `StringArena` + `Vec<Candidate>` 存储条目（与 Dict 共享格式），
/// `Candidate.code` 对应标签字符串，`Candidate.text` 对应展示文本（如 emoji）。
///
/// 查询时用 `,` 或 `|` 分隔多个关键词，每个关键词独立做模糊匹配，
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
        let has_letter = input.iter().any(|c| c.is_ascii_alphabetic());
        if !has_letter {
            return false;
        }
        input
            .iter()
            .all(|c| c.is_ascii_alphabetic() || *c == ',' || *c == '|')
    }

    /// 模糊搜索，返回 `(candidate_index, score)` 列表，按得分降序排列。
    pub fn search(&self, query: &str) -> Vec<(usize, u16)> {
        let tokens: Vec<&str> = query.split([',', '|']).filter(|t| !t.is_empty()).collect();

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
                let total_score: u16 = atoms
                    .iter()
                    .filter_map(|atom| atom.score(haystack, &mut matcher))
                    .sum();
                if total_score > 0 {
                    Some((i, total_score))
                } else {
                    None
                }
            })
            .collect();

        // 按得分降序，得分相同时按权重降序，再按 code 长度升序（短标签优先）
        results.sort_by(|(i1, s1), (i2, s2)| match s2.cmp(s1) {
            std::cmp::Ordering::Equal => {
                match self.candidates[*i2]
                    .weight
                    .cmp(&self.candidates[*i1].weight)
                {
                    std::cmp::Ordering::Equal => {
                        let len1 = self.get_code(&self.candidates[*i1]).len();
                        let len2 = self.get_code(&self.candidates[*i2]).len();
                        len1.cmp(&len2)
                    }
                    ord => ord,
                }
            }
            ord => ord,
        });
        results
    }
}
