import * as wasm from "rust-wasm-v-js";

//wasm.greet();

const heading = wasm.get_element("#main-heading");

heading.textContent = "mmm";
