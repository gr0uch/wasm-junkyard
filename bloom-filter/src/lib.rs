mod utils;

use bloomfilter::Bloom;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const BIGINT_SERIALIZER: Serializer =
    Serializer::new().serialize_large_number_types_as_bigints(true);

#[wasm_bindgen]
pub struct BloomFilter {
    bloom: Bloom<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BloomConfiguration {
    #[serde(with = "serde_bytes")]
    bit_vec: Vec<u8>,
    bitmap_bits: u64,
    k_num: u32,
    sip_keys: [(u64, u64); 2],
}

#[wasm_bindgen]
impl BloomFilter {
    pub fn new(num: usize, fp: f64) -> Self {
        utils::set_panic_hook();
        let bloom = Bloom::new_for_fp_rate(num, fp);
        Self { bloom }
    }

    pub fn set(&mut self, item: String) {
        self.bloom.set(&item)
    }

    pub fn check(&mut self, item: String) -> bool {
        self.bloom.check(&item)
    }

    pub fn import(config: JsValue) -> Self {
        let config: BloomConfiguration = serde_wasm_bindgen::from_value(config).unwrap();
        let bloom = Bloom::from_existing(
            &config.bit_vec,
            config.bitmap_bits,
            config.k_num,
            config.sip_keys,
        );
        Self { bloom }
    }

    pub fn export(&mut self) -> JsValue {
        let config = BloomConfiguration {
            bit_vec: self.bloom.bitmap(),
            bitmap_bits: self.bloom.number_of_bits(),
            k_num: self.bloom.number_of_hash_functions(),
            sip_keys: self.bloom.sip_keys(),
        };
        config.serialize(&BIGINT_SERIALIZER).unwrap()
    }
}
