use std::path::PathBuf;

pub fn test_tmp_dir() -> PathBuf {
    let dir = std::env::var("CARGO_TARGET_TMPDIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(std::env::var("HOME").unwrap()).join(".cache/senime"));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn gen_test_config() -> String {
    r#"
selection_keys = ["U","I","O","H","J","K","B","N","M"]
dict = "senime-test-dict.txt"
[punctuations]
',' = ["，", ",", "……"]
'.' = ["。", ".", "……"]
'!' = ["！", "!"]
'/' = ["？", "/"]
';' = ["：", "；", ";"]
'[' = ["「", "“", "[", "【"]
']' = ["」", "”", "]", "】"]
"#
    .to_string()
}

/// 生成 senime-test-config.toml 和 senime-test-dict.txt（20万条，确定性），
/// 返回 (config_path, dict_path)。
pub fn gen_test_dict_files() -> (PathBuf, PathBuf) {
    let tmp_dir = test_tmp_dir();

    let config_path = tmp_dir.join("senime-test-config.toml");
    let dict_path = tmp_dir.join("senime-test-dict.txt");

    let config_content = gen_test_config();
    std::fs::write(&config_path, config_content).unwrap();

    let letters: [u8; 26] = *b"abcdefghijklmnopqrstuvwxyz";
    let cjk_start = 0x4E00u32;
    let cjk_count = 0x9FFF - cjk_start + 1; // 20992
    let mut buf = String::with_capacity(50_000 * 20);
    for i in 0u32..50_000 {
        let c1 = char::from_u32(cjk_start + (i % cjk_count)).unwrap();
        let c2 = char::from_u32(cjk_start + ((i / cjk_count + i + 300) % cjk_count)).unwrap();
        let l0 = letters[(i % 26) as usize] as char;
        let l1 = letters[((i / 26) % 26) as usize] as char;
        let l2 = letters[((i / 676) % 26) as usize] as char;
        let l3 = letters[((i / 17576) % 26) as usize] as char;
        buf.push(c1);
        buf.push(c2);
        buf.push('\t');
        buf.push(l0);
        buf.push(l1);
        buf.push(l2);
        buf.push(l3);
        buf.push('\t');
        buf.push_str("100\n");
    }
    std::fs::write(&dict_path, buf).unwrap();

    (config_path, dict_path)
}

pub fn remove_test_dict_files() {
    let tmp_dir = test_tmp_dir();
    let config_path = tmp_dir.join("senime-test-config.toml");
    let dict_path = tmp_dir.join("senime-test-dict.txt");
    let bin_path = tmp_dir.join("senime-test-dict.txt.bin");
    let _ = std::fs::remove_file(config_path);
    let _ = std::fs::remove_file(dict_path);
    let _ = std::fs::remove_file(bin_path);
}
