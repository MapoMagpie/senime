use std::iter::Chain;
use std::mem;
use std::ops::Range;
use std::slice::Iter;

use crossterm::event::KeyCode;
pub type SentenceChars<'a> =
    Chain<Chain<Chain<Iter<'a, char>, Iter<'a, char>>, Iter<'a, char>>, Iter<'a, char>>;

/// 输入的字符
/// 在中间插入时，为了避免频繁对Vec中间进行修改影响性能
/// 故采用多级机制
/// chars:      输入字符的最终状态，不过一直没有过中间修改的话，pending才是输入字符的最终状态。
/// append_at: 当进行中间修改时，pending_at将对应char中的某个位置，直到光标再移动到另一位置后，将pending合并到char，并更新pending_at为光标的位置。
/// appending:    这是在pending_at后，写入的字符，只有在变动pending_at(光标位置)后，才会将pending放入chars对应的位置
///             一种情况是，如输入的过程中，始终没有进行过中间修改，那么chars会一直是空，真正的输入在pending里，这是正常且符合预期的。
///             另一种情况便是中间修改，当要修改中间某处时，先将pending归并到chars里，并设置新的pending_at与pending，
///               这样在中间写入大量的字符时，始终在pending里追加，对性能影响不大。
/// pending:   未决的输入，在输入时，由于还有其他候选，这段字符变动非常频繁。它的主要作用是参与diff
#[derive(Debug, Default)]
pub(crate) struct Sentence {
    chars: Vec<char>,
    appends: Vec<char>,
    append_at: usize,
    pending: Vec<char>,
    pending_origin: Vec<char>,
}

impl Sentence {
    pub(crate) fn len(&self) -> usize {
        self.chars.len() + self.appends.len() + self.pending.len()
    }

    /// 到pending的长度
    pub(crate) fn pending_end(&self) -> usize {
        self.append_at + self.appends.len() + self.pending.len()
    }
    pub(crate) fn append_end(&self) -> usize {
        self.append_at + self.appends.len()
    }

