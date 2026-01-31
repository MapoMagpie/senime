use std::fmt::Display;
use std::fs::DirBuilder;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::time::{Duration, Instant};

use clap::{ArgAction, Parser};
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{event, execute};
use dirs::data_dir;
use dirs::state_dir;
use ratatui::Terminal;
use ratatui::layout::Size;
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::{Block, Borders, Paragraph};

use senime_lib::{AnalysisResult, Dict, InputAnalyzer, Looker};

use crate::context::{Context, Record, WrappedText};
use crate::popup::Popup;

mod context;
mod popup;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 码表文件，其结构应为: 字词<TAB>编码<TAB>权重(可选) 每行
    /// 当没有权重时则以行的顺序判断编码对应的字词的首选还是候选
    #[arg(short, long)]
    pub table: String,

    /// 将此文件中的内容作为预设文本
    /// 此功能类似赛码器，将以灰色的文本展示这些预设文本
    #[arg(short, long)]
    pub preset: Option<String>,

    /// 保持预设文本的格式，默认False
    /// 不设置此项时，默认移除预设文本中的空格、换行符
    #[arg(long, action = ArgAction::SetTrue)]
    pub keep: bool,

    /// 选择预设文本的范围
    /// 格式为: 1-10 (行一到行十)，1.3-10.6 (行一的第三字到行十的第六字)
    #[arg(long)]
    pub pick: Option<String>,

    /// 将标准输入流中的内容作为预设文本，与--preset功能一样
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub stdin: bool,

    /// 使用标准输出流做出界面绘制区
    /// 默认使用/dev/tty做为界面绘制区，若无法打开/dev/tty或其不存在，可使用--stdout解决此问题
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub stdout: bool,
}

fn read_stdin() -> Result<String, std::io::Error> {
    use std::io::Read;
    let mut stdin = std::io::stdin();
    let mut str = String::new();
    stdin.read_to_string(&mut str)?;
    Ok(str)
}

fn read_file<F>(path: F) -> Result<String, std::io::Error>
where
    F: AsRef<Path>,
{
    use std::io::Read;
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(path)?;
    let mut str = String::new();
    file.read_to_string(&mut str)?;
    Ok(str)
}

#[cfg(unix)]
fn create_backend() -> Result<CrosstermBackend<File>, Box<dyn std::error::Error>> {
    let mut stdout = OpenOptions::new()
        .read(false)
        .write(true)
        .open("/dev/tty")
        .map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                format!("无法打开 /dev/tty {:?}", err),
            )
        })?;
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(backend)
}

#[cfg(not(unix))]
fn create_backend() -> Result<CrosstermBackend<Stdout>, Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(backend)
}

