<div align="center">
  <h1><code>tower-lsp-web-demo</code></h1>
  <p>
    <strong>A minimal browser-hosted WASM demo for tower-lsp</strong>
  </p>
</div>

## Demo

You can experiment with a live demo of the example server integrated with an in-browser editor here:

https://silvanshade.github.io/tower-lsp-web-demo/

## Building

```sh
cargo install cargo-make
cargo make deps
cargo make build
```

## Running

```sh
cargo make run
```

## Project Structure

The server implementation:

```
crates
├── browser                   -- entry-point for launching the server in the browser
│   └── src
│       └── lib.rs
├── language                  -- handles definitions for working with tree-sitter javascript grammar
│   └── src
│       ├── language.rs       -- handles loading the pre-compiled tree-sitter-javascript.wasm blob
│       ├── lib.rs
│       └── parser.rs         -- creates tree-sitter parsers from the loaded grammar blob
└── server
    └── src
        ├── core
        │   ├── document.rs   -- definitions for working with document related data
        │   ├── error.rs
        │   ├── session.rs    -- definitions for lsp session and related state
        │   ├── syntax.rs     -- definitions for updating syntax text area in browser
        │   └── text.rs       -- definitions for handling text and edits
        ├── core.rs
        ├── handler.rs        -- definitions for various feature handlers
        ├── lib.rs
        └── server.rs         -- definitions for the lsp server and impl of tower-lsp trait
```

The webapp and client implementation for wiring up the Monaco editor to  communicate with the server:

```
packages
└── app
    └── src
        ├── app.ts            -- the browser app which launches the client and server and displays the user interface
        ├── client.ts         -- definitions for the lsp client
        ├── codec             -- definitions for encoding and decoding/demuxing messages between the client and server
        │   ├── bytes.ts      -- utilities for working with Uint8Array
        │   ├── demuxer.ts    -- demuxer for splitting output from server into streams of notifications, requests, and responses
        │   ├── headers.ts    -- utilities for working with http headers
        │   ├── map.ts        -- map for storing responses from server yet to be processed (fed by demuxer)
        │   └── queue.ts      -- promise based queue used for storing notifications and requests
        ├── index.ts
        ├── language.ts       -- definition for the javascript language (e.g., language id, extensions, mime, etc.)
        ├── queue.ts          -- promise based queue structure
        ├── server.ts         -- prepares client->server and client<-server web streams and launches tower-lsp server
        └── tracer.ts         -- utilities for tracing JSON-RPC messages and displaying in browser interface
```
