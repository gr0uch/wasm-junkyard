mod utils;

use fuzzy_matcher::clangd::ClangdMatcher;
use fuzzy_matcher::FuzzyMatcher;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
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

    fn iter_search_chunks(
        input: String,
        inner_chunks: Vec<String>,
    ) -> Vec<(String, (i64, Vec<usize>))> {
        let matcher = ClangdMatcher::default();
        inner_chunks
            .iter()
            .filter_map(move |sample| {
                let input_len = input.len();
                if input_len > sample.len() || input_len == 0 {
                    return None;
                }
                let match_opt = matcher.fuzzy_indices(&sample, &input);
                if match_opt.is_none() {
                    return None;
                }
                Some((sample.clone(), match_opt.unwrap()))
            })
            .collect()
    }

    fn iter_search<'a>(
        input: String,
        chunks: Vec<Vec<String>>,
    ) -> impl 'a + ParallelIterator<Item = (String, (i64, Vec<usize>))> {
        chunks.into_par_iter().flat_map_iter(move |inner_chunks| {
            Self::iter_search_chunks(input.clone(), inner_chunks)
        })
    }

    pub fn search(&mut self, input: String, results_length: usize) -> JsValue {
        let sample_space = &self.sample_space;

        // Choosing a chunk size is important for parallelization. It has to be
        // high enough to overcome the overhead cost.
        let chunks: Vec<_> = sample_space.chunks(60_000).map(|s| s.to_vec()).collect();

        let mut results: Vec<(String, (i64, Vec<usize>))> =
            Self::iter_search(input, chunks).collect();

        results.par_sort_unstable_by(|(s1, m1), (s2, m2)| {
            (m2.0 + s1.len() as i64)
                .partial_cmp(&(m1.0 + s2.len() as i64))
                .unwrap()
        });

        let count = results.len();
        results.truncate(results_length);

        let formatted_results: Vec<(String, String)> = results
            .into_iter()
            .map(|(s, (_, indices))| (s.clone(), Self::format_result(s, indices)))
            .collect();

        let search_results = SearchResults {
            count,
            results: formatted_results,
        };

        serde_wasm_bindgen::to_value(&search_results).unwrap()
    }

    fn format_result(s: String, indices: Vec<usize>) -> String {
        let mut formatted = String::new();
        let mut is_open = false;
        for (i, c) in s.char_indices() {
            if indices.contains(&i) && !is_open {
                formatted.push_str("<strong>");
                is_open = true;
            } else if is_open && !indices.contains(&i) {
                formatted.push_str("</strong>");
                is_open = false;
            }
            formatted.push(c);
        }
        if is_open {
            formatted.push_str("</strong>");
        }
        formatted
    }
}
