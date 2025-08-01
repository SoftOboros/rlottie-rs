// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
//! Module: JSON composition loader
//! Mirrors: rlottie/src/lottie/lottiecomposition.cpp

use crate::types::{Composition, Layer, PathCommand, ShapeLayer, Vec2};
use serde_json::Value;
use std::io::Read;

/// Load a composition from a reader containing Lottie JSON.
pub fn from_reader<R: Read>(mut reader: R) -> Result<Composition, Box<dyn std::error::Error>> {
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    let root: Value = serde_json::from_str(&s)?;
    let width = root.get("w").and_then(Value::as_u64).unwrap_or(0) as u32;
    let height = root.get("h").and_then(Value::as_u64).unwrap_or(0) as u32;
    let fps = root.get("fr").and_then(Value::as_f64).unwrap_or(0.0) as f32;
    let mut layers = Vec::new();
    if let Some(layer_arr) = root.get("layers").and_then(Value::as_array) {
        for layer in layer_arr {
            if layer.get("ty").and_then(Value::as_i64) == Some(4) {
                let mut paths = Vec::new();
                if let Some(shape_arr) = layer.get("shapes").and_then(Value::as_array) {
                    for shape in shape_arr {
                        if shape.get("ty").and_then(Value::as_str) == Some("sh") {
                            if let Some(d) = shape
                                .get("ks")
                                .and_then(|k| k.get("d"))
                                .and_then(Value::as_str)
                            {
                                paths.push(parse_path(d));
                            }
                        }
                    }
                }
                layers.push(Layer::Shape(ShapeLayer { paths }));
            }
        }
    }
    Ok(Composition {
        width,
        height,
        fps,
        layers,
    })
}

/// Load a composition directly from a byte slice containing Lottie JSON.
pub fn from_slice(data: &[u8]) -> Result<Composition, Box<dyn std::error::Error>> {
    let cursor = std::io::Cursor::new(data);
    from_reader(cursor)
}

/// Parse a simple path string using m/l/c/o verbs.
fn parse_path(data: &str) -> Vec<PathCommand> {
    let mut cmds = Vec::new();
    let mut it = data.split_whitespace();
    while let Some(tok) = it.next() {
        match tok {
            "m" => {
                let x: f32 = it.next().unwrap_or("0").parse().unwrap_or(0.0);
                let y: f32 = it.next().unwrap_or("0").parse().unwrap_or(0.0);
                cmds.push(PathCommand::MoveTo(Vec2 { x, y }));
            }
            "l" => {
                let x: f32 = it.next().unwrap_or("0").parse().unwrap_or(0.0);
                let y: f32 = it.next().unwrap_or("0").parse().unwrap_or(0.0);
                cmds.push(PathCommand::LineTo(Vec2 { x, y }));
            }
            "c" => {
                let x1: f32 = it.next().unwrap_or("0").parse().unwrap_or(0.0);
                let y1: f32 = it.next().unwrap_or("0").parse().unwrap_or(0.0);
                let x2: f32 = it.next().unwrap_or("0").parse().unwrap_or(0.0);
                let y2: f32 = it.next().unwrap_or("0").parse().unwrap_or(0.0);
                let x: f32 = it.next().unwrap_or("0").parse().unwrap_or(0.0);
                let y: f32 = it.next().unwrap_or("0").parse().unwrap_or(0.0);
                cmds.push(PathCommand::CubicTo(
                    Vec2 { x: x1, y: y1 },
                    Vec2 { x: x2, y: y2 },
                    Vec2 { x, y },
                ));
            }
            "o" => cmds.push(PathCommand::Close),
            _ => {}
        }
    }
    cmds
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn parse_min_shape() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/data/min_shape.json");
        let file = File::open(path).unwrap();
        let comp = from_reader(file).unwrap();
        assert_eq!(comp.layers.len(), 1);
    }

    #[test]
    fn from_slice_matches_reader() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/data/min_shape.json");
        let mut bytes = Vec::new();
        File::open(&path).unwrap().read_to_end(&mut bytes).unwrap();
        let from_reader_comp = from_reader(File::open(&path).unwrap()).unwrap();
        let from_slice_comp = from_slice(&bytes).unwrap();
        assert_eq!(from_reader_comp.layers.len(), from_slice_comp.layers.len());
    }
}
