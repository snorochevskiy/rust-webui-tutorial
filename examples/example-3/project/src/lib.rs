#[unsafe(no_mangle)]
pub unsafe extern "C" fn sum2(a: i32, b: i32) -> i32 {
    a + b
}
