use std::path::PathBuf;

use clap::Parser;
use dashmap::DashMap;
use log::LevelFilter;
use ropey::Rope;
use senime_lib::{AnalysisResult, Dict, InputAnalyzer, secondary_dict_path};
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
    // 忽略INVOKED, CompletionTriggerKind::INVOKED 会在双引号""时触发，即使该行不是注释
    ignore_invoked: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            trigger_characters: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ,."
                .to_string(),
            comment_prefixes: vec![],
            ignore_invoked: false,
        }
    }
}

#[derive(Debug)]
struct Backend {
    // client: Client,
    engine: InputAnalyzer,
    doc_map: DashMap<String, Rope>,
    state: RwLock<State>,
    config: RwLock<Config>,
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

    // 为了避免影响正常编码，需要让中文补全只在注释或字符串中生效
    // 最好的办法是使用tree-sitter等语法解析器来判断光标位置的上下文，这能精确识别当前是在注释中还是在字符串中，不过暂时不采用
    // 目前只简单的支持单行注释，当前行的前缀与config.comment_prefixes中的元素匹配的话，则在此行启用补全
    // 如果config.comment_prefixes为空（即默认）则启用补全
    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        log::info!("completion start");
        let st = self.state.read().await;
        if !st.completion {
            return Ok(None);
        }
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let rope = match self.doc_map.get(uri.as_str()) {
            Some(rope) => rope,
            None => return Ok(None),
        };

