import * as Comlink from "./comlink.mjs";
import init, { /*match_single,*/ SearchIndex } from "../pkg/multi_search.js";

const internalMap = new Map();

const api = {
  async init() {
    await init();
  },

  SearchIndex() {
    const index = SearchIndex.new();
    internalMap.set(index.ptr, index);
    return index.ptr;
  },

  async fetchResults(ptr, url) {
    const t0 = Date.now();
    const index = internalMap.get(ptr);
    const res = await fetch(url);
    const json = await res.json();
    console.log(`fetched json in ${Date.now() - t0} ms`);

    const t1 = Date.now();
    let i = 0;
    for (const word in json) {
      index.load_result(word);
      i++;
    }
    console.log(`loaded ${i} results in ${Date.now() - t1} ms`);
  },

  searchSingleThread(ptr, value, len) {
    const index = internalMap.get(ptr);
    return index.search_single_thread(value, len);
  }
};

Comlink.expose(api);