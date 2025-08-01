// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
//! Module: type definitions
//! Mirrors: rlottie/src/lottie/lottiemodel.h

use crate::timeline::Animator;
use fontdue::Font;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

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
/// RGBA color in 8-bit per channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    /// Red channel
    pub r: u8,
    /// Green channel
    pub g: u8,
    /// Blue channel
    pub b: u8,
    /// Alpha channel
    pub a: u8,
}

/// A color stop used in gradients.
#[derive(Debug, Clone, Copy)]
pub struct GradientStop {
    /// Offset along the gradient 0..1
    pub offset: f32,
    /// Color at this stop
    pub color: Color,
}

/// Linear gradient parameters.
#[derive(Debug, Clone)]
pub struct LinearGradient {
    /// Start position in object space
    pub start: Vec2,
    /// End position in object space
    pub end: Vec2,
    /// Color stops sorted by offset
    pub stops: Vec<GradientStop>,
}

/// Radial gradient parameters.
#[derive(Debug, Clone)]
pub struct RadialGradient {
    /// Center of the gradient
    pub center: Vec2,
    /// Radius of the gradient
    pub radius: f32,
    /// Color stops sorted by offset
    pub stops: Vec<GradientStop>,
}

/// Paint style for filling paths.
#[derive(Debug, Clone)]
pub enum Paint {
    /// Solid color fill
    Solid(Color),
    /// Linear gradient fill
    Linear(LinearGradient),
    /// Radial gradient fill
    Radial(RadialGradient),
}

/// Type of matte compositing to apply with the previous mask layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatteType {
    /// Use the alpha of the mask as-is.
    Alpha,
    /// Use the inverse of the mask alpha.
    AlphaInv,
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
    /// Property animations keyed by name
    #[serde(skip)]
    pub animators: HashMap<&'static str, Animator<f32>>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            anchor: Vec2::default(),
            position: Vec2::default(),
            scale: Vec2 { x: 1.0, y: 1.0 },
            rotation: 0.0,
            opacity: 1.0,
            animators: HashMap::new(),
        }
    }
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
#[derive(Debug, Clone, Default)]
pub struct ShapeLayer {
    /// Collection of paths within the shape
    pub paths: Vec<Vec<PathCommand>>,
    /// Fill color if present
    pub fill: Option<Color>,
    /// Stroke color if present
    pub stroke: Option<Color>,
    /// Stroke width in pixels
    pub stroke_width: f32,
    /// Optional mask paths to clip this shape
    pub mask: Option<Vec<Vec<PathCommand>>>,
    /// Optional trim start/end fractions
    pub trim: Option<(f32, f32)>,
    /// Animations for fill or stroke properties
    pub animators: HashMap<&'static str, Animator<f32>>,
    /// If true this layer acts as a matte for the next layer
    pub is_mask: bool,
    /// Matte mode applied using the previous mask layer
    pub matte: Option<MatteType>,
}

/// Bitmap image layer decoded from assets.
#[derive(Debug, Clone)]
pub struct ImageLayer {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Raw RGBA8888 pixel data
    pub pixels: Vec<u8>,
}
#[derive(Debug, Clone)]
pub struct PreCompLayer {
    /// Nested composition to render
    pub comp: Box<Composition>,
}

#[derive(Debug, Clone)]
pub struct TextLayer {
    /// UTF-8 string to render
    pub text: String,
    /// Text color
    pub color: Color,
    /// Font size in pixels
    pub size: f32,
    /// Baseline position of the text
    pub position: Vec2,
    /// Font used for rasterization
    pub font: Arc<Font>,
}

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
    /// First frame of the animation
    pub start_frame: u32,
    /// Last frame of the animation
    pub end_frame: u32,
    /// Frames per second
    pub fps: f32,
    /// Flattened layer list
    pub layers: Vec<Layer>,
}

impl Composition {
    /// Calculate the actual frame index after applying start/end offsets and looping.
    pub fn frame_at(&self, frame: u32) -> u32 {
        let total = self.end_frame.saturating_sub(self.start_frame) + 1;
        let local = frame % total;
        self.start_frame + local
    }

