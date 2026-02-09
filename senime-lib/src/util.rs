use std::path::PathBuf;

pub fn secondary_dict_path(dict_path: &str, sec_dict_path: &str) -> PathBuf {
    let path: PathBuf = sec_dict_path.into();
    if path.is_absolute() {
        path
    } else {
        let main: PathBuf = dict_path.into();
        main.parent().map(|p| p.join(path.clone())).unwrap_or(path)
    }
}
