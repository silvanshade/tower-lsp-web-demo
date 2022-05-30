#![cfg(web_sys_unstable_apis)]

use futures::stream::TryStreamExt;
use js_sys::Uint8Array;
use tower_lsp::{jsonrpc, lsp_types::*, LanguageServer, LspService, Server};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::stream::JsStream;

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

// NOTE: we don't use web_sys::ReadableStream for input here because on the
// browser side we need to use a ReadableByteStreamController to construct it
// and so far only Chromium-based browsers support that functionality.

// NOTE: input needs to be an AsyncIterator<Uint8Array> specifically
#[wasm_bindgen]
pub async fn serve(input: js_sys::AsyncIterator, output: web_sys::WritableStream) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"server::serve".into());

    let input = JsStream::from(input);
    let input = input
        .map_ok(|value| {
            value
                .dyn_into::<Uint8Array>()
                .expect("could not cast stream item to Uint8Array")
                .to_vec()
        })
        .map_err(|_err| std::io::Error::from(std::io::ErrorKind::Other))
        .into_async_read();

    let output = JsCast::unchecked_into::<wasm_streams::writable::sys::WritableStream>(output);
    let output = wasm_streams::WritableStream::from_raw(output);
    let output = output.try_into_async_write().map_err(|err| err.0)?;

    let (service, messages) = LspService::new(|_client| LspServer {});
    Server::new(input, output, messages).serve(service).await;

    Ok(())
}
