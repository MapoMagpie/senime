use std::collections::HashMap;

use crate::trie::{Candidate, Trie};

pub struct InputAnalyzer {
    trie: Trie,
    selection_keys: HashMap<char, usize>,
}

impl InputAnalyzer {
    pub fn new(trie: Trie) -> Self {
        let mut selection_keys = HashMap::new();
        selection_keys.insert('U', 0);
        selection_keys.insert('I', 1);
        selection_keys.insert('O', 2);
        selection_keys.insert('H', 3);
        selection_keys.insert('J', 4);
        selection_keys.insert('K', 5);
        selection_keys.insert('B', 6);
        selection_keys.insert('N', 7);
        selection_keys.insert('M', 8);
        Self {
            trie,
            selection_keys,
        }
    }
}

impl InputAnalyzer {
    pub fn analyze<I>(&self, input: I) -> AnalysisResult
    where
        I: IntoIterator<Item = char>,
    {
        let mut segments: Vec<(usize, Vec<char>)> = vec![];
        let mut codes = vec![];
        for c in input.into_iter().chain(std::iter::once('+')) {
            let is_code = c >= 'a' && c <= 'z';
            if is_code {
                codes.push(c);
            }
            let reachable = self.trie.reachable(&codes);
            // 继续构建codes，可允许的条件为：
            //   1. 目前的codes有效
            //   2. 当前的char的小写字母，表示当前操作并非选词
            //   3. 当前的char不是+，+意味着input结束
            if reachable && is_code && c != '+' {
                continue;
            }
            // 如果reachable为false，且当前char是小写字母，说明chars[0..-1]有效，应自动选词
            // 如果reachable为true， 但当前char不是小写字母，说明chars[..]有效，应解析当前char为选词索引，并选词
            if is_code && c != '+' {
                codes.pop();
            }
            let code = codes.clone();
            if codes.len() > 0 {
                let select_index = self.selection_keys.get(&c).map_or(0, |&i| i);
                segments.push((select_index, code));
                codes.clear();
            }
            if is_code {
                codes.push(c);
            }
        }
        // println!("segments: {:?}", segments);
        let segment_len = segments.len();
        let text_with_last_candidates = segments
            .into_iter()
            .enumerate()
            .map(|(i, seg)| {
                self.trie
                    .search(&seg.1)
                    .map_or(("".to_string(), vec![]), |candidates| {
                        let text = if candidates.is_empty() {
                            "".to_string()
                        } else {
                            candidates.get(seg.0).unwrap_or(&candidates[0]).text.clone()
                        };
                        (
                            text,
                            if i == segment_len - 1 {
                                candidates[0..candidates.len().min(9)].to_vec()
                            } else {
                                vec![]
                            },
                        )
                    })
            })
            .collect::<Vec<_>>();
        let candidates = text_with_last_candidates.last().map_or(vec![], |e| {
            e.1.iter().map(|c| (*c).clone()).collect::<Vec<_>>()
        });
        let sentence = text_with_last_candidates
            .into_iter()
            .map(|e| e.0)
            .collect::<Vec<_>>();
        AnalysisResult {
            sentence,
            candidates,
        }
    }
}

// 解析结果
pub struct AnalysisResult {
    pub sentence: Vec<String>,
    pub candidates: Vec<Candidate>,
}

#[cfg(test)]
mod tests {
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
    fn test_analyzer() {
        let mut trie = Trie::new();
        let candidates = create_candidate();
        for (code, text, weight) in candidates {
            trie.insert(
                code.chars().collect::<Vec<_>>().as_slice(),
                text.to_string(),
                weight,
            );
        }
        let analyzer = InputAnalyzer::new(trie);
        let input = "a cIzk";
        let result = analyzer.analyze(input.chars());
        assert_eq!(result.sentence, vec!["来", "不是", "可能"]);
        let input = "acIzk";
        let result = analyzer.analyze(input.chars());
        assert_eq!(result.sentence, vec!["来", "不是", "可能"]);
        let input = "zk cuahcI";
        let result = analyzer.analyze(input.chars());
        assert_eq!(result.sentence, vec!["来", "不是", "可能"]);
    }
}
