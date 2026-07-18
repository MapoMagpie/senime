use std::fs::DirBuilder;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use std::str::FromStr;
use std::time::Instant;

use clap::{ArgAction, Parser};
use crossterm::cursor::SetCursorStyle;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{event, execute};
use ratatui::Terminal;
use ratatui::layout::Position;
use ratatui::layout::Size;
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::prelude::CrosstermBackend;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Clear;
use ratatui::widgets::Widget;
use ratatui::widgets::{Block, Borders};

use senime_lib::PAGE_DOWN;
use senime_lib::PAGE_UP;
use senime_lib::input_analyzer::load_input_analyzer;
use senime_lib::{AnalysisResult, Looker};

use crate::context::{Context, WrappedText};
use crate::popup::Popup;

mod context;
mod js;
mod measurement;
mod popup;
mod sentence;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 码表文件或配置文件
    /// 如果指定的是配置文件，则需要在配置中指定码表文件。
    /// 如果指定的是码表文件，其结构应为: 字词<TAB>编码<TAB>权重(可选) 每行，当没有权重时则以行的顺序判断编码对应的字词的首选还是候选。
    /// 同时，还可以直接指定二进制格式的码表文件，它是由本程序编译码表后产生的bin文件。
    /// 如果不指定此参数，默认将尝试加载 `XDG_CONFIG_HOME/senime/config.toml`
    #[arg(short, long, verbatim_doc_comment)]
    pub table: Option<String>,

    /// 将此文件中的内容作为预设文本
    /// 此功能类似赛码器，将以灰色的文本展示这些预设文本
    #[arg(short, long, verbatim_doc_comment)]
    pub preset: Option<String>,

    /// 保持预设文本的格式，默认False
    /// 不设置此项时，默认移除预设文本中的空格、换行符
    #[arg(long, action = ArgAction::SetTrue, verbatim_doc_comment)]
    pub keep: bool,

    /// 选择预设文本的范围
    /// 格式为: 1-10 (行一到行十)，1.3-10.6 (行一的第三字到行十的第六字)
    #[arg(long, verbatim_doc_comment)]
    pub pick: Option<String>,

    /// 将标准输入流中的内容作为预设文本，与--preset功能一样
    #[arg(short = 'i', long, action = ArgAction::SetTrue, verbatim_doc_comment)]
    pub stdin: bool,

    /// 退出程序时，将输入的内容输出到标准输出流
    #[arg(short = 'o', long, action = ArgAction::SetTrue, verbatim_doc_comment)]
    pub stdout: bool,

    /// 是否保存输入记录，退出程序后，会将输入内容与额外信息(速度、击键、耗时、码长等)保存到`XDG_DATA_HOME/senitui`中
    #[arg(short = 'r', long, action = ArgAction::SetTrue, verbatim_doc_comment)]
    pub record: bool,

    /// 极速中文网设置(TOML格式)
    #[arg(long, verbatim_doc_comment)]
    pub js_settings: Option<String>,

    /// 极速中文网赛文获取方式
    /// random: 随机文本
    /// daily:  每日赛文
    #[arg(long, verbatim_doc_comment)]
    pub js_action: Option<String>,
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
use std::fs::File;
#[cfg(unix)]
fn create_backend() -> Result<CrosstermBackend<File>, Box<dyn std::error::Error>> {
    use crossterm::cursor::SetCursorStyle;

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
    execute!(
        stdout,
        EnterAlternateScreen,
        SetCursorStyle::DefaultUserShape
    )?;
    let backend = CrosstermBackend::new(stdout);
    Ok(backend)
}

#[cfg(not(unix))]
use std::io::{Stdout, stdout};
#[cfg(not(unix))]
fn create_backend() -> Result<CrosstermBackend<Stdout>, Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, SetCursorStyle::BlinkingBar)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(backend)
}

fn get_default_table() -> Result<String, std::io::Error> {
    use std::io::{Error, ErrorKind};
    // 寻找XDG_CONFIG_HOME/senime/config.toml，并检查文件是否存在，不存在时，返回错误。
    dirs::config_dir()
        .map(|dir| dir.join("senime").join("config.toml"))
        .filter(|path| path.is_file())
        .map(|path| path.to_str().unwrap().to_owned())
        .ok_or(Error::new(
            ErrorKind::NotFound,
            "未指定 --table 参数，且无法找到默认配置文件路径",
        ))
}

