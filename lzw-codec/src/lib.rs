use fnv::FnvHashMap;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compress_lzw(s: String) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    let mut table: FnvHashMap<Vec<u8>, u32> = (0u32..255).map(|i| (vec![i as u8], i)).collect();

    let mut code: u32 = 256;
    let mut buf: Vec<u8> = vec![];
    for c in s.as_bytes().iter() {
        buf.push(*c);
        if table.get(&buf).is_none() {
            table.insert(buf.clone(), code);
            code += 1;
            buf.pop();
            result.push(table[&buf]);
            buf = vec![*c];
        }
    }

    if !buf.is_empty() {
        result.push(*table.get(&buf).unwrap());
    }

    result
}

#[wasm_bindgen]
pub fn decompress_lzw(p: Vec<u32>) -> String {
    let mut result: Vec<u8> = vec![];
    let mut table: FnvHashMap<u32, Vec<u8>> = (0u32..255).map(|i| (i, vec![i as u8])).collect();

    let mut iter = p.iter();
    let mut prev = iter.next().unwrap();
    let mut chunk = &table[&prev];
    let mut current_char = chunk[0];
    result.extend(chunk);
    let mut code: u32 = 256;
    for c in iter {
        let mut new_chunk;
        if table.get(c).is_none() {
            new_chunk = table[&prev].clone();
            new_chunk.push(current_char);
            chunk = &new_chunk;
        } else {
            chunk = &table[&c];
        }
        result.extend(chunk);
        current_char = chunk[0];
        let mut next_chunk = table[&prev].clone();
        next_chunk.push(current_char);
        table.insert(code, next_chunk);
        code += 1;
        prev = c;
    }

    String::from_utf8(result).unwrap()
}
