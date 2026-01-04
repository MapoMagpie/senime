use std::io;
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

use crate::input_alanyzer::InputAnalyzer;
use crate::trie::Trie;
mod input_alanyzer;
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
    // let start = Instant::now();
    let trie = Trie::load(args.table);
    // let load_table_time = Instant::now().duration_since(start);
    // println!(
    //     "读取码表成功，加载[{}]个条目, 耗时[{:?}]",
    //     trie.count, load_table_time
    // );
    let an = InputAnalyzer::new(trie);
    // let result = an.analyze("kislo fj ppd kylku lylxbbI".chars());
    // println!("{}", result.join(""));
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();

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
            let an_result = an.analyze(input.chars());
            let time_end = Instant::now();
            let candidate_widget = Paragraph::new(
                an_result
                    .candidates
                    .iter()
                    .enumerate()
                    .map(|(i, c)| {
                        format!("{}[{}]: {}  {}", (i + 1), selection_keys[i], c.text, c.code)
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
            .block(Block::default().borders(Borders::ALL).title("候选"));

            let sentence_widget = Paragraph::new(an_result.sentence.join("")).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("成句 ({:?})", time_end.duration_since(time_start))),
            );
            let input = format!("> {}", input);
            let input_len = input.chars().count() as u16;
            let input_widget =
                Paragraph::new(input).block(Block::default().borders(Borders::ALL).title("输入"));

            frame.render_widget(sentence_widget, chunks[0]);
            frame.render_widget(input_widget, chunks[1]);
            frame.render_widget(candidate_widget, chunks[2]);
            frame.set_cursor_position((chunks[1].x + input_len + 1, chunks[1].y + 1));
        })?;
        // 事件处理
        // if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => {
                    break;
                }
                KeyCode::Char(c) => input.push(c),
                KeyCode::Backspace => {
                    input.pop();
                }
                // KeyCode::Enter => {
                //     input.clear();
                // }
                _ => {}
            }
        }
        // }
    }

    // 恢复终端
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
