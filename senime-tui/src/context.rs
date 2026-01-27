use std::{borrow::Borrow, collections::HashMap, ops::Range, time::Instant};

use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::Style,
    widgets::Widget,
};
use senime_lib::Looker;
use unicode_width::UnicodeWidthChar;

use crate::diff_sequence;

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
    style: HashMap<usize, Style>,
    record: Vec<Record>,
    pending: Vec<char>,
    max_pending_len: usize,
    input: Vec<char>,
    input_start: Instant,
    enc: &'a Looker,
}
impl<'a> Context<'a> {
    pub fn new(enc: &'a Looker) -> Self {
        Self {
            preset: Default::default(),
            sentence: Default::default(),
            style: Default::default(),
            record: Default::default(),
            pending: Default::default(),
            max_pending_len: 0,
            input: Default::default(),
            input_start: Instant::now(),
            enc,
        }
    }
    pub fn set_preset(&mut self, preset: Option<Vec<char>>) {
        if let Some(preset) = preset.as_ref() {
            let color_palette = [
                Style::default().on_light_cyan().dark_gray(),
                Style::default().on_light_blue().dark_gray(),
            ];
            let ret = self.enc.analyze(preset);
            for (g, seg) in ret.into_iter().enumerate() {
                let mut style = color_palette[g % 2];
                for i in seg.range.clone() {
                    if seg.pos > 0 {
                        style = style.patch(Style::default().crossed_out());
                    } else if !seg.auto_select && seg.pos == 0 {
                        style = style.patch(Style::default().underlined());
                    }
                    self.patch_style(i, Some(style));
                }
            }
        }
        self.preset = preset;
    }

    fn patch_style(&mut self, i: usize, style: Option<Style>) {
        match (self.style.get_mut(&i), style) {
            (Some(old_style), Some(style)) => {
                // 使用patch太过复杂，会有残留问题
                *old_style = style;
            }
            (None, Some(style)) => {
                self.style.insert(i, style);
            }
            _ => {
                self.style.remove(&i);
            }
        };
    }

