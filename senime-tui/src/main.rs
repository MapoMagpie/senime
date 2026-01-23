use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::{Duration, Instant};

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
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};

use senime_lib::{AnalysisResult, Dict, InputAnalyzer};
use unicode_width::UnicodeWidthStr;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 码表文件，其结构应为: 字词<TAB>编码<TAB>权重(可选) 每行
    /// 当没有权重时则以行的顺序判断编码对应的字词的首选还是候选
    #[arg(short, long)]
    pub table: String,
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
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}

struct SentenceRecord {
    text: String,
    origin: Vec<char>,
    width: u16,
    satrt: Instant,
    end: Instant,
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

    let mut sentence_rec: Vec<SentenceRecord> = vec![];
    let mut input: Vec<char> = vec![];
    let mut input_start = Instant::now();
    loop {
        let mut pending: String = String::new();
        let mut pending_width = 0;
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Length(4)])
                .split(frame.area());

            let AnalysisResult {
                candidates,
                mut segments,
            } = an.analyze(&input);
            let poped = segments.pop();
            if segments.len() > 0 {
                sentence_rec.extend(segments.into_iter().map(|seg| {
                    let width = seg.0.width_cjk() as u16;
                    SentenceRecord {
                        text: seg.0,
                        origin: seg.1,
                        width,
                        satrt: input_start,
                        end: Instant::now(),
                    }
                }));
            }
            if let Some((text, chars)) = poped {
                input = chars;
                pending = text;
                pending_width = pending.width_cjk() as u16;
            }

            // sentence
            let area = chunks[0];
            let sentence_iter = sentence_rec
                .iter()
                .map::<(Span, u16), _>(|se| (Span::from(se.text.clone()), se.width))
                .chain(std::iter::once((
                    Span::from(&pending).red().underlined(),
                    pending_width,
                )));
            let (text, last_width, text_height) = sentence_iter.fold::<(Text, u16, u16), _>(
                (Text::from(""), 0, 1),
                |(mut text, mut width, mut height), (word, mut word_width)| {
                    if width + word_width > area.width - 2 || word.content == "\n" {
                        text.push_line("");
                        width = 0;
                        height += 1;
                    }
                    if word.content == "\n" {
                        word_width = 0; // "\n" word_width = 1, fix it to 0;
                    } else {
                        text.push_span(word);
                    }
                    (text, width + word_width, height)
                },
            );
            let inner_height = area.height - 2;
            let sentence_widget = Paragraph::new(text)
                .scroll((text_height.max(inner_height) - inner_height, 0))
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "成句 [输入中:{}]",
                    input.iter().collect::<String>()
                )));
            frame.render_widget(sentence_widget, chunks[0]);
            frame.set_cursor_position((
                area.x + last_width.max(1),
                area.y + text_height.min(area.height - 2),
            ));

            // measurements
            let measurement = calc_measurements(&sentence_rec);
            let measurement_widget = Paragraph::new(measurement.to_string())
                .block(Block::default().borders(Borders::ALL).title("计量"));
            frame.render_widget(measurement_widget, chunks[1]);

            // candidates
            if let Some(candidates) = candidates
                && candidates.len() > 1
            {
                let cand_count = candidates.len() as u16;
                let mut cand_max_width = 0;
                let mut cand_text: Vec<Line> = vec![];
                for cand in candidates.into_iter() {
                    let mut cand_line = Line::from("[");
                    cand_line.push_span(Span::from(cand.select_key.to_string()).green());
                    cand_line.push_span("]: ");
                    cand_line.push_span(cand.text);
                    if cand.code.len() > input.len() {
                        cand_line
                            .push_span(Span::from(cand.code.clone().split_off(input.len())).red());
                    }
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
                KeyCode::Enter => {
                    if !pending.is_empty() {
                        sentence_rec.push(SentenceRecord {
                            text: pending,
                            origin: input.to_vec(),
                            width: pending_width,
                            satrt: input_start,
                            end: Instant::now(),
                        });
                        input.clear();
                    }
                    sentence_rec.push(SentenceRecord {
                        text: "\n".to_string(),
                        origin: vec!['\n'],
                        width: 0,
                        satrt: Instant::now(),
                        end: Instant::now(),
                    });
                }
                KeyCode::Backspace => {
                    if pending.is_empty() {
                        sentence_rec.pop();
                        input.clear();
                    } else {
                        input.pop();
                    }
                }
                KeyCode::Char(c) => {
                    if input.is_empty() {
                        input_start = Instant::now();
                    }
                    input.push(c)
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
    let bs = sentence_rec
        .iter()
        .map(|se| se.text.as_bytes().to_vec())
        .flatten()
        .collect::<Vec<u8>>();
    io::stdout().write(&bs)?;
    io::stdout().write_all(b"\n")?;
    Ok(())
}

/// 计量速度.
/// 需要的信息:
///   开始时间-结束时间
///   总字数
///   总输入
///   码长
///   顶字次数?
///   空格次数?
///   回退次数?
fn calc_measurements(records: &[SentenceRecord]) -> Measurement {
    let (start, end) = if let (Some(first), Some(last)) = (records.first(), records.last()) {
        (first.satrt, last.end)
    } else {
        (Instant::now(), Instant::now())
    };
    let (total_text, total_code, _space_times) =
        records
            .iter()
            .fold((0, 0, 0), |(total_text, total_code, space_times), rec| {
                (
                    total_text + rec.text.chars().count(),
                    total_code + rec.origin.len(),
                    space_times,
                )
            });

    let duration = end.duration_since(start);
    let wpm = total_text as f32 / (duration.as_secs_f32() / 60.0);
    let kps = total_code as f32 / duration.as_secs_f32();
    let avg_len = total_code as f32 / total_text as f32;

    Measurement {
        // start,
        // end,
        duration,
        total_text,
        total_code,
        wpm,
        kps,
        avg_len,
    }
}

struct Measurement {
    // start: Instant,
    // end: Instant,
    duration: Duration,
    total_text: usize,
    total_code: usize,
    kps: f32,
    wpm: f32,
    avg_len: f32,
}

impl ToString for Measurement {
    fn to_string(&self) -> String {
        format!(
            "  耗时: [{}]秒, 速度: [{:.2}]字/分, 击键: [{:.2}]键/秒\n  总字数: [{}], 总输入: [{}], 平均码长: [{:.2}]",
            self.duration.as_secs(),
            self.wpm,
            self.kps,
            self.total_text,
            self.total_code,
            self.avg_len,
        )
    }
}
