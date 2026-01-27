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
use ratatui::layout::{Constraint, Direction, Layout, Margin, Position, Rect};
use ratatui::prelude::CrosstermBackend;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};

use senime_lib::input_analyzer::CandidateRich;
use senime_lib::{AnalysisResult, Dict, InputAnalyzer, Looker};

use crate::context::{Context, Record, WrappedText};

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

mod context;

fn read_stdin() -> Result<String, std::io::Error> {
    use std::io::Read;
    let mut stdin = std::io::stdin();
    let mut str = String::new();
    stdin.read_to_string(&mut str)?;
    Ok(str)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    enable_raw_mode()?;
    let mut stdout = OpenOptions::new()
        .read(false)
        .write(true)
        .open("/dev/tty")?;
    // let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let preset: Option<Vec<char>> = if args.stdin {
        read_stdin().map(|str| Some(str.chars().collect())).unwrap()
    } else {
        None
    };
    let dict = Dict::load(args.table);
    // 分词器
    let enc = Looker::new(&dict.candidates);
    // 输入解析器 aka.输入法核心
    let an = InputAnalyzer::new(dict);

    let mut ctx = Context::new(&enc);
    ctx.set_preset(preset);

    loop {
        let draw_start = Instant::now();
        let AnalysisResult {
            candidates,
            mut segments,
        } = an.analyze(ctx.get_input());

        let poped = segments.pop();
        if !segments.is_empty() {
            segments.into_iter().for_each(|(text, origin)| {
                ctx.push(text.chars(), origin);
            });
        }
        if let Some((text, chars)) = poped {
            let text_chars: Vec<char> = text.chars().collect();
            if candidates.is_none() && text_chars != chars {
                ctx.push(text_chars, chars);
                ctx.clear_pending();
            } else {
                ctx.set_pending(text_chars, chars);
            }
        }
        // 当应用全屏时与frame.area() 一致，目前是默认的全屏
        let area: Rect = terminal.size()?.into();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(5)])
            .split(area);
        let b_area = chunks[0];
        let t_area = b_area.inner(Margin::new(1, 1));
        let (pre_render, cursor) = ctx.calc_pre_render(t_area);

        let measurement = calc_measurements(ctx.get_recorders(), ctx.preset_len());

        let draw_duration = draw_start.elapsed();
        // candidates
        terminal.draw(|frame| {
            let block = Block::default().borders(Borders::ALL).title(format!(
                "成句 [输入中:{}] 绘图时间: [{:?}]",
                ctx.get_input().iter().collect::<String>(),
                draw_duration
            ));
            frame.render_widget(block, area);
            frame.render_widget(WrappedText::new(&pre_render), t_area);
            frame.set_cursor_position(cursor);

            let popup = create_pupup(candidates, t_area, cursor, ctx.get_input().len());
            if let Some((popup, p_area)) = popup {
                frame.render_widget(popup, p_area);
            }

            let measurement_widget = Paragraph::new(measurement.to_string())
                .block(Block::default().borders(Borders::ALL).title("计量"));
            frame.render_widget(measurement_widget, chunks[1]);
        })?;
        // 事件处理
        // if event::poll(Duration::from_millis(100))? {
        match event::read()? {
            // Event::Resize(_, _) => {
            //     need_wrapped_preset_text = true;
            // }
            Event::Key(key) => match key.code {
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                    break;
                }
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Enter => {
                    ctx.confrim_pending();
                    ctx.push(vec!['\n'], vec!['\n']);
                }
                KeyCode::Backspace => {
                    // TODO
                    ctx.backspace();
                }
                KeyCode::Char(c) => {
                    ctx.push_input(c);
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
    //     .map(|se| se.text.iter())
    //     .flatten()
    //     .collect::<String>();
    // io::stdout().write(bs.as_bytes())?;
    // io::stdout().write_all(b"\n")?;
    Ok(())
}

fn create_pupup(
    candidates: Option<Vec<CandidateRich>>,
    root: Rect,
    cursor: Position,
    input_len: usize,
) -> Option<(Popup<'static>, Rect)> {
    let popup = if let Some(candidates) = candidates {
        let cand_count = candidates.len();
        let mut cand_max_width = 0;
        let mut cand_text: Vec<Line> = vec![];
        for cand in candidates.into_iter() {
            let mut cand_line = Line::from("[");
            cand_line.push_span(Span::from(cand.select_key.to_string()).green());
            cand_line.push_span("]: ");
            cand_line.push_span(cand.text);
            if cand.code.len() > input_len {
                cand_line.push_span(Span::from(cand.code.clone().split_off(input_len)).red());
            }
            cand_max_width = cand_line.width().max(cand_max_width);
            cand_text.push(cand_line);
        }
        let mut p_area = Rect {
            x: root.x + (cursor.x.max(3) - 2),
            y: cursor.y + 1,
            width: (cand_max_width as u16 + 2).min(root.width),
            height: cand_count as u16 + 2,
        };
        if p_area.right() > root.right() {
            p_area.x -= p_area.right() - root.right();
        }
        if p_area.bottom() > root.bottom() {
            p_area.height -= p_area.bottom() - root.bottom();
        }
        // 如果指针下方小于6的空间，则将popup上移至cursor.y + 1并反转
        if root.bottom() - cursor.y < 6 {
            p_area.height = (cursor.y - 1 - root.y).min(cand_count as u16 + 2);
            p_area.y = cursor.y - p_area.height;
            if p_area.height > 2 {
                let _ = cand_text.split_off(p_area.height as usize - 2);
                cand_text.reverse();
            }
        }

        Some((
            Popup::default()
                .content(cand_text)
                .style(Style::new().yellow())
                .border_style(Style::new().red()),
            p_area,
        ))
    } else {
        None
    };
    popup
}

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
        .map(|(a, b)| a != b)
        .collect()
}

