import * as wasm from "./rust_wasm_bg.wasm";
export * from "./rust_wasm_bg.js";
import { __wbg_set_wasm } from "./rust_wasm_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
