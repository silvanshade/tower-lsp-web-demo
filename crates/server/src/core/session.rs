use anyhow::anyhow;
use async_lock::{Mutex, RwLock};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionResourceKind {
    Document,
    Parser,
    Tree,
}

pub struct Session {
    pub server_capabilities: RwLock<lsp::ServerCapabilities>,
    pub client_capabilities: RwLock<Option<lsp::ClientCapabilities>>,
    client: Option<tower_lsp::Client>,
    pub language: tree_sitter::Language,
    pub document_states: DashMap<lsp::Url, crate::core::DocumentState>,
    document_texts: DashMap<lsp::Url, crate::core::Text>,
    document_parsers: DashMap<lsp::Url, Mutex<tree_sitter::Parser>>,
    document_trees: DashMap<lsp::Url, Mutex<tree_sitter::Tree>>,
}

impl Session {
    pub fn new(client: Option<tower_lsp::Client>, language: tree_sitter::Language) -> Arc<Self> {
        let server_capabilities = RwLock::new(crate::server::capabilities());
        let client_capabilities = Default::default();
        let document_states = Default::default();
        let document_texts = Default::default();
        let document_parsers = Default::default();
        let document_trees = Default::default();
        Arc::new(Session {
            server_capabilities,
            client_capabilities,
            client,
            language,
            document_states,
            document_texts,
            document_parsers,
            document_trees,
        })
    }

    pub fn client(&self) -> anyhow::Result<&tower_lsp::Client> {
        self.client
            .as_ref()
            .ok_or_else(|| crate::core::Error::ClientNotInitialized.into())
    }

    pub fn insert_document(&self, uri: lsp::Url, document: crate::core::Document) -> anyhow::Result<()> {
        let result = self.document_texts.insert(uri.clone(), document.text());
        debug_assert!(result.is_none());
        let result = self.document_parsers.insert(uri.clone(), Mutex::new(document.parser));
        debug_assert!(result.is_none());
        let result = self.document_trees.insert(uri, Mutex::new(document.tree));
        debug_assert!(result.is_none());
        Ok(())
    }

    pub fn remove_document(&self, uri: &lsp::Url) -> anyhow::Result<()> {
        let result = self.document_texts.remove(uri);
        debug_assert!(result.is_some());
        let result = self.document_parsers.remove(uri);
        debug_assert!(result.is_some());
        let result = self.document_trees.remove(uri);
        debug_assert!(result.is_some());
        Ok(())
    }

    pub async fn semantic_tokens_legend(&self) -> Option<lsp::SemanticTokensLegend> {
        let capabilities = self.server_capabilities.read().await;
        if let Some(capabilities) = &capabilities.semantic_tokens_provider {
            match capabilities {
                lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(options) => Some(options.legend.clone()),
                lsp::SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(options) => {
                    Some(options.semantic_tokens_options.legend.clone())
                },
            }
        } else {
            None
        }
    }

    pub async fn get_text(&self, uri: &lsp::Url) -> anyhow::Result<Ref<'_, lsp::Url, crate::core::Text>> {
        self.document_texts.get(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_text(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, crate::core::Text>> {
        self.document_texts.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_parser(
        &self,
        uri: &lsp::Url,
    ) -> anyhow::Result<RefMut<'_, lsp::Url, Mutex<tree_sitter::Parser>>> {
        self.document_parsers.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Parser;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_tree(&self, uri: &lsp::Url) -> anyhow::Result<Ref<'_, lsp::Url, Mutex<tree_sitter::Tree>>> {
        self.document_trees.get(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_tree(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, Mutex<tree_sitter::Tree>>> {
        self.document_trees.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub fn get_channel_syntax() -> anyhow::Result<web_sys::HtmlTextAreaElement> {
        use wasm_bindgen::JsCast;
        let element_id = "channel-syntax";
        let channel_syntax = web_sys::window()
            .ok_or(anyhow!("failed to get window"))?
            .document()
            .ok_or(anyhow!("failed to get document"))?
            .get_element_by_id(element_id)
            .ok_or(anyhow!("failed to get channel-syntax element"))?
            .unchecked_into();
        Ok(channel_syntax)
    }
}
