// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
//! Module: JSON composition loader
//! Mirrors: rlottie/src/lottie/lottiecomposition.cpp

use base64::{engine::general_purpose, Engine as _};
use image::ImageReader;
use crate::types::{Color, Composition, Layer, ImageLayer, PathCommand, MatteType, PathCommand, ShapeLayer, Vec2};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;

/// Load a composition from a reader containing Lottie JSON.
pub fn from_reader<R: Read>(mut reader: R) -> Result<Composition, Box<dyn std::error::Error>> {
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    let root: Value = serde_json::from_str(&s)?;
    let width = root.get("w").and_then(Value::as_u64).unwrap_or(0) as u32;
    let height = root.get("h").and_then(Value::as_u64).unwrap_or(0) as u32;
    let start = root.get("ip").and_then(Value::as_f64).unwrap_or(0.0) as u32;
    let end = root.get("op").and_then(Value::as_f64).unwrap_or(0.0) as u32;
    let fps = root.get("fr").and_then(Value::as_f64).unwrap_or(0.0) as f32;
    let mut images: HashMap<String, (u32, u32, Vec<u8>)> = HashMap::new();
    if let Some(asset_arr) = root.get("assets").and_then(Value::as_array) {
        for asset in asset_arr {
            if let Some(id) = asset.get("id").and_then(Value::as_str) {
                if let Some(p) = asset.get("p").and_then(Value::as_str) {
                    let width_a = asset.get("w").and_then(Value::as_u64).unwrap_or(0) as u32;
                    let height_a = asset.get("h").and_then(Value::as_u64).unwrap_or(0) as u32;
                    let bytes = if asset.get("e").and_then(Value::as_i64) == Some(1) {
                        if let Some(idx) = p.find(',') {
                            let mut b64 = p[idx + 1..].trim();
                            while b64.len() % 4 != 0 {
                                b64 = &b64[..b64.len() - 1];
                            }
                            general_purpose::STANDARD.decode(b64)?
                        } else {
                            Vec::new()
                        }
                    } else {
                        let mut path = String::new();
                        if let Some(u) = asset.get("u").and_then(Value::as_str) {
                            path.push_str(u);
                        }
                        path.push_str(p);
                        fs::read(Path::new(&path))?
                    };
                    if !bytes.is_empty() {
                        let img = ImageReader::new(std::io::Cursor::new(bytes))
                            .with_guessed_format()?
                            .decode()?
                            .to_rgba8();
                        images.insert(id.to_string(), (width_a, height_a, img.into_raw()));
                    }
                }
            }
        }
    }

    let mut layers = Vec::new();
    if let Some(layer_arr) = root.get("layers").and_then(Value::as_array) {
        for layer in layer_arr {
            if layer.get("ty").and_then(Value::as_i64) == Some(4) {
                let mut paths = Vec::new();
                let mut fill = None;
                let mut stroke = None;
                let mut stroke_width = 1.0;
                let is_mask = layer.get("td").and_then(Value::as_i64) == Some(1);
                let matte = match layer.get("tt").and_then(Value::as_i64) {
                    Some(1) => Some(MatteType::Alpha),
                    Some(2) => Some(MatteType::AlphaInv),
                    _ => None,
                };
                let mut trim: Option<(f32, f32)> = None;
                if let Some(shape_arr) = layer.get("shapes").and_then(Value::as_array) {
                    for shape in shape_arr {
                        if let Some(ty) = shape.get("ty").and_then(Value::as_str) {
                            match ty {
                                "sh" => {
                                    if let Some(d) = shape
                                        .get("ks")
                                        .and_then(|k| k.get("d"))
                                        .and_then(Value::as_str)
                                    {
                                        paths.push(parse_path(d));
                                    }
                                }
                                "fl" => {
                                    fill = parse_color(shape);
                                }
                                "st" => {
                                    stroke = parse_color(shape);
                                    if let Some(w) = shape
                                        .get("w")
                                        .and_then(|k| k.get("k"))
                                        .and_then(Value::as_f64)
                                    {
                                        stroke_width = w as f32;
                                    }
                                }
                                "tm" => {
                                    let s = shape
                                        .get("s")
                                        .and_then(|v| v.get("k"))
                                        .and_then(Value::as_f64)
                                        .unwrap_or(0.0)
                                        as f32
                                        / 100.0;
                                    let e = shape
                                        .get("e")
                                        .and_then(|v| v.get("k"))
                                        .and_then(Value::as_f64)
                                        .unwrap_or(1.0)
                                        as f32
                                        / 100.0;
                                    trim = Some((s, e));
                                }
                                _ => {}
                            }
                        }
                    }
                }
                layers.push(Layer::Shape(ShapeLayer {
                    paths,
                    fill,
                    stroke,
                    stroke_width,
                    mask: None,
                    animators: HashMap::new(),
                    is_mask,
                    matte,
                    trim,
                }));
            } else if layer.get("ty").and_then(Value::as_i64) == Some(2) {
                if let Some(id) = layer.get("refId").and_then(Value::as_str) {
                    if let Some((w, h, data)) = images.get(id).cloned() {
                        layers.push(Layer::Image(ImageLayer {
                            width: w,
                            height: h,
                            pixels: data,
                        }));
                    }
                }
            }
        }
    }
    let layers = root
        .get("layers")
        .and_then(Value::as_array)
        .map(|arr| parse_layers(arr, &assets, width, height, fps))
        .unwrap_or_default();
    Ok(Composition {
        width,
        height,
        start_frame: start,
        end_frame: end,
        fps,
        layers,
    })
}

