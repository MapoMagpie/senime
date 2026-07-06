//! 文件变动监控与引擎热重载。
//!
//! 监听码表/配置文件变动，防抖后重建 `InputAnalyzer` 并通过 `ArcSwap`
//! 原子替换。仅响应被监听文件的实质性变更（创建/修改/删除），忽略
//! `Access`（打开/关闭/读取）事件，避免重建时读取码表触发 `IN_OPEN` 形成无限循环。
//!
//! 监听方式为 watch 文件所在**父目录**而非文件本身：编辑器（如 helix、vim）
//! 保存时常以"写入临时文件后 rename 覆盖原文件"的原子方式替换，这会使原文件
//! inode 的 watch 失效；而目录 inode 不会被替换，watch 始终有效，新文件的
//! `CREATE`/`MODIFY` 事件都会被目录 watch 捕获，再经路径过滤命中目标文件。

use crate::input_analyzer::load_input_analyzer;
use crate::InputAnalyzer;
use arc_swap::ArcSwap;
use log::{info, warn};
use notify::{RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, mpsc};
use std::time::Duration;

/// 重导出 `notify` 的推荐 watcher 类型，使前端无需直接依赖 `notify`。
pub type RecommendedWatcher = notify::RecommendedWatcher;

/// 创建一个后台文件监控器，在码表/配置文件变动时重建引擎并原子替换。
///
/// 返回的 `RecommendedWatcher` 必须由调用方持有以保持监控存活。
///
/// # 参数
/// - `inner`: 共享的 `ArcSwap<InputAnalyzer>`，变动后原子替换其中的引擎。
/// - `paths`: 被监听的文件路径列表（`paths[0]` 为重建时使用的主路径）。
pub fn spawn_watcher(
    inner: Arc<ArcSwap<InputAnalyzer>>,
    paths: Vec<String>,
) -> notify::Result<RecommendedWatcher> {
    let (tx, rx) = mpsc::channel();

    let mut watcher = notify::recommended_watcher(tx)?;
    let main_path = paths[0].clone();
    // 被监听的文件路径集合，用于过滤事件：只响应这些文件的变更
    let watched: HashSet<PathBuf> = paths.iter().map(PathBuf::from).collect();
    // 监听各文件的父目录（去重）——目录 inode 不会被编辑器替换，watch 更稳定
    let mut watch_dirs: HashSet<PathBuf> = HashSet::new();
    for path in &paths {
        match Path::new(path).parent() {
            Some(dir) => {
                if watch_dirs.insert(dir.to_path_buf()) {
                    watcher.watch(dir, RecursiveMode::NonRecursive)?;
                    info!("watching {} for hot-reload", dir.display());
                }
            }
            None => {
                // 无父目录时退化为监听文件本身
                watcher.watch(Path::new(path), RecursiveMode::NonRecursive)?;
                info!("watching {path} for hot-reload");
            }
        }
    }

    // 防抖线程：等待平静期，排空积压事件后重建引擎。
    std::thread::spawn(move || {
        while let Ok(event) = rx.recv() {
            let event = match event {
                Ok(e) => e,
                Err(e) => {
                    warn!("watcher error: {e}");
                    continue;
                }
            };

            // 只对内容的实质性变更（创建/修改/删除）触发重建，
            // 忽略 Access（打开/关闭/读取）事件——否则 load_input_analyzer
            // 内部读取码表文件的 IN_OPEN 事件会再次触发 watcher，形成无限循环。
            if !event.kind.is_create() && !event.kind.is_modify() && !event.kind.is_remove() {
                continue;
            }

            // 只处理被监听的源文件本身的事件，忽略同目录其他文件
            // （例如 load_from_txt 写入的 .txt.bin 缓存文件）。
            if !event.paths.iter().any(|p| watched.contains(p)) {
                continue;
            }

            // 防抖：先等待一段平静期，让连续的保存事件（如编辑器分多次写入）
            // 全部落地，再排空积压事件后只重建一次。
            std::thread::sleep(Duration::from_millis(200));
            while rx.try_recv().is_ok() {}

            match load_input_analyzer(&main_path) {
                Ok(new_inner) => {
                    info!("hot-reload succeeded: reloaded from {main_path}");
                    inner.swap(Arc::new(new_inner));
                }
                Err(e) => {
                    warn!("hot-reload failed: {e}");
                }
            }
        }
    });

    Ok(watcher)
}
