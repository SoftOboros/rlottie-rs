// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
//! Text rendering test

use fontdue::Font;
use rlottie_core::types::{Color, Composition, Layer, TextLayer, Vec2};
use std::sync::Arc;

#[test]
fn render_simple_text() {
    let font_bytes = std::fs::read("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf").unwrap();
    let font = Arc::new(Font::from_bytes(font_bytes, fontdue::FontSettings::default()).unwrap());
    let layer = TextLayer {
        text: "A".to_string(),
        color: Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },
        size: 32.0,
        position: Vec2 { x: 0.0, y: 32.0 },
        font,
    };
    let comp = Composition {
        width: 64,
        height: 64,
        start_frame: 0,
        end_frame: 0,
        fps: 60.0,
        layers: vec![Layer::Text(layer)],
    };
    let mut buf = vec![0u8; 64 * 64 * 4];
    comp.render_sync(0, &mut buf, 64, 64, 64 * 4);
    assert!(buf.iter().any(|&b| b != 0));
}