// TODO: 实现中间编辑，删除新增
// TODO: 重构setpending，接续分词，降低复杂性
// TODO: 数据记录，每次使用时，生成一个时间相关的ID，并在适当的时候将所有的输入记录保存下来
// TODO: 减少PreRender的计算
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let preset: Option<Vec<char>> = if args.stdin {
        Some(read_stdin()?)
    } else if let Some(preset_path) = args.preset {
        Some(read_file(&preset_path)?)
    } else {
        None
    }
    .map(|str| {
        process_preset(
            &str,
            args.keep,
            args.pick.map(|pi| PickPreset::from_str(&pi).unwrap()),
        )
    });

    let time_id = generate_time_id();
    let dict = Dict::load(args.table);
    // 分词器
    let enc = Looker::new(&dict.candidates);
    // 输入解析器 aka.输入法核心
    let an = InputAnalyzer::new(dict);
    // 上下文，存储输入记录、分词结果，aka.缓存一些计算结果，提升性能
    let mut ctx = Context::new(&enc);
    ctx.set_preset(preset);

    let mut first = true;
    enable_raw_mode()?;
    let backend = create_backend()?;
    let mut terminal = Terminal::new(backend)?;
    let mut area: Rect = terminal.size()?.into();

    loop {
        if first {
            first = false;
        } else {
            match event::read()? {
                Event::Resize(w, h) => {
                    area = Size::new(w, h).into();
                }
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                        break;
                    }
                    KeyCode::Char('x') if key.modifiers == KeyModifiers::CONTROL => {
                        ctx.clear();
                    }
                    KeyCode::Char('s') if key.modifiers == KeyModifiers::CONTROL => {
                        let measurement = calc_measurements(ctx.get_recorders(), ctx.preset_len());
                        let _ = write_input_data(&time_id, &ctx, &measurement);
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Enter => {
                        ctx.confrim_pending();
                        ctx.push(vec!['\n'], vec!['\n']);
                    }
                    KeyCode::Backspace => {
                        ctx.backspace();
                    }
                    KeyCode::Char(c) => {
                        ctx.push_input(c);
                    }
                    _ => {
                        continue;
                    }
                },
                _ => {
                    continue;
                }
            }
        }

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
            ctx.clear_pending();
        }
        if let Some((text, chars)) = poped {
            // 会出现text为空，而chars为 ' '(空格)
            let text_chars: Vec<char> = text.chars().collect();
            if candidates.is_none() && text_chars != chars {
                ctx.clear_pending();
                ctx.push(text_chars, chars);
            } else {
                ctx.set_pending(text_chars, chars);
            }
        }
        // 当应用全屏时与frame.area() 一致，目前是默认的全屏
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(5)])
            .split(area);

        let b_area = chunks[0];
        let m_area = chunks[1];
        let t_area = b_area.inner(Margin::new(1, 1));
        let (pre_render, cursor) = ctx.calc_pre_render(t_area);

        let measurement = calc_measurements(ctx.get_recorders(), ctx.preset_len());

        let draw_duration = draw_start.elapsed();
        // candidates
        terminal.draw(|frame| {
            let block = Block::default().borders(Borders::ALL).title(format!(
                "输入中: [{}] 计算时间: [{:?}]",
                ctx.get_input().iter().collect::<String>(),
                draw_duration
            ));
            frame.render_widget(block, b_area);
            frame.render_widget(WrappedText::new(pre_render), t_area);
            frame.set_cursor_position(cursor);

            if let Some(cands) = candidates {
                let (popup, p_area) = Popup::create(
                    &cands,
                    t_area,
                    cursor,
                    ctx.get_input().iter().map(|c| c.len_utf8()).sum(),
                );
                frame.render_widget(popup, p_area);
            }

            let measurement_widget = Paragraph::new(measurement.to_string())
                .block(Block::default().borders(Borders::ALL).title("计量"));
            frame.render_widget(measurement_widget, m_area);
        })?;
    }

    disable_raw_mode()?;
    {
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
    }
    let measurement = calc_measurements(ctx.get_recorders(), ctx.preset_len());
    match write_input_data(&time_id, &ctx, &measurement) {
        Err(err) => {
            eprintln!("写入输入数据时出错: {:?}", err);
        }
        _ => {}
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
        |(total_text, total_code, space_times, pause_time, last_time), rec| {
            if rec.range.is_empty() {
                return (total_text, total_code, space_times, pause_time, last_time);
            }
            // 计算暂停时间，如果两个record之间间隔了5秒，则认为这是暂停
            let mut pause_dur = Duration::from_secs(0);
            if last_time < rec.start {
                let dur = rec.start - last_time;
                if dur > pause_assert {
                    pause_dur = dur;
                }
            }
            (
                total_text + rec.range.len(),
                total_code + rec.origin.len(),
                space_times,
                pause_time + pause_dur,
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

/// 文本选择范围，左闭右开
struct PickPreset {
    /// 开始的行
    line_start: usize,
    /// 在开始的行中，字符的起始位置
    char_start: usize,
    /// 结束的行
    line_end: usize,
    /// 在结束的行中，字符的结束位置
    char_end: usize,
}

impl FromStr for PickPreset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error =
            "预设文本选择范围格式应为: 1-10(行一到行十)，或1.3-10.6(行一的第三字到行十的第六字)"
                .to_string();
        let split = s.split('-').collect::<Vec<_>>();
        if split.is_empty() {
            return Err(error);
        }
        let parse = |str: &str| -> Result<(usize, usize), String> {
            let mut sp = str.split('.');
            let line = sp
                .next()
                .unwrap()
                .parse()
                .map_err(|e| format!("{e}\n{error}"))?;
            let pos = sp
                .next()
                .unwrap_or("0")
                .parse()
                .map_err(|e| format!("{e}\n{error}"))?;
            Ok((line, pos))
        };

        let (start, char_start) = parse(&split[0])?;
        let (end, char_end) = split
            .get(1)
            .map(|sp| parse(*sp))
            .unwrap_or(Ok((usize::MAX, usize::MAX)))?;
        Ok(Self {
            line_start: start.saturating_sub(1),
            char_start: char_start.saturating_sub(1),
            line_end: end,
            char_end: char_end,
        })
    }
}

impl Default for PickPreset {
    fn default() -> Self {
        Self {
            line_start: 0,
            char_start: 0,
            line_end: usize::MAX,
            char_end: usize::MAX,
        }
    }
}

fn process_preset(str: &str, keep: bool, pick: Option<PickPreset>) -> Vec<char> {
    let pick = pick.unwrap_or(PickPreset::default());
    let mut lines = str
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if i < pick.line_start || i >= pick.line_end {
                return vec![];
            }
            let chars = line
                .chars()
                .enumerate()
                .filter_map(|(j, c)| {
                    if (i == pick.line_start && j < pick.char_start)
                        || (i == pick.line_end && j >= pick.char_end)
                           // 当不保持预设文本原样时，则过滤空白和控制字符
                        || !keep && (c.is_whitespace() || c.is_control())
                    {
                        None
                    } else {
                        Some(c)
                    }
                })
                .chain(if keep {
                    vec!['\n'].into_iter()
                } else {
                    vec![].into_iter()
                })
                .collect();
            chars
        })
        .flatten()
        .collect::<Vec<_>>();
    if keep {
        lines.pop();
    }
    lines
}

