use std::{
    fs::{DirBuilder, OpenOptions},
    io::Write,
    iter::once,
    path::Path,
    str::FromStr,
    time::Instant,
};

use clap::{ArgAction, Parser};
use crossterm::{
    cursor::SetCursorStyle,
    event::{Event, KeyCode, KeyEventKind, KeyModifiers, read},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    layout::{Constraint, Direction, Layout, Margin, Position, Rect, Size},
    prelude::CrosstermBackend,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Widget},
};

use senime_lib::{AnalysisResult, Looker, PAGE_DOWN, PAGE_UP, input_analyzer::load_input_analyzer};

use crate::{
    context::{Context, WrappedText},
    js::{JSAction, JSContent, js_get_content, js_report},
    popup::Popup,
};

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

    /// 将此文件中的内容作为预设文本，本程序将化身赛码器
    /// 格式为: <文件名>:[范围]，范围为可选
    /// 示例:
    ///   -p test/文章.txt            #选择此文件作为预设文本
    ///   -p test/文章.txt:10-20      #选择第10行到第20行的内容作为预设文本
    ///   -p test/文章.txt:10.3-20.1  #从第10行第3个字符到第20行第1个字符结束作为预设文本
    ///   -p test/文章.txt:10         #从第10行开始到文件结束
    ///   -p test/文章.txt:10-10      #只选择第10行
    #[arg(short, long, verbatim_doc_comment)]
    pub preset: Option<String>,

    /// 保持预设文本的格式，默认False
    /// 不设置此项时，默认移除预设文本中的空格、换行符
    #[arg(long, action = ArgAction::SetTrue, verbatim_doc_comment)]
    pub keep: bool,

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
    /// 用于`API`认证，然后获取预设文本(赛文)，并上传输入数据到极速中文网
    /// 上传数据含: 计量数据（速度、键速等），所输入的文本内容
    /// 如果是本地自由发文(未从极速获取预设文本)，上传时将进行脱敏处理，替换为同等字符数量的常用字
    /// 如果不指定此参数，默认将尝试加载 `XDG_CONFIG_HOME/senime/js-settings.toml`
    #[arg(long, verbatim_doc_comment)]
    pub js_settings: Option<String>,

    /// 从极速中文网获取预设文本的方式
    /// random     随机文本
    /// daily      每日赛文
    /// dailyonce  每日锦标赛(每天限一次，有时间限制)
    /// 参数为空不会阻止`--js-settings`参数上传数据，可用`--js-action none`阻止上传数据
    /// none       阻止`--js-settings`参数自动上传数据
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
    let js_action = args
        .js_action
        .as_ref()
        .map(|a| JSAction::from_str(a))
        .transpose()?;
    let js_settings = js::js_get_settings(args.js_settings.as_ref())?;
    let js_content = js_settings
        .as_ref()
        .zip(js_action)
        .map(|(settings, action)| js_get_content(settings, action))
        .transpose()?;

    let table_path: String = match args.table {
        Some(t) => t,
        None => get_default_table()?,
    };
    let preset: Option<Vec<char>> = if args.stdin {
        let str = read_stdin()?;
        Some(process_preset(&str, args.keep, None))
    } else if let Some(preset_arg) = args.preset {
        let (path, pick) = parse_preset(&preset_arg)?;
        let str = read_file(&path)?;
        Some(process_preset(&str, args.keep, Some(pick)))
    } else {
        js_content
            .as_ref()
            .map(|content| process_preset(&content.content, args.keep, None))
    };

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
    let mut copied_last: Option<char> = None;

    loop {
        if first {
            first = false;
        } else {
            match read()? {
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
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                        copied_last = ctx.get_sentence().last().copied();
                    }
                    KeyCode::Char('v') if key.modifiers == KeyModifiers::CONTROL => {
                        copied_last.inspect(|c| {
                            ctx.confrim_pending();
                            ctx.push(once(*c), vec![*c], false)
                        });
                    }
                    KeyCode::Char('v') if key.modifiers == KeyModifiers::ALT => {
                        // 从系统剪切板中读取内容，将其转成字符序列，然后`ctx.push`
                        if let Ok(mut clip) = arboard::Clipboard::new() {
                            if let Ok(text) = clip.get_text() {
                                ctx.confrim_pending();
                                ctx.push(text.chars(), text.chars().collect(), false);
                            }
                        }
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
        // 若`js-action none`，则不上传数据
        // 若输入内容少于预设文本，则不上传数据
        if let Some(ref settings) = js_settings
            && !matches!(js_action, Some(JSAction::None))
            && ctx.sentence_len() >= ctx.preset_len()
        {
            eprint!("上传打字数据中...");
            let content: &JSContent = if let Some(js_content) = js_content.as_ref() {
                js_content
            } else {
                &JSContent {
                    title: "自由发文".to_string(),
                    content: gen_article(ctx.preset_len().max(ctx.sentence_len())),
                    is_local: true,
                }
            };
            match js_report(settings, ctx.measure(), content) {
                Ok(msg) => eprintln!("{msg}"),
                Err(e) => eprintln!("{e}"),
            }
        }
    }
    Ok(())
}

// 生成指定字符数量的文本，循环使用常用汉字表
fn gen_article(char_count: usize) -> String {
    // include_str! 将文件嵌入 .rodata 段，不占堆内存；函数内的临时 String 在返回后即被消费
    const HANZI: &str = include_str!("../assets/common-hanzi.txt");
    // 去除可能的末尾换行，取纯汉字内容
    let hanzi = HANZI.trim();
    hanzi.chars().cycle().take(char_count).collect()
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
#[derive(Debug, PartialEq)]
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
        let parse = |str: &str, end: bool| -> Result<(usize, usize), String> {
            // println!("parse sp: {}, end: {}", str, end);
            let defv = if end { usize::MAX } else { 0 };
            if str.is_empty() {
                return Ok((defv, defv));
            };
            let mut sp = str.split('.');
            let line = sp
                .next()
                .map_or(Ok(defv), |v| v.parse())
                .map_err(|e| format!("{e}; {error}"))?;
            let pos = sp
                .next()
                .map_or(Ok(defv), |pos| pos.parse())
                .map_err(|e| format!("{e}; {error}"))?;
            Ok((line, pos))
        };

        let (line_start, char_start) = parse(split[0], false)?;
        let (line_end, char_end) = split
            .get(1)
            .map(|sp| parse(sp, true))
            .unwrap_or(Ok((usize::MAX, usize::MAX)))?;
        Ok(Self {
            line_start: line_start.saturating_sub(1),
            char_start: char_start.saturating_sub(1),
            line_end: line_end.saturating_sub(1),
            char_end: char_end.saturating_sub(1),
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

/// 解析 preset 参数，分离文件名与可选的选择范围。
/// 格式:
///   `<文件名>`               —— 不带范围，使用整篇文件
///   `<文件名>:<范围>`        —— 按指定范围选取
/// 范围格式见 [`PickPreset::from_str`]。
///
/// 解析规则: 以最后一个冒号 `:` 作为文件名与范围的分隔符。
/// - 冒号后为空字符串时，等价于不指定范围 (整篇文件)。
/// - 冒号后能解析为范围时，按范围选取。
/// - 冒号后无法解析为范围时，视为错误返回 (避免与文件名混淆)。
fn parse_preset(s: &str) -> Result<(String, PickPreset), String> {
    if let Some((path, range)) = s.rsplit_once(':') {
        if range.is_empty() {
            return Ok((path.to_string(), PickPreset::default()));
        }
        PickPreset::from_str(range).map(|pick| (path.to_string(), pick))
    } else {
        Ok((s.to_string(), PickPreset::default()))
    }
}

fn process_preset(str: &str, keep: bool, pick: Option<PickPreset>) -> Vec<char> {
    let pick = pick.unwrap_or_default();
    let filter = |ch: &char| keep || (!ch.is_whitespace() && !ch.is_control());
    let mut ret: Vec<char> = vec![];
    for (i, line) in str.lines().enumerate() {
        if i < pick.line_start {
            continue;
        }
        if i > pick.line_end {
            break;
        }
        let chars = line.chars();
        if i == pick.line_start && i == pick.line_end {
            // 同一行内的字符范围选择，起止均包含(1-indexed 转 0-indexed)
            if pick.char_end >= pick.char_start {
                ret.extend(
                    chars
                        .skip(pick.char_start)
                        .take(pick.char_end - pick.char_start + 1)
                        .filter(filter),
                );
            }
        } else if i == pick.line_start {
            ret.extend(chars.skip(pick.char_start).filter(filter));
        } else if i == pick.line_end {
            ret.extend(chars.take(pick.char_end + 1).filter(filter));
        } else {
            ret.extend(chars.filter(filter));
        }
        if keep && i < pick.line_end {
            ret.push('\n');
        }
    }
    ret
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

#[cfg(test)]
mod tests {

    use super::parse_preset;
    use super::process_preset;
    use std::fs::File;
    use std::io::Write;

    /// 构造临时预设文本文件并断言 `--preset` 解析与选取行为。
    ///
    /// 临时文件内容:
    ///   行1: 你好
    ///   行2: 世界
    ///   行3: 测试
    ///   行4: 赛码
    #[test]
    fn test_parse_preset_with_temp_file() {
        let mut tmp = std::env::temp_dir();
        tmp.push("senime_preset_test.txt");
        {
            let mut f = File::create(&tmp).unwrap();
            writeln!(f, "你好世界，和平美好。").unwrap();
            writeln!(f, "携手共进，共享未来。").unwrap();
            writeln!(f, "美味萝卜干，一块钱两斤，管够。").unwrap();
            writeln!(f, "打字是一种浪费时间的爱好，收获甚微，我要戒掉").unwrap();
        }
        let path_str = tmp.to_string_lossy().to_string();

        let (path, pick) = parse_preset(&format!("{path_str}:2-3")).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let chars = process_preset(&content, true, Some(pick));
        let out: String = chars.iter().collect();
        assert_eq!(out, "携手共进，共享未来。\n美味萝卜干，一块钱两斤，管够。");

        // 第2行第6个字符到第3行第5个字符结束
        let (path, pick) = parse_preset(&format!("{path_str}:2.6-3.5")).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let chars = process_preset(&content, true, Some(pick));
        let out: String = chars.iter().collect();
        assert_eq!(out, "共享未来。\n美味萝卜干");

        // 从头开始到第3行第5个字符结束
        let (path, pick) = parse_preset(&format!("{path_str}:-3.5")).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let chars = process_preset(&content, true, Some(pick));
        let out: String = chars.iter().collect();
        assert_eq!(
            out,
            "你好世界，和平美好。\n携手共进，共享未来。\n美味萝卜干"
        );
        // 从第三行开始文件末尾
        let (path, pick) = parse_preset(&format!("{path_str}:3-")).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let chars = process_preset(&content, true, Some(pick));
        let out: String = chars.iter().collect();
        assert_eq!(
            out,
            "美味萝卜干，一块钱两斤，管够。\n打字是一种浪费时间的爱好，收获甚微，我要戒掉\n"
        );

        // 只选择第三行
        let (path, pick) = parse_preset(&format!("{path_str}:3-3")).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let chars = process_preset(&content, true, Some(pick));
        let out: String = chars.iter().collect();
        assert_eq!(out, "美味萝卜干，一块钱两斤，管够。");

        // 只选择第三行中的第2个字符到第5个字符
        let (path, pick) = parse_preset(&format!("{path_str}:3.2-3.5")).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let chars = process_preset(&content, true, Some(pick));
        let out: String = chars.iter().collect();
        assert_eq!(out, "味萝卜干");

        // 非正常范围，但自动修正，从第一行开始
        let (path, pick) = parse_preset(&format!("{path_str}:0-3.100")).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        let chars = process_preset(&content, true, Some(pick));
        let out: String = chars.iter().collect();
        assert_eq!(
            out,
            "你好世界，和平美好。\n携手共进，共享未来。\n美味萝卜干，一块钱两斤，管够。"
        );
    }
}
