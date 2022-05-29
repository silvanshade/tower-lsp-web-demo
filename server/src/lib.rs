#![cfg(web_sys_unstable_apis)]

use tower_lsp::{jsonrpc, lsp_types::*, LanguageServer, LspService, Server};
use wasm_bindgen::{prelude::*, JsCast};

struct LspServer {
}

#[tower_lsp::async_trait]
impl LanguageServer for LspServer {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        web_sys::console::log_1(&"server::initialize".into());
        Ok(InitializeResult {
            ..InitializeResult::default()
        })
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        web_sys::console::log_1(&"server::shutdown".into());
        Ok(())
    }
}

#[wasm_bindgen]
// NOTE: input needs to be a ReadableByteStream specifically
pub async fn serve(input: web_sys::ReadableStream, output: web_sys::WritableStream) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"server::serve".into());

    let input = JsCast::unchecked_into::<wasm_streams::readable::sys::ReadableStream>(input);
    let input = wasm_streams::ReadableStream::from_raw(input);
    let input = input.try_into_async_read().map_err(|err| err.0)?;

    let output = JsCast::unchecked_into::<wasm_streams::writable::sys::WritableStream>(output);
    let output = wasm_streams::WritableStream::from_raw(output);
    let output = output.try_into_async_write().map_err(|err| err.0)?;

    let (service, messages) = LspService::new(|_client| LspServer {});
    Server::new(input, output, messages).serve(service).await;

    Ok(())
}
