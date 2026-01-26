use std::fmt::Display;
use std::fs::OpenOptions;
use std::ops::Range;
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

type PreRender = Vec<Vec<(u16, Option<char>, Option<Style>)>>;
type PreRenderSlice<'a> = &'a [Vec<(u16, Option<char>, Option<Style>)>];

fn calc_pre_render(
    content: impl IntoIterator<Item = char>,
    modifies: &[(usize, Style)],
    rect: Rect,
    cursor_at: usize,
) -> (PreRender, Position) {
    let (mut x, mut y, width) = (1, 0, rect.width as usize);
    let init_line = || {
        let mut v = vec![(0, None, None); rect.width as usize];
        (0..v.len()).for_each(|i| v[i].0 = rect.x + i as u16);
        v
    };
    let mut cursor: Option<Position> = None;
    let mut mod_i = 0;
    let mut ret: PreRender = vec![];
    let mut first = true;
    for (i, c) in content.into_iter().enumerate() {
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
        if mod_i < modifies.len() && i == modifies[mod_i].0 {
            ret[y][x].2 = Some(modifies[mod_i].1);
            mod_i += 1;
        }
        if c != '\n' {
            ret[y][x].1 = Some(c);
        }
        if i + 1 >= cursor_at && cursor.is_none() {
            cursor = Some(Position::new(rect.x + x as u16, rect.y + y as u16))
        }
        x += wid;
    }
    (
        ret,
        cursor.unwrap_or(Position::new(rect.x + x as u16, rect.y + y as u16)),
    )
}

#[derive(Debug)]
struct WrappedText<'a> {
    pre_render: PreRenderSlice<'a>,
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

struct SentenceRecord {
    text: Vec<char>,
    origin: Vec<char>,
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
    let preset_text: Option<Vec<char>> = if args.stdin {
        read_stdin().map(|str| Some(str.chars().collect())).unwrap()
    } else {
        None
    };
    // let mut need_wrapped_preset_text = true;

    let dict = Dict::load(args.table);
    let an = InputAnalyzer::new(dict);
    enable_raw_mode()?;
    let mut stdout = OpenOptions::new()
        .read(false)
        .write(true)
        .open("/dev/tty")?;
    // let mut stdout = io::stdout();
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
            let area = chunks[0];

            let AnalysisResult {
                candidates,
                mut segments,
            } = an.analyze(&input);
            let poped = segments.pop();
            if !segments.is_empty() {
                sentence_rec.extend(segments.into_iter().map(|seg| {
                    let text: Vec<char> = seg.0.chars().collect();
                    SentenceRecord {
                        text,
                        origin: seg.1,
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
            let new_chars = sentence_rec
                .iter()
                .flat_map(|sen| sen.text.iter())
                .chain(pending.iter())
                .map(|c| *c)
                .collect::<Vec<_>>();
            let diff_style = Style::default().on_light_red().crossed_out();
            let mut char_modifies =
                diff_sequence(new_chars.iter(), preset_text.as_ref().map(|p| p.iter()))
                    .into_iter()
                    .map(|i| (i, diff_style))
                    .collect::<Vec<_>>();
            let suff_chars: Option<(&[char], Range<usize>)> = preset_text
                .as_ref()
                .map(|p| {
                    if new_chars.len() < p.len() {
                        Some((&p[new_chars.len()..], new_chars.len()..p.len()))
                    } else {
                        None
                    }
                })
                .flatten();

            let char_iter = new_chars.iter().chain({
                if let Some((suff_chars, mod_range)) = suff_chars {
                    let gray_style = Style::default().dark_gray();
                    char_modifies.extend(mod_range.map(|i| (i, gray_style)));
                    suff_chars
                } else {
                    Default::default()
                }
            });
            let t_area = area.inner(Margin::new(1, 1));
            let (pre_render, mut cursor) = calc_pre_render(
                char_iter.map(|c| *c),
                &char_modifies,
                t_area,
                new_chars.len(),
            );
            // 根据cursor.y计算切片窗口，cursor.y 从向下2行开始向上倒止t_area.height，并最终修正cursor.y到t_area内
            let mut l_end = cursor.y as usize + 1;
            let l_start = l_end.saturating_sub(t_area.height as usize);
            l_end = (l_start + t_area.height as usize).min(pre_render.len());
            cursor.y = cursor.y - l_start as u16;
            frame.render_widget(
                WrappedText {
                    pre_render: &pre_render[l_start..l_end],
                },
                t_area,
            );
            frame.set_cursor_position(cursor);
            let block = Block::default().borders(Borders::ALL).title(format!(
                "成句 [输入中:{}]",
                input.iter().collect::<String>(),
            ));
            frame.render_widget(block, area);

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
                let cand_count = candidates.len();
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
                let mut p_area = Rect {
                    x: t_area.x + (cursor.x.max(3) - 2),
                    y: cursor.y + 1,
                    width: (cand_max_width as u16 + 2).min(t_area.width),
                    height: cand_count as u16 + 2,
                };
                if p_area.right() > t_area.right() {
                    p_area.x -= p_area.right() - t_area.right();
                }
                if p_area.bottom() > t_area.bottom() {
                    p_area.height -= p_area.bottom() - t_area.bottom();
                }
                // 如果指针下方小于6的空间，则将popup上移至cursor.y + 1并反转
                if t_area.bottom() - cursor.y < 6 {
                    p_area.height = (cursor.y - 1 - t_area.y).min(cand_count as u16 + 2);
                    p_area.y = cursor.y - p_area.height;
                    if p_area.height > 2 {
                        let _ = cand_text.split_off(p_area.height as usize - 2);
                        cand_text.reverse();
                    }
                }

                let popup = Popup::default()
                    .content(cand_text)
                    .style(Style::new().yellow())
                    .border_style(Style::new().red());
                frame.render_widget(popup, p_area);
            }
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
                    if !pending.is_empty() {
                        sentence_rec.push(SentenceRecord {
                            text: pending,
                            origin: input.to_vec(),
                            satrt: input_start,
                            end: Instant::now(),
                        });
                        input.clear();
                    }
                    sentence_rec.push(SentenceRecord {
                        text: vec!['\n'],
                        origin: vec!['\n'],
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
    //     .map(|se| se.text.iter())
    //     .flatten()
    //     .collect::<String>();
    // io::stdout().write(bs.as_bytes())?;
    // io::stdout().write_all(b"\n")?;
    Ok(())
}

// fn push_to_text(text: &mut Text<'_>, chars: &[char], style: Option<Style>) {
//     if chars.is_empty() {
//         return;
//     }
//     let mut start = 0;
//     for i in 0..chars.len() {
//         if chars[i] == '\n' {
//             let mut line = Span::from(chars[start..i].iter().collect::<String>());
//             if let Some(style) = style {
//                 line = line.style(style);
//             }
//             text.push_span(line);
//             text.push_line("");
//             start = i + 1;
//         }
//     }
//     if start < chars.len() {
//         let mut line = Span::from(chars[start..].iter().collect::<String>());
//         if let Some(style) = style {
//             line = line.style(style);
//         }
//         text.push_span(line);
//     }
// }

fn diff_sequence<'a>(
    chars: impl IntoIterator<Item = &'a char>,
    other: Option<impl IntoIterator<Item = &'a char>>,
) -> Vec<usize> {
    if other.is_none() {
        return Vec::default();
    }
    chars
        .into_iter()
        .zip(other.unwrap())
        .enumerate()
        .filter_map(|(i, (a, b))| (a != b).then(|| i))
        .collect::<Vec<usize>>()
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