    pub(crate) fn get_chars<'a>(&'a self) -> SentenceChars<'a> {
        self.chars[..self.append_at]
            .iter()
            .chain(self.appends.iter())
            .chain(self.pending.iter())
            .chain(self.chars[self.append_at..].iter())
    }

    pub(crate) fn set_append_at(&mut self, at: usize) {
        if !self.appends.is_empty() {
            let mut old_append = mem::take(&mut self.appends);
            if !self.pending.is_empty() {
                let old_pending = mem::take(&mut self.pending);
                self.pending_origin.clear();
                old_append.extend(old_pending);
            }
            let _ = self
                .chars
                .splice(self.append_at..self.append_at, old_append);
        }
        self.append_at = at;
    }

    pub(crate) fn extend(&mut self, chars: impl IntoIterator<Item = char>) {
        self.appends.extend(chars);
    }

    pub(crate) fn clear(&mut self) {
        self.chars.clear();
        self.appends.clear();
        self.pending.clear();
        self.append_at = 0;
    }

    pub(crate) fn pop(&mut self) {
        if !self.pending.is_empty() {
            self.pending_origin.pop();
            if self.pending_origin.is_empty() {
                self.clear_pending();
            }
        } else if !self.appends.is_empty() {
            self.appends.pop();
        } else if !self.chars.is_empty() {
            if self.append_at == 0 {
                return;
            }
            if self.append_at == self.chars.len() {
                self.chars.pop();
            } else {
                // WARN: 影响性能
                self.chars.remove(self.append_at - 1);
            }
            self.append_at -= 1;
        }
    }

    pub(crate) fn push_input(&mut self, c: char) {
        self.pending_origin.push(c);
    }

    pub(crate) fn set_pending(&mut self, pending: Vec<char>, origin: Vec<char>) {
        self.pending = pending;
        self.pending_origin = origin;
    }

    pub(crate) fn get_pending(&self) -> &[char] {
        &self.pending
    }
    pub(crate) fn get_pending_origin(&self) -> &[char] {
        &self.pending_origin
    }
    pub(crate) fn get_append_at(&self) -> usize {
        self.append_at
    }
    pub(crate) fn move_append_at(&mut self, action: Movement) {
        match action {
            Movement::Left => {
                if self.append_at > 0 {
                    self.append_at -= 1;
                }
            }
            Movement::Right => {
                if self.append_at < self.len() {
                    self.append_at += 1;
                }
            }
            _ => {
                panic!(" 未实现 ");
            } // Movement::Up => self.sentence.append_at -= 1,
              // Movement::Down => self.sentence.append_at -= 1,
        }
    }

    pub(crate) fn clear_pending(&mut self) {
        self.pending.clear();
        self.pending_origin.clear();
    }

    /// 根据一个宏观的range从chars, appending, pending中先出正确范围的字符
    /// 情况1：  chars     = ['a', 'b', 'g', 'h'];
    ///          appending = ['c', 'd']; append_at = 2;
    ///          pending   = ['e', 'f'];
    ///        range =  (1..4)时，对应的字符该是 ['b', 'c', 'd']
    ///          c_range  = (1..2)
    ///          a_range  = (0..2)
    ///          p_range  = (0..0)
    ///          c_range2 = (0..0)
    /// 情况2：  chars     = ['a', 'b', 'g', 'h'];
    ///          appending = ['c', 'd']; append_at = 2;
    ///          pending   = ['e', 'f'];
    ///        range =  (0..7)时，对应的字符该是 ['a', 'b', 'c', 'd', 'e', 'f', 'g']
    ///          c_range  = (0..2)
    ///          a_range  = (0..2)
    ///          p_range  = (0..2)
    ///          c_range2 = (2..3)
    pub(crate) fn get_chars_by<'a>(&'a self, range: Range<usize>) -> SentenceChars<'a> {
        if range.start > range.end {
            panic!("the range's start > range's end");
        }
        // 定义四个物理分段在“宏观空间”中的起止位置
        let c1_len = self.append_at;
        let a_len = self.appends.len();
        let p_len = self.pending.len();

        // 宏观布局：
        // [0 .. c1_len] -> self.chars[0..append_at]
        // [c1_len .. c1_len + a_len] -> self.appends
        // [c1_len + a_len .. c1_len + a_len + p_len] -> self.pending
        // [c1_len + a_len + p_len .. total] -> self.chars[append_at..]

        let seg_a_start = c1_len;
        let seg_p_start = seg_a_start + a_len;
        let seg_c2_start = seg_p_start + p_len;

        // 计算宏观 range 与某个分段 [seg_start, seg_end) 的交集
        // 返回值是相对于该分段起始位置的本地索引
        let intersect = |seg_start: usize, seg_len: usize| -> Range<usize> {
            let seg_end = seg_start + seg_len;
            let overlap_start = range.start.max(seg_start).min(seg_end);
            let overlap_end = range.end.max(seg_start).min(seg_end);
            (overlap_start - seg_start)..(overlap_end - seg_start)
        };

        // 直接计算四个本地 range
        let c_range_1 = intersect(0, c1_len);
        let a_range = intersect(seg_a_start, a_len);
        let p_range = intersect(seg_p_start, p_len);

        // c_range_2 对应的是 self.chars 的后半部分，其本地起始索引是 self.append_at
        let c2_local = intersect(seg_c2_start, self.chars.len() - self.append_at);
        let c_range_2 = (c2_local.start + self.append_at)..(c2_local.end + self.append_at);

        self.chars[c_range_1]
            .iter()
            .chain(self.appends[a_range].iter())
            .chain(self.pending[p_range].iter())
            .chain(self.chars[c_range_2].iter())
    }
}

pub enum Movement {
    Left,
    Right,
    Up,
    Down,
}

