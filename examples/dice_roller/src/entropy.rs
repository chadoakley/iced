// Custom getrandom backend registration for wasm targets.
//
// The wasm build has `--cfg getrandom_backend="custom"` set in
// .cargo/config.toml, so getrandom expects a user-provided
// `__getrandom_v03_custom` symbol. This module wires that symbol up to a
// JS-side helper that consults the browser's entropy pool.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/js/entropy.js")]
extern "C" {
    fn read_entropy_byte() -> u8;
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
unsafe extern "Rust" fn __getrandom_v03_custom(
    dest: *mut u8,
    len: usize,
) -> Result<(), getrandom::Error> {
    for i in 0..len {
        unsafe { *dest.add(i) = read_entropy_byte(); }
    }
    Ok(())
}
