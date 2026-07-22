pub mod dict;
pub mod fuzz_dict;
pub mod input_analyzer;
pub mod lookup_code;
pub mod prefix_dict;
pub mod util;

pub use dict::{DictKind, DictKindName, DictMeta};
pub use fuzz_dict::FuzzDict;
pub use input_analyzer::{
    AnalysisResult, CandidateRich, Config, InputAnalyzer, PAGE_DOWN, PAGE_UP,
};
pub use lookup_code::Looker;
pub use prefix_dict::PrefixDict;
pub use util::resolve_relative_path;

#[cfg(feature = "watcher")]
pub mod watcher;
#[cfg(feature = "watcher")]
pub use watcher::{RecommendedWatcher, spawn_watcher};

#[cfg(test)]
pub mod test_utils;
