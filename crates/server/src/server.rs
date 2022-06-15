use std::sync::Arc;
use tower_lsp::{jsonrpc, lsp_types::*, LanguageServer};

pub fn capabilities() -> lsp::ServerCapabilities {
    let document_symbol_provider = Some(lsp::OneOf::Left(true));

    let text_document_sync = {
        let options = lsp::TextDocumentSyncOptions {
            open_close: Some(true),
            change: Some(lsp::TextDocumentSyncKind::FULL),
            ..Default::default()
        };
        Some(lsp::TextDocumentSyncCapability::Options(options))
    };

    lsp::ServerCapabilities {
        text_document_sync,
        document_symbol_provider,
        ..Default::default()
    }
}

pub struct Server {
    pub client: tower_lsp::Client,
    pub session: Arc<crate::core::Session>,
}

impl Server {
    pub fn new(client: tower_lsp::Client, language: tree_sitter::Language) -> Self {
        let session = crate::core::Session::new(Some(client.clone()), language);
        Server { client, session }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    async fn initialize(&self, params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        web_sys::console::log_1(&"server::initialize".into());
        *self.session.client_capabilities.write().await = Some(params.capabilities);
        let capabilities = capabilities();
        Ok(InitializeResult {
            capabilities,
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: lsp::InitializedParams) {
        web_sys::console::log_1(&"server::initialized".into());
        let typ = lsp::MessageType::INFO;
        let message = "demo language server initialized!";
        self.client.log_message(typ, message).await;
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        web_sys::console::log_1(&"server::shutdown".into());
        Ok(())
    }

    // FIXME: for some reason this doesn't trigger
    async fn did_open(&self, params: lsp::DidOpenTextDocumentParams) {
        web_sys::console::log_1(&"server::did_open".into());

        let typ = lsp::MessageType::INFO;
        let message = format!("opened document: {}", params.text_document.uri.as_str());
        self.client.log_message(typ, message).await;

        let session = self.session.clone();
        crate::handler::text_document::did_open(session, params).await.unwrap();
    }

    async fn did_change(&self, params: lsp::DidChangeTextDocumentParams) {
        web_sys::console::log_1(&"server::did_change".into());
        let session = self.session.clone();
        crate::handler::text_document::did_change(session, params)
            .await
            .unwrap();
    }

    async fn document_symbol(
        &self,
        params: lsp::DocumentSymbolParams,
    ) -> jsonrpc::Result<Option<lsp::DocumentSymbolResponse>> {
        web_sys::console::log_1(&"server::document_symbol".into());
        let session = self.session.clone();
        let result = crate::handler::text_document::document_symbol(session, params).await;
        Ok(result.map_err(crate::core::IntoJsonRpcError)?)
    }
}
