// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
use rlottie_core::loader::json;

#[test]
fn render_repeater() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/data/repeater.json");
    let data = std::fs::read(path).unwrap();
    let comp = json::from_slice(&data).unwrap();
    let mut buf = vec![0u8; 8 * 4 * 4];
    comp.render_sync(0, &mut buf, 8, 4, 8 * 4);
    // third copy should affect pixel around x=5,y=1
    let idx = 1 * 8 * 4 + 5 * 4;
    assert_eq!(&buf[idx..idx + 4], &[0, 0, 0, 255]);
}
