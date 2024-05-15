# lzw-codec

Homebrew LZW codec. Notes on implementation:

- **Works with UTF-8 strings only** (theoretically also works with arbitrary byte sequences, but the decompression assumes UTF-8 output).
- This uses [FNV hasher](https://crates.io/crates/fnv) for the lookup table, which is slightly more efficient than the default SipHash 1-3.
- This uses a `Vec<u32>` as the output of compression and input for decompression. Due to the ceiling (`2^32`), it can possibly overflow.
- `wee_alloc` is used for smaller code size.
- There's an [example on Rosetta Code](https://rosettacode.org/wiki/LZW_compression#Rust) that clones the current `Vec<u8>` on each iteration during compression, this avoids that.

## Dev

Rebuilding binaries:

```sh
wasm-pack build --target web
```

Test:

```sh
wasm-pack test
```
