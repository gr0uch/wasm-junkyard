# Parallel search

Just a little implementation of a multi-threaded parallel search, meant for WebAssembly use. The search algorithm itself uses [ClangdMatcher](https://docs.rs/fuzzy-matcher/0.3.7/fuzzy_matcher/clangd/index.html).

## Dev

Rebuilding binaries:

```
wasm-pack build --target web
```

Running dev server:

```
deno run --allow-net --allow-read web/mod.ts
```
