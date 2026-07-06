pub mod dict;
// pub mod fuzz_search;
pub mod input_analyzer;
pub mod lookup_code;
pub mod util;

pub use dict::Dict;
pub use input_analyzer::{
    AnalysisResult, CandidateRich, Config, DictMeta, InputAnalyzer, PAGE_DOWN, PAGE_UP,
};
pub use lookup_code::Looker;
pub use util::resolve_relative_path;

#[cfg(feature = "watcher")]
pub mod watcher;
#[cfg(feature = "watcher")]
pub use watcher::{RecommendedWatcher, SharedWatcher, spawn_watcher};

#[cfg(test)]
pub mod test_utils;
