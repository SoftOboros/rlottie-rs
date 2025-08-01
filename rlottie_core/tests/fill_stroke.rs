// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
use rlottie_core::loader::json;

#[test]
fn render_fill_and_stroke() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/data/fill_stroke.json");
    let data = std::fs::read(path).unwrap();
    let comp = json::from_slice(&data).unwrap();
    let mut buf = vec![0u8; 8 * 8 * 4];
    comp.render_sync(0, &mut buf, 8, 8, 8 * 4);
    // inside pixel should be blue fill
    let inside = 4 * 8 * 4 + 4 * 4;
    assert_eq!(&buf[inside..inside + 4], &[0, 0, 255, 255]);
    // border pixel should be red stroke
    let border = 1 * 8 * 4 + 1 * 4;
    assert_eq!(&buf[border..border + 4], &[255, 0, 0, 255]);
}
