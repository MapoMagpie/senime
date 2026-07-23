use std::path::Path;
use std::sync::Arc;

use clap::Parser;
use dashmap::DashMap;
use log::LevelFilter;
use ropey::Rope;
use senime_lib::input_analyzer::load_input_analyzer;
use senime_lib::{AnalysisResult, InputAnalyzer, resolve_relative_path, spawn_watcher};
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{LanguageServer, LspService, Server};

#[derive(Debug)]
struct State {
    completion: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct Config {
    // 触发补全的字符，如 [a-z, A-Z, 空格]
    trigger_characters: String,
    // 行首注释，如 [//, --, #]
    comment_prefixes: Vec<String>,
    // 编码模式：为 true 时仅在特定上下文（汉字后、特殊前缀后、注释行）触发补全；
    // 为 false 时只要满足基础条件即触发补全。
    coding_mode: bool,
    // 特殊识别前缀，光标前出现该前缀 + ASCII 编码字符时触发补全。
    special_prefix: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            trigger_characters: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ,."
                .to_string(),
            comment_prefixes: vec![],
            coding_mode: true,
            special_prefix: "@@".to_string(),
        }
    }
}

/// LSP position encoding，由客户端在 initialize 时协商确定。
/// 决定 `Position.character` 的含义：行内 UTF-8 字节偏移 / UTF-16 码元偏移 / UTF-32 码点偏移。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionEncoding {
    Utf8,
    Utf16,
    Utf32,
}

impl PositionEncoding {
    fn from_lsp(encoding: &PositionEncodingKind) -> Option<Self> {
        match encoding.as_str() {
            "utf-8" => Some(Self::Utf8),
            "utf-16" => Some(Self::Utf16),
            "utf-32" => Some(Self::Utf32),
            _ => None,
        }
    }

    fn to_lsp(self) -> PositionEncodingKind {
        match self {
            Self::Utf8 => PositionEncodingKind::UTF8,
            Self::Utf16 => PositionEncodingKind::UTF16,
            Self::Utf32 => PositionEncodingKind::UTF32,
        }
    }
}

#[derive(Debug)]
struct Backend {
    // client: Client,
    engine: Arc<std::sync::RwLock<InputAnalyzer>>,
    doc_map: DashMap<String, Rope>,
    state: RwLock<State>,
    config: RwLock<Config>,
    encoding: RwLock<PositionEncoding>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        log::info!("initialize: {:?}", params.initialization_options);
        if let Some(value) = params.initialization_options {
            match serde_json::from_value::<Config>(value) {
                Ok(new_config) => {
                    let mut config = self.config.write().await;
                    *config = new_config;
                    log::info!("update config: {:?}", config);
                }
                Err(err) => {
                    log::info!("deserialize config err: {:?}", err);
                }
            }
        };

        // 协商 position encoding：客户端通过 general.position_encodings 声明支持的编码列表。
        // 服务器选择一个返回在 capabilities.position_encoding 中。
        // 我们偏好 UTF-32（与 ropey 的 char 索引原生对应），次选 UTF-8，再次 UTF-16。
        let client_encodings = params
            .capabilities
            .general
            .as_ref()
            .and_then(|g| g.position_encodings.as_deref())
            .unwrap_or(&[]);

        // 设置协商后的编码
        *self.encoding.write().await = client_encodings
            .iter()
            .find_map(PositionEncoding::from_lsp)
            .unwrap_or(PositionEncoding::Utf16); // 客户端未声明则默认 UTF-16（LSP 规范）
        let encoding = *self.encoding.read().await;

