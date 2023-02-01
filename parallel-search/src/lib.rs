mod utils;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sublime_fuzzy::{best_match, format_simple, Match};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> u32;
}

#[wasm_bindgen]
pub struct SearchIndex {
    sample_space: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResults {
    count: usize,
    results: Vec<(String, String)>,
}

#[wasm_bindgen(js_name = matchSingle)]
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
    pub fn new() -> Self {
        utils::set_panic_hook();
        let sample_space = Vec::new();
        Self { sample_space }
    }

    #[wasm_bindgen(js_name = loadResult)]
    pub fn load_result(&mut self, result: &str) {
        self.sample_space.push(result.to_owned())
    }

    fn iter_search_chunks(input: String, inner_chunks: Vec<String>) -> Vec<(String, Match)> {
        inner_chunks
            .iter()
            .filter_map(move |sample| {
                if input.len() > sample.len() {
                    return None;
                }
                let match_opt = best_match(&input, &sample);
                if match_opt.is_none() {
                    return None;
                }
                Some((sample.clone(), match_opt.unwrap()))
            })
            .collect()
    }

    // FIXME: this should probably not use static lifetime...
    fn iter_search(
        input: String,
        chunks: Vec<Vec<String>>,
    ) -> impl 'static + ParallelIterator<Item = (String, Match)> {
        chunks.into_par_iter().flat_map_iter(move |inner_chunks| {
            Self::iter_search_chunks(input.clone(), inner_chunks)
        })
    }

    pub fn search(&mut self, input: String, results_length: usize) -> JsValue {
        let sample_space = &self.sample_space;
        let chunks: Vec<_> = sample_space.chunks(1000).map(|s| s.to_vec()).collect();

        let mut results: Vec<(String, Match)> = Self::iter_search(input, chunks).collect();

        results.par_sort_unstable_by(|(s1, m1), (s2, m2)| {
            (m2.score() + s1.len() as isize)
                .partial_cmp(&(m1.score() + s2.len() as isize))
                .unwrap()
        });

        let count = results.len();
        results.truncate(results_length);

        let formatted_results: Vec<(String, String)> = results
            .into_par_iter()
            .map(|(s, match_obj)| {
                let formatted = format_simple(&match_obj, &s, "<strong>", "</strong>");
                (s.to_owned(), formatted)
            })
            .collect();

        let search_results = SearchResults {
            count,
            results: formatted_results,
        };

        serde_wasm_bindgen::to_value(&search_results).unwrap()
    }
}
