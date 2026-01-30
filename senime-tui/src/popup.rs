use ratatui::{
    buffer::Buffer,
    layout::{Margin, Position, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Clear, Widget},
};
use senime_lib::input_analyzer::CandidateRich;

#[derive(Debug, Default)]
pub struct Popup<'a> {
    content: Text<'a>,
}

impl<'a> Popup<'a> {
    pub fn create(
        candidates: &'a [CandidateRich],
        root_area: Rect,
        cursor: Position,
        input_byte_len: usize,
    ) -> (Self, Rect) {
        let cand_count = candidates.len();
        let mut cand_max_width = 0;
        let mut cand_text: Vec<Line> = vec![];
        for cand in candidates.iter() {
            let mut cand_line = Line::from("[");
            cand_line.push_span(Span::from(cand.select_key.to_string()).green());
            cand_line.push_span("]: ");
            cand_line.push_span(&cand.text);
            if cand.code.len() > input_byte_len {
                cand_line.push_span(Span::from(&cand.code[input_byte_len..]).red());
            }
            cand_max_width = cand_line.width().max(cand_max_width);
            cand_text.push(cand_line);
        }
        let margin_x = 2;
        let margin_y = 0;
        let mut p_area = Rect {
            x: root_area.x + (cursor.x.max(3) - 2),
            y: cursor.y + 1,
            width: ((cand_max_width + margin_x) as u16).min(root_area.width), // +4 使边界宽度为2，防止双宽度的字符遮盖到边框
            height: (cand_count + margin_y) as u16,
        };
        if p_area.right() > root_area.right() {
            p_area.x -= p_area.right() - root_area.right();
        }
        if p_area.bottom() > root_area.bottom() {
            p_area.height -= p_area.bottom() - root_area.bottom();
        }
        // 如果指针下方小于6的空间，则将popup上移至cursor.y + 1并反转
        if root_area.bottom() - cursor.y < 6 {
            p_area.height = (cursor.y - 1 - root_area.y).min((cand_count + margin_y) as u16);
            p_area.y = cursor.y - p_area.height;
            if p_area.height > margin_y as u16 {
                let _ = cand_text.split_off(p_area.height as usize - margin_y);
                cand_text.reverse();
            }
        }
        let content = Text::from(cand_text).style(Style::new().yellow());
        (Popup { content }, p_area)
    }
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let t_area = area.inner(Margin::new(1, 0));
        self.content.render(t_area, buf);
    }
}
