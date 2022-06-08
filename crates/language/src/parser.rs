pub fn javascript(language: &tree_sitter::Language) -> anyhow::Result<tree_sitter::Parser> {
    let mut parser = tree_sitter::Parser::new()?;
    parser.set_language(language)?;
    Ok(parser)
}