    fn diff(&mut self, range: Range<usize>) {
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
            let style_diff = Style::default().on_light_red().crossed_out();

            let diff_range = diff_sequence(chars, Some(other.iter()))
                .into_iter()
                .enumerate()
                .map(|(i, d)| (range.start + i, d.then(|| style_diff)))
                .collect::<Vec<_>>();

            diff_range
                .into_iter()
                .for_each(|(i, s)| self.patch_style(i, s));
        }
    }

    fn segment(&mut self, range: Range<usize>) {
        if let Some(preset) = self.preset.as_ref() {
            let color_palette = [
                Style::default().on_light_cyan().dark_gray(),
                Style::default().on_light_blue().dark_gray(),
            ];
            // 找到重新分词计算的范围
            let seg_range = self
                .enc
                .analyze(&preset[range.clone()])
                .iter()
                .enumerate()
                .map(|(g, seg)| {
                    seg.range
                        .clone()
                        .map(|i| {
                            let fix_i = i + range.start;
                            let mut style = color_palette[g % 2];
                            if seg.pos > 0 {
                                style = style.patch(Style::default().crossed_out())
                            } else if !seg.auto_select && seg.pos == 0 {
                                style = style.patch(Style::default().underlined())
                            }
                            (fix_i, Some(style))
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>();
            seg_range.into_iter().for_each(|(i, s)| {
                self.patch_style(i, s);
            });
        }
    }

    /// 添加新的输入结果（汉字）
    /// 同时维护好输入记录中的索引
    /// 如果存在预设文章（赛文用）
    ///     进行当前输入差异比对.
    ///     进行当前索引之后到下一标点符号为止的分词计算
    pub fn push(&mut self, text: impl IntoIterator<Item = char>, origin: Vec<char>) {
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

        self.diff(old_len..old_len + txt_len.max(self.max_pending_len));
        if let Some(preset) = self.preset.as_ref()
            && self.sentence.len() < preset.len()
        {
            let end = advance_to_word_boundary(&preset[self.sentence.len()..], 0);
            self.segment(self.sentence.len()..self.sentence.len() + end);
        }
    }

    pub fn set_pending(&mut self, pending: impl IntoIterator<Item = char>, input: Vec<char>) {
        let pending: Vec<char> = pending.into_iter().collect();
        self.max_pending_len = self.max_pending_len.max(pending.len());
        let range = self.sentence.len()..self.sentence.len() + self.max_pending_len;
        self.pending = pending;
        self.input = input;
        self.diff(range);
    }

    pub fn clear_pending(&mut self) {
        self.input.clear();
        self.pending.clear();
        self.max_pending_len = 0;
    }

    pub fn get_input(&self) -> &[char] {
        &self.input
    }

    pub fn get_recorders(&self) -> &[Record] {
        &self.record
    }

    pub fn preset_len(&self) -> Option<usize> {
        match self.preset.as_ref() {
            Some(preset) => Some(preset.len()),
            None => None,
        }
    }

    pub fn calc_pre_render(&self, area: Rect) -> (PreRender, Position) {
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
        let (pre_render, mut cursor) = calc_pre_render(
            chars,
            &self.style,
            area,
            self.sentence.len() + self.pending.len(),
        );
        // 根据cursor.y计算切片窗口，cursor.y 从向下2行开始向上倒止t_area.height，并最终修正cursor.y到t_area内
        let mut l_end = cursor.y as usize + 1;
        let l_start = l_end.saturating_sub(area.height as usize);
        l_end = (l_start + area.height as usize).min(pre_render.len());
        cursor.y = cursor.y - l_start as u16;
        (pre_render[l_start..l_end].to_vec(), cursor)
    }

    pub fn confrim_pending(&mut self) {
        if self.pending.is_empty() {
            let pending = self.pending.clone();
            let input = self.input.clone();
            self.push(pending, input);
            self.clear_pending();
        }
    }

    pub fn push_input(&mut self, c: char) {
        if self.input.is_empty() {
            self.input_start = Instant::now();
        }
        self.input.push(c);
    }

    pub fn backspace(&mut self) {
        let mut splice_len = 0;
        if self.pending.is_empty() {
            if let Some(pop) = self.record.pop() {
                splice_len = pop.range.len();
                self.sentence.splice(pop.range, vec![]);
                if let Some(last) = self.record.last_mut() {
                    last.end = Instant::now();
                }
            }
        } else {
            // FIXME
            splice_len = self.max_pending_len;
            self.clear_pending();
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
}

#[derive(Debug)]
pub struct WrappedText<'a> {
    pub pre_render: PreRenderSlice<'a>,
}

impl<'a> WrappedText<'a> {
    pub fn new(pre_render: PreRenderSlice<'a>) -> Self {
        Self { pre_render }
    }
}
impl Widget for WrappedText<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.pre_render
            .into_iter()
            .enumerate()
            .for_each(|(i, line)| {
                let y = area.y + i as u16;
                line.iter().for_each(|(x, c, s)| {
                    let x = *x;
                    if let Some(c) = c {
                        buf[(x, y)].set_char(*c);
                        s.and_then(|s| Some(buf[(x, y)].set_style(s)));
                    } else {
                        buf[(x, y)].reset();
                    }
                });
            });
    }
}
pub fn calc_pre_render<C>(
    content: impl IntoIterator<Item = C>,
    modifies: &HashMap<usize, Style>,
    rect: Rect,
    cursor_at: usize,
) -> (PreRender, Position)
where
    C: Borrow<char>,
{
    let (mut x, mut y, width) = (1, 0, rect.width as usize);
    let init_line = || {
        let mut v = vec![(0, None, None); rect.width as usize];
        (0..v.len()).for_each(|i| v[i].0 = rect.x + i as u16);
        v
    };
    let mut cursor: Option<Position> = None;
    let mut ret: PreRender = vec![];
    let mut first = true;
    for (i, c) in content.into_iter().enumerate() {
        let c = *c.borrow();
        if first {
            ret.push(init_line());
            first = false;
        }
        let wid = c.width().unwrap_or(0);
        // if wid == 0 {
        //     continue;
        // }
        if x + wid >= width || c == '\n' {
            y += 1;
            x = 1;
            ret.push(init_line());
        }
        if let Some(style) = modifies.get(&i) {
            ret[y][x].2 = Some(*style);
        }
        if c != '\n' {
            ret[y][x].1 = Some(c);
        }
        if i + 1 >= cursor_at && cursor.is_none() {
            // x + 1 的表现更好
            cursor = Some(Position::new(rect.x + (x + 1) as u16, rect.y + y as u16))
        }
        x += wid;
    }
    (
        ret,
        cursor.unwrap_or(Position::new(rect.x + x as u16, rect.y + y as u16)),
    )
}
pub type PreRender = Vec<Vec<(u16, Option<char>, Option<Style>)>>;
pub type PreRenderSlice<'a> = &'a [Vec<(u16, Option<char>, Option<Style>)>];

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
