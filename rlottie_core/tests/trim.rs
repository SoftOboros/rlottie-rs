// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
use rlottie_core::loader::json;

#[test]
fn parse_trim() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/data/trim.json");
    let data = std::fs::read(path).unwrap();
    let comp = json::from_slice(&data).unwrap();
    if let rlottie_core::types::Layer::Shape(shape) = &comp.layers[0] {
        assert_eq!(shape.trim, Some((0.0, 0.5)));
    } else {
        panic!("expected shape layer");
    }
}
