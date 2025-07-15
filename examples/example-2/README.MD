# Example 2: Importing JS function to WebAssembly

## Prerequisites

1) Install Rust using [instruction](https://www.rust-lang.org/tools/install)
2) Install [watb](https://github.com/webassembly/wabt) from [releases page](https://github.com/WebAssembly/wabt/releases).
3) Install simple-http-server by executing `cargo install simple-http-server`

## How to build and run

1) Navigate to `project` directory.

```shell
cd project
```

2) User watb to translate `sum2.wat` file to binary wasm (`wat2wasm` should be in bin directory of `watb`):

```shell
$WATB_HOME/bin/wat2wasm print_num.wat -o print_num.wasm
```

3) Start HTTP server to serve `project` folder:

```shell
simple-http-server --cors -i -p 8080 ./
```

4) Open a web browser and navigate to http://localhost:8080/index.html
