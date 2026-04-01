use std::{borrow::Borrow, ops::Range, time::Instant};

use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::Style,
    widgets::Widget,
};
use senime_lib::Looker;
use unicode_width::UnicodeWidthChar;

use crate::{
    measurement::Measurement,
    sentence::{Movement, Sentence, SentenceChars},
};

const COLOR_DIFF: usize = 2;
const COLOR_SEG_LEN: usize = 2;
const COLOR_NORMAL: usize = 3;
const COLOR_PENDING: usize = 4;
const COLOR_PALETTE: [Style; 5] = [
    Style::new().on_light_cyan().black().bold(),
    Style::new().on_light_blue().black().bold(),
    Style::new().on_light_red().crossed_out().white(),
    Style::new().on_green().black(),
    Style::new().magenta().underlined(),
];

pub struct Context {
    preset: Option<Vec<char>>,
    preset_first_code: Option<String>,
    sentence: Sentence,
    styles: Vec<(usize, Option<Style>)>,
    input_start: Instant,
    before_pending_style: Vec<(usize, Option<Style>)>,
    enc: Looker,
    pre_render: PreRender,
    abs_cursor: Position,
    measurement: Measurement,
}

impl Context {
    pub fn new(enc: Looker) -> Self {
        Self {
            preset: Default::default(),
            preset_first_code: Default::default(),
            sentence: Default::default(),
            styles: Default::default(),
            input_start: Instant::now(),
            before_pending_style: Default::default(),
            pre_render: Default::default(),
            abs_cursor: Default::default(),
            measurement: Measurement::new(),
            enc,
        }
    }
    pub fn set_preset(&mut self, preset: Option<Vec<char>>) {
        self.preset = preset;
        if let Some(preset) = self.preset.as_ref() {
            self.measurement.preset_wc = Some(preset.len());
            self.segment(0..preset.len());
        }
    }

    /// 获取剩余的预设文本的首个编码提示
    pub fn get_preset_segment_hint(&self) -> Option<&str> {
        self.preset_first_code.as_deref()
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
        self.before_pending_style.clear();
        self.sentence.clear();
        self.styles.clear();
        self.input_start = Instant::now();
        self.pre_render.clear();
        self.abs_cursor = Default::default();
        self.measurement.clear();
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
            if range.start == 0 && range.end == preset.len() {
                let code_len: usize = ret
                    .iter()
                    .map(|seg| {
                        if seg.pos > 0 || !seg.auto_select {
                            seg.code.len() + 1
                        } else {
                            seg.code.len()
                        }
                    })
                    .sum();
                self.measurement.preset_avg_len = Some(code_len as f32 / preset.len() as f32);
            }
            self.preset_first_code = ret.first().map(|seg| {
                let mut code = seg.code.to_vec();
                if seg.pos > 0 {
                    code.extend(seg.pos.to_string().chars());
                } else if !seg.auto_select {
                    code.push('_');
                }
                code.iter().collect()
            });
            let mut iter = ret.into_iter().map(|seg| seg.simple()).collect::<Vec<_>>(); //.collect::<Vec<_>>();
            // 从后向前，在patch_style中会根据i扩容，先用最大数避免多次扩容
            iter.reverse();

            // 在不断的输入中，为了保持分词色彩不乱变，需要让修正group_idx
            // 如果range.end 小于 preset.len，表示局部小范围分词，
            // 从preset[range.end]上找到COLOR_PALETTE所在的位置，如1
            // 那么preset[range.end]所在的group idx % COLOR_SEG_LEN后 该为0
            // 由于要递减，所以选一个较大的固定基数，并且是偶数
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
        self.measurement
            .push_record(txt_len as i32, origin.clone(), self.input_start);
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
                    .extend((self.styles[old_pen_end..new_pen_end]).to_vec());
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

    pub fn confrim_pending(&mut self) {
        self.before_pending_style.clear();
        if !self.sentence.get_pending().is_empty() {
            let pending = self.sentence.get_pending().to_vec();
            let pending_origin = self.sentence.get_pending_origin().to_vec();
            self.measurement
                .push_record(pending.len() as i32, pending_origin, self.input_start);
            self.sentence.extend(pending);
            self.sentence.clear_pending();
        }
    }

    pub fn push_input(&mut self, c: char) {
        if self.sentence.get_pending_origin().is_empty() {
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
            self.measurement.push_record(-1, vec![], Instant::now());
        }
        // 需要diff
        if self.sentence.get_append_at() != self.sentence.len() {
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
        self.sentence.move_append_at(action);
    }

    pub fn get_input(&self) -> &[char] {
        self.sentence.get_pending_origin()
    }

    pub fn get_sentence<'a>(&'a self) -> SentenceChars<'a> {
        self.sentence.get_chars()
    }

    pub fn resize(&mut self) {
        self.abs_cursor = Default::default();
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
        let left_lines = if line_idx + 1 >= area.height as usize {
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
            .copied();

        let (pre_render, mut abs_cursor) = calc_pre_render(
            chars,
            &self.styles,
            area.width as usize,
            left_lines,
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
        cursor.y -= start;
        let slice = &self.pre_render[start as usize..end as usize];
        (slice, cursor)
    }

    pub fn calc_measurement(&mut self) {
        self.measurement.calc(self.sentence.len());
    }

    pub fn measure(&self) -> &Measurement {
        &self.measurement
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
        self.pre_render.iter().enumerate().for_each(|(i, line)| {
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
            if let Some(cursor) = cursor
                && y > lines
            {
                return (ret, cursor);
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