/// Load a composition directly from a byte slice containing Lottie JSON.
pub fn from_slice(data: &[u8]) -> Result<Composition, Box<dyn std::error::Error>> {
    let cursor = std::io::Cursor::new(data);
    from_reader(cursor)
}

fn parse_layers(
    arr: &[Value],
    assets: &HashMap<String, Value>,
    width: u32,
    height: u32,
    fps: f32,
) -> Vec<Layer> {
    let mut out = Vec::new();
    for layer in arr {
        if let Some(l) = parse_layer(layer, assets, width, height, fps) {
            out.push(l);
        }
    }
    out
}

fn parse_layer(
    layer: &Value,
    assets: &HashMap<String, Value>,
    width: u32,
    height: u32,
    fps: f32,
) -> Option<Layer> {
    match layer.get("ty").and_then(Value::as_i64)? {
        4 => {
            let mut paths = Vec::new();
            let mut fill = None;
            let mut stroke = None;
            let mut stroke_width = 1.0;
            if let Some(shape_arr) = layer.get("shapes").and_then(Value::as_array) {
                for shape in shape_arr {
                    if let Some(ty) = shape.get("ty").and_then(Value::as_str) {
                        match ty {
                            "sh" => {
                                if let Some(d) = shape
                                    .get("ks")
                                    .and_then(|k| k.get("d"))
                                    .and_then(Value::as_str)
                                {
                                    paths.push(parse_path(d));
                                }
                            }
                            "fl" => fill = parse_color(shape),
                            "st" => {
                                stroke = parse_color(shape);
                                if let Some(w) = shape
                                    .get("w")
                                    .and_then(|k| k.get("k"))
                                    .and_then(Value::as_f64)
                                {
                                    stroke_width = w as f32;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Some(Layer::Shape(ShapeLayer {
                paths,
                fill,
                stroke,
                stroke_width,
                animators: HashMap::new(),
            }))
        }
        0 => {
            let ref_id = layer.get("refId").and_then(Value::as_str)?;
            if let Some(asset) = assets.get(ref_id) {
                if let Some(arr) = asset.get("layers").and_then(Value::as_array) {
                    let comp = Composition {
                        width,
                        height,
                        start_frame: 0,
                        end_frame: 0,
                        fps,
                        layers: parse_layers(arr, assets, width, height, fps),
                    };
                    return Some(Layer::PreComp(PreCompLayer {
                        comp: Box::new(comp),
                    }));
                }
            }
            None
        }
        _ => None,
    }
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

fn parse_color(obj: &Value) -> Option<Color> {
    if let Some(arr) = obj
        .get("c")
        .and_then(|c| c.get("k"))
        .and_then(Value::as_array)
    {
        if arr.len() >= 4 {
            let r = arr[0].as_f64().unwrap_or(0.0);
            let g = arr[1].as_f64().unwrap_or(0.0);
            let b = arr[2].as_f64().unwrap_or(0.0);
            let a = arr[3].as_f64().unwrap_or(1.0);
            return Some(Color {
                r: (r * 255.0) as u8,
                g: (g * 255.0) as u8,
                b: (b * 255.0) as u8,
                a: (a * 255.0) as u8,
            });
        }
    }
    None
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

    #[test]
    fn parse_fill_stroke() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/data/fill_stroke.json");
        let file = File::open(path).unwrap();
        let comp = from_reader(file).unwrap();
        if let Layer::Shape(shape) = &comp.layers[0] {
            assert!(shape.fill.is_some());
            assert!(shape.stroke.is_some());
        } else {
            panic!("expected shape layer");
        }
    }

    #[test]
    fn parse_embedded_image() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../tests/data/image_embedded.json");
        let file = File::open(path).unwrap();
        let comp = from_reader(file).unwrap();
        if let Layer::Image(img) = &comp.layers[0] {
            assert_eq!(img.width, 1);
            assert_eq!(img.height, 1);
            assert_eq!(img.pixels.len(), 4);
        } else {
            panic!("expected image layer");
        }
    }
}
