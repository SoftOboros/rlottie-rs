// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
use rlottie_core::loader::json;

#[test]
fn frame_looping() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/data/fill_stroke.json");
    let data = std::fs::read(path).unwrap();
    let comp = json::from_slice(&data).unwrap();
    assert_eq!(comp.frame_at(0), comp.start_frame);
    // end_frame is 10 in fixture
    assert_eq!(comp.frame_at(12), comp.start_frame + 1);
}