// TODO: 实现中间编辑，删除新增
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // 解析 js_action，调用 js bridge 获取预设文本
    let js_action: Option<js::JSAction> = args
        .js_action
        .as_ref()
        .map(|a| js::JSAction::from_str(a).expect("无效的 --js-action，应为 random 或 daily"));
    let js_bridge: Option<(js::JSSettings, js::JSContent)> = match (&args.js_settings, js_action) {
        (Some(path), Some(action)) => Some(js::js_get_content(path, action)?),
        _ => None,
    };

    let table_path: String = match args.table {
        Some(t) => t,
        None => get_default_table()?,
    };
    let preset: Option<Vec<char>> = if args.stdin {
        Some(read_stdin()?)
    } else if let Some(preset_path) = args.preset {
        Some(read_file(&preset_path)?)
    } else {
        js_bridge.as_ref().map(|(_, c)| c.content.clone())
    }
    .map(|str| {
        process_preset(
            &str,
            args.keep,
            args.pick.map(|pi| PickPreset::from_str(&pi).unwrap()),
        )
    });

    let time_id = generate_time_id();
    // 输入法引擎
    let ime = load_input_analyzer(&table_path)?;

    // 分词器
    let encoder = Looker::new(ime.main_dict());
    // 上下文，存储输入记录、分词结果，aka.缓存一些计算结果，提升性能
    let mut ctx = Context::new(encoder);
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
                    ctx.resize();
                }
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    // KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => {
                    //     break;
                    // }
                    KeyCode::Char('x') if key.modifiers == KeyModifiers::CONTROL => {
                        ctx.clear();
                    }
                    KeyCode::Char('s') if key.modifiers == KeyModifiers::CONTROL => {
                        ctx.calc_measurement();
                        let _ = record_input_data(&time_id, &ctx);
                    }
                    KeyCode::PageUp => {
                        ctx.push_input(PAGE_UP);
                    }
                    KeyCode::PageDown => {
                        ctx.push_input(PAGE_DOWN);
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Enter => {
                        ctx.confrim_pending();
                        ctx.push(vec!['\n'], vec!['\n'], false);
                    }
                    KeyCode::Backspace => {
                        ctx.backspace();
                    }
                    KeyCode::Left | KeyCode::Right => {
                        ctx.move_cursor(key.code.into());
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

        let calc_start = Instant::now();
        let AnalysisResult {
            candidates,
            pending,
            mut segments,
        } = ime.analyze(ctx.get_input());

        let poped = segments.pop();
        if !segments.is_empty() {
            segments.into_iter().for_each(|(text, origin, tag)| {
                ctx.push(text.chars(), origin, tag.has_selection());
            });
        }
        if let Some((text, chars, tag)) = poped {
            if pending {
                ctx.set_pending(text.chars(), chars);
            } else {
                ctx.push(text.chars(), chars, tag.has_selection());
            }
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(6)])
            .split(area);

        let b_area = chunks[0];
        let m_area = chunks[1];
        // 当应用全屏时与frame.area() 一致，目前是默认的全屏
        let t_area = b_area.inner(Margin::new(1, 1));

        // 折行计算
        ctx.calc_pre_render(t_area);
        ctx.calc_measurement();

        let (pre_render, Position { x, y }) = ctx.get_pre_render_lines(t_area.height);
        let cursor = Position::new(t_area.x + x, t_area.y + y);

        let calc_duration = calc_start.elapsed();
        // candidates
        terminal.draw(|frame| {
            let block = Block::default().borders(Borders::ALL).title(format!(
                "输入中: [{}]{} 计算耗时: [{:?}]",
                ctx.get_input().iter().collect::<String>(),
                ctx.get_preset_segment_hint()
                    .map_or_else(String::new, |hint| format!(" 提示: [{hint}]")),
                calc_duration
            ));
            frame.render_widget(block, b_area);
            frame.render_widget(WrappedText::new(pre_render), t_area);
            frame.set_cursor_position(cursor);

            if let Some(cands) = candidates {
                let (popup, p_area) = Popup::create(&cands, t_area, cursor);
                frame.render_widget(popup, p_area);
            }
            frame.render_widget(Block::default().borders(Borders::ALL).title("计量"), m_area);
            let m_inner_area = m_area.inner(Margin::new(1, 1));
            frame.render_widget(WrappedSpans::from_iter(ctx.measure().spans()), m_inner_area);
        })?;
    }

    disable_raw_mode()?;
    {
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            SetCursorStyle::DefaultUserShape
        )?;
        terminal.show_cursor()?;
    }
    if ctx.sentence_len() > 0 {
        if args.stdout {
            println!("{}", ctx.get_sentence().collect::<String>())
        }
        if args.record {
            record_input_data(&time_id, &ctx)?;
        }
        // 向 jsxiaoshi.com 上报输入数据
        if let (Some((ref settings, ref content)), Some(action)) = (js_bridge, js_action)
            && ctx.sentence_len() >= ctx.preset_len()
        {
            eprint!("上传打字数据中...");
            match js::js_report(settings, action, ctx.measure(), content) {
                Ok(msg) => eprintln!("{msg}"),
                Err(e) => eprintln!("{e}"),
            }
        }
    }
    Ok(())
}

