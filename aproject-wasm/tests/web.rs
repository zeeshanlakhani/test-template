//! Test suite for the Web and headless browsers.

use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn test_add() {
    assert_eq!(aproject_wasm::add(3, 2), 5);
    aproject_wasm::console_log!("{}", "Test passes!");
}
