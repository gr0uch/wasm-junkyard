#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use lzw_codec::{compress_lzw, decompress_lzw};
use wasm_bindgen_test::*;

// wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(
        compress_lzw(String::from("WYS*WYGWYS*WYSWYSG")),
        vec![87, 89, 83, 42, 256, 71, 256, 258, 262, 262, 71]
    );
    assert_eq!(
        decompress_lzw(vec![87, 89, 83, 42, 256, 71, 256, 258, 262, 262, 71]),
        String::from("WYS*WYGWYS*WYSWYSG")
    );
}
