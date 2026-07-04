use clap::Parser;
use senime_lib::{Dict, Looker, lookup_code::Segment};
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
    #[arg(short, long, verbatim_doc_comment)]
    pub with_text: bool,

    /// 输出json格式
    #[arg(long, verbatim_doc_comment)]
    pub json: bool,
}

/// 分析结果中的一段
#[derive(Serialize)]
struct AnSegment {
    text: String,
    code: String,
    pos: u16,
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

fn build_result(segments: &[Segment], elapsed: Duration, text_len: usize) -> AnResult {
    let mut use_space_times: u32 = 0;
    let mut use_candidate_times: u32 = 0;
    let mut code_len: usize = 0;
    let codes = segments
        .iter()
        .map(|seg| {
            code_len += seg.code.len();
            if !seg.auto_select && seg.pos == 0 {
                use_space_times += 1;
            } else if seg.pos > 0 {
                use_candidate_times += 1;
            }
            AnSegment {
                text: seg.text.iter().collect(),
                code: seg.code.iter().collect(),
                pos: seg.pos,
            }
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

fn main() {
    let args = Args::parse();
    let start = Instant::now();
    let dict = Dict::try_load(&args.table).expect("读取码表失败");
    // let looker_new = Instant::now();
    let looker = Looker::new(&dict);
    // println!("初始化looker耗时: {:?}", looker_new.elapsed());
    let load_table_time = Instant::now().duration_since(start);
    println!(
        "读取码表成功，加载[{}]个条目, 耗时[{:?}]",
        dict.count(),
        load_table_time
    );
    let article = {
        let mut article = String::new();
        // .expect("无法从文本文件中读取内容，请确保该文件的编码格式为UTF-8");
        let mut file = File::open(args.input).expect("无法读取文本文件");
        file.read_to_string(&mut article).unwrap();
        article.chars().collect::<Vec<_>>()
    };
    println!("读取文本成功");

    let start = Instant::now();
    let segments = looker.analyze(&article);
    let segments_time = start.elapsed();
    let result = build_result(&segments, segments_time, article.len());

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
        let (colors, color_reset) = if args.color {
            (
                vec!["\x1b[30;47;1m", "\x1b[31;42;1m", "\x1b[37;40;1m"],
                vec!["\x1b[0m".to_string()],
            )
        } else {
            (vec![""], vec![])
        };
        let select_pos_map = vec!["¹", "²", "³", "⁴", "⁶", "⁶", "⁷", "⁸", "⁹", "^"];
        let mut colors_id: usize = 0;
        let codes = segments
            .iter()
            .map(|seg| {
                colors_id += 1;
                let mut str = colors[colors_id % colors.len()].to_string();
                if args.with_text {
                    str.extend(seg.text);
                }
                str.extend(seg.code);
                if !seg.auto_select && seg.pos == 0 {
                    str += "_";
                } else if seg.pos > 0 {
                    str += select_pos_map[(seg.pos as usize).min(select_pos_map.len() - 1)];
                }
                str
            })
            .chain(color_reset)
            .collect::<String>();
        println!(
            "计算耗时: {:?}\n平均码长: {}\n空格顶码次数: {}\n进行候选次数: {}\n编码:\n{}\n",
            segments_time, result.mnpc, result.use_space_times, result.use_candidate_times, codes
        );
    }
}
