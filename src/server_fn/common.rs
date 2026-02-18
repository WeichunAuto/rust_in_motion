use pulldown_cmark::{Options, Parser, html};

/**
 * 标签类型枚举
 */
#[derive(Debug)]
pub enum TagType {
    Like,
    Dislike,
    Tag
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
pub fn markdown_to_html(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(md, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}