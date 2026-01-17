use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::Instant;

use clap::Parser;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{event, execute};
use derive_setters::Setters;
use ratatui::Terminal;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::CrosstermBackend;
use ratatui::style::Style;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap};

use senime::{AnalysisResult, Dict, InputAnalyzer};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 码表文件，其结构应为: 字词<TAB>编码<TAB>权重(可选) 每行
    /// 当没有权重时则以行的顺序判断编码对应的字词的首选还是候选
    #[arg(short, long)]
    pub table: String,
    // /// 文本文件，纯文本
    // #[arg(short, long)]
    // pub input: String,
}

#[derive(Debug, Default, Setters)]
struct Popup<'a> {
    #[setters(into)]
    title: Line<'a>,
    #[setters(into)]
    content: Text<'a>,
    border_style: Style,
    title_style: Style,
    style: Style,
}

impl Widget for Popup<'_> {
    fn render(self, mut area: Rect, buf: &mut Buffer) {
        // ensure that all cells under the popup are cleared to avoid leaking content
        area.x = area.x - 1;
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);
        Paragraph::new(self.content)
            // .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let dict = Dict::load(args.table);
    let an = InputAnalyzer::new(dict);
    enable_raw_mode()?;
    let mut stdout = OpenOptions::new()
        .read(false)
        .write(true)
        .open("/dev/tty")?;
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut sentence_rec: Vec<(String, Vec<char>)> = vec![];
    let mut input: Vec<char> = vec![];
    loop {
        let mut pending: String = String::new();
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Length(3)])
                .split(frame.area());

            let time_start = Instant::now();
            let AnalysisResult {
                candidates,
                mut segments,
            } = an.analyze(&input);
            let poped = segments.pop();
            if segments.len() > 0 {
                sentence_rec.extend(segments);
            }
            if let Some((text, chars)) = poped {
                input = chars;
                pending = text;
            }
            let elapsed = time_start.elapsed();

            // sentence
            let area = chunks[0];
            let sentence_iter = sentence_rec
                .iter()
                .map(|(s, _)| s)
                .chain(std::iter::once(&pending))
                .filter(|s| !s.is_empty());
            let (text, last_width, text_height) = sentence_iter.fold::<(String, u16, u16), _>(
                (String::new(), 0, 1),
                |(mut lines, mut width, mut height), word| {
                    let word_width = word.width_cjk() as u16;
                    if width + word_width > area.width - 2 {
                        lines.push_str("\n");
                        width = 0;
                        height += 1;
                    }
                    lines.push_str(word);
                    (lines, width + word_width, height)
                },
            );
            let inner_height = area.height - 2;
            let sentence_widget = Paragraph::new(text)
                .scroll((text_height.max(inner_height) - inner_height, 0))
                .wrap(Wrap { trim: false })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("成句 [{:?}]", elapsed)),
                );
            frame.render_widget(sentence_widget, chunks[0]);

            // candidates
            if let Some(candidates) = candidates
                && candidates.len() > 1
            {
                let cand_count = candidates.len() as u16;
                let mut cand_max_width = 0;
                let mut cand_text: Vec<Line> = vec![];
                for cand in candidates.into_iter() {
                    let mut cand_line = Line::from("[");
                    cand_line.push_span(
                        Span::from(cand.select_key.to_string()).style(Style::default().green()),
                    );
                    cand_line.push_span("]: ");
                    cand_line.push_span(cand.text);
                    cand_max_width = cand_line.width_cjk().max(cand_max_width);
                    cand_text.push(cand_line);
                }
                let popup_width = (cand_max_width as u16 + 2).min((area.width - 4) / 2);
                let mut popup_height = (area.height - (text_height + 2)).min(cand_count + 2);
                let mut popup_y = (area.y + text_height + 1).max(2);
                let popup_x = (area.x + last_width)
                    .max(1)
                    .min(area.width - popup_width - 1);
                if text_height + 1 > area.height / 2 {
                    popup_height = (text_height - 1).min(area.height - 3).min(cand_count + 2);
                    popup_y = area.y + (text_height.min(area.height - 2)) - popup_height;
                    if popup_height - 2 < cand_count {
                        let _ = cand_text.split_off((popup_height - 2) as usize);
                    }
                    cand_text.reverse();
                }
                let popup_area = Rect {
                    x: popup_x,
                    y: popup_y,
                    width: popup_width,
                    height: popup_height,
                };

                let popup = Popup::default()
                    .content(cand_text)
                    .style(Style::new().yellow())
                    .border_style(Style::new().red());
                frame.render_widget(popup, popup_area);
            }

            // input
            let (input_width, mut new_input) = input.iter().rfold::<(u16, Vec<char>), _>(
                (0, vec![]),
                |(mut width, mut chars), word| {
                    if width + 5 < area.width {
                        let word_width = word.width_cjk().unwrap_or(0) as u16;
                        if width + 5 + word_width < area.width {
                            chars.push(*word);
                            width += word_width;
                        }
                    }
                    (width, chars)
                },
            );
            new_input.reverse();

            let input_widget =
                Paragraph::new(format!("> {}", new_input.iter().collect::<String>()))
                    .block(Block::default().borders(Borders::ALL).title("输入"));

            frame.render_widget(input_widget, chunks[1]);
            frame.set_cursor_position((input_width + 3, chunks[1].y + 1));
        })?;
        // 事件处理
        // if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                    break;
                }
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Backspace => {
                    if pending.is_empty() {
                        sentence_rec.pop();
                        input.clear();
                    } else {
                        input.pop();
                    }
                }
                KeyCode::Char(c) => input.push(c),
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    {
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
    }
    let bs = sentence_rec
        .iter()
        .map(|(s, _c)| s.as_bytes().to_vec())
        .flatten()
        .collect::<Vec<u8>>();
    io::stdout().write(&bs)?;
    io::stdout().write_all(b"\n")?;
    Ok(())
}
