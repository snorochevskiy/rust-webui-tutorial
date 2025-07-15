# Example 11: Leptos – HTTP calls

## Prerequisites

1) Install Rust using [instruction](https://www.rust-lang.org/tools/install)
2) Add `wasm32-unknown-unknown` build target: `rustup target add wasm32-unknown-unknown`
3) Install `trunk` tool: `cargo install trunk`

## How to build and run

### Run backend

1) Navigate to `backend` directory.

```shell
cd backend
```

2) Run server:

```shell
cargo run
```


### Run UI

1) Navigate to `frontend` directory.

```shell
cd frontend
```

2) Build and open in browser:

```shell
trunk serve --port 3000 --open
```