#[cfg(test)]
mod test {
    use unicode_width::UnicodeWidthStr;

    use crate::diff_sequence;

    #[test]
    fn test_create_diff_text() {
        let left = "hello, world".chars().collect::<Vec<_>>();
        let right = "hella,_world, gray".chars().collect::<Vec<_>>();
        let diff_indices = diff_sequence(left.iter(), Some(right.iter()));
        println!("text: {diff_indices:?}");
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

/// 计量速度.
/// 需要的信息:
///   开始时间-结束时间
///   总字数
///   总输入
///   码长
///   顶字次数?
///   空格次数?
///   回退次数?
fn calc_measurements(records: &[Record], preset_wc: Option<usize>) -> Measurement {
    let (start, end) = if let (Some(first), Some(last)) = (records.first(), records.last()) {
        (first.start, last.end)
    } else {
        (Instant::now(), Instant::now())
    };
    let pause_assert = Duration::from_secs(5);
    let (text_wc, code_cc, _space_times, pause_duration, _last_time) = records.iter().fold(
        (0, 0, 0, Duration::from_millis(0), Instant::now()),
        |(total_text, total_code, space_times, mut pause_time, last_time), rec| {
            // 计算暂停时间，如果两个record之间间隔了5秒，则认为这是暂停
            if last_time < rec.start {
                let dur = rec.start - last_time;
                if dur > pause_assert {
                    pause_time += dur;
                }
            }
            (
                total_text + rec.range.len(),
                total_code + rec.origin.len(),
                space_times,
                pause_time,
                rec.end,
            )
        },
    );

    let duration = end.duration_since(start) - pause_duration;
    let wpm = text_wc as f32 / (duration.as_secs_f32() / 60.0);
    let kps = code_cc as f32 / duration.as_secs_f32();
    let avg_len = code_cc as f32 / text_wc as f32;

    Measurement {
        duration,
        pause_duration,
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
    pause_duration: Duration,
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
            "  耗时: [{:.2?}] 速度: [{:.2}], 击键: [{:.2}] \n  暂停: [{:.2?}] 字数: [{}{}] 键数: [{}]\n  码长: [{:.2}]",
            self.duration,
            self.wpm,
            self.kps,
            self.pause_duration,
            self.text_wc,
            self.preset_wc.map_or("".to_string(), |pw| format!("/{pw}")),
            self.code_cc,
            self.avg_len,
        )
    }
}
