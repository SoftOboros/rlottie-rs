#![cfg(target_arch = "wasm32")]
use rlottie_core::renderer::wasm::RlottieWasm;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn render_imagedata_dimensions() {
    let json = include_str!("../data/min_shape.json");
    let mut r = RlottieWasm::new(json).unwrap();
    let img = r.render(0, 16, 16).unwrap();
    assert_eq!(img.width(), 16);
    assert_eq!(img.height(), 16);
}
