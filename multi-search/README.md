# Multi-search

Just a little implementation of a multi-threaded search, meant for WebAssembly
use. The search algorithm itself uses an informally specified version of Sublime
Text's search function.

## Dev

Rebuilding binaries:

```
wasm-pack build --target web
```

Running dev server:

```
deno run --allow-net --allow-read web/mod.ts
```
