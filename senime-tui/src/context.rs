use std::{borrow::Borrow, mem, ops::Range, time::Instant};

use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::Style,
    widgets::Widget,
};
use senime_lib::Looker;
use unicode_width::UnicodeWidthChar;

const COLOR_DIFF: usize = 2;
const COLOR_SEG_LEN: usize = 2;
const COLOR_NORMAL: usize = 3;
const COLOR_PENDING: usize = 4;
const COLOR_PALETTE: [Style; 5] = [
    Style::new().on_light_cyan().dark_gray().dim(),
    Style::new().on_light_blue().dark_gray().dim(),
    Style::new().on_light_red().crossed_out().white(),
    Style::new(),
    Style::new().magenta().underlined(),
];

#[derive(Debug)]
pub struct Record {
    pub len: i32,
    pub origin: Vec<char>,
    pub start: Instant,
    pub end: Instant,
}

use std::iter::Chain;
use std::slice::Iter;
type SentenceChars<'a> =
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
#[allow(dead_code)]
#[derive(Debug)]
struct Sentence {
    chars: Vec<char>,
    appends: Vec<char>,
    append_at: usize,
    pending: Vec<char>,
    pending_origin: Vec<char>,
}

impl Default for Sentence {
    fn default() -> Self {
        Self {
            chars: Default::default(),
            appends: Default::default(),
            append_at: Default::default(),
            pending: Default::default(),
            pending_origin: Default::default(),
        }
    }
}

#[allow(dead_code)]
impl Sentence {
    fn len(&self) -> usize {
        self.chars.len() + self.appends.len() + self.pending.len()
    }

    /// 到pending的长度
    fn pending_end(&self) -> usize {
        self.append_at + self.appends.len() + self.pending.len()
    }
    fn append_end(&self) -> usize {
        self.append_at + self.appends.len()
    }

    fn get_chars<'a>(&'a self) -> SentenceChars<'a> {
        self.chars[..self.append_at]
            .iter()
            .chain(self.appends.iter())
            .chain(self.pending.iter())
            .chain(self.chars[self.append_at..].iter())
    }

    fn set_append_at(&mut self, at: usize) {
        if !self.appends.is_empty() {
            let mut old_append = mem::replace(&mut self.appends, vec![]);
            if !self.pending.is_empty() {
                let old_pending = mem::replace(&mut self.pending, vec![]);
                self.pending_origin.clear();
                old_append.extend(old_pending);
            }
            let _ = self
                .chars
                .splice(self.append_at..self.append_at, old_append);
        }
        self.append_at = at;
    }

    fn extend(&mut self, chars: impl IntoIterator<Item = char>) {
        self.appends.extend(chars);
    }

    fn clear(&mut self) {
        self.chars.clear();
        self.appends.clear();
        self.append_at = 0;
    }

    fn pop(&mut self) {
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

    fn push_input(&mut self, c: char) {
        self.pending_origin.push(c);
    }

    fn set_pending(&mut self, pending: Vec<char>, origin: Vec<char>) {
        self.pending = pending;
        self.pending_origin = origin;
    }

    fn clear_pending(&mut self) {
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
    fn get_chars_by<'a>(&'a self, range: Range<usize>) -> SentenceChars<'a> {
        if range.start > range.end {
            panic!("the range's start > range's end");
        }
        let mut c_range_1 = 0..0;
        let mut c_range_2 = self.append_at..self.append_at;
        let mut a_range = 0..self.appends.len();
        let mut p_range = 0..self.pending.len();
        // 实际只计算一轮，但中间有可以提早结束的条件
        loop {
            if range.start < self.append_at {
                c_range_1.start = range.start;
                c_range_1.end = self.append_at;
                if range.end < self.append_at {
                    c_range_1.end = range.end;
                    a_range.end = 0;
                    p_range.end = 0;
                    break;
                }
            }
            let append_end = self.append_at + self.appends.len();
            let pending_end = append_end + self.pending.len();

            if range.end > pending_end {
                c_range_2.end = self.append_at + (range.end - pending_end);
                if range.start > pending_end {
                    c_range_2.start = self.append_at + range.start - pending_end;
                    a_range.end = 0;
                    p_range.end = 0;
                    break;
                }
            }
            if range.start > self.append_at && range.start <= append_end {
                a_range.start = range.start - self.append_at;
            }
            // append_at = 3 range 2..6 期待 a_range = 0..3
            // append_end = 8
            // 6 - 3 = 3 range.end - append_at = append_end
            if range.end < append_end {
                a_range.end = range.end - self.append_at;
                p_range.end = 0;
                break;
            }
            if range.start > append_end && range.start <= pending_end {
                p_range.start = range.start - append_end;
            }
            if pending_end > range.end {
                p_range.end = range.end - append_end;
            }
            break;
        }
        // eprintln!(
        //     "append_at: [{}], chars len: [{}], append len: [{}], pending len: [{}]",
        //     self.append_at,
        //     self.chars.len(),
        //     self.appends.len(),
        //     self.pending.len(),
        // );
        // eprintln!(
        //     "--\nrange: {range:?}, \nc_range_1: {c_range_1:?}\na_range: {a_range:?}\np_range: {p_range:?}\nc_range_2: {c_range_2:?}"
        // );
        self.chars[c_range_1]
            .iter()
            .chain(self.appends[a_range].iter())
            .chain(self.pending[p_range].iter())
            .chain(self.chars[c_range_2].iter())
    }
}

