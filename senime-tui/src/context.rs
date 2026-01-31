use std::{borrow::Borrow, ops::Range, time::Instant};

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
    pub range: Range<usize>,
    pub origin: Vec<char>,
    pub start: Instant,
    pub end: Instant,
}

pub struct Context<'a> {
    preset: Option<Vec<char>>,
    sentence: Vec<char>,
    styles: Vec<(usize, Option<Style>)>,
    record: Vec<Record>,
    pending: Vec<char>,
    pending_max_len: usize,
    pending_backup_styles: Vec<(usize, Option<Style>)>,
    pending_input: Vec<char>,
    input_start: Instant,
    enc: &'a Looker,
    pre_render: PreRender,
}
impl<'a> Context<'a> {
    pub fn new(enc: &'a Looker) -> Self {
        Self {
            preset: Default::default(),
            sentence: Default::default(),
            styles: Default::default(),
            record: Default::default(),
            pending: Default::default(),
            pending_max_len: 0,
            pending_backup_styles: Default::default(),
            pending_input: Default::default(),
            input_start: Instant::now(),
            pre_render: Default::default(),
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
        self.record.clear();
        self.segment(0..self.preset.as_ref().map(|p| p.len()).unwrap_or(0));
    }

    fn diff(&mut self, range: Range<usize>, style_on_same: Option<(usize, Option<Style>)>) {
        if let Some(preset) = self.preset.as_ref()
            && range.start < preset.len()
        {
            let sen_len = self.sentence.len();
            let pen_len = self.pending.len();
            let sen_range = range.start.min(sen_len)..range.end.min(sen_len);
            let pen_range =
                range.start.saturating_sub(sen_len)..range.end.saturating_sub(sen_len).min(pen_len);

            let chars = (&self.sentence[sen_range])
                .iter()
                .chain((&self.pending[pen_range]).iter());
            let other = &preset[range.start..range.end.min(preset.len())];

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

    /// 添加新的输入结果（汉字）
    /// 同时维护好输入记录中的索引
    /// 如果存在预设文章（赛文用）
    ///     进行当前输入差异比对.
    ///     进行当前索引之后到下一标点符号为止的分词计算
    pub fn push(&mut self, text: impl IntoIterator<Item = char>, origin: Vec<char>) {
        // eprintln!("push");
        let text: Vec<char> = text.into_iter().collect();
        let old_len = self.sentence.len();
        let txt_len = text.len();
        let record = Record {
            range: old_len..old_len + txt_len,
            origin,
            start: self.input_start,
            end: Instant::now(),
        };
        self.sentence.extend(text);
        self.record.push(record);

        self.diff(old_len..old_len + txt_len, None);
        if let Some(preset) = self.preset.as_ref()
            && self.sentence.len() < preset.len()
        {
            let end = advance_to_word_boundary(
                &preset[self.sentence.len()..],
                self.pending_max_len.saturating_sub(txt_len),
            );
            self.segment(self.sentence.len()..self.sentence.len() + end);
        }
    }

    pub fn set_pending(&mut self, pending: impl IntoIterator<Item = char>, input: Vec<char>) {
        // eprintln!("set_pending");
        let pending: Vec<char> = pending.into_iter().collect();
        let range = self.sentence.len()..self.sentence.len() + pending.len();
        self.pending = pending;
        self.pending_input = input;

        if self.pending.len() > self.pending_max_len {
            // set_pending时不进行分词，
            // 但是在diff阶段，上一次过长的候选词会影响后面的字符样式，
            // 需要恢复其原本的样式
            if range.end <= self.styles.len() {
                let pick_range = self.sentence.len() + self.pending_max_len
                    ..self.sentence.len() + self.pending.len();
                self.pending_backup_styles
                    .extend((&self.styles[pick_range]).to_vec());
                // eprintln!("backup style: {:?}", self.pending_backup_styles);
            }
            self.pending_max_len = self.pending.len();
        }
        // 恢复上一次过长的pending改变的样式为分词的样式
        if self.pending.len() < self.pending_max_len
            && self.sentence.len() + self.pending_max_len <= self.styles.len()
        {
            // eprintln!("restore style: {:?}", self.pending_backup_styles);
            self.pending_backup_styles
                .iter()
                .enumerate()
                .for_each(|(i, style)| {
                    let i = self.sentence.len() + i;
                    self.styles[i] = *style;
                });
        }

        self.diff(range, Some((COLOR_PENDING, None)));
    }

    pub fn clear_pending(&mut self) {
        // eprintln!("clear_pending");
        self.pending_input.clear();
        self.pending.clear();
        self.pending_max_len = 0;
        self.pending_backup_styles = vec![];
    }

    pub fn get_input(&self) -> &[char] {
        &self.pending_input
    }

    pub fn get_recorders(&self) -> &[Record] {
        &self.record
    }

    pub fn get_sentence(&self) -> &[char] {
        &self.sentence
    }

    pub fn preset_len(&self) -> Option<usize> {
        match self.preset.as_ref() {
            Some(preset) => Some(preset.len()),
            None => None,
        }
    }

    // TODO: 缓存结果
    pub fn calc_pre_render(&mut self, area: Rect) -> (PreRender, Position) {
        let preset_start = self.sentence.len() + self.pending.len();
        let chars = self
            .sentence
            .iter()
            .chain(self.pending.iter())
            .chain({
                if let Some(preset) = self.preset.as_ref()
                    && preset_start < preset.len()
                {
                    preset[preset_start..].iter()
                } else {
                    Default::default()
                }
            })
            .map(|c| *c);
        let (pre_render, mut cursor) = calc_pre_render(chars, &self.styles, area, preset_start);
        self.pre_render = pre_render;
        // 根据cursor.y计算切片窗口，cursor.y 从向下2行开始向上倒止t_area.height，并最终修正cursor.y到t_area内
        let mut l_end = cursor.y as usize + 1;
        let l_start = l_end.saturating_sub(area.height as usize);
        l_end = (l_start + area.height as usize).min(self.pre_render.len());
        cursor.y = cursor.y - l_start as u16;
        let slice = self.pre_render[l_start..l_end].to_vec();
        (slice, cursor)
    }

    pub fn confrim_pending(&mut self) {
        if !self.pending.is_empty() {
            let pending = self.pending.clone();
            let input = self.pending_input.clone();
            self.push(pending, input);
            self.clear_pending();
        }
    }

    pub fn push_input(&mut self, c: char) {
        if self.pending_input.is_empty() {
            self.input_start = Instant::now();
        }
        self.pending_input.push(c);
    }

    pub fn backspace(&mut self) {
        let mut splice_len = 0;
        if self.pending_input.is_empty() {
            self.clear_pending();
            // eprintln!("backspace: remote record");
            // 有一种情况，range(text)为empty，chars非empry，通常为空格顶字
            // 当text为empty，继续修改下一个，直到非空并从中删除一个char
            // 注意：所谓的删除，是修改record中的range，使其-1,
            // record仍在self.record里面，而range对应的self.sentence确实删除了一个char
            // 这是为了更多的统计信息，比如回改次数
            for i in (0..self.record.len()).rev() {
                let last = &mut self.record[i];
                last.end = Instant::now();
                if last.range.is_empty() {
                    continue;
                }
                last.range.end -= 1;
                self.sentence.pop();
                if last.range.is_empty() && i > 0 {
                    self.record[i - 1].end = Instant::now();
                }
                break;
            }
        } else {
            // eprintln!("backspace: clear_pending");
            // FIXME
            splice_len = self.pending_max_len;
            self.pending_input.pop();
            if self.pending_input.is_empty() {
                self.clear_pending();
            }
        }
        // 如果删除的是中间，需要重新diff，但目前不支持从中间删除，因此 TODO:diff
        // 重新分词
        let sen_len = self.sentence.len();
        if let Some(preset) = self.preset.as_ref()
            && sen_len < preset.len()
        {
            let end = advance_to_word_boundary(&preset[sen_len..], splice_len);
            self.segment(sen_len..sen_len + end);
        }
    }

    // pub fn get_preset(&self) -> Option<&Vec<char>> {
    //     self.preset.as_ref()
    // }
}

#[derive(Debug)]
pub struct WrappedText {
    pub pre_render: PreRender,
}

impl WrappedText {
    pub fn new(pre_render: PreRender) -> Self {
        Self { pre_render }
    }
}
impl Widget for WrappedText {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.pre_render
            .into_iter()
            .enumerate()
            .for_each(|(i, line)| {
                let y = area.y + i as u16;
                line.iter().for_each(|cell| cell.render(y, buf));
            });
    }
}
pub fn calc_pre_render<C>(
    content: impl IntoIterator<Item = C>,
    styles: &[(usize, Option<Style>)],
    rect: Rect,
    cursor_at: usize,
) -> (PreRender, Position)
where
    C: Borrow<char>,
{
    let (mut x, mut y, width) = (1, 0, rect.width as usize);
    let init_line = || {
        let mut v = vec![RenderCell::default(); rect.width as usize];
        (0..v.len()).for_each(|i| v[i].x = rect.x + i as u16);
        v
    };
    let mut cursor: Option<Position> = None;
    let mut ret: PreRender = vec![];
    let mut first = true;
    let mut char_wid = 0;
    for (i, c) in content.into_iter().enumerate() {
        let c = *c.borrow();
        if first {
            ret.push(init_line());
            first = false;
        }
        char_wid = c.width().unwrap_or(0);
        // if wid == 0 {
        //     eprintln!("zero width: {c} at {i}");
        //     continue;
        // }
        if x + char_wid >= width || c == '\n' {
            y += 1;
            x = 1;
            ret.push(init_line());
        }

        if c != '\n' {
            ret[y][x].char = Some(c);
            ret[y][x].sentence_i = i;
            if let Some((palette_idx, patch)) = styles.get(i) {
                ret[y][x].style = COLOR_PALETTE[*palette_idx];
                if let Some(patch) = patch {
                    ret[y][x].style = COLOR_PALETTE[*palette_idx].patch(*patch);
                }
            }
        }
        if i == cursor_at && cursor.is_none() {
            cursor = Some(Position::new((x + char_wid - 1) as u16, rect.y + y as u16))
        }
        x += char_wid;
    }
    eprintln!("cursor: {cursor:?}, x: {x}, char wid: {char_wid}");
    (
        ret,
        cursor.unwrap_or(Position::new(
            (x + char_wid.max(1) - 1) as u16,
            rect.y + y as u16,
        )),
    )
}

pub type PreRender = Vec<Vec<RenderCell>>;

#[derive(Debug, Copy, Clone)]
pub struct RenderCell {
    x: u16,
    sentence_i: usize,
    char: Option<char>,
    style: Style,
}

impl RenderCell {
    fn render(&self, y: u16, buf: &mut Buffer) {
        let x = self.x;
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

// #[cfg(test)]
// mod test {
//     use unicode_width::UnicodeWidthStr;

//     #[test]
//     fn test_create_diff_text() {
//         let left = "hello, world".chars().collect::<Vec<_>>();
//         let right = "hella,_world, gray".chars().collect::<Vec<_>>();
//         let diff_indices = diff_sequence(left.iter(), Some(right.iter()));
//         println!("text: {diff_indices:?}");
//     }

//     #[test]
//     fn test_punc_length() {
//         let punc = "……";
//         let width_cjk = punc.width_cjk();
//         let width2 = punc.width();
//         println!("{punc} > width cjk: {width_cjk}, width 2: {width2}]");
//         let punc = "——";
//         let width_cjk = punc.width_cjk();
//         let width2 = punc.width();
//         println!("{punc} > width cjk: {width_cjk}, width 2: {width2}]");
//         let punc = "你好";
//         let width_cjk = punc.width_cjk();
//         let width2 = punc.width();
//         println!("{punc} > width cjk: {width_cjk}, width 2: {width2}]");
//         let punc = "你好abc";
//         let width_cjk = punc.width_cjk();
//         let width2 = punc.width();
//         println!("{punc} > width cjk: {width_cjk}, width 2: {width2}]");
//     }
// }
