let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

/**
 * @param {bigint} n
 * @returns {bigint}
 */
export function fib(n) {
    const ret = wasm.fib(n);
    return BigInt.asUintN(64, ret);
}

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_0;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

