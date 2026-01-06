use dashmap::DashMap;
use log::LevelFilter;
use ropey::Rope;
use tim::{AnalysisResult, Dict, InputAnalyzer};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    // client: Client,
    engine: InputAnalyzer,
    doc_map: DashMap<String, Rope>,
    selection_keys: Vec<&'static str>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        log::info!("initialize");
        return Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "timlsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(
                        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ,.]"
                            .chars()
                            .map(|c| c.to_string())
                            .collect(),
                    ),
                    ..CompletionOptions::default()
                }),
                ..ServerCapabilities::default()
            },
        });
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let rope = match self.doc_map.get(uri.as_str()) {
            Some(rope) => rope,
            None => return Ok(None),
        };

        let line_chars: Vec<char> = rope.line(position.line as usize).chars().collect();
        let end = position.character as usize;
        let mut start = end;
        for i in (0..end).rev() {
            let char = line_chars[i];
            // log::info!("completion char {}", char);
            if char.is_ascii() && !char.is_control() {
                if i > 0 && char.is_ascii_whitespace() && line_chars[i - 1].is_ascii_whitespace() {
                    break;
                }
                start = i;
            } else {
                break;
            }
        }
        let mut filter_text_start = start;
        for i in (0..start).rev() {
            let char = line_chars[i as usize];
            if char.is_alphabetic() {
                filter_text_start = i;
            } else {
                break;
            }
        }
        if start == end {
            log::info!("completion empty");
            return Ok(None);
        }
        let AnalysisResult {
            sentence,
            candidates,
        } = self
            .engine
            .analyze(&line_chars[start as usize..end as usize]);
        // 编辑器在收到补全后，全根据fiter_text进行过滤，比如helix会用[向前日后一个字..当前光标]这个范围的字符去搜索，如果搜索的分数太低就会丢弃
        // 所谓的字，就是英文字母、汉字、等其他非标点符号的字
        // 设置fiter_text最简单的方式是从当前行的首位开始也就是0，到当前光标的位置
        // 不过如果一行太长的话，可能有性能问题，更好的方式是从start开始，再向前找到字的位置。
        let filter_text: String = (&line_chars[filter_text_start as usize..end as usize])
            .iter()
            .collect();
        // log::info!(
        //     "completion word: [{}-{}], filter_text: {}",
        //     start,
        //     end,
        //     filter_text
        // );
        let sentence = sentence.join("");
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
        let cand_items: Vec<CompletionItem> = if candidates.len() == 1 {
            vec![]
        } else {
            candidates
                .into_iter()
                .enumerate()
                .map(|(i, c)| CompletionItem {
                    label: format!(
                        "{}[{}]: {} {}",
                        i + 1,
                        self.selection_keys[i],
                        c.text,
                        c.code
                    ),
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
        };
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

    // async fn initialized(&self, _: InitializedParams) {
    //     self.client
    //         .log_message(MessageType::INFO, "server initialized!")
    //         .await;
    // }
    //
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
    }
}

#[tokio::main]
async fn main() {
    simple_logging::log_to_file("/home/mapomagpie/.cache/helix/timls.log", LevelFilter::Info)
        .unwrap();
    log::info!("start");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let selection_keys = vec!["U", "I", "O", "H", "J", "K", "B", "N", "M"];
    let dict = Dict::load("/home/mapomagpie/code/tableim/test/虎码码表_我.txt");
    let engine = InputAnalyzer::new(dict);
    let doc_map = DashMap::default();
    let (service, socket) = LspService::new(|_client| Backend {
        engine,
        doc_map,
        selection_keys,
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