        let trigger_characters = {
            let config = self.config.read().await;
            config.trigger_characters.clone()
        };
        return Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "senimels".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            capabilities: ServerCapabilities {
                position_encoding: Some(encoding.to_lsp()),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec![
                        "senime_completion_enable".to_string(),
                        "senime_completion_disable".to_string(),
                    ],
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                }),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(
                        trigger_characters.chars().map(|c| c.to_string()).collect(),
                    ),
                    ..CompletionOptions::default()
                }),
                ..ServerCapabilities::default()
            },
        });
    }

    // 补全触发条件（满足"基础条件"后，若 coding_mode 为 true 还需命中以下任一）:
    //   1. 编码段前紧邻汉字或全角标点（非 ASCII 字符）→ 段从该字符之后开始
    //   2. 编码段前紧邻特殊识别前缀（默认 "@@"）→ 分析从前缀之后开始，但 text_edit 覆盖前缀
    //   3. 当前行是注释行（行首匹配 comment_prefixes 之一）→ 段从基础条件范围开始
    // 若 coding_mode 为 false，则仅需满足"基础条件"即触发补全。
    //
    // 基础条件: 从光标向前回溯，收集连续的 ASCII 非控制字符作为编码段；
    //           遇到连续两个空格或非 ASCII 字符即停止。
    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        log::info!("completion start");
        // 全局开关
        if !self.state.read().await.completion {
            return Ok(None);
        }
        // log::info!("position: {:?}", params.text_document_position.position);
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let rope = match self.doc_map.get(uri.as_str()) {
            Some(rope) => rope,
            None => return Ok(None),
        };

        let line_chars: Vec<char> = rope.line(position.line as usize).chars().collect();
        log::info!("line chars: {:?}", line_chars);
        let config = self.config.read().await;

        // 将 LSP position.character 转换为行内字符索引（考虑协商后的编码）
        let encoding = *self.encoding.read().await;
        let end = {
            let line = position.line as usize;
            let col = position.character as usize;
            match encoding {
                PositionEncoding::Utf32 => col,
                PositionEncoding::Utf8 => rope
                    .line(line)
                    .try_byte_to_char(col)
                    .unwrap_or(line_chars.len()),
                PositionEncoding::Utf16 => rope
                    .line(line)
                    .try_utf16_cu_to_char(col)
                    .unwrap_or(line_chars.len()),
            }
            .min(line_chars.len())
        };

        // 注释行匹配：确定回溯下界 start_at（仅注释前缀之后的内容可参与编码段）
        let comment_boundary = match_comment_prefix(&line_chars, &config.comment_prefixes);
        let start_at = comment_boundary.unwrap_or(0);

        // 基础条件: 回溯定位编码词边界
        let Boundary {
            start,
            reduce_ws,
            cjk_before,
        } = locate_code_boundary(&line_chars, end, start_at);
        if start >= end {
            // log::info!("completion empty");
            return Ok(None);
        }

        let mut edit_start = start;
        let mut analysis_start = if reduce_ws { start + 1 } else { start };

        // coding_mode 为 true 时，必须命中某一触发条件
        if config.coding_mode {
            let mut triggered = false;
            // 条件2: 特殊识别前缀（覆盖 analysis 范围，text_edit 会连同前缀一起替换）
            if let Some(idx) = find_special_prefix(&line_chars, end, &config.special_prefix) {
                let plen = config.special_prefix.chars().count();
                edit_start = idx;
                analysis_start = idx + plen;
                triggered = true;
            } else if cjk_before {
                // 条件1: 编码段前紧邻汉字/全角标点
                triggered = true;
            } else if comment_boundary.is_some() {
                // 条件3: 注释行
                triggered = true;
            }
            if !triggered {
                // log::info!("completion not triggered (coding_mode)");
                return Ok(None);
            }
        }

        let analysis_chars = &line_chars[analysis_start..end];
        let AnalysisResult {
            segments,
            pending: _,
            candidates,
        } = { self.engine.read().unwrap().analyze(analysis_chars) };
        let sentence: String = segments.into_iter().map(|seg| seg.0).collect();
        // filter_text: 编辑器据此过滤补全项（如 helix 用光标前文本做模糊匹配评分）。
        // 从 edit_start 向前收集连续字母字符，兼顾编辑器匹配与性能。
        let filter_text: String = line_chars[locate_filter_start(&line_chars, edit_start)..end]
            .iter()
            .collect();
        if sentence.trim().is_empty() {
            return Ok(None);
        }

        let line_idx = position.line;
        let start_col = char_to_lsp_col(&rope, line_idx as usize, edit_start, encoding);
        let end_col = char_to_lsp_col(&rope, line_idx as usize, end, encoding);

        let sentence_item = CompletionItem {
            label: sentence.clone(),
            preselect: Some(true),
            kind: Some(CompletionItemKind::TEXT),
            filter_text: Some(filter_text.clone()),
            sort_text: Some("0".to_string()),
            text_edit: Some(CompletionTextEdit::Edit(TextEdit::new(
                make_range(line_idx, start_col, end_col),
                sentence.clone(),
            ))),
            ..Default::default()
        };
        let cand_items: Vec<CompletionItem> = match candidates {
            Some(cands) if !cands.is_empty() => cands
                .into_iter()
                .enumerate()
                .map(|(i, c)| CompletionItem {
                    label: format!("[{}]: {} {}", c.select_key, c.text, c.code),
                    preselect: Some(false),
                    kind: Some(CompletionItemKind::TEXT),
                    filter_text: Some(filter_text.clone()),
                    sort_text: Some((i + 1).to_string()),
                    text_edit: Some(CompletionTextEdit::Edit(TextEdit::new(
                        make_range(line_idx, start_col, end_col),
                        c.text,
                    ))),
                    ..Default::default()
                })
                .collect(),
            _ => vec![],
        };
        log::info!(
            "completion result: {}, candidates: {}, analyzer_chars: [{}]",
            sentence,
            cand_items.len(),
            analysis_chars.iter().collect::<String>(),
        );
        Ok(Some(CompletionResponse::List(CompletionList {
            is_incomplete: true,
            items: [vec![sentence_item], cand_items].concat(),
        })))
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let url = params.text_document.uri.into();
        let rope = Rope::from(params.text_document.text);
        self.doc_map.insert(url, rope);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        log::info!("did_change");
        let url = params.text_document.uri;
        if let Some(mut rope) = self.doc_map.get_mut(url.as_str()) {
            let encoding = *self.encoding.read().await;
            for change in params.content_changes {
                // log::info!("did change: {:?}", change);
                let TextDocumentContentChangeEvent { range, text, .. } = change;
                match range {
                    // incremental change
                    Some(Range { start, end }) => {
                        let s = char_index(&rope, start, encoding);
                        let e = char_index(&rope, end, encoding);
                        match (s, e) {
                            (Ok(s), Ok(e)) => {
                                rope.remove(s..e);
                                rope.insert(s, &text);
                                // log::info!("\n{}", rope.line(start.line as usize));
                            }
                            (Err(err), _) | (_, Err(err)) => {
                                log::error!("did change: {err}");
                                let mut guard = self.state.write().await;
                                guard.completion = false;
                            }
                        }
                    }
                    // full content change
                    None => {
                        *rope = Rope::from(text);
                    }
                }
            }
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.doc_map.remove(params.text_document.uri.as_str());
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn code_action(&self, _params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        log::info!("code_action");
        let st = self.state.read().await;
        let command = if st.completion {
            Command::new(
                "Disable Senime Completion".into(),
                "senime_completion_disable".into(),
                None,
            )
        } else {
            Command::new(
                "Enable Senime Completion".into(),
                "senime_completion_enable".into(),
                None,
            )
        };
        Ok(Some(vec![command.into()]))
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        log::info!("execute_command: {}", params.command);
        let mut st = self.state.write().await;
        match params.command.as_str() {
            "senime_completion_disable" => {
                st.completion = false;
            }
            "senime_completion_enable" => {
                st.completion = true;
            }
            _ => {}
        }
        Ok(None)
    }
}

