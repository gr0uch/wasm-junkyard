mod utils;

use image::{
    codecs::png::PngEncoder, io::Reader as ImageReader, DynamicImage, ImageBuffer, ImageEncoder,
    Rgba, RgbaImage,
};
use imageproc::drawing::{draw_text_mut, text_size};
use once_cell::sync::Lazy;
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::{collections::HashMap, sync::Mutex};
use wasm_bindgen::prelude::*;

static FONT_MAP: Lazy<Mutex<HashMap<String, Font>>> = Lazy::new(|| {
    let h = HashMap::new();
    Mutex::new(h)
});

#[derive(Serialize, Deserialize)]
pub struct FontConfig {
    pub rgba: Vec<u8>,
    pub size: f32,
    pub font_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SpriteOptions {
    pub resize: [u32; 2],
    pub filter_type: String,
}

#[wasm_bindgen(js_name = loadFont)]
pub fn load_font(font_name: String, font_data: Vec<u8>) {
    let mut font_map = FONT_MAP.lock().unwrap();
    let font = Font::try_from_vec(font_data).unwrap();
    font_map.insert(font_name, font);
}

#[wasm_bindgen]
pub struct DrawImage {
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

#[allow(clippy::boxed_local)]
#[wasm_bindgen]
impl DrawImage {
    pub fn new(width: u32, height: u32) -> DrawImage {
        utils::set_panic_hook();
        let img: RgbaImage = ImageBuffer::new(width, height);
        DrawImage { img }
    }

    #[wasm_bindgen(js_name = fillRectangle)]
    pub fn fill_rectangle(&mut self, coords: Box<[u32]>, rgba: Box<[u8]>) {
        let [x1, y1, x2, y2]: [u32; 4] = (*coords).try_into().unwrap();
        let color: [u8; 4] = (*rgba).try_into().unwrap();
        let px = Rgba::from(color);

        for x in x1..x2 {
            for y in y1..y2 {
                self.img.put_pixel(x, y, px);
            }
        }
    }

    #[wasm_bindgen(js_name = drawText)]
    pub fn draw_text(&mut self, coords: Box<[i32]>, font_config: JsValue, text: String) -> Vec<i32> {
        let [left, top]: [i32; 2] = (*coords).try_into().unwrap();
        let config: FontConfig = serde_wasm_bindgen::from_value(font_config).unwrap();
        let color: [u8; 4] = (*config.rgba).try_into().unwrap();
        let font_map = FONT_MAP.lock().unwrap();
        let scale = Scale::uniform(config.size);
        let font = font_map.get(&config.font_name).unwrap();
        draw_text_mut(
            &mut self.img,
            Rgba::from(color),
            left,
            top,
            scale,
            font,
            &text,
        );
        let size = text_size(scale, font, &text);
        vec![size.0, size.1]
    }

    #[wasm_bindgen(js_name = drawSprite)]
    pub fn draw_sprite(&mut self, coords: Box<[u32]>, sprite: Box<[u8]>, options: JsValue) {
        let decoded_sprite = ImageReader::new(Cursor::new(&*sprite))
            .with_guessed_format()
            .unwrap();
        let sprite_image = decoded_sprite.decode().unwrap();
        self.draw_on_image(coords, sprite_image, options)
    }

    fn draw_on_image(
        &mut self,
        coords: Box<[u32]>,
        mut sprite_image: DynamicImage,
        options: JsValue,
    ) {
        let [left, top]: [u32; 2] = (*coords).try_into().unwrap();
        let opts = serde_wasm_bindgen::from_value(options);
        if opts.is_ok() {
            let opt: SpriteOptions = opts.unwrap();
            let target_w = opt.resize[0];
            let target_h = opt.resize[1];
            let filter_type = match opt.filter_type.as_str() {
                "Nearest" => image::imageops::FilterType::Nearest,
                "Triangle" => image::imageops::FilterType::Triangle,
                "CatmullRom" => image::imageops::FilterType::CatmullRom,
                "Gaussian" => image::imageops::FilterType::Gaussian,
                "Lanczos3" => image::imageops::FilterType::Lanczos3,
                _ => panic!("invalid filter type!"),
            };
            sprite_image = image::DynamicImage::ImageRgba8(image::imageops::resize(
                &sprite_image,
                target_w,
                target_h,
                filter_type,
            ));
        }

        image::imageops::overlay(&mut self.img, &sprite_image, left.into(), top.into());
    }

    #[wasm_bindgen(js_name = getPNGImage)]
    pub fn get_png_image(self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        let (width, height) = self.img.dimensions();
        PngEncoder::new(&mut output)
            .write_image(&self.img, width, height, image::ColorType::Rgba8)
            .expect("failed to write png");
        output
    }
}
