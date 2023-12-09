import init, { BloomFilter } from "../pkg/bloom_filter.js";

await init();
const MAX = 10000;
const FP = 0.00001;
const filter = BloomFilter.new(MAX, FP);
for (let i = 0; i < MAX; i++) {
  filter.set(`foo${i}`);
}

const feedback = document.createElement("strong");
const input = document.createElement("input");
input.placeholder = "foo<N>"
input.addEventListener("input", (e) => {
  const { target: { value } } = e;
  feedback.textContent = filter.check(value);
});
document.body.appendChild(input);
document.body.appendChild(feedback);

const pre = document.createElement("pre");
const config = filter.export();
pre.textContent = `items: ${MAX}
false positive rate: ${FP}
---
bit_vec length: ${config.bit_vec.length}
bits: ${config.bitmap_bits}
k_num: ${config.k_num}
sip_keys: ${config.sip_keys}`;
document.body.appendChild(pre);
