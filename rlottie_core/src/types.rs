// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
//! Module: type definitions
//! Mirrors: rlottie/src/lottie/lottiemodel.h

use serde::{Deserialize, Serialize};

/// 2D vector used throughout the engine.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Vec2 {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
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
