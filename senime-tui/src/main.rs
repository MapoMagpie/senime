use std::borrow::Borrow;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::time::{Duration, Instant};

use clap::{ArgAction, Parser};
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
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};

use senime_lib::{AnalysisResult, Dict, InputAnalyzer};
use unicode_width::UnicodeWidthChar;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 码表文件，其结构应为: 字词<TAB>编码<TAB>权重(可选) 每行
    /// 当没有权重时则以行的顺序判断编码对应的字词的首选还是候选
    #[arg(short, long)]
    pub table: String,
    /// 是否接受输入流，并将输入流的数据作为预设文本
    /// 此功能类似赛码器，将以灰色的文本展示这些预设文本
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub stdin: bool,
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
        area.x -= 1;
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
    text: Vec<char>,
    origin: Vec<char>,
    width: Vec<usize>,
    satrt: Instant,
    end: Instant,
}

fn read_stdin() -> Result<String, std::io::Error> {
    use std::io::Read;
    let mut stdin = std::io::stdin();
    let mut str = String::new();
    stdin.read_to_string(&mut str)?;
    Ok(str)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut preset_text: Option<Vec<char>> = if args.stdin {
        read_stdin().map(|str| Some(str.chars().collect())).unwrap()
    } else {
        None
    };
    let mut need_wrapped_preset_text = true;

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
        let mut pending: Vec<char> = vec![];
        let mut pending_width: Vec<usize> = vec![];
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Length(4)])
                .split(frame.area());

            if need_wrapped_preset_text && preset_text.is_some() {
                let (wrapped, _, _) = wrap(
                    preset_text
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|s| (s, s.width().unwrap_or(0))),
                    chunks[0].width as usize - 2,
                );
                preset_text = Some(wrapped);
                need_wrapped_preset_text = false;
            }

            let AnalysisResult {
                candidates,
                mut segments,
            } = an.analyze(&input);
            let poped = segments.pop();
            if segments.is_empty() {
                sentence_rec.extend(segments.into_iter().map(|seg| {
                    let (width, text): (Vec<usize>, Vec<char>) =
                        seg.0.chars().map(|c| (c.width().unwrap_or(0), c)).unzip();
                    SentenceRecord {
                        text,
                        origin: seg.1,
                        width,
                        satrt: input_start,
                        end: Instant::now(),
                    }
                }));
            }
            if let Some((text, chars)) = poped {
                input = chars;
                pending = text.chars().collect();
                pending_width = pending.iter().map(|c| c.width().unwrap_or(0)).collect();
            }

            // sentence
            let area = chunks[0];

            let aa = sentence_rec
                .iter()
                .flat_map(|sen| sen.text.iter().zip(sen.width.iter()))
                .chain(pending.iter().zip(pending_width.iter()));
            let wrapped = wrap(aa, area.width as usize - 2);
            let last_width = wrapped.1 as u16;
            let lines = wrapped.2 as u16;
            let wrapped = wrapped.0;

            let text: Text = create_diff_text(wrapped, preset_text.as_ref(), pending.len());
            let inner_height = area.height - 2;
            let sentence_widget = Paragraph::new(text)
                .scroll(((lines + 1).max(inner_height) - inner_height, 0))
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "成句 [输入中:{}]",
                    input.iter().collect::<String>()
                )));
            frame.render_widget(sentence_widget, chunks[0]);
            frame.set_cursor_position((
                area.x + last_width.max(1),
                area.y + lines.min(area.height - 3),
            ));

            // measurements
            let measurement =
                calc_measurements(&sentence_rec, preset_text.as_ref().map(|p| p.len()));
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
                    cand_max_width = cand_line.width().max(cand_max_width);
                    cand_text.push(cand_line);
                }
                let popup_width = (cand_max_width as u16 + 2).min((area.width - 4) / 2);
                let mut popup_height = (area.height - (lines + 2)).min(cand_count + 2);
                let mut popup_y = (area.y + lines + 1).max(2);
                let popup_x = (area.x + last_width)
                    .max(1)
                    .min(area.width - popup_width - 1);
                if lines + 1 > area.height / 2 {
                    popup_height = (lines - 1).min(area.height - 3).min(cand_count + 2);
                    popup_y = area.y + ((lines - 1).min(area.height - 3)) - popup_height;
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
        match event::read()? {
            Event::Resize(_, _) => {
                need_wrapped_preset_text = true;
            }
            Event::Key(key) => match key.code {
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
                        text: vec!['\n'],
                        origin: vec!['\n'],
                        width: vec![0],
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
            },
            _ => {}
        }
    }

    disable_raw_mode()?;
    {
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
    }
    // let bs = sentence_rec
    //     .iter()
    //     .map(|se| se.text.as_bytes().to_vec())
    //     .flatten()
    //     .collect::<Vec<u8>>();
    // io::stdout().write(&bs)?;
    // io::stdout().write_all(b"\n")?;
    Ok(())
}

