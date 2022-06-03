<div align="center">
  <h1><code>tower-lsp-wasm-example</code></h1>
  <p>
    <strong>A minimal WASM target example for tower-lsp</strong>
  </p>
</div>


## Overview

[tower](https://github.com/tower-rs/tower) is a library for building network clients and servers in rust. The [tower-lsp](https://github.com/ebkalderon/tower-lsp) project implements the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/), which allows you to build your own language server using `rust` and `tower`. Editors and other tools can communicate with your language server process to build features such as auto-complete, documentation on hover, and more.

By default `tower` will use the [tokio](https://tokio.rs/) library as its runtime, but you can target different backends. For `tower-lsp`, this is made possible with the [async-codec-lite](https://github.com/silvanshade/async-codec-lite) library.

The demo in this repository targets a `wasm` backend and launches a language server that runs in your browser.

## Building

```sh
cargo install wasm-bindgen-cli --version 0.2.80
cd server
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ../app/dist --target web --typescript ./target/wasm32-unknown-unknown/release/server.wasm
cd ..
cd app
npm i
npm run build
```

## Running

```sh
cd app
npm run app
```

After the browser window opens, you can try copying and pasting the listed messages into the `stdin` textarea and hitting the `send` button.

