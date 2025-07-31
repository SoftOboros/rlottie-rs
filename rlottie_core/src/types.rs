// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
//! Module: type definitions
//! Mirrors: rlottie/src/lottie/lottiemodel.h

use serde::{Deserialize, Serialize};

/// 2D vector used throughout the engine.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub struct Vec2 {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
}

/// Fixed-point 2D vector using Q16.16 representation for `no_std` builds.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Vec2Fx {
    /// X coordinate in Q16.16 format
    pub x: i32,
    /// Y coordinate in Q16.16 format
    pub y: i32,
}

impl Vec2Fx {
    /// Scaling factor applied to raw integer values.
    pub const SCALE: i32 = 1 << 16;

    /// Convert from a floating point [`Vec2`].
    pub fn from_vec2(v: Vec2) -> Self {
        Self {
            x: (v.x * Self::SCALE as f32) as i32,
            y: (v.y * Self::SCALE as f32) as i32,
        }
    }

    /// Convert to a floating point [`Vec2`].
    pub fn to_vec2(self) -> Vec2 {
        Vec2 {
            x: self.x as f32 / Self::SCALE as f32,
            y: self.y as f32 / Self::SCALE as f32,
        }
    }
}

/// Transform parameters for a layer or object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    /// Anchor point
    pub anchor: Vec2,
    /// Position vector
    pub position: Vec2,
    /// Scale factor
    pub scale: Vec2,
    /// Rotation in degrees
    pub rotation: f32,
    /// Opacity 0..1
    pub opacity: f32,
}

/// Path drawing commands.
#[derive(Debug, Clone)]
pub enum PathCommand {
    /// Move to absolute position
    MoveTo(Vec2),
    /// Line to absolute position
    LineTo(Vec2),
    /// Cubic Bezier curve
    CubicTo(Vec2, Vec2, Vec2),
    /// Close current sub-path
    Close,
}

/// Vector shape layer.
#[derive(Debug, Clone)]
pub struct ShapeLayer {
    /// Collection of paths within the shape
    pub paths: Vec<Vec<PathCommand>>,
}

/// Placeholder types for other layer kinds.
#[derive(Debug, Clone)]
pub struct ImageLayer;
#[derive(Debug, Clone)]
pub struct PreCompLayer;
#[derive(Debug, Clone)]
pub struct TextLayer;

/// Animation layer variants.
#[derive(Debug, Clone)]
pub enum Layer {
    /// Vector shape layer
    Shape(ShapeLayer),
    /// Bitmap image layer
    Image(ImageLayer),
    /// Pre-composed layer
    PreComp(PreCompLayer),
    /// Text layer
    Text(TextLayer),
}

/// Root composition loaded from JSON.
#[derive(Debug, Clone)]
pub struct Composition {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Frames per second
    pub fps: f32,
    /// Flattened layer list
    pub layers: Vec<Layer>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2fx_roundtrip() {
        let v = Vec2 { x: 1.5, y: -2.25 };
        let fx = Vec2Fx::from_vec2(v);
        let v2 = fx.to_vec2();
        assert!((v.x - v2.x).abs() < 0.0001);
        assert!((v.y - v2.y).abs() < 0.0001);
    }
}