pub struct Context<'a> {
    preset: Option<Vec<char>>,
    sentence: Sentence,
    styles: Vec<(usize, Option<Style>)>,
    records: Vec<Record>,
    input_start: Instant,
    before_pending_style: Vec<(usize, Option<Style>)>,
    enc: &'a Looker,
    pre_render: PreRender,
    abs_cursor: Position,
}
impl<'a> Context<'a> {
    pub fn new(enc: &'a Looker) -> Self {
        Self {
            preset: Default::default(),
            sentence: Default::default(),
            styles: Default::default(),
            records: Default::default(),
            input_start: Instant::now(),
            before_pending_style: Default::default(),
            pre_render: Default::default(),
            abs_cursor: Default::default(),
            enc,
        }
    }
    pub fn set_preset(&mut self, preset: Option<Vec<char>>) {
        self.preset = preset;
        if let Some(preset) = self.preset.as_ref() {
            self.segment(0..preset.len());
        }
    }

    fn set_style(&mut self, i: usize, style: Option<(usize, Option<Style>)>) {
        // 扩容
        if i + 1 > self.styles.len() {
            let new_vec = vec![(COLOR_NORMAL, None); (i + 1) - self.styles.len()];
            self.styles.extend(new_vec);
        }
        match style {
            Some(style_with_patch) => self.styles[i] = style_with_patch,
            None => self.styles[i] = (COLOR_NORMAL, None),
        }
    }

    pub fn clear(&mut self) {
        self.clear_pending();
        self.sentence.clear();
        self.segment(0..self.preset.as_ref().map(|p| p.len()).unwrap_or(0));
    }

    fn diff(&mut self, range: Range<usize>, style_on_same: Option<(usize, Option<Style>)>) {
        if let Some(preset) = self.preset.as_ref()
            && range.start < preset.len()
        {
            // eprintln!("sentence: {:?}, range: {:?}", self.sentence, range);
            let chars = self.sentence.get_chars_by(range.clone());
            let other = &preset[range.start..range.end.min(preset.len())];

            // let chars = chars.collect::<Vec<_>>();
            // eprintln!(
            //     "range: {:?}, \nchars: {}\nother: {}",
            //     range,
            //     chars.iter().map(|c| c.to_owned()).collect::<String>(),
            //     other.iter().collect::<String>(),
            // );
            let diff_range = diff_sequence(chars, Some(other.iter()))
                .into_iter()
                .enumerate()
                .map(|(i, d)| {
                    (
                        range.start + i,
                        if d {
                            style_on_same
                        } else {
                            Some((COLOR_DIFF, None))
                        },
                    )
                })
                .collect::<Vec<_>>();

            diff_range
                .into_iter()
                .for_each(|(i, s)| self.set_style(i, s));
        }
    }

