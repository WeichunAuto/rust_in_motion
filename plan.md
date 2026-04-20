# 代码语法高亮技术方案

## 背景

BlogDetailPage 使用 `pulldown-cmark` 将 Markdown 转为 HTML，代码块目前以纯文本渲染，缺乏语法高亮。需要为 SQL、Java、Rust 三种语言的代码块添加服务端语法高亮。

---

## 技术选型

使用 `syntect` crate（Rust 生态最主流的语法高亮库）实现服务端高亮，生成带 inline style 的 HTML，无需任何客户端 JavaScript。

**主题**：`InspiredGitHub`（白底、GitHub 风格，与现有博客样式一致）

---

## 架构约束

`render_markdown` 函数同时在 SSR（服务端）和 WASM（客户端 hydration）环境编译。`syntect` 不能编译到 WASM，因此必须用 `#[cfg(feature = "ssr")]` 隔离所有 syntect 代码，提供两条路径：

- **SSR 路径**：syntect 高亮，输出带 inline style 的 `<pre>` HTML
- **WASM 路径**：回退到 pulldown-cmark 原始渲染（hydration 阶段不重渲染 `inner_html`，不影响水合）

---

## 修改文件清单

### 1. `Cargo.toml`

在 `[dependencies]` 中添加 syntect 为可选依赖：

```toml
syntect = { version = "5", default-features = false, features = [
    "default-fancy",   # 纯 Rust 的 fancy-regex 引擎，无 C 库依赖
    "default-themes",  # 内置 InspiredGitHub 等主题
    "html",            # 暴露 highlighted_html_for_string API
], optional = true }
```

在 `[features]` 的 `ssr` 中追加 `"dep:syntect"`。

### 2. `src/server_fn/common.rs`

重写 `render_markdown`，用 pulldown-cmark 事件流拦截代码块：

- `Event::Start(Tag::CodeBlock(...))` → 记录语言、开始收集代码文本
- `Event::Text` (in_code_block) → 累积到 `code_buf`
- `Event::End(TagEnd::CodeBlock)` → 调用 `highlight_code` 生成高亮 HTML，以 `Event::Html` 注入输出流

SSR 路径通过 `#[cfg(feature = "ssr")]` 启用 syntect，WASM 路径维持原样。

**语言 token 对应**（大小写不敏感）：

| Markdown fence | syntect token |
|----------------|---------------|
| ` ```sql `     | `sql`         |
| ` ```java `    | `java`        |
| ` ```rust `    | `rust` / `rs` |
| 无语言/未知    | plain text    |

### 3. `style/main.scss`

syntect 输出带 inline style 的 `<pre>`（无 `<code>` 子元素），移除 `.markdown-body pre` 中的 `background-color` 和 `padding`，由 syntect inline style 控制；追加 `font-family` 确保等宽字体继承。

---

## 性能优化（可选，后续迭代）

用 `OnceLock` 缓存 `SyntaxSet` 和 `ThemeSet`，避免每次调用都反序列化内置二进制数据：

```rust
#[cfg(feature = "ssr")]
static SYNTAX_SET: std::sync::OnceLock<syntect::parsing::SyntaxSet> = std::sync::OnceLock::new();
#[cfg(feature = "ssr")]
static THEME_SET: std::sync::OnceLock<syntect::highlighting::ThemeSet> = std::sync::OnceLock::new();
```

---

## 验证方式

1. 在博客中编写包含 ` ```rust `、` ```sql `、` ```java ` 代码块的文章
2. 访问博客详情页，确认关键字着色正确
3. 访问无语言标注的代码块，确认回退为纯文本无报错
4. 运行 `cargo leptos build` 确认 WASM 编译不受影响
