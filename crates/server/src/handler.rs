pub mod text_document {
    use std::sync::Arc;

    pub async fn did_open(
        session: Arc<crate::core::Session>,
        params: lsp::DidOpenTextDocumentParams,
    ) -> anyhow::Result<()> {
        let uri = params.text_document.uri.clone();

        if let Some(document) = crate::core::Document::open(session.clone(), params).await? {
            session.insert_document(uri.clone(), document)?;
        } else {
            log::warn!("'textDocument/didOpen' failed :: uri: {:#?}", uri);
        }

        Ok(())
    }

    pub async fn did_change(
        session: Arc<crate::core::Session>,
        params: lsp::DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        let uri = &params.text_document.uri;
        let mut text = session.get_mut_text(uri).await?;
        *text = crate::core::Text::new(params.content_changes[0].text.clone())?;
        crate::core::Document::change(session.clone(), uri, &text.content).await?;
        Ok(())
    }

    pub async fn did_close(
        session: Arc<crate::core::Session>,
        params: lsp::DidCloseTextDocumentParams,
    ) -> anyhow::Result<()> {
        let uri = params.text_document.uri;
        session.remove_document(&uri)?;
        let diagnostics = Default::default();
        let version = Default::default();
        session.client()?.publish_diagnostics(uri, diagnostics, version).await;
        Ok(())
    }
}