        let line_chars: Vec<char> = rope.line(position.line as usize).chars().collect();
        let config = self.config.read().await;
        let is_invoked = if config.ignore_invoked {
            false
        } else {
            params.context.as_ref().map_or(false, |ctx| {
                ctx.trigger_kind == CompletionTriggerKind::INVOKED
            })
        };
        let mut start_at = 0;
        // 如果触发类型是自动的（非手动），则继续判断行首是否有注释前缀
        // log::info!("completion is invoked: {}", is_invoked,);
        if !is_invoked {
            if !config.comment_prefixes.is_empty() {
                let prefix_start = line_chars
                    .iter()
                    .position(|c| !c.is_ascii_whitespace())
                    .unwrap_or(0);
                let matched = config.comment_prefixes.iter().find_map(|prefix| {
                    prefix
                        .chars()
                        .zip(line_chars[prefix_start..].iter())
                        .all(|(a, b)| a == *b)
                        .then_some(prefix.chars().count())
                });
                match matched {
                    Some(len) => {
                        start_at = prefix_start + len + 1;
                    }
                    None => {
                        log::info!("comment prefix not matched, disable completion");
                        return Ok(None);
                    }
                }
            }
        }
        let end = position.character as usize;
        let mut start = end;
        for i in (0..end).rev() {
            let char = line_chars[i];
            // log::info!("completion char {}", char);
            if start >= start_at && char.is_ascii() && !char.is_control() {
                // 连续空格
                if i > 0 && char.is_ascii_whitespace() && line_chars[i - 1].is_ascii_whitespace() {
                    break;
                }
                start = i;
            } else {
                break;
            }
        }
        if start >= end {
            log::info!("completion empty");
            return Ok(None);
            // } else {
            //     log::info!("completion chars: {:?}", &line_chars[start..end]);
        }
        let mut filter_text_start = start;
        for i in (0..start).rev() {
            let char = line_chars[i];
            if char.is_alphabetic() {
                filter_text_start = i;
            } else {
                break;
            }
        }
        let AnalysisResult {
            segments,
            candidates,
        } = self.engine.analyze(&line_chars[start..end]);
        let (sentence, _) = segments.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
        // 编辑器在收到补全后，全根据fiter_text进行过滤，比如helix会用[向前到后一个字..当前光标]这个范围的字符去搜索，如果搜索的分数太低就会丢弃
        // 所谓的字，就是英文字母、汉字、等其他非标点符号的字
        // 设置fiter_text最简单的方式是从当前行的首位开始也就是0，到当前光标的位置
        // 不过如果一行太长的话，可能有性能问题，更好的方式是从start开始，再向前找到字的位置。
        let filter_text: String = line_chars[filter_text_start..end].iter().collect();
        // log::info!(
        //     "completion word: [{}-{}], filter_text: {}",
        //     start,
        //     end,
        //     filter_text
        // );
        let sentence = sentence.join("");
        if sentence.trim().is_empty() {
            return Ok(None);
        }
        let sentence_item = CompletionItem {
            label: sentence.clone(),
            preselect: Some(true),
            kind: Some(CompletionItemKind::TEXT),
            filter_text: Some(filter_text.clone()),
            sort_text: Some("0".to_string()),
            text_edit: Some(CompletionTextEdit::Edit(TextEdit::new(
                Range {
                    start: Position {
                        line: position.line,
                        character: start as u32,
                    },
                    end: Position {
                        line: position.line,
                        character: end as u32,
                    },
                },
                sentence.clone(),
            ))),
            // command: todo!(),
            // commit_characters: todo!(),
            ..Default::default()
        };
        let cand_items: Vec<CompletionItem> = if let Some(cands) = candidates
            && cands.len() > 1
        {
            cands
                .into_iter()
                .enumerate()
                .map(|(i, c)| CompletionItem {
                    label: format!("[{}]: {} {}", c.select_key, c.text, c.code),
                    preselect: Some(false),
                    kind: Some(CompletionItemKind::TEXT),
                    filter_text: Some(filter_text.clone()),
                    sort_text: Some((i + 1).to_string()),
                    text_edit: Some(CompletionTextEdit::Edit(TextEdit::new(
                        Range {
                            start: Position {
                                line: position.line,
                                character: start as u32,
                            },
                            end: Position {
                                line: position.line,
                                character: end as u32,
                            },
                        },
                        c.text,
                    ))),
                    ..Default::default()
                })
                .collect()
        } else {
            vec![]
        };
        log::info!(
            "completion result: {}, candidates: {}",
            sentence,
            cand_items.len()
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
        // log::info!("did_change");
        let url = params.text_document.uri;
        if let Some(mut rope) = self.doc_map.get_mut(url.as_str()) {
            for change in params.content_changes {
                let TextDocumentContentChangeEvent { range, text, .. } = change;
                match range {
                    // incremental change
                    Some(Range { start, end }) => {
                        let s = char_index(&rope, start);
                        let e = char_index(&rope, end);
                        if let (Some(s), Some(e)) = (s, e) {
                            rope.remove(s..e);
                            rope.insert(s, &text);
                            // log::info!(
                            //     "did_change now rope: {}-{} {}-{}",
                            //     start.character,
                            //     end.character,
                            //     s,
                            //     e
                            // );
                            // log::info!("\n{}", rope.to_string());
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

pub fn char_index(rope: &Rope, pos: Position) -> Option<usize> {
    let (line, col) = (pos.line as usize, pos.character as usize);
    // position is at the end of rope
    if line == rope.len_lines() && col == 0 {
        return Some(rope.len_chars());
    }
    (line < rope.len_lines()).then_some(line).and_then(|line| {
        let len_chars = rope.line(line).len_chars();
        let offset = rope.try_line_to_char(line).ok()? + col.min(len_chars);
        Some(offset)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_to_offset() {
        let rope = Rope::from("Hello\nWorld");
        assert_eq!(2, rope.len_lines());
        assert_eq!(char_index(&rope, Position::new(0, 0)), Some(0));
        assert_eq!(char_index(&rope, Position::new(0, 5)), Some(5));
        assert_eq!(char_index(&rope, Position::new(0, 6)), Some(6)); // over
        assert_eq!(char_index(&rope, Position::new(0, 8)), Some(6)); // over
        assert_eq!(char_index(&rope, Position::new(1, 0)), Some(6));
        assert_eq!(char_index(&rope, Position::new(1, 5)), Some(11));
        assert_eq!(char_index(&rope, Position::new(2, 0)), Some(11));
        assert_eq!(char_index(&rope, Position::new(2, 1)), None);
        let rope = Rope::from("你好\na世界");
        // let c_index = rope.line(0).char_to_byte(1);
        // println!("c_index: {}", c_index);
        assert_eq!(char_index(&rope, Position::new(0, 0)), Some(0));
        assert_eq!(char_index(&rope, Position::new(0, 2)), Some(2));
        assert_eq!(char_index(&rope, Position::new(1, 0)), Some(3));
        assert_eq!(char_index(&rope, Position::new(1, 1)), Some(4));
        assert_eq!(char_index(&rope, Position::new(1, 3)), Some(6));
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
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 码表文件，其结构应为: 字词<TAB>编码<TAB>权重(可选) 每行
    /// 当没有权重时则以行的顺序判断编码对应的字词的首选还是候选
    #[arg(short, long)]
    pub table: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    simple_logging::log_to_file("/home/mapomagpie/.cache/helix/timls.log", LevelFilter::Info)
        .unwrap();
    log::info!("start");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let dict = Dict::load(&args.table);
    let reverse_dict = dict.config().reverse_dict.as_ref().map(|path| {
        let hint = PathBuf::from(path)
            .file_name()
            .and_then(|name| name.to_str().map(|n| n.chars().take(1).collect::<String>()))
            .unwrap_or("反".to_string());
        (Dict::load(secondary_dict_path(&args.table, path)), hint)
    });
    let engine = InputAnalyzer::new(dict, reverse_dict);
    let doc_map = DashMap::default();
    let state = RwLock::new(State { completion: true });
    let config = RwLock::new(Config::default());
    let (service, socket) = LspService::new(|_client| Backend {
        engine,
        doc_map,
        state,
        config,
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
