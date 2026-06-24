use std::path::{Path, PathBuf};

/// 解析相对于基准文件所在目录的路径
pub fn resolve_relative_path(base_file: &Path, relative: &str) -> String {
    if PathBuf::from(relative).is_absolute() {
        relative.to_owned()
    } else {
        base_file
            .parent()
            .map(|p| p.join(relative))
            .and_then(|p| p.to_str().map(|s| s.to_owned()))
            .unwrap_or(relative.to_owned())
    }
}