/// 将 LSP `Position` 转换为 ropey 的绝对字符索引。
///
/// `character` 的语义由 `encoding` 决定：
/// - UTF-8:  行内字节偏移
/// - UTF-16: 行内 UTF-16 码元偏移
/// - UTF-32: 行内 Unicode 码点偏移（与 ropey 的 char 索引一致）
pub fn char_index(rope: &Rope, pos: Position, encoding: PositionEncoding) -> ropey::Result<usize> {
    let (line, col) = (pos.line as usize, pos.character as usize);
    let len_lines = rope.len_lines();
    // position is at the end of rope
    if line == len_lines && col == 0 {
        return Ok(rope.len_chars());
    }
    // line beyond the document, or past-end line with non-zero column
    if line > len_lines || (line == len_lines && col > 0) {
        return Err(ropey::Error::CharIndexOutOfBounds(col, 0));
    }
    let line_start = rope.try_line_to_char(line)?;
    let col_in_chars = match encoding {
        PositionEncoding::Utf32 => col,
        PositionEncoding::Utf8 => rope.line(line).try_byte_to_char(col)?,
        PositionEncoding::Utf16 => rope.line(line).try_utf16_cu_to_char(col)?,
    };
    Ok(line_start + col_in_chars)
}

/// 回溯定位编码词边界的结果。
struct Boundary {
    start: usize,
    /// 是否去掉 `start` 处的空格（编码段前为汉字且仅隔一个空格时）
    reduce_ws: bool,
    /// 编码段前是否紧邻非 ASCII 字符（汉字、全角标点等）
    cjk_before: bool,
}

