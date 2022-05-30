<div align="center">
  <h1><code>tower-lsp-wasm-example</code></h1>
  <p>
    <strong>A minimal WASM target example for tower-lsp</strong>
  </p>
</div>

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
