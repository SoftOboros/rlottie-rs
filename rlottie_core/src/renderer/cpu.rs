// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
//! Module: software rasterizer
//! Mirrors: rlottie/src/vector/vpainter.cpp (simplified)

use crate::geometry::{tessellate, Path};
use crate::types::{Color, GradientStop, LinearGradient, Paint, RadialGradient, Vec2};

/// Fill a path with the given paint into the RGBA8888 buffer.
pub fn draw_path(
    path: &Path,
    paint: Paint,
    buffer: &mut [u8],
    width: usize,
    height: usize,
    stride: usize,
) {
    let mesh = tessellate(path, 0.2);
    for tri in mesh.indices.chunks(3) {
        if tri.len() < 3 {
            continue;
        }
        let v0 = mesh.vertices[tri[0] as usize];
        let v1 = mesh.vertices[tri[1] as usize];
        let v2 = mesh.vertices[tri[2] as usize];
        fill_triangle_paint(v0, v1, v2, &paint, buffer, width, height, stride);
    }
}

/// Stroke a path with the given paint and width.
pub fn draw_stroke(
    path: &Path,
    width_px: f32,
    paint: Paint,
    buffer: &mut [u8],
    width: usize,
    height: usize,
    stride: usize,
) {
    let segs = path.flatten(0.2);
    for seg in segs {
        let dx = seg.to.x - seg.from.x;
        let dy = seg.to.y - seg.from.y;
        let len = (dx * dx + dy * dy).sqrt();
        if len == 0.0 {
            continue;
        }
        let nx = -dy / len * width_px * 0.5;
        let ny = dx / len * width_px * 0.5;
        let p1 = Vec2 {
            x: seg.from.x + nx,
            y: seg.from.y + ny,
        };
        let p2 = Vec2 {
            x: seg.from.x - nx,
            y: seg.from.y - ny,
        };
        let p3 = Vec2 {
            x: seg.to.x - nx,
            y: seg.to.y - ny,
        };
        let p4 = Vec2 {
            x: seg.to.x + nx,
            y: seg.to.y + ny,
        };
        fill_triangle_paint(p1, p2, p3, &paint, buffer, width, height, stride);
        fill_triangle_paint(p1, p3, p4, &paint, buffer, width, height, stride);
    }
}
#[allow(clippy::too_many_arguments)]
fn fill_triangle_paint(
    a: Vec2,
    b: Vec2,
    c: Vec2,
    paint: &Paint,
    buf: &mut [u8],
    width: usize,
    height: usize,
    stride: usize,
) {
    let min_x = a.x.min(b.x).min(c.x).floor().max(0.0) as i32;
    let max_x = a.x.max(b.x).max(c.x).ceil().min(width as f32) as i32;
    let min_y = a.y.min(b.y).min(c.y).floor().max(0.0) as i32;
    let max_y = a.y.max(b.y).max(c.y).ceil().min(height as f32) as i32;

    for y in min_y..max_y {
        for x in min_x..max_x {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            if inside_triangle(px, py, a, b, c) {
                let color = sample_paint(paint, Vec2 { x: px, y: py });
                blend_pixel(buf, stride, x as usize, y as usize, color);
            }
        }
    }
}

fn edge(px: f32, py: f32, a: Vec2, b: Vec2) -> f32 {
    (px - a.x) * (b.y - a.y) - (py - a.y) * (b.x - a.x)
}

fn inside_triangle(px: f32, py: f32, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let e1 = edge(px, py, a, b);
    let e2 = edge(px, py, b, c);
    let e3 = edge(px, py, c, a);
    (e1 >= 0.0 && e2 >= 0.0 && e3 >= 0.0) || (e1 <= 0.0 && e2 <= 0.0 && e3 <= 0.0)
}

