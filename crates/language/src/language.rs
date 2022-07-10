pub async fn javascript() -> anyhow::Result<tree_sitter::Language> {
    let bytes: &[u8] = include_bytes!("../../../node_modules/tree-sitter-javascript/tree-sitter-javascript.wasm");
    let result = web_tree_sitter_sys::Language::load_bytes(&bytes.into())
        .await
        .map(Into::into)
        .map_err(Into::<tree_sitter::LanguageError>::into)?;
    Ok(result)
}

pub static ID: &str = "javascript";
