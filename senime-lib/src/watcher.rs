//! 文件变动监控与引擎热重载。
//!
//! 监听码表/配置文件变动，防抖后重建 `InputAnalyzer` 并通过 `ArcSwap`
//! 原子替换。仅响应被监听文件的实质性变更（创建/修改/删除），忽略
//! `Access`（打开/读取）事件，避免重建时读取码表触发 `IN_OPEN` 形成无限循环。
//!
//! 当编辑器（如 helix、vim）以"写入临时文件后 rename 覆盖原文件"的方式保存时，
//! 原文件的 inode 被删除，inotify 对该 inode 的 watch 随之失效。此处通过在收到
//! `Remove` 事件后重新 `watch` 同一路径来恢复监控（新 inode 会被重新加入 watch）。

use crate::InputAnalyzer;
use crate::input_analyzer::load_input_analyzer;
use arc_swap::ArcSwap;
use log::{info, warn};
use notify::{RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

/// 重导出 `notify` 的推荐 watcher 类型，使前端无需直接依赖 `notify`。
pub type RecommendedWatcher = notify::RecommendedWatcher;

/// 由 `Arc<Mutex>` 包裹的 watcher，允许后台线程在 watch 失效后重新添加监控。
/// 调用方仅需持有该值以保持 watcher 存活。
pub type SharedWatcher = Arc<Mutex<RecommendedWatcher>>;

/// 创建一个后台文件监控器，在码表/配置文件变动时重建引擎并原子替换。
///
/// 返回的 `SharedWatcher` 必须由调用方持有以保持监控存活。
///
/// # 参数
/// - `inner`: 共享的 `ArcSwap<InputAnalyzer>`，变动后原子替换其中的引擎。
/// - `paths`: 被监听的文件路径列表（`paths[0]` 为重建时使用的主路径）。
pub fn spawn_watcher(
    inner: Arc<ArcSwap<InputAnalyzer>>,
    paths: Vec<String>,
) -> notify::Result<SharedWatcher> {
    let (tx, rx) = mpsc::channel();

    let watcher = notify::recommended_watcher(tx)?;
    let watcher = Arc::new(Mutex::new(watcher));
    let main_path = paths[0].clone();
    // 被监听的文件路径集合，用于过滤事件：只响应这些文件的变更
    let watched: HashSet<PathBuf> = paths.iter().map(PathBuf::from).collect();
    for path in &paths {
        watcher
            .lock()
            .unwrap()
            .watch(Path::new(path), RecursiveMode::NonRecursive)?;
        info!("watching {path} for hot-reload");
    }

    // 防抖线程：排空事件队列，等待平静期后重建引擎。
    let rewatcher = Arc::clone(&watcher);
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
            let touched: Vec<&PathBuf> = event
                .paths
                .iter()
                .filter(|p| watched.contains(*p))
                .collect();
            if touched.is_empty() {
                continue;
            }

            // 编辑器以 rename 覆盖原文件时，旧 inode 的 watch 会被 inotify 自动移除。
            // 收到 Remove 事件后重新 watch 同一路径，恢复对新 inode 的监控。
            if event.kind.is_remove() {
                for path in &touched {
                    if let Err(e) = rewatcher
                        .lock()
                        .unwrap()
                        .watch(path, RecursiveMode::NonRecursive)
                    {
                        warn!("re-watch {} failed: {e}", path.display());
                    }
                }
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