    /// Render a frame into the provided RGBA8888 buffer.
    pub fn render_sync(
        &self,
        frame: u32,
        buffer: &mut [u8],
        width: usize,
        height: usize,
        stride: usize,
    ) {
        use crate::geometry::Path;
        use crate::renderer::cpu::{
            blend_masked, draw_mask, draw_path, draw_path_masked, draw_stroke, draw_stroke_masked,
            draw_text,
        };
        use crate::types::{Color, Paint, Vec2};

        let _frame_no = self.frame_at(frame);
        buffer.fill(0);
        let sx = width as f32 / self.width as f32;
        let sy = height as f32 / self.height as f32;

        let mut mask_buf = vec![0u8; width * height * 4];
        let mut layer_buf = vec![0u8; buffer.len()];
        let mut have_mask = false;

        for layer in &self.layers {
            match layer {
                Layer::Shape(shape) => {
                    if shape.is_mask {
                        mask_buf.fill(0);
                        for cmds in &shape.paths {
                            let mut path = Path::new();
                            for cmd in cmds {
                                match *cmd {
                                    PathCommand::MoveTo(p) => path.move_to(Vec2 {
                                        x: p.x * sx,
                                        y: p.y * sy,
                                    }),
                                    PathCommand::LineTo(p) => path.line_to(Vec2 {
                                        x: p.x * sx,
                                        y: p.y * sy,
                                    }),
                                    PathCommand::CubicTo(c1, c2, p) => path.cubic_to(
                                        Vec2 {
                                            x: c1.x * sx,
                                            y: c1.y * sy,
                                        },
                                        Vec2 {
                                            x: c2.x * sx,
                                            y: c2.y * sy,
                                        },
                                        Vec2 {
                                            x: p.x * sx,
                                            y: p.y * sy,
                                        },
                                    ),
                                    PathCommand::Close => path.close(),
                                }
                            }
                            draw_mask(&path, &mut mask_buf, width, height);
                        }
                        have_mask = true;
                        continue;
                    }

                    let mut local_mask = None;
                    if let Some(mask_paths) = &shape.mask {
                        let mut buf_m = vec![0u8; buffer.len()];
                        for cmds in mask_paths {
                            let mut mask_path = Path::new();
                            for cmd in cmds {
                                match *cmd {
                                    PathCommand::MoveTo(p) => mask_path.move_to(Vec2 {
                                        x: p.x * sx,
                                        y: p.y * sy,
                                    }),
                                    PathCommand::LineTo(p) => mask_path.line_to(Vec2 {
                                        x: p.x * sx,
                                        y: p.y * sy,
                                    }),
                                    PathCommand::CubicTo(c1, c2, p) => mask_path.cubic_to(
                                        Vec2 {
                                            x: c1.x * sx,
                                            y: c1.y * sy,
                                        },
                                        Vec2 {
                                            x: c2.x * sx,
                                            y: c2.y * sy,
                                        },
                                        Vec2 {
                                            x: p.x * sx,
                                            y: p.y * sy,
                                        },
                                    ),
                                    PathCommand::Close => mask_path.close(),
                                }
                            }
                            draw_path(
                                &mask_path,
                                Paint::Solid(Color {
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                    a: 255,
                                }),
                                &mut buf_m,
                                width,
                                height,
                                stride,
                            );
                        }
                        local_mask = Some(buf_m);
                    }

                    for cmds in &shape.paths {
                        let mut path = Path::new();
                        for cmd in cmds {
                            match *cmd {
                                PathCommand::MoveTo(p) => path.move_to(Vec2 {
                                    x: p.x * sx,
                                    y: p.y * sy,
                                }),
                                PathCommand::LineTo(p) => path.line_to(Vec2 {
                                    x: p.x * sx,
                                    y: p.y * sy,
                                }),
                                PathCommand::CubicTo(c1, c2, p) => path.cubic_to(
                                    Vec2 {
                                        x: c1.x * sx,
                                        y: c1.y * sy,
                                    },
                                    Vec2 {
                                        x: c2.x * sx,
                                        y: c2.y * sy,
                                    },
                                    Vec2 {
                                        x: p.x * sx,
                                        y: p.y * sy,
                                    },
                                ),
                                PathCommand::Close => path.close(),
                            }
                        }
                        let render_path = if let Some((s, e)) = shape.trim {
                            path.trim(s, e, 0.2)
                        } else {
                            path.clone()
                        };

                        if let Some(fill) = shape.fill {
                            if have_mask && shape.matte.is_some() {
                                draw_path(
                                    &render_path,
                                    Paint::Solid(fill),
                                    &mut layer_buf,
                                    width,
                                    height,
                                    stride,
                                );
                            } else if let Some(mask) = local_mask.as_ref() {
                                draw_path_masked(
                                    &render_path,
                                    Paint::Solid(fill),
                                    mask,
                                    buffer,
                                    width,
                                    height,
                                    stride,
                                );
                            } else {
                                draw_path(
                                    &render_path,
                                    Paint::Solid(fill),
                                    buffer,
                                    width,
                                    height,
                                    stride,
                                );
                            }
                        }

                        if let Some(stroke) = shape.stroke {
                            if have_mask && shape.matte.is_some() {
                                draw_stroke(
                                    &render_path,
                                    shape.stroke_width,
                                    Paint::Solid(stroke),
                                    &mut layer_buf,
                                    width,
                                    height,
                                    stride,
                                );
                            } else if let Some(mask) = local_mask.as_ref() {
                                draw_stroke_masked(
                                    &render_path,
                                    shape.stroke_width,
                                    Paint::Solid(stroke),
                                    mask,
                                    buffer,
                                    width,
                                    height,
                                    stride,
                                );
                            } else {
                                draw_stroke(
                                    &render_path,
                                    shape.stroke_width,
                                    Paint::Solid(stroke),
                                    buffer,
                                    width,
                                    height,
                                    stride,
                                );
                            }
                        }
                    }

                    if have_mask {
                        if let Some(m) = shape.matte {
                            blend_masked(buffer, &layer_buf, &mask_buf, m, width, height, stride);
                        }
                        layer_buf.fill(0);
                        mask_buf.fill(0);
                        have_mask = false;
                    }
                }
                Layer::Text(text) => {
                    let mut tl = text.clone();
                    tl.position.x *= sx;
                    tl.position.y *= sy;
                    draw_text(&tl, buffer, width, height, stride);
                }
                Layer::PreComp(pre) => {
                    pre.comp.render_sync(frame, buffer, width, height, stride);
                }
                Layer::Image(_) => {}
            }
        }
    }
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

    #[test]
    fn transform_default_animators() {
        let t = Transform::default();
        assert!(t.animators.is_empty());
        assert_eq!(t.scale, Vec2 { x: 1.0, y: 1.0 });
    }
}
