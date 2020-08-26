import * as wasm from "rust-wasm-v-js";

//wasm.greet();

const heading = wasm.get_element("#main-heading");

heading.textContent = "mmm";

function transform_array_rust_ms() {
  const t0 = performance.now();
  wasm.transform_array();
  const t1 = performance.now();

  return t1 - t0;
}

function transform_array_js() {
  const arr = Array(500000)
    .fill()
    .map(() => 2);

  const arr2 = arr.map((v) => v + 1);
  return arr2;
}

function transform_array__js_ms() {
  const t0 = performance.now();
  transform_array_js();
  const t1 = performance.now();
  return t1 - t0;
}

const js_ms = transform_array__js_ms();
const rust_ms = transform_array_rust_ms();

console.log(`JS took ${js_ms} ms`);
console.log(`Rust took ${rust_ms} ms`);
