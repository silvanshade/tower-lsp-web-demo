pub mod text_document {
    use std::sync::Arc;

    use lsp_text::RopeExt;

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

    pub async fn document_symbol(
        session: Arc<crate::core::Session>,
        params: lsp::DocumentSymbolParams,
    ) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
        use wasm_bindgen::JsCast;

        fn make_symbol(
            uri: &lsp::Url,
            content: &ropey::Rope,
            declaration: tree_sitter::Node,
            identifier: tree_sitter::Node,
            kind: lsp::SymbolKind,
        ) -> lsp::SymbolInformation {
            let name = content.utf8_text_for_tree_sitter_node(&identifier).into();
            let range = content.tree_sitter_range_to_lsp_range(declaration.range());
            #[allow(deprecated)]
            lsp::SymbolInformation {
                name,
                kind,
                tags: Default::default(),
                deprecated: Default::default(),
                location: lsp::Location::new(uri.clone(), range),
                container_name: Default::default(),
            }
        }

        let uri = &params.text_document.uri;

        let text = session.get_text(uri).await?;
        let content = &text.content;

        let tree = session.get_tree(uri).await?;
        let tree = tree.lock().await.clone();

        // NOTE: transmutes here because we do not yet support query functionality in
        // tree-sitter-facade; thus we use the raw bindings from web-tree-sitter-sys.

        #[allow(unsafe_code)]
        let node = unsafe { std::mem::transmute::<_, web_tree_sitter_sys::SyntaxNode>(tree.root_node()) };
        #[allow(unsafe_code)]
        let language = unsafe { std::mem::transmute::<_, web_tree_sitter_sys::Language>(session.language.clone()) };

        static QUERY: &str = indoc::indoc! {r"
          (function_declaration
            name: (identifier) @identifier) @function_declaration
          (lexical_declaration
            (variable_declarator
              name: (identifier) @identifier)) @class_declaration
          (variable_declaration
            (variable_declarator
              name: (identifier) @identifier)) @variable_declaration
          (class_declaration
            name: (identifier) @identifier) @class_declaration
        "};
        let query = language.query(&QUERY.into()).expect("failed to create query");
        let matches = {
            let start_position = None;
            let end_position = None;
            query
                .matches(&node, start_position, end_position)
                .into_vec()
                .into_iter()
                .map(JsCast::unchecked_into::<web_tree_sitter_sys::QueryMatch>)
        };

        let mut symbols = vec![];

        for r#match in matches {
            let captures = r#match
                .captures()
                .into_vec()
                .into_iter()
                .map(JsCast::unchecked_into::<web_tree_sitter_sys::QueryCapture>)
                .collect::<Vec<_>>();
            if let [declaration, identifier] = captures.as_slice() {
                // NOTE: reverse the transmutes from above so we can use tree-sitter-facade bindings for Node
                #[allow(unsafe_code)]
                let declaration_node = unsafe { std::mem::transmute::<_, tree_sitter::Node>(declaration.node()) };
                #[allow(unsafe_code)]
                let identifier_node = unsafe { std::mem::transmute::<_, tree_sitter::Node>(identifier.node()) };
                match String::from(declaration.name()).as_str() {
                    "function_declaration" => {
                        symbols.push(make_symbol(
                            uri,
                            content,
                            declaration_node,
                            identifier_node,
                            lsp::SymbolKind::FUNCTION,
                        ));
                    },
                    "lexical_declaration" => {
                        symbols.push(make_symbol(
                            uri,
                            content,
                            declaration_node,
                            identifier_node,
                            lsp::SymbolKind::VARIABLE,
                        ));
                    },
                    "variable_declaration" => {
                        symbols.push(make_symbol(
                            uri,
                            content,
                            declaration_node,
                            identifier_node,
                            lsp::SymbolKind::VARIABLE,
                        ));
                    },
                    "class_declaration" => {
                        symbols.push(make_symbol(
                            uri,
                            content,
                            declaration_node,
                            identifier_node,
                            lsp::SymbolKind::VARIABLE,
                        ));
                    },
                    _ => {},
                }
            }
        }

        Ok(Some(lsp::DocumentSymbolResponse::Flat(symbols)))
    }
}
