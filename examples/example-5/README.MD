# Example 5: wasm-bindgen – console log

## Prerequisites

1) Install Rust using [instruction](https://www.rust-lang.org/tools/install)
2) Add `wasm32-unknown-unknown` build target: `rustup target add wasm32-unknown-unknown`
3) Install `wasm-pack` tool: `cargo install wasm-pack`
4) Install simple-http-server by executing `cargo install simple-http-server`

## How to build and run

1) Navigate to `project` directory.

```shell
cd project
```

2) Build wasm bundle:

```shell
wasm-pack build --target web
```

3) Start HTTP server to serve `project` folder:

```shell
simple-http-server --cors -i -p 8080 ./
```

4) Open a web browser and navigate to http://localhost:8080/index.html
