// 生成指定字符数量的文本，循环使用常用汉字表
pub fn gen_article(char_count: usize) -> String {
    // include_str! 将文件嵌入 .rodata 段，不占堆内存；函数内的临时 String 在返回后即被消费
    const HANZI: &str = include_str!("../assets/common-hanzi.txt");
    // 去除可能的末尾换行，取纯汉字内容
    let hanzi = HANZI.trim();
    hanzi.chars().cycle().take(char_count).collect()
}

// 生成指定字符数量的文本，通过指定在`../.assets`目录下的文件
// 这些文件不是内嵌的，如果不存在则`panic`
#[allow(unused)]
pub fn gen_article_by_file(char_count: usize, file: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = format!("{manifest_dir}/assets/{}", file);
    println!("path: {path}");
    let content = std::fs::read_to_string(path).expect("Failed to read file");
    content.chars().cycle().take(char_count).collect()
}
