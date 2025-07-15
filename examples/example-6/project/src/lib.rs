use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let h1 = document.create_element("h1")?;
    h1.set_text_content(Some("Hello from WebAssembly"));
    body.append_child(&h1)?;

    Ok(())
}