    fn segment(&mut self, range: Range<usize>) {
        if let Some(preset) = self.preset.as_ref() {
            // 找到重新分词计算的范围
            let ret = self.enc.analyze(&preset[range.clone()]);
            let mut iter = ret.into_iter().map(|seg| seg.simple()).collect::<Vec<_>>(); //.collect::<Vec<_>>();
            // 从后向前，在patch_style中会根据i扩容，先用最大数避免多次扩容
            iter.reverse();

            // 在不断的输入中，为了保持分词色彩不乱变，需要让修正group_idx
            // 如果range.end 小于 preset.len，表示局部小范围分词，
            // 从preset[range.end]上找到COLOR_PALETTE所在的位置，如1
            // 那么preset[range.end]所在的group idx % COLOR_SEG_LEN后 该为0
            // 由于要递减，所以选一般较大的固定基数，并且是偶数
            let mut group_idx = if range.end < preset.len() {
                // eprintln!(
                //     "next char: [{}]{}, style: {:?}",
                //     range.end, preset[range.end], self.styles[range.end]
                // );
                100_000_000 + self.styles[range.end].0
            } else {
                100_000_000
            };
            // eprintln!("group_idx: {}", (group_idx - 1) % COLOR_SEG_LEN);

            for (slice_range, pos, auto_select) in iter {
                group_idx -= 1;
                for in_slice_idx in slice_range.rev() {
                    let i = in_slice_idx + range.start;
                    let patch = if pos > 0 {
                        Some(Style::default().crossed_out())
                    } else if !auto_select && pos == 0 {
                        Some(Style::default().underlined())
                    } else {
                        None
                    };
                    self.set_style(i, Some((group_idx % COLOR_SEG_LEN, patch)));
                }
            }
        }
    }

    /// 添加新的输入结果
    /// 如果存在预设文章（赛文用）
    ///     进行当前输入差异比对.
    ///     进行当前索引之后到下一标点符号为止的分词计算
    pub fn push(&mut self, text: impl IntoIterator<Item = char>, origin: Vec<char>) {
        let text: Vec<char> = text.into_iter().collect();
        let txt_len = text.len();
        // let old_sen_len = self.sentence.len();
        let record = Record {
            len: txt_len as i32,
            origin: origin.clone(),
            start: self.input_start,
            end: Instant::now(),
        };
        self.records.push(record);
        self.sentence.extend(text);
        self.sentence.clear_pending();
        let start_at = self.sentence.append_end() - txt_len;
        let end = self.sentence.len();
        let range = start_at..end;
        if !self.before_pending_style.is_empty() {
            self.before_pending_style
                .iter()
                .enumerate()
                .for_each(|(i, s)| {
                    self.styles[range.start + i] = *s;
                });
            self.before_pending_style.clear();
        }
        // eprintln!(
        //     "push diff range: {:?}, old len: {}, new len: {}",
        //     range, old_sen_len, new_sen_len
        // );
        self.diff(range.clone(), None);
        if let Some(preset) = self.preset.as_ref()
            && range.end < preset.len()
        {
            let end = advance_to_word_boundary(&preset[range.end..], 0);
            self.segment(range.end..range.end + end);
        }
    }

