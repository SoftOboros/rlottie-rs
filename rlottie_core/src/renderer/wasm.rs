// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
//! Module: wasm renderer
//! Mirrors: rlottie/src/wasm/rlottiewasm.cpp (simplified)

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use wasm_bindgen::Clamped;
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use web_sys::ImageData;

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use crate::{
    geometry::Path,
    loader::json,
    renderer::cpu,
    types::{Color, Layer, Paint, PathCommand},
};

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen]
pub struct RlottieWasm {
    comp: crate::types::Composition,
    buffer: Vec<u8>,
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[wasm_bindgen]
impl RlottieWasm {
    /// Create a new renderer from Lottie JSON data.
    #[wasm_bindgen(constructor)]
    pub fn new(data: &str) -> Result<RlottieWasm, JsValue> {
        let comp =
            json::from_slice(data.as_bytes()).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(Self {
            comp,
            buffer: Vec::new(),
        })
    }

    /// Number of frames in the animation.
    #[wasm_bindgen]
    pub fn frames(&self) -> u32 {
        // Composition does not yet expose duration, so assume single frame.
        1
    }

    /// Render a specific frame into a new [`ImageData`].
    #[wasm_bindgen]
    pub fn render(&mut self, _frame: u32, width: u32, height: u32) -> Result<ImageData, JsValue> {
        let len = (width * height * 4) as usize;
        self.buffer.clear();
        self.buffer.resize(len, 0);

        for layer in &self.comp.layers {
            if let Layer::Shape(shape) = layer {
                for path_cmds in &shape.paths {
                    let mut path = Path::new();
                    for cmd in path_cmds {
                        match *cmd {
                            PathCommand::MoveTo(p) => path.move_to(p),
                            PathCommand::LineTo(p) => path.line_to(p),
                            PathCommand::CubicTo(c1, c2, p) => path.cubic_to(c1, c2, p),
                            PathCommand::Close => path.close(),
                        }
                    }
                    cpu::draw_path(
                        &path,
                        Paint::Solid(Color {
                            r: 0,
                            g: 0,
                            b: 0,
                            a: 255,
                        }),
                        &mut self.buffer,
                        width as usize,
                        height as usize,
                        (width * 4) as usize,
                    );
                }
            }
        }

        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&self.buffer), width, height)
            .map_err(|e| e)
    }
}

#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
pub struct RlottieWasm;

#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
impl RlottieWasm {
    /// Stub constructor when compiled for non-wasm targets.
    pub fn new(_data: &str) -> Result<Self, &'static str> {
        Err("wasm feature requires wasm32 target")
    }
}