/// 基础条件：从 `end` 向前回溯，收集连续的 ASCII 非控制字符作为编码段。
/// 遇到连续两个空格或非 ASCII 字符即停止。`start_at` 为回溯下界（注释前缀之后）。
fn locate_code_boundary(line_chars: &[char], end: usize, start_at: usize) -> Boundary {
    let mut start = end;
    let mut reduce_ws = false;
    let mut cjk_before = false;
    for i in (0..end).rev() {
        let c = line_chars[i];
        if start >= start_at && c.is_ascii() && !c.is_control() {
            // 连续两个空格作为分隔
            if i > 0 && c.is_ascii_whitespace() && line_chars[i - 1].is_ascii_whitespace() {
                break;
            }
            start = i;
        } else {
            // 非编码字符：判断其前是否为汉字/全角标点等（非 ASCII）
            cjk_before = !c.is_ascii();
            reduce_ws = line_chars[start].is_whitespace()
                && !c.is_ascii_alphanumeric()
                && c.is_alphanumeric();
            break;
        }
    }
    Boundary {
        start,
        reduce_ws,
        cjk_before,
    }
}

/// 匹配行首注释前缀，返回注释前缀之后的首个字符索引（回溯下界）。
/// 未配置注释前缀或当前行不匹配时返回 `None`。
fn match_comment_prefix(line_chars: &[char], comment_prefixes: &[String]) -> Option<usize> {
    if comment_prefixes.is_empty() {
        return None;
    }
    let prefix_start = line_chars
        .iter()
        .position(|c| !c.is_ascii_whitespace())
        .unwrap_or(0);
    comment_prefixes.iter().find_map(|prefix| {
        let pre: Vec<char> = prefix.chars().collect();
        line_chars[prefix_start..]
            .starts_with(pre.as_slice())
            .then_some(prefix_start + pre.len() + 1)
    })
}

/// 在光标前的文本中查找特殊识别前缀（如 "@@"），要求前缀到光标之间为有效编码段
/// （ASCII 非控制字符、非空、无连续两个空格）。返回最近一次前缀出现的起始索引。
fn find_special_prefix(line_chars: &[char], end: usize, prefix: &str) -> Option<usize> {
    if prefix.is_empty() {
        return None;
    }
    let pre: Vec<char> = prefix.chars().collect();
    let plen = pre.len();
    if end < plen {
        return None;
    }
    (0..=end - plen).rev().find(|&idx| {
        if line_chars[idx..idx + plen] != pre[..] {
            return false;
        }
        let seg = &line_chars[idx + plen..end];
        !seg.is_empty()
            && seg.iter().all(|c| c.is_ascii() && !c.is_control())
            && !seg
                .windows(2)
                .any(|w| w[0].is_ascii_whitespace() && w[1].is_ascii_whitespace())
    })
}

