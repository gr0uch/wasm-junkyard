//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use serde_wasm_bindgen::*;
use simple_draw::{load_font, DrawImage, FontConfig};
use wasm_bindgen_test::*;

// wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    load_font(
        String::from("inter"),
        include_bytes!("./Inter-Medium.otf").to_vec(),
    );
    let mut img = DrawImage::new(64, 64);
    img.fill_rectangle(Box::from([0, 0, 32, 64]), Box::from([255, 0, 128, 255]));
    img.fill_rectangle(Box::from([32, 0, 64, 64]), Box::from([0, 128, 255, 255]));
    let sprite = include_bytes!("./sprite.png").to_vec();
    img.draw_sprite_png(Box::from([4, 4]), Box::from(sprite.clone()), None);
    img.draw_sprite_png(Box::from([6, 6]), Box::from(sprite.clone()), None);
    let mut fc = FontConfig {
        rgba: vec![0, 255, 0, 255],
        size: 18 as f32,
        font_name: "inter".to_string(),
    };
    img.draw_text(
        Box::from([2, 26]),
        to_value(&fc).unwrap(),
        String::from("fk!"),
    );
    fc.rgba = vec![255, 0, 0, 64];
    img.draw_text(
        Box::from([28, 40]),
        to_value(&fc).unwrap(),
        String::from("shiet"),
    );
    let png = img.get_png_image();
    assert_eq!(png, include_bytes!("test-1.png"));
}