/// 写入输入信息，
/// 包括：输入文本、
///       输入记录
///       预设文本
///       计量信息
/// 输出文件：
///       文件1、输入文本
///       文件2、chunk_1>计量信息 chunk_2>输入记录 chunk_3预设文本
fn write_input_data(
    id: &str,
    ctx: &Context,
    measurement: &Measurement,
) -> Result<(), std::io::Error> {
    let fire_prefix = format!("sentui_{id}_");
    let state_dir = match data_dir() {
        Some(mut dir) => {
            dir.push("senitui");
            if !dir.exists() {
                DirBuilder::new().create(dir.clone())?;
            }
            dir
        }
        None => ".".into(),
    };
    // 找出 当前目录下所有包含此前缀的文件并删除
    if let Ok(entries) = std::fs::read_dir(&state_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                if file_name_str.starts_with(&fire_prefix) {
                    let _ = std::fs::remove_file(entry.path());
                }
            }
        }
    }
    let sentence = ctx.get_sentence();
    if sentence.is_empty() {
        return Ok(());
    }
    // 从sentence中挑出前5个字(is_alphabetic)作为文件后缀
    let file_suffix = sentence
        .iter()
        .filter(|c| c.is_alphabetic())
        .take(5)
        .collect::<String>();
    {
        let mut path = state_dir.clone();
        path.push(format!("{}{}.txt", fire_prefix, file_suffix));
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        writeln!(file, "{}", sentence.iter().collect::<String>())?;
    }
    {
        let mut path = state_dir.clone();
        path.push(format!("{}{}_record.txt", fire_prefix, file_suffix));
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        // chunk 1: 计量信息
        writeln!(file, "\n# Measurement\n{}", measurement)?;
        // // chunk 2: 输入记录
        // writeln!(file, "\n# Records")?;
        // for rec in ctx.get_recorders() {
        //     writeln!(
        //         file,
        //         "{:?}\t{}\t{:.2?}",
        //         rec.range,
        //         rec.origin.iter().collect::<String>(),
        //         rec.end.elapsed()
        //     )?;
        // }

        // // chunk 3: 预设文本
        // if let Some(preset) = ctx.get_preset() {
        //     let preset_str: String = preset.iter().collect();
        //     writeln!(file, "\n# Preset\n{}", preset_str)?;
        // }
    }
    Ok(())
}

/// 根据时间生成一个ID
fn generate_time_id() -> String {
    use chrono::prelude::*;
    let now: DateTime<Local> = Local::now();
    now.format("%Y%m%d%H%M%S").to_string()
}

#[test]
fn test_generate_id() {
    let id = generate_time_id();
    println!("Generated ID: {}", id);
}
