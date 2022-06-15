use anyhow::anyhow;
pub async fn javascript() -> anyhow::Result<tree_sitter::Language> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    let bytes: &[u8] = include_bytes!("../../../node_modules/tree-sitter-javascript/tree-sitter-javascript.wasm");
    let promise = web_tree_sitter_sys::Language::load_bytes(&bytes.into());
    let future = JsFuture::from(promise);
    let value = future
        .await
        .map_err(|_| anyhow!("failed to load tree-sitter-javascript.wasm"))?;
    let inner = value.unchecked_into::<web_tree_sitter_sys::Language>();
    let result = inner.into();
    Ok(result)
}

pub static ID: &str = "javascript";