/// 从 `start` 向前收集连续字母字符，作为 filter_text 的起点。
fn locate_filter_start(line_chars: &[char], start: usize) -> usize {
    let mut filter_text_start = start;
    for i in (0..start).rev() {
        if line_chars[i].is_alphabetic() {
            filter_text_start = i;
        } else {
            break;
        }
    }
    filter_text_start
}

/// 将行内字符索引转换为 LSP `character` 列值（按协商后的编码）。
fn char_to_lsp_col(rope: &Rope, line: usize, char_idx: usize, encoding: PositionEncoding) -> u32 {
    match encoding {
        PositionEncoding::Utf32 => char_idx as u32,
        PositionEncoding::Utf8 => rope.line(line).char_to_byte(char_idx) as u32,
        PositionEncoding::Utf16 => rope.line(line).char_to_utf16_cu(char_idx) as u32,
    }
}

fn make_range(line: u32, start_col: u32, end_col: u32) -> Range {
    Range {
        start: Position {
            line,
            character: start_col,
        },
        end: Position {
            line,
            character: end_col,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_to_offset_utf32() {
        let rope = Rope::from("Hello\nWorld");
        assert_eq!(2, rope.len_lines());
        let enc = PositionEncoding::Utf32;
        assert!(matches!(char_index(&rope, Position::new(0, 0), enc), Ok(0)));
        assert!(matches!(char_index(&rope, Position::new(0, 5), enc), Ok(5)));
        assert!(matches!(char_index(&rope, Position::new(0, 6), enc), Ok(6))); // '\n'
        assert!(matches!(char_index(&rope, Position::new(1, 0), enc), Ok(6)));
        assert!(matches!(
            char_index(&rope, Position::new(1, 5), enc),
            Ok(11)
        ));
        assert!(matches!(
            char_index(&rope, Position::new(2, 0), enc),
            Ok(11)
        ));
        assert!(matches!(
            char_index(&rope, Position::new(2, 1), enc),
            Err(_)
        ));
        let rope = Rope::from("你好\na世界");
        assert!(matches!(char_index(&rope, Position::new(0, 0), enc), Ok(0)));
        assert!(matches!(char_index(&rope, Position::new(0, 2), enc), Ok(2)));
        assert!(matches!(char_index(&rope, Position::new(1, 0), enc), Ok(3)));
        assert!(matches!(char_index(&rope, Position::new(1, 1), enc), Ok(4)));
        assert!(matches!(char_index(&rope, Position::new(1, 3), enc), Ok(6)));
    }

    #[test]
    fn test_position_to_offset_utf16() {
        // "你好吗💘" 中 💘(U+1F498) 占 2 个 UTF-16 码元
        let rope = Rope::from("你好吗💘");
        let enc = PositionEncoding::Utf16;
        // 4 个字符: 你(U+4F60)=1CU, 好(U+597D)=1CU, 吗(U+5417)=1CU, 💘(U+1F498)=2CU
        // 行内 UTF-16 偏移: 0→'你', 1→'好', 2→'吗', 3→'💘'(高位), 4→'💘'(低位), 5→EOL
        assert!(matches!(char_index(&rope, Position::new(0, 0), enc), Ok(0)));
        assert!(matches!(char_index(&rope, Position::new(0, 1), enc), Ok(1)));
        assert!(matches!(char_index(&rope, Position::new(0, 2), enc), Ok(2)));
        // UTF-16 offset 3 和 4 都在 💘 内部，应映射到 char index 3
        assert!(matches!(char_index(&rope, Position::new(0, 3), enc), Ok(3)));
        assert!(matches!(char_index(&rope, Position::new(0, 4), enc), Ok(3)));
        // UTF-16 offset 5 = 行尾，映射到 char index 4 (end of line)
        assert!(matches!(char_index(&rope, Position::new(0, 5), enc), Ok(4)));
    }

    #[test]
    fn test_position_to_offset_utf8() {
        // "你好吗💘" 中 💘 占 4 个 UTF-8 字节
        let rope = Rope::from("你好吗💘");
        let enc = PositionEncoding::Utf8;
        // 字节偏移: 你[0..3], 好[3..6], 吗[6..9], 💘[9..13]
        assert!(matches!(char_index(&rope, Position::new(0, 0), enc), Ok(0)));
        assert!(matches!(char_index(&rope, Position::new(0, 3), enc), Ok(1)));
        assert!(matches!(char_index(&rope, Position::new(0, 6), enc), Ok(2)));
        assert!(matches!(char_index(&rope, Position::new(0, 9), enc), Ok(3)));
        // byte offset 10,11,12 在 💘 内部，映射到 char index 3
        assert!(matches!(
            char_index(&rope, Position::new(0, 10), enc),
            Ok(3)
        ));
        // byte offset 13 = 行尾，映射到 char index 4
        assert!(matches!(
            char_index(&rope, Position::new(0, 13), enc),
            Ok(4)
        ));
    }

    #[test]
    fn test_char_is_whitespace() {
        let char = ' ';
        assert!(!char.is_alphabetic());
        let char = '，';
        assert!(!char.is_alphabetic());
        let char = '你';
        assert!(char.is_alphabetic());
        let char = '好';
        assert!(char.is_alphabetic());
        let char = '\n';
        assert!(char.is_ascii_whitespace());
    }

    #[test]
    fn test_prefix_match() {
        let prefix = "..";
        let line: Vec<char> = "..asdald".chars().collect();
        assert!(prefix.chars().zip(line.iter()).all(|(a, b)| a == *b));
    }

    #[test]
    fn test_locate_code_boundary_plain() {
        // 编码段为 "abc"
        let line: Vec<char> = "abc".chars().collect();
        let b = locate_code_boundary(&line, 3, 0);
        assert_eq!(b.start, 0);
        assert!(!b.reduce_ws);
        assert!(!b.cjk_before);
    }

    #[test]
    fn test_locate_code_boundary_after_cjk() {
        // "你好 abc" -> end=6, 编码段 " abc"，前面是汉字
        let line: Vec<char> = "你好 abc".chars().collect();
        let b = locate_code_boundary(&line, 6, 0);
        assert_eq!(b.start, 2);
        assert!(b.reduce_ws);
        assert!(b.cjk_before);
    }

    #[test]
    fn test_locate_code_boundary_double_space() {
        // "a  bc" -> 连续两空格截断，编码段仅 "bc"
        let line: Vec<char> = "a  bc".chars().collect();
        let b = locate_code_boundary(&line, 5, 0);
        assert_eq!(b.start, 3);
    }

    #[test]
    fn test_match_comment_prefix_hit() {
        let line: Vec<char> = "// hello".chars().collect();
        assert_eq!(match_comment_prefix(&line, &["//".to_string()]), Some(3));
    }

    #[test]
    fn test_match_comment_prefix_miss() {
        let line: Vec<char> = "let x = 1;".chars().collect();
        assert_eq!(
            match_comment_prefix(&line, &["//".to_string(), "--".to_string()]),
            None
        );
    }

    #[test]
    fn test_match_comment_prefix_empty_config() {
        let line: Vec<char> = "anything".chars().collect();
        assert_eq!(match_comment_prefix(&line, &[]), None);
    }

    #[test]
    fn test_find_special_prefix_basic() {
        // "@@abc" -> 找到 @@ 于 idx 0
        let line: Vec<char> = "@@abc".chars().collect();
        assert_eq!(find_special_prefix(&line, 5, "@@"), Some(0));
    }

    #[test]
    fn test_find_special_prefix_none() {
        let line: Vec<char> = "abc".chars().collect();
        assert_eq!(find_special_prefix(&line, 3, "@@"), None);
    }

    #[test]
    fn test_find_special_prefix_double_space_in_seg() {
        // "@@a  b" -> 前缀之后有连续两空格，不合格
        let line: Vec<char> = "@@a  b".chars().collect();
        assert_eq!(find_special_prefix(&line, 6, "@@"), None);
    }

    #[test]
    fn test_find_special_prefix_after_cjk() {
        // "你@@abc" -> idx 1
        let line: Vec<char> = "你@@abc".chars().collect();
        assert_eq!(find_special_prefix(&line, 6, "@@"), Some(1));
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 码表文件或配置文件
    /// 如果指定的是配置文件，则需要在配置中指定码表文件。
    /// 如果指定的是码表文件，其结构应为: 字词<TAB>编码<TAB>权重(可选) 每行，当没有权重时则以行的顺序判断编码对应的字词的首选还是候选。
    /// 同时，还可以直接指定二进制格式的码表文件，它是由本程序编译码表后产生的bin文件。
    /// 如果未指定，则默认查找 $XDG_CONFIG_HOME/senime/config.toml。
    #[arg(short, long, verbatim_doc_comment)]
    pub table: Option<String>,
}

fn get_default_table() -> std::result::Result<String, std::io::Error> {
    use std::io::{Error, ErrorKind};
    dirs::config_dir()
        .map(|dir| dir.join("senime").join("config.toml"))
        .filter(|path| path.is_file())
        .map(|path| path.to_str().unwrap().to_owned())
        .ok_or(Error::new(
            ErrorKind::NotFound,
            "未指定 --table 参数，且无法找到默认配置文件路径",
        ))
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    simple_logging::log_to_file("/home/mapomagpie/.cache/helix/timls.log", LevelFilter::Info)
        .unwrap();
    log::info!("start");

    let table_path: String = match args.table {
        Some(t) => t,
        None => get_default_table().unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        }),
    };

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let engine = load_input_analyzer(&table_path).expect("failed to load dict");
    let mut watch_paths = vec![table_path.clone()];
    for dict_meta in engine.dict_metas() {
        watch_paths.push(resolve_relative_path(
            Path::new(&table_path),
            &dict_meta.path,
        ));
    }
    watch_paths.dedup();
    let engine = Arc::new(std::sync::RwLock::new(engine));

    // Spawn file watcher — failure is non-fatal.
    let watcher_engine = engine.clone();
    let main_path = table_path.clone();
    let _watcher = spawn_watcher(
        move || {
            // 1. 先从锁中取出旧引擎，用 Default 占位，释放内存
            let old = {
                match watcher_engine.write() {
                    Ok(mut guard) => std::mem::take(&mut *guard),
                    Err(e) => {
                        log::warn!("[senime] hot-reload lock poisoned: {e}");
                        return;
                    }
                }
            };
            drop(old);
            // 2. 加载新引擎（旧引擎已释放，内存峰值可控）
            match load_input_analyzer(&main_path) {
                Ok(new_ia) => {
                    match watcher_engine.write() {
                        Ok(mut guard) => {
                            *guard = new_ia;
                            log::info!("[senime] hot-reload succeeded")
                        }
                        Err(e) => log::warn!("[senime] hot-reload lock poisoned: {e}"),
                    };
                }
                Err(e) => log::warn!("[senime] hot-reload failed: {e}"),
            }
        },
        watch_paths,
    )
    .map_err(|e| log::warn!("[senime] file watcher init failed: {e}"))
    .ok();

    let doc_map = DashMap::default();
    let state = RwLock::new(State { completion: true });
    let config = RwLock::new(Config::default());
    let (service, socket) = LspService::new(|_client| Backend {
        engine,
        doc_map,
        state,
        config,
        encoding: RwLock::new(PositionEncoding::Utf16), // 初始默认 UTF-16，initialize 时协商
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
