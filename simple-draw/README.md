# simple-draw

Just a simple little wrapper around `image`, `imageproc`, `rusttype` intended for WebAssembly usage from Node.js or any headless JS runtime.

![test image](/simple-draw/tests/test-1.png?raw=true)


## Features

- [x] Fill rectangle with solid color.
- [x] Draw PNGs and WebPs as blended sprites, and optionally resize them.
- [x] Draw text with custom fonts, sizes, & colors.
- [x] Export PNG image.
- [ ] Do a barrel roll.


## Usage

See [benchmark script](/simple-draw/benchmark/index.mjs) and auto-generated TS annotations.


## Benchmark

```
makeImgSimpleDraw done in 105 ms
makeImgSatori done in 7708 ms
```


## Dev

Build:

```
wasm-pack build --target nodejs
```

Test:

```
wasm-pack test --node
```


## License

MIT
