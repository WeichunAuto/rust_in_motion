use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

/**
 * 标签类型枚举
 */
#[derive(Debug)]
pub enum TagType {
    Like,
    Dislike,
    Tag,
}

/**
 * 从 markdown文件 读取内容
 */
pub fn read_from_markdown(path: &str) -> anyhow::Result<String> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

/// SSR 路径：使用 syntect 对代码块进行语法高亮，返回带 inline style 的 HTML
#[cfg(feature = "ssr")]
fn highlight_code(code: &str, lang: &str) -> String {
    use syntect::highlighting::ThemeSet;
    use syntect::html::highlighted_html_for_string;
    use syntect::parsing::SyntaxSet;

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["InspiredGitHub"];
    let syntax = ss
        .find_syntax_by_token(lang)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    highlighted_html_for_string(code, &ss, syntax, theme)
        .unwrap_or_else(|_| format!("<pre><code>{}</code></pre>", code))
}

/**
 * 把 markdown 转换成 html
 * SSR 路径：对 SQL/Java/Rust 代码块使用 syntect 语法高亮
 * WASM 路径：回退到 pulldown-cmark 原始渲染（不影响水合）
 */
pub fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);

    #[cfg(feature = "ssr")]
    {
        let mut html_output = String::new();
        let mut in_code_block = false;
        let mut current_lang = String::new();
        let mut code_buf = String::new();
        let mut events: Vec<Event> = Vec::new();

        for event in parser {
            match event {
                Event::Start(Tag::CodeBlock(ref kind)) => {
                    in_code_block = true;
                    current_lang = match kind {
                        CodeBlockKind::Fenced(lang) => lang.to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                    code_buf.clear();
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                    let highlighted = highlight_code(&code_buf, &current_lang);
                    events.push(Event::Html(highlighted.into()));
                    code_buf.clear();
                    current_lang.clear();
                }
                Event::Text(ref text) if in_code_block => {
                    code_buf.push_str(text);
                }
                other => events.push(other),
            }
        }

        html::push_html(&mut html_output, events.into_iter());
        return format!("<div class=\"markdown-body\">{}</div>", html_output);
    }

    #[cfg(not(feature = "ssr"))]
    {
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        format!("<div class=\"markdown-body\">{}</div>", html_output)
    }
}
