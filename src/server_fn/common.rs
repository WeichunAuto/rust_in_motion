// 从 markdown 读取内容
pub fn read_from_markdown(path: &str) -> anyhow::Result<String> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}
