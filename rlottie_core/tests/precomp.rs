// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
use rlottie_core::loader::json;

#[test]
fn render_precomp_layer() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/data/precomp.json");
    let data = std::fs::read(path).unwrap();
    let comp = json::from_slice(&data).unwrap();
    let mut buf = vec![0u8; 8 * 8 * 4];
    comp.render_sync(0, &mut buf, 8, 8, 8 * 4);
    let off = 4 * 8 * 4 + 4 * 4;
    assert_eq!(&buf[off..off + 4], &[0, 0, 255, 255]);
}