    pub fn set_pending(&mut self, pending: impl IntoIterator<Item = char>, input: Vec<char>) {
        let pending: Vec<char> = pending.into_iter().collect();
        let start_at = self.sentence.append_end();
        let old_pen_end = self.sentence.pending_end();
        self.sentence.set_pending(pending, input);
        let new_pen_end = self.sentence.pending_end();
        let end = self.sentence.len();
        let range = start_at..end;

        if range.end <= self.styles.len() {
            if old_pen_end < new_pen_end {
                self.before_pending_style
                    .extend((&self.styles[old_pen_end..new_pen_end]).to_vec());
            } else {
                self.before_pending_style
                    .iter()
                    .enumerate()
                    .for_each(|(i, s)| {
                        self.styles[range.start + i] = *s;
                    });
            }
        }
        self.diff(range, Some((COLOR_PENDING, None)));
    }

    pub fn clear_pending(&mut self) {
        self.before_pending_style.clear();
        self.sentence.clear_pending();
    }

    pub fn confrim_pending(&mut self) {
        self.before_pending_style.clear();
        if !self.sentence.pending.is_empty() {
            let pending = self.sentence.pending.clone();
            let pending_origin = self.sentence.pending_origin.clone();
            let record = Record {
                len: pending.len() as i32,
                origin: pending_origin,
                start: self.input_start,
                end: Instant::now(),
            };
            self.records.push(record);
            self.sentence.extend(pending);
            self.sentence.clear_pending();
        }
    }

    pub fn push_input(&mut self, c: char) {
        if self.sentence.pending_origin.is_empty() {
            self.before_pending_style.clear();
            self.input_start = Instant::now();
        }
        self.sentence.push_input(c);
    }

    pub fn backspace(&mut self) {
        if !self.before_pending_style.is_empty() {
            self.before_pending_style.clear();
        }
        if self.sentence.len() > 0 {
            self.sentence.pop();
            let record = Record {
                len: -1,
                origin: vec![],
                start: Instant::now(),
                end: Instant::now(),
            };
            self.records.push(record);
        }
        // 需要diff
        if self.sentence.append_at != self.sentence.len() {
            let start = self.sentence.pending_end();
            let end = self.sentence.len();
            self.diff(start..end, None);
        }
        if let Some(preset) = self.preset.as_ref() {
            let sen_len = self.sentence.len();
            if sen_len < preset.len() {
                let end = advance_to_word_boundary(&preset[sen_len..], 0);
                self.segment(sen_len..sen_len + end);
            }
        }
    }

    /// 移动指针
    pub fn move_cursor(&mut self, action: Movement) {
        self.confrim_pending();
        self.sentence.set_append_at(self.sentence.pending_end());
        // eprintln!(
        //     "sentence len:[{}] append_at:[{}] appends: {:?}",
        //     self.sentence.len(),
        //     self.sentence.append_at,
        //     self.sentence.appends
        // );
        match action {
            Movement::Left => {
                if self.sentence.append_at > 0 {
                    self.sentence.append_at -= 1;
                }
            }
            Movement::Right => {
                if self.sentence.append_at < self.sentence.len() {
                    self.sentence.append_at += 1;
                }
            }
            _ => {
                panic!(" 未实现 ");
            } // Movement::Up => self.sentence.append_at -= 1,
              // Movement::Down => self.sentence.append_at -= 1,
        }
    }

    pub fn get_input(&self) -> &[char] {
        &self.sentence.pending_origin
    }

    pub fn get_recorders(&self) -> &[Record] {
        &self.records
    }

