mod utils;

use sublime_fuzzy::{best_match, format_simple, Match};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct SearchIndex {
    sample_space: Vec<String>,
}

#[wasm_bindgen]
pub fn match_single(input: &str, target: &str, should_format: bool) -> JsValue {
    let match_opt = best_match(input, target);

    if match_opt.is_none() {
        return serde_wasm_bindgen::to_value(&()).unwrap();
    }

    match should_format {
        true => {
            let formatted = format_simple(&match_opt.unwrap(), target, "<strong>", "</strong>");
            serde_wasm_bindgen::to_value(&formatted).unwrap()
        }
        _ => serde_wasm_bindgen::to_value(&match_opt.unwrap()).unwrap(),
    }
}

#[wasm_bindgen]
impl SearchIndex {
    pub fn new() -> SearchIndex {
        utils::set_panic_hook();
        let sample_space = Vec::new();
        SearchIndex { sample_space }
    }

    pub fn load_result(&mut self, result: String) {
        self.sample_space.push(result)
    }

    pub fn search_single_thread(&mut self, input: String, results_length: usize) -> JsValue {
        let sample_space = &self.sample_space;
        let mut results: Vec<(&str, Match)> = Vec::new();

        for sample in sample_space {
            if input.len() > sample.len() {
                continue;
            }
            let match_opt = best_match(&input, sample);
            if match_opt.is_none() {
                continue;
            }
            results.push((sample, match_opt.unwrap()));
        }

        results.sort_by(|(s1, m1), (s2, m2)| {
            (m2.score() + s1.len() as isize)
                .partial_cmp(&(m1.score() + s2.len() as isize))
                .unwrap()
        });

        results.truncate(results_length);

        let formatted_results: Vec<(&str, String, Match)> = results
            .into_iter()
            .map(|(s, match_obj)| {
                let formatted = format_simple(&match_obj, s, "<strong>", "</strong>");
                (s, formatted, match_obj)
            })
            .collect();

        serde_wasm_bindgen::to_value(&formatted_results).unwrap()
    }

    // todo: multi-threading
}
