use clap::Parser;
use senime_lib::{Looker, PAGE_DOWN, input_analyzer::load_input_analyzer, lookup_code::Segment};
use serde::Serialize;
use std::{
    fs::File,
    io::Read,
    time::{Duration, Instant},
};

/// 通过码表对文章进行平均码长和编码计算，将输出到终端，若文本太长，请将输出重定向至文件中。
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 码表文件或配置文件
    /// 如果指定的是配置文件，则需要在配置中指定码表文件。
    /// 如果指定的是码表文件，其结构应为: 字词<TAB>编码<TAB>权重(可选) 每行，当没有权重时则以行的顺序判断编码对应的字词的首选还是候选。
    /// 同时，还可以直接指定二进制格式的码表文件，它是由本程序编译码表后产生的bin文件。
    #[arg(short, long, verbatim_doc_comment)]
    pub table: String,

    /// 文本文件，纯文本
    #[arg(short, long, verbatim_doc_comment)]
    pub input: String,

    /// 输出颜色，如果启用则会给每段分词添加色彩，适用于终端
    #[arg(short, long, verbatim_doc_comment)]
    pub color: bool,

    /// 输出时包含字词，将与编码拼接在一起
    #[arg(long, verbatim_doc_comment)]
    pub with_text: bool,

    /// 输出时将参与选词的空格修改为其他字符
    /// 参与选词的空格是指一个编码需要空格上屏时，会在编码后追加一个空格字符
    /// 但在输出时不直观，你可以通过此选项将空格修改为其他字符，如 "_"
    /// 不影响输入文本里原本的空格
    #[arg(long, verbatim_doc_comment)]
    pub alt_space: Option<String>,

    /// 输出json格式
    #[arg(long, verbatim_doc_comment)]
    pub json: bool,

    /// 不输出编码，也就是说，只输出性能指标
    #[arg(long, verbatim_doc_comment)]
    pub nocode: bool,
}

/// 分析结果中的一段
#[derive(Serialize)]
struct AnSegment {
    text: String,
    code: String,
    pos: u16,
    auto_select: bool,
    page_num: usize,
    select_key: char,
}

impl AnSegment {
    fn write(
        &self,
        writer: &mut impl std::fmt::Write,
        with_text: bool,
        alt_space: Option<&String>,
        colors: Option<&Vec<&str>>,
        i: usize,
    ) -> std::fmt::Result {
        if let Some(colors) = colors {
            writer.write_str(colors[i % colors.len()])?;
        }
        if with_text {
            writer.write_str(&self.text)?;
        }
        writer.write_str(&self.code)?;
        if self.page_num > 0 {
            for _ in 0..self.page_num {
                writer.write_char(PAGE_DOWN)?;
            }
        }
        if !self.auto_select && self.pos == 0 {
            if let Some(alt) = alt_space {
                writer.write_str(alt)?;
            } else {
                writer.write_str(" ")?;
            }
        } else if self.pos > 0 {
            writer.write_char(self.select_key)?;
        }
        Ok(())
    }
}

/// 分析结果统计
#[derive(Serialize)]
struct AnResult {
    /// 计算耗时(毫秒)
    elapsed_ms: u64,
    /// 平均码长
    mnpc: f32,
    /// 空格顶码次数
    use_space_times: u32,
    /// 候选选择次数
    use_candidate_times: u32,
    /// 编码总长度
    code_len: usize,
    /// 文本总字数
    text_len: usize,
    /// 分词段列表
    codes: Vec<AnSegment>,
}

fn build_result(
    segments: Vec<Segment<'_>>,
    selection_keys: Vec<char>,
    page_count: usize,
    elapsed: Duration,
    text_len: usize,
) -> AnResult {
    let mut use_space_times: u32 = 0;
    let mut use_candidate_times: u32 = 0;
    let mut code_len: usize = 0;
    let codes = segments
        .into_iter()
        .map(|seg| {
            code_len += seg.code.len();
            if !seg.auto_select && seg.pos == 0 {
                use_space_times += 1;
            } else if seg.pos > 0 {
                use_candidate_times += 1;
            }
            map_an_segment(&selection_keys, page_count, seg)
        })
        .collect();
    code_len += use_space_times as usize;
    code_len += use_candidate_times as usize;
    let mnpc = code_len as f32 / text_len as f32;
    AnResult {
        elapsed_ms: elapsed.as_millis() as u64,
        mnpc,
        use_space_times,
        use_candidate_times,
        code_len,
        text_len,
        codes,
    }
}