struct WrappedSpans<'a> {
    spans: Vec<Span<'a>>,
}

impl<'a, T> FromIterator<T> for WrappedSpans<'a>
where
    T: Into<Span<'a>>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let spans = iter.into_iter().map(Into::into).collect::<Vec<_>>();
        Self { spans }
    }
}

impl Widget for WrappedSpans<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        Clear.render(area, buf);
        let mut y = 0;
        let mut line = Line::default();
        let mut line_width = 0;
        for span in self.spans {
            let span_width = span.width() + 1;
            if line_width + span_width > area.width as usize {
                buf.set_line(area.x, area.y + y, &line, area.width);
                y += 1;
                if y >= area.height {
                    break;
                }
                line = Line::default();
                line_width = span_width;
                line.push_span(span);
                line.push_span(" ");
            } else {
                line.push_span(span);
                line.push_span(" ");
                line_width += span_width;
            }
        }
        buf.set_line(area.x, area.y + y, &line, area.width);
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

        let (start, char_start) = parse(split[0])?;
        let (line_end, char_end) = split
            .get(1)
            .map(|sp| parse(sp))
            .unwrap_or(Ok((usize::MAX, usize::MAX)))?;
        Ok(Self {
            line_start: start.saturating_sub(1),
            char_start: char_start.saturating_sub(1),
            line_end,
            char_end,
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
    let pick = pick.unwrap_or_default();
    let mut lines = str
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            if i < pick.line_start || i >= pick.line_end {
                return vec![];
            }
            line.chars()
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
                .collect()
        })
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
fn record_input_data(id: &str, ctx: &Context) -> Result<(), std::io::Error> {
    let fire_prefix = format!("sentui_{id}_");
    let state_dir = match dirs::data_dir() {
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
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            if file_name_str.starts_with(&fire_prefix) {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }
    let mut sentence = ctx.get_sentence();
    let mut suffix = String::new();
    let mut suffix_chars = 0;
    let mut taked = vec![];
    while let Some(c) = sentence.next()
        && suffix_chars < 5
    {
        taked.push(c);
        if c.is_alphanumeric() {
            suffix.push(*c);
            suffix_chars += 1;
        }
    }
    {
        // 从sentence中选出前5个字符做为文件的标识
        let mut path = state_dir.clone();
        path.push(format!("{}{}.txt", fire_prefix, suffix.clone()));
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        let str = taked.into_iter().chain(sentence).collect::<String>();
        writeln!(file, "{}", str)?;
    }
    {
        let mut path = state_dir.clone();
        path.push(format!("{}{}_record.txt", fire_prefix, suffix));
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        // chunk 1: 计量信息
        writeln!(file, "# Measurement\n{}", ctx.measure())?;
        // // chunk 2: 输入记录
        writeln!(file, "# Records")?;
        for rec in &ctx.measure().records {
            // instant to time
            writeln!(
                file,
                "{:?}\t{}",
                rec.len,
                rec.origin.iter().collect::<String>(),
            )?;
        }

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
