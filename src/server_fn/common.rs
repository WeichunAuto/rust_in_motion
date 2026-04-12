use pulldown_cmark::{html, Options, Parser};

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

/**
 * 把 markdown 转换成 html
 */
pub fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    format!("<div class=\"markdown-body\">{}</div>", html_output)
}
