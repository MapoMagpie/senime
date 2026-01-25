use clap::Parser;
use senime_lib::{Dict, Looker};
use std::{fs::File, io::Read, time::Instant};

/// 通过码表对文章进行平均码长和编码计算，将输出到终端，若文本太长，请将输出重定向至文件中。
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 码表文件，其结构应为: 字词<TAB>编码<TAB>权重(可选) 每行
    /// 当没有权重时则以行的顺序判断编码对应的字词的首选还是候选
    #[arg(short, long)]
    pub table: String,

    /// 文本文件，纯文本
    #[arg(short, long)]
    pub input: String,

    /// 输出颜色，如果启用则会给每段分词添加色彩，适用于终端
    #[arg(short, long)]
    pub color: bool,

    /// 输出时包含字词，将与编码拼接在一起
    #[arg(short, long)]
    pub with_text: bool,
}

fn main() {
    let args = Args::parse();
    let start = Instant::now();
    let dict = Dict::load(args.table);
    let looker = Looker::new(&dict.candidates);
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
    // 使用空格上屏的次数
    let mut use_space_times = 0;
    // 选择候选的次数
    let mut use_candidate_times = 0;
    let mut code_len = 0;
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
        .enumerate()
        .map(|(_i, seg)| {
            // 检查是否可以顶字上屏，将当前段的code与下一段的code首个字符相连，在字典中查询是否存在
            // 如果不存在表示可以顶字上屏，此方式无需检测下一段是否是标点符号
            colors_id += 1;
            code_len += seg.code.chars().count(); // 没有按char来，不严谨
            let mut str = colors[colors_id % colors.len()].to_string();
            if args.with_text {
                str += &seg.text;
            }
            str += &seg.code;
            // 需要空格
            if !seg.auto_select && seg.pos == 0 {
                use_space_times += 1;
                str += "_";
            } else if seg.pos > 0 {
                use_candidate_times += 1;
                str += select_pos_map[(seg.pos).min(select_pos_map.len() - 1)];
            }
            str
        })
        .chain(color_reset.into_iter())
        .collect::<String>();
    code_len += use_space_times;
    code_len += use_candidate_times;
    let mnpc: f32 = code_len as f32 / article.len() as f32;
    println!(
        "计算耗时: {:?}\n平均码长: {}\n空格顶码次数: {}\n进行候选次数: {}\n编码:\n{}\n",
        segments_time, mnpc, use_space_times, use_candidate_times, codes
    );
}