fn blend_pixel(buf: &mut [u8], stride: usize, x: usize, y: usize, src: Color) {
    let offset = y * stride + x * 4;
    if offset + 3 >= buf.len() {
        return;
    }
    let dst_r = buf[offset] as f32;
    let dst_g = buf[offset + 1] as f32;
    let dst_b = buf[offset + 2] as f32;
    let dst_a = buf[offset + 3] as f32;

    let sa = src.a as f32 / 255.0;
    let ia = 1.0 - sa;

    let out_a = sa + dst_a / 255.0 * ia;
    let out_r = src.r as f32 * sa + dst_r * ia;
    let out_g = src.g as f32 * sa + dst_g * ia;
    let out_b = src.b as f32 * sa + dst_b * ia;

    buf[offset] = out_r.min(255.0) as u8;
    buf[offset + 1] = out_g.min(255.0) as u8;
    buf[offset + 2] = out_b.min(255.0) as u8;
    buf[offset + 3] = (out_a * 255.0).min(255.0) as u8;
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let clamped = t.clamp(0.0, 1.0);
    let ir = a.r as f32 + (b.r as f32 - a.r as f32) * clamped;
    let ig = a.g as f32 + (b.g as f32 - a.g as f32) * clamped;
    let ib = a.b as f32 + (b.b as f32 - a.b as f32) * clamped;
    let ia = a.a as f32 + (b.a as f32 - a.a as f32) * clamped;
    Color {
        r: ir.round() as u8,
        g: ig.round() as u8,
        b: ib.round() as u8,
        a: ia.round() as u8,
    }
}

fn sample_stops(stops: &[GradientStop], t: f32) -> Color {
    if stops.is_empty() {
        return Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };
    }
    if t <= stops[0].offset {
        return stops[0].color;
    }
    for win in stops.windows(2) {
        let s0 = win[0];
        let s1 = win[1];
        if t <= s1.offset {
            let local = (t - s0.offset) / (s1.offset - s0.offset);
            return lerp_color(s0.color, s1.color, local);
        }
    }
    stops.last().unwrap().color
}

fn sample_linear(g: &LinearGradient, p: Vec2) -> Color {
    let dx = g.end.x - g.start.x;
    let dy = g.end.y - g.start.y;
    let len_sq = dx * dx + dy * dy;
    let t = if len_sq == 0.0 {
        0.0
    } else {
        ((p.x - g.start.x) * dx + (p.y - g.start.y) * dy) / len_sq
    };
    sample_stops(&g.stops, t)
}

fn sample_radial(g: &RadialGradient, p: Vec2) -> Color {
    let dx = p.x - g.center.x;
    let dy = p.y - g.center.y;
    let dist = (dx * dx + dy * dy).sqrt();
    let t = dist / g.radius;
    sample_stops(&g.stops, t)
}

fn sample_paint(paint: &Paint, p: Vec2) -> Color {
    match paint {
        Paint::Solid(c) => *c,
        Paint::Linear(g) => sample_linear(g, p),
        Paint::Radial(g) => sample_radial(g, p),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_simple_rect() {
        let mut path = Path::new();
        path.move_to(Vec2 { x: 1.0, y: 1.0 });
        path.line_to(Vec2 { x: 5.0, y: 1.0 });
        path.line_to(Vec2 { x: 5.0, y: 5.0 });
        path.line_to(Vec2 { x: 1.0, y: 5.0 });
        path.close();

        let mut buf = vec![255u8; 8 * 8 * 4];
        draw_path(
            &path,
            Paint::Solid(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }),
            &mut buf,
            8,
            8,
            8 * 4,
        );
        let off = 3 * 8 * 4 + 3 * 4;
        assert_eq!(&buf[off..off + 4], &[0, 0, 0, 255]);
    }

    #[test]
    fn stroke_simple_rect() {
        let mut path = Path::new();
        path.move_to(Vec2 { x: 1.0, y: 1.0 });
        path.line_to(Vec2 { x: 6.0, y: 1.0 });
        path.line_to(Vec2 { x: 6.0, y: 6.0 });
        path.line_to(Vec2 { x: 1.0, y: 6.0 });
        path.close();

        let mut buf = vec![0u8; 8 * 8 * 4];
        draw_stroke(
            &path,
            1.0,
            Paint::Solid(Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            }),
            &mut buf,
            8,
            8,
            8 * 4,
        );
        let off = 1 * 8 * 4 + 1 * 4;
        assert_eq!(&buf[off..off + 4], &[255, 0, 0, 255]);
    }
}
