# Senime

Senime 是一个专注于性能与简洁的码表输入法引擎。加载码表，输入编码，输出中文。易于嵌入各种应用。

## 项目结构

| crate | 说明 | 产物 |
|---|---|---|
| **senime-lib** | 核心引擎：码表加载、输入分析、分词 | 库 |
| senime-tui | 终端输入器 / 赛码器 | `senitui` |
| senime-lsp | 编辑器语言服务，以补全方式输入中文 | `senimels` |
| senime-fcitx5 | Fcitx5 输入法插件 | `senime-fcitx5/build/senime.so` |
| senime-encode | 分词器，输出分词结果与码表性能分析 | `senienc` |
| senime-wasm | WASM 包装，提供 `init_ime` / `completion` 接口 | `senime-wasm/pkg` |
| senime-browser-extension | Firefox 浏览器扩展 (Senibe) | 扩展包 |

> 各应用可用 `--help` 查看使用说明

## senime-lib的特性 

- **连续编码输入**

  `InputAnalyzer::analyze` 接受一串ascii编码，内部会自动分段，查找对应字词。甚至可以一次输入数千的编码，然后输出这段编码对应的文章，速度为微秒级。

- **API简单**

  输入法功能只有一个API:`InputAnalyzer::analyze`，非常简单。此外，还有一个输入法无关的分词API: `Looker::analyze`。

- **内部无状态**

  能让`senime-lib`易于集成到其他平台上，它是无状态的，就像LLM，打一段话的过程中，每次输入都要将全部的编码交给`InputAnalyzer::analyze`，但好在它非常快！

- **码表要求简单**

  `senime-lib::Dict`接受一张txt码表，每行以`tab`(制表符)分割字词、编码、权重，但顺序不限制。`senime-lib::Dict`读取后，会生成二进制缓存，加快下次读取。
  此外，`senime-lib::Dict`还可加载配置文件、二进制码表，`Dict::load`能接受三种文件输入: toml后缀的配置文件、txt后缀的码表文件、bin后缀的二进制码表缓存。

- **反查**

  `senime-lib`支持第二码表来进行反查，可在配置文件中指定第二码表。

- **自定义候选键、标点符号、反查键**

  `senime-lib::Dict`可加载配置文件，在配置文件中自定义候选键，比如大写的字母`I`作为次选，`O`作为三选。
  自定义标点符号，同时支持大写字母。标点符号键连续输入时，能依次选择次选、三选等。

## 码表格式

码表文件为 `.txt` 纯文本，每行以 tab 分隔，且顺序不限：

```
字词\t编码\t权重
```

或

```
编码\t字词\t权重
```

## 开发环境与构建

### 纯 Rust

> 适用: `senime-lib` `senime-tui` `senime-lsp` `senime-encode`

依赖：

- `rust` (edition 2024)
- `rust-analyzer`
- `rust-bindgen`

```bash
# 构建全部
cargo build --release
# 指定构建
cargo build --release -p senime-tui
cargo build --release -p senime-lsp
cargo build --release -p senime-encode
```

### Rust + Wasm（用于浏览器集成）

> 适用: `senime-wasm` `senime-browser-extension`

依赖：上述 Rust 工具链 +

- `wasm-pack`
- `wasm-bindgen-cli`

```bash
cd senime-wasm && wasm-pack build --target web
```

### Rust + C++（用于 Fcitx5）

> 适用: `senime-fcitx5`

依赖：上述 Rust 工具链 +

- `gcc`
- `cmake` + `ninja`
- `pkg-config`
- `fcitx5` (开发头文件)
- `extra-cmake-modules`
- `gettext`
- `libclang`

```bash
cd senime-fcitx5 && cmake -B build . && cmake --build build
```

### 测试

```bash
cargo test
cargo clippy
```

## 许可证

见 [LICENSE](./LICENSE)。