    pub fn get_sentence(&'a self) -> SentenceChars<'a> {
        self.sentence.get_chars()
    }

    pub fn sentence_len(&self) -> usize {
        self.sentence.len()
    }

    /// 根据宽度进行折行计算
    /// 为了提高计算效率，对于光标所在行之前的行不再计算
    pub fn calc_pre_render(&mut self, area: Rect) {
        let mut line_idx = self.abs_cursor.y as usize;
        if self.abs_cursor.y > 0 {
            line_idx -= 1;
        }
        // 找出当前行在chars中的开始范围
        let start = self
            .pre_render
            .get(line_idx)
            .map(|line| line[1].sentence_i)
            .unwrap_or(0);
        // 在当前光标后，有几行位于视图内，如果不在视图内，则停止折行计算
        // self.abs_cursor永远出现在视图内，
        //    当其y值大于area.height时，会在下方留下一行的空间
        //    当其y值小于area.height时，则直接相减
        let left_lines = if line_idx > area.height as usize {
            3 // 当前光标前一行 + 当前光标行 + 留空一行 = 3行
        } else {
            area.height as usize - line_idx
        };

        let cursor_at = self.sentence.pending_end() - start;
        // eprintln!(
        //     "calc_pre_render: start: [{start}] sen_len: [{}] line_idx: [{line_idx}] left_lines: [{left_lines}]",
        //     self.sentence.len()
        // );

        let chars = self
            .sentence
            .get_chars_by(start..self.sentence.len())
            .chain({
                if let Some(preset) = self.preset.as_ref()
                    && self.sentence.len() < preset.len()
                {
                    preset[self.sentence.len()..].iter()
                } else {
                    Default::default()
                }
            })
            .map(|c| *c);

        let (pre_render, mut abs_cursor) = calc_pre_render(
            chars,
            &self.styles,
            area.width as usize,
            left_lines as usize,
            cursor_at,
            start,
        );
        abs_cursor.y += line_idx as u16;
        // eprintln!("calc_pre_render abs_corsor: {abs_cursor:?}");
        self.abs_cursor = abs_cursor;
        self.pre_render.splice(line_idx.., pre_render);
    }

    pub fn get_pre_render_lines(&self, height: u16) -> (&[Vec<RenderCell>], Position) {
        let mut cursor = self.abs_cursor;
        // 根据cursor.y计算切片窗口，cursor.y 从向下1行开始向上倒止t_area.height，并最终修正cursor.y到t_area内
        let mut end = cursor.y + 2;
        let start = end.saturating_sub(height);
        end = (start + height).min(self.pre_render.len() as u16);
        cursor.y = cursor.y - start as u16;
        let slice = &self.pre_render[start as usize..end as usize];
        (slice, cursor)
    }
    // let dbg_render = pre_render
    //     .iter()
    //     .map(|l| {
    //         l.iter()
    //             .filter_map(|c| c.char.as_ref().map(|c| *c))
    //             // .map(|c| c.sentence_i.to_string())
    //             .collect::<String>()
    //     })
    //     .collect::<Vec<_>>()
    //     .join("\n");

    // eprintln!("calc_pre_render pre_render:\n{}", dbg_render);

    // eprintln!(
    //     "calc_pre_render _early_fin: [{}] pre_render_len: [{}]",
    //     _early_fin,
    //     pre_render.len()
    // );

    // pub fn get_preset(&self) -> Option<&Vec<char>> {
    //     self.preset.as_ref()
    // }
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

#[derive(Debug)]
pub struct WrappedText<'a> {
    pub pre_render: &'a [Vec<RenderCell>],
}

impl<'a> WrappedText<'a> {
    pub fn new(pre_render: &'a [Vec<RenderCell>]) -> Self {
        Self { pre_render }
    }
}
impl<'a> Widget for WrappedText<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.pre_render
            .into_iter()
            .enumerate()
            .for_each(|(i, line)| {
                let y = area.y + i as u16;
                line.iter().for_each(|cell| cell.render(area.x, y, buf));
            });
    }
}
pub fn calc_pre_render<C>(
    content: impl IntoIterator<Item = C>,
    styles: &[(usize, Option<Style>)],
    width: usize,
    lines: usize,
    cursor_at: usize,
    idx_offset: usize,
) -> (PreRender, Position)
where
    C: Borrow<char>,
{
    let (mut x, mut y) = (1, 0);
    let init_line = |sentence_i| {
        let mut v = vec![RenderCell::default(); width];
        (0..v.len()).for_each(|i| v[i].x = i as u16);
        v[1].sentence_i = sentence_i;
        // v[1].char = Some(sentence_i.to_string().chars().next().unwrap());
        v
    };
    let mut cursor: Option<Position> = None;
    let mut ret: PreRender = vec![];
    let mut first = true;
    for (i, c) in content.into_iter().enumerate() {
        let c = *c.borrow();
        if first {
            ret.push(init_line(i + idx_offset));
            first = false;
        }
        ret[y][x].sentence_i = i + idx_offset;
        let char_wid = c.width().unwrap_or(0);
        // if wid == 0 {
        //     eprintln!("zero width: {c} at {i}");
        //     continue;
        // }
        if x + char_wid >= width || c == '\n' {
            y += 1;
            if cursor.is_some() && y > lines {
                return (ret, cursor.expect("cursor should exist"));
            }
            x = 1;
            ret.push(init_line(i + idx_offset));
        }
        ret[y][x].sentence_i = i + idx_offset;
        if c != '\n' {
            ret[y][x].char = Some(c);
            ret[y][x].sentence_i = i + idx_offset;
            if let Some((palette_idx, patch)) = styles.get(i + idx_offset) {
                ret[y][x].style = COLOR_PALETTE[*palette_idx];
                if let Some(patch) = patch {
                    ret[y][x].style = COLOR_PALETTE[*palette_idx].patch(*patch);
                }
            }
            // } else {
            //     ret[y][x].char = Some((i + idx_offset).to_string().chars().next().unwrap());
        }
        if i == cursor_at && cursor.is_none() {
            cursor = Some(Position::new(x as u16, y as u16))
        }
        x += char_wid;
    }
    // eprintln!("cursor: {cursor:?}, x: {x}, char wid: {char_wid}");
    (ret, cursor.unwrap_or(Position::new(x as u16, y as u16)))
}

