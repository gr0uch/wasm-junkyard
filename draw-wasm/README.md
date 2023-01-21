# draw-wasm

Just a simple little wrapper around `image`, `imageproc`, `rusttype` intended for WebAssembly usage from Node.js or any headless JS runtime.

![test image](/draw-wasm/tests/test-1.png?raw=true)


## Features

- [x] Fill rectangle with solid color.
- [x] Draw PNGs as blended sprites.
- [x] Draw text.
- [x] Export PNG image.
- [ ] Do a barrel roll.


## Usage

See auto-generated TS annotations.


## Benchmark

```
makeImgDrawWasm done in 105 ms
makeImgSatori done in 7708 ms
```


## Dev

Build:

```
wasm-pack build
```

Test:

```
wasm-pack test --node
```


## License

MIT
