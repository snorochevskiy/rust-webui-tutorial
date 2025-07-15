mod console {
    #[link(wasm_import_module = "console")]
    unsafe extern "C" {
        pub fn log(x: i32);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn entry_point() {
    console::log(5);
}