fn map_an_segment(selection_keys: &[char], page_count: usize, seg: Segment<'_>) -> AnSegment {
    let page_num = seg.pos as usize / page_count;
    let idx_modded = seg.pos as usize % page_count;
    // if over selection_keys, use the (idx_modded + 1) as char
    let select_key = selection_keys
        .get(idx_modded)
        .copied()
        .unwrap_or_else(|| (idx_modded as u8 + b'0') as char);
    AnSegment {
        text: seg.text.iter().collect(),
        code: seg.code.iter().collect(),
        auto_select: seg.auto_select,
        pos: seg.pos,
        page_num,
        select_key,
    }
}

fn main() {
    let args = Args::parse();

    let ia = load_input_analyzer(&args.table).expect("读取码表或配置失败");
    let dict = ia.main_dict();
    let looker = Looker::new(dict);
    let selection_keys = ia.get_selection_keys().to_vec();
    let page_count = ia.get_page_count();

    let article = {
        let mut article = String::new();
        let mut file = File::open(args.input).expect("无法读取文本文件");
        file.read_to_string(&mut article).unwrap();
        article.chars().collect::<Vec<_>>()
    };

    let start = Instant::now();
    let segments = looker.analyze(&article);
    let segments_time = start.elapsed();
    let result = build_result(
        segments,
        selection_keys,
        page_count,
        segments_time,
        article.len(),
    );

    if args.json {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        // 黑色	30	40
        // 红色	31	41
        // 绿色	32	42
        // 黄色	33	43
        // 蓝色	34	44
        // 洋红	35	45
        // 青色	36	46
        // 白色	37	47
        let color = args
            .color
            .then_some(vec!["\x1b[30;47;1m", "\x1b[34;42;1m", "\x1b[37;45;1m"]);
        let mut codes = String::new();
        if !args.nocode {
            codes.push_str("\n编码:\n");
            result.codes.into_iter().enumerate().for_each(|(i, seg)| {
                let _ = seg.write(
                    &mut codes,
                    args.with_text,
                    args.alt_space.as_ref(),
                    color.as_ref(),
                    i,
                );
            });
            if args.color {
                codes.push_str("\x1b[0m");
            }
        }
        println!(
            "计算耗时: {}ms\n总字符数: {}\n总编码数: {}\n平均码长: {}\n空格顶码次数: {}\n进行候选次数: {}{}",
            result.elapsed_ms,
            result.text_len,
            result.code_len,
            result.mnpc,
            result.use_space_times,
            result.use_candidate_times,
            codes
        );
    }
}

#[cfg(test)]
mod test {
    use senime_lib::lookup_code::Segment;

    use crate::{AnSegment, map_an_segment};
    impl AnSegment {
        fn new(
            text: &str,
            code: &str,
            pos: u16,
            auto_select: bool,
            page_num: usize,
            select_key: char,
        ) -> Self {
            AnSegment {
                text: text.to_string(),
                code: code.to_string(),
                pos,
                auto_select,
                page_num,
                select_key,
            }
        }
    }

    #[test]
    fn test_map_an_segment() {
        let selection_keys = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];
        let page_count = 4;
        let text: Vec<char> = "abc".chars().collect();
        let code: Vec<char> = "xy".chars().collect();
        let seg = Segment {
            text: &text,
            code: &code,
            pos: 9,
            auto_select: false,
            range: (0..0),
            cost: 0,
        };
        let an_seg = map_an_segment(&selection_keys, page_count, seg);
        assert_eq!(an_seg.page_num, 2);
        assert_eq!(an_seg.select_key, 'B');
    }

    #[test]
    fn test_an_segment_write() {
        let segments = vec![
            (AnSegment::new("abc", "abc", 0, true, 0, 'U'), "abc"),
            (AnSegment::new("abc", "abc", 2, false, 0, 'O'), "abcO"),
            (AnSegment::new("abc", "abc", 0, true, 3, 'U'), "abc⇟⇟⇟"),
            (AnSegment::new("abc", "abc", 1, false, 3, 'I'), "abc⇟⇟⇟I"),
        ];

        segments.into_iter().for_each(|(seg, expected)| {
            let mut str = String::new();
            let _ = seg.write(&mut str, false, None, None, 0);
            assert_eq!(expected, str);
        });
    }
}
