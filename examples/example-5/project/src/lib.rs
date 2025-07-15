use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[wasm_bindgen(start)]
fn startup_hello() {
    log("Hello from WebAssembly");
}

#[wasm_bindgen]
pub fn hello_name(name: &str) {
    log(format!("Hello, {name}!").as_str());
}