pub type PreRender = Vec<Vec<RenderCell>>;
// pub type PreRenderSlice<'a> = &'a [Vec<RenderCell>];

#[derive(Debug, Copy, Clone)]
pub struct RenderCell {
    x: u16,
    // 此字符的sentence中的位置
    sentence_i: usize,
    char: Option<char>,
    style: Style,
}

impl RenderCell {
    fn render(&self, x_offset: u16, y: u16, buf: &mut Buffer) {
        let x = self.x + x_offset;
        if let Some(c) = self.char {
            buf[(x, y)].set_char(c).set_style(self.style);
        } else {
            buf[(x, y)].reset();
        }
    }
}

impl Default for RenderCell {
    fn default() -> Self {
        Self {
            x: 0,
            sentence_i: 0,
            char: None,
            style: COLOR_PALETTE[COLOR_NORMAL],
        }
    }
}

fn advance_to_word_boundary(slice: &[char], at_least: usize) -> usize {
    let re = slice.iter().enumerate().find_map(|(i, c)| {
        if i > at_least && !c.is_alphabetic() {
            Some(i)
        } else {
            None
        }
    });
    re.unwrap_or(slice.len())
}

/// 比较两个char序列间的不同.
/// 简单比较，也就是直接比较相同索引下的字符
/// true为字符相同，false为不同
fn diff_sequence<'a>(
    chars: impl IntoIterator<Item = &'a char>,
    other: Option<impl IntoIterator<Item = &'a char>>,
) -> Vec<bool> {
    if other.is_none() {
        return Vec::default();
    }
    chars
        .into_iter()
        .zip(other.unwrap())
        .map(|(a, b)| a == b)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::context::Sentence;

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
        assert_eq!(chars, vec![]);

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
        assert_eq!(chars, vec![]);
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
        assert_eq!(chars, vec![]);
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
        assert_eq!(chars, vec![]);
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