impl From<KeyCode> for Movement {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Left => Movement::Left,
            KeyCode::Right => Movement::Right,
            KeyCode::Up => Movement::Up,
            KeyCode::Down => Movement::Down,
            _ => unreachable!("movement from other keycodes"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sentence::Sentence;

    #[test]
    fn test_sentence() {
        let mut sentence = Sentence::default();
        sentence.extend(vec!['a', 'b', 'c', 'd', 'e']);
        sentence.set_append_at(3);
        sentence.extend(vec!['1', '2', '3', '4', '5']);
        let chars = sentence
            .get_chars()
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            chars,
            vec!['a', 'b', 'c', '1', '2', '3', '4', '5', 'd', 'e']
        );
        let chars = sentence
            .get_chars_by(2..6)
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(chars, vec!['c', '1', '2', '3']);

        let chars = sentence
            .get_chars_by(0..0)
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(chars, Vec::<char>::new());

        let chars = sentence
            .get_chars_by(3..8)
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(chars, vec!['1', '2', '3', '4', '5']);

        sentence.set_append_at(6);
        sentence.extend(vec!['A', 'B']);
        let chars = sentence
            .get_chars()
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            chars,
            vec!['a', 'b', 'c', '1', '2', '3', 'A', 'B', '4', '5', 'd', 'e']
        );
        sentence.set_pending(vec!['你', '好'], vec![]);
        let chars = sentence
            .get_chars()
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            chars,
            vec![
                'a', 'b', 'c', '1', '2', '3', 'A', 'B', '你', '好', '4', '5', 'd', 'e'
            ]
        );
        sentence.set_append_at(6);
        let chars = sentence
            .get_chars()
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            chars,
            vec![
                'a', 'b', 'c', '1', '2', '3', 'A', 'B', '你', '好', '4', '5', 'd', 'e'
            ]
        );
        let chars = sentence
            .get_chars_by(8..10)
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(chars, vec!['你', '好']);

        sentence.pop();
        sentence.pop();
        let chars = sentence
            .get_chars_by(8..10)
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(chars, vec!['4', '5']);

        let chars = sentence
            .get_chars()
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            chars,
            vec!['a', 'b', 'c', '1', 'A', 'B', '你', '好', '4', '5', 'd', 'e']
        );
        sentence.set_append_at(sentence.len());
        sentence.set_pending(vec!['悬', '决'], vec!['f', 'k', 'w', 'n']);
        let chars = sentence
            .get_chars()
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            chars,
            vec![
                'a', 'b', 'c', '1', 'A', 'B', '你', '好', '4', '5', 'd', 'e', '悬', '决'
            ]
        );
        sentence.pop();
        let chars = sentence
            .get_chars_by(10..sentence.len())
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(chars, vec!['d', 'e', '悬', '决']);
        sentence.pop();
        sentence.pop();
        sentence.pop();
        let chars = sentence
            .get_chars_by(10..sentence.len())
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(chars, vec!['d', 'e']);
        sentence.clear();
        sentence.extend(vec!['a', 'b', 'c', 'd']);
        let chars = sentence
            .get_chars_by(2..4)
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(chars, vec!['c', 'd']);
        sentence.clear();
        sentence.extend(vec!['a', 'b', 'c', 'd']);
        sentence.set_append_at(2);
        sentence.extend(vec!['1', '2']);
        let chars = sentence
            .get_chars_by(2..sentence.len())
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(chars, vec!['1', '2', 'c', 'd']);
        sentence.set_pending(vec!['你', '好'], vec![]);
        let chars = sentence
            .get_chars_by(2..sentence.len())
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(chars, vec!['1', '2', '你', '好', 'c', 'd']);
        let chars = sentence
            .get_chars_by(2..2)
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(chars, Vec::<char>::new());
        sentence.set_append_at(0);
        sentence.set_pending(vec!['你', '好'], vec![]);
        let chars = sentence
            .get_chars_by(1..2)
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(chars, vec!['好']);
        let chars = sentence
            .get_chars_by(2..2)
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(chars, Vec::<char>::new());
        let chars = sentence
            .get_chars_by(0..sentence.len())
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            chars,
            vec!['你', '好', 'a', 'b', '1', '2', '你', '好', 'c', 'd']
        );
        let chars = sentence
            .get_chars_by(10..sentence.len())
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(chars, Vec::<char>::new());
    }

    // use crate::context::diff_sequence;
    // use unicode_width::UnicodeWidthStr;
    // #[test]
    // fn test_create_diff_text() {
    //     let left = "hello, world".chars().collect::<Vec<_>>();
    //     let right = "hella,_world, gray".chars().collect::<Vec<_>>();
    //     let diff_indices = diff_sequence(left.iter(), Some(right.iter()));
    //     println!("text: {diff_indices:?}");
    // }

    // #[test]
    // fn test_punc_length() {
    //     let punc = "……";
    //     let width_cjk = punc.width_cjk();
    //     let width2 = punc.width();
    //     println!("{punc} > width cjk: {width_cjk}, width 2: {width2}]");
    //     let punc = "——";
    //     let width_cjk = punc.width_cjk();
    //     let width2 = punc.width();
    //     println!("{punc} > width cjk: {width_cjk}, width 2: {width2}]");
    //     let punc = "你好";
    //     let width_cjk = punc.width_cjk();
    //     let width2 = punc.width();
    //     println!("{punc} > width cjk: {width_cjk}, width 2: {width2}]");
    //     let punc = "你好abc";
    //     let width_cjk = punc.width_cjk();
    //     let width2 = punc.width();
    //     println!("{punc} > width cjk: {width_cjk}, width 2: {width2}]");
    // }
}
