use std::io::{self, Write};
use std::time::Instant;

use clap::Parser;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{event, execute};
use ratatui::Terminal;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::{Block, Borders, Paragraph};

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::input_analyzer::{AnalysisResult, InputAnalyzer};
use crate::trie::Dict;
mod input_analyzer;
mod trie;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let trie = Dict::load(args.table);
    let an = InputAnalyzer::new(trie);
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input: Vec<char> = vec![];
    let mut sentence_ret = vec![];

    let selection_keys = vec!["U", "I", "O", "H", "J", "K", "B", "N", "M"];
    loop {
        terminal.draw(|frame| {
            let size = frame.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(5),
                    Constraint::Length(3),
                    Constraint::Length(11),
                ])
                .split(size);
            let time_start = Instant::now();
            let AnalysisResult {
                candidates,
                sentence,
            } = an.analyze(&input);
            let time_end = Instant::now();
            let candidate_widget = Paragraph::new(
                candidates
                    .into_iter()
                    .enumerate()
                    .map(|(i, c)| {
                        format!("{}[{}]: {}  {}", (i + 1), selection_keys[i], c.text, c.code)
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
            .block(Block::default().borders(Borders::ALL).title("候选"));

            let (lines, _, height) = sentence.iter().fold::<(String, u16, u16), _>(
                (String::new(), 0, 1),
                |(mut lines, mut width, mut height), word| {
                    let word_width = word.width_cjk() as u16;
                    if width + word_width > size.width - 2 {
                        lines.push_str("\n");
                        width = 0;
                        height += 1;
                    }
                    lines.push_str(word);
                    (lines, width + word_width, height)
                },
            );
            sentence_ret = sentence;

            let sentence_widget = Paragraph::new(lines).scroll((height.max(3) - 3, 0)).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("成句 [{:?}]", time_end.duration_since(time_start))),
            );

            let (input_width, mut new_input) = input.iter().rfold::<(u16, Vec<char>), _>(
                (0, vec![]),
                |(mut width, mut chars), word| {
                    if width + 5 < size.width {
                        let word_width = word.width_cjk().unwrap_or(0) as u16;
                        if width + 5 + word_width < size.width {
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
            frame.render_widget(sentence_widget, chunks[0]);
            frame.render_widget(input_widget, chunks[1]);
            frame.render_widget(candidate_widget, chunks[2]);
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
                KeyCode::Char(c) => input.push(c),
                KeyCode::Backspace => {
                    input.pop();
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    {
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
    }
    let bs = sentence_ret
        .into_iter()
        .map(String::into_bytes)
        .flatten()
        .collect::<Vec<u8>>();
    io::stdout().write(&bs)?;
    io::stdout().write_all(b"\n")?;
    Ok(())
}