fn push_to_text(text: &mut Text<'_>, chars: &[char], style: Option<Style>) {
    if chars.is_empty() {
        return;
    }
    let mut start = 0;
    for i in 0..chars.len() {
        if chars[i] == '\n' {
            let mut line = Span::from(chars[start..i].iter().collect::<String>());
            if let Some(style) = style {
                line = line.style(style);
            }
            text.push_span(line);
            text.push_line("");
            start = i + 1;
        }
    }
    if start < chars.len() {
        let mut line = Span::from(chars[start..].iter().collect::<String>());
        if let Some(style) = style {
            line = line.style(style);
        }
        text.push_span(line);
    }
}

fn create_diff_text(chars: Vec<char>, other: Option<&Vec<char>>, pending_chars: usize) -> Text<'_> {
    if other.is_none() {
        let mut text = Text::from("");
        let end = chars.len() - pending_chars;
        push_to_text(&mut text, &chars[..end], None);
        if pending_chars > 0 {
            push_to_text(&mut text, &chars[end..], Some(Color::Green.into()));
        }
        return text;
    }
    let preset = other.unwrap();
    let mut start = 0;
    let mut text = Text::from("");
    for i in 0..chars.len().min(preset.len()) {
        let (l, r) = (chars[i], preset[i]);
        if l != r {
            if start < i {
                push_to_text(&mut text, &chars[start..i], None);
            }
            let sp = l.to_string();
            // FIXME
            // if r.width() != l.width() {
            //     sp = r.to_string();
            // }
            text.push_span(Span::from(sp).on_light_red().crossed_out());
            start = i + 1;
        }
    }
    if start < chars.len() {
        push_to_text(&mut text, &chars[start..], None);
    }
    if chars.len() < preset.len() {
        push_to_text(
            &mut text,
            &preset[chars.len()..],
            Some(Color::DarkGray.into()),
        );
    }
    text
}

#[cfg(test)]
mod test {
    use unicode_width::UnicodeWidthStr;

    use crate::create_diff_text;

    #[test]
    fn test_create_diff_text() {
        let left = "hello, world".chars().collect::<Vec<_>>();
        let right = "hella,_world, gray".chars().collect::<Vec<_>>();
        let text = create_diff_text(left, Some(&right), 0);
        println!("text: {text:?}");
    }

    #[test]
    fn test_punc_length() {
        let punc = "……";
        let width_cjk = punc.width_cjk();
        let width2 = punc.width();
        println!("{punc} > width cjk: {width_cjk}, width 2: {width2}]");
        let punc = "——";
        let width_cjk = punc.width_cjk();
        let width2 = punc.width();
        println!("{punc} > width cjk: {width_cjk}, width 2: {width2}]");
        let punc = "你好";
        let width_cjk = punc.width_cjk();
        let width2 = punc.width();
        println!("{punc} > width cjk: {width_cjk}, width 2: {width2}]");
        let punc = "你好abc";
        let width_cjk = punc.width_cjk();
        let width2 = punc.width();
        println!("{punc} > width cjk: {width_cjk}, width 2: {width2}]");
    }
}

fn wrap<I, C, U>(chars: I, limit: usize) -> (Vec<char>, usize, usize)
where
    I: IntoIterator<Item = (C, U)>,
    C: Borrow<char>,
    U: Borrow<usize>,
{
    let (mut wrapped, mut last_width, mut lines) = (vec![], 0, 1);
    for (cha, wid) in chars {
        let cha = *cha.borrow();
        let wid = *wid.borrow();
        if cha == '\n' || last_width + wid > limit {
            last_width = 0;
            lines += 1;
            if cha != '\n' {
                wrapped.push('\n');
            }
        }
        if cha != '\n' {
            last_width += wid;
        }
        wrapped.push(cha);
    }
    (wrapped, last_width, lines)
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
fn calc_measurements(records: &[SentenceRecord], preset_wc: Option<usize>) -> Measurement {
    let (start, end) = if let (Some(first), Some(last)) = (records.first(), records.last()) {
        (first.satrt, last.end)
    } else {
        (Instant::now(), Instant::now())
    };
    let (text_wc, code_cc, _space_times) =
        records
            .iter()
            .fold((0, 0, 0), |(total_text, total_code, space_times), rec| {
                (
                    total_text + rec.text.len(),
                    total_code + rec.origin.len(),
                    space_times,
                )
            });

    let duration = end.duration_since(start);
    let wpm = text_wc as f32 / (duration.as_secs_f32() / 60.0);
    let kps = code_cc as f32 / duration.as_secs_f32();
    let avg_len = code_cc as f32 / text_wc as f32;

    Measurement {
        // start,
        // end,
        duration,
        text_wc,
        code_cc,
        preset_wc,
        wpm,
        kps,
        avg_len,
    }
}

struct Measurement {
    // start: Instant,
    // end: Instant,
    duration: Duration,
    text_wc: usize,
    code_cc: usize,
    preset_wc: Option<usize>,
    kps: f32,
    wpm: f32,
    avg_len: f32,
}

impl Display for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  耗时: [{}]秒, 速度: [{:.2}]字/分, 击键: [{:.2}]键/秒\n  总字数: [{}{}], 总输入: [{}], 平均码长: [{:.2}]",
            self.duration.as_secs(),
            self.wpm,
            self.kps,
            self.text_wc,
            self.preset_wc.map_or("".to_string(), |pw| format!("/{pw}")),
            self.code_cc,
            self.avg_len,
        )
    }
}
