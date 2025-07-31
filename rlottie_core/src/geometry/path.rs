// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
//! Module: vector path representation
//! Mirrors: rlottie/src/vector/vpath.h

use crate::types::Vec2;
use smallvec::SmallVec;

/// A line segment represented by two end points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineSegment {
    /// Start point of the segment
    pub from: Vec2,
    /// End point of the segment
    pub to: Vec2,
}

/// Basic path drawing commands.
#[derive(Debug, Clone, PartialEq)]
pub enum PathSeg {
    /// Move to absolute position.
    MoveTo(Vec2),
    /// Line to absolute position.
    LineTo(Vec2),
    /// Cubic Bézier curve with two control points and end point.
    Cubic(Vec2, Vec2, Vec2),
    /// Close current sub-path.
    Close,
}

/// A sequence of [`PathSeg`] items forming a vector path.
#[derive(Debug, Default, Clone)]
pub struct Path {
    /// Ordered list of path segments
    pub segments: Vec<PathSeg>,
}

impl Path {
    /// Create a new empty path.
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    /// Append a move command.
    pub fn move_to(&mut self, p: Vec2) {
        self.segments.push(PathSeg::MoveTo(p));
    }

    /// Append a line command.
    pub fn line_to(&mut self, p: Vec2) {
        self.segments.push(PathSeg::LineTo(p));
    }

    /// Append a cubic Bézier curve command.
    pub fn cubic_to(&mut self, c1: Vec2, c2: Vec2, p: Vec2) {
        self.segments.push(PathSeg::Cubic(c1, c2, p));
    }

    /// Close the current sub-path.
    pub fn close(&mut self) {
        self.segments.push(PathSeg::Close);
    }

    /// Flatten the path into line segments using recursive subdivision of cubics.
    pub fn flatten(&self, tolerance: f32) -> SmallVec<[LineSegment; 32]> {
        let mut result: SmallVec<[LineSegment; 32]> = SmallVec::new();
        let mut start = Vec2::default();
        let mut current = Vec2::default();
        let mut has_start = false;
        for seg in &self.segments {
            match *seg {
                PathSeg::MoveTo(p) => {
                    current = p;
                    start = p;
                    has_start = true;
                }
                PathSeg::LineTo(p) => {
                    result.push(LineSegment {
                        from: current,
                        to: p,
                    });
                    current = p;
                }
                PathSeg::Cubic(c1, c2, p) => {
                    flatten_cubic(current, c1, c2, p, tolerance, &mut result);
                    current = p;
                }
                PathSeg::Close => {
                    if has_start && current != start {
                        result.push(LineSegment {
                            from: current,
                            to: start,
                        });
                    }
                    current = start;
                }
            }
        }
        result
    }
}

fn flatten_cubic(
    p0: Vec2,
    c1: Vec2,
    c2: Vec2,
    p3: Vec2,
    tolerance: f32,
    out: &mut SmallVec<[LineSegment; 32]>,
) {
    if cubic_flat_enough(p0, c1, c2, p3, tolerance) {
        out.push(LineSegment { from: p0, to: p3 });
    } else {
        let (p0a, c1a, c2a, p3a, p0b, c1b, c2b, p3b) = split_cubic(p0, c1, c2, p3);
        flatten_cubic(p0a, c1a, c2a, p3a, tolerance, out);
        flatten_cubic(p0b, c1b, c2b, p3b, tolerance, out);
    }
}

fn cubic_flat_enough(p0: Vec2, c1: Vec2, c2: Vec2, p3: Vec2, tol: f32) -> bool {
    let d1 = point_line_distance_sq(c1, p0, p3);
    let d2 = point_line_distance_sq(c2, p0, p3);
    d1 <= tol * tol && d2 <= tol * tol
}

fn point_line_distance_sq(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let vx = b.x - a.x;
    let vy = b.y - a.y;
    let u = ((p.x - a.x) * vx + (p.y - a.y) * vy) / (vx * vx + vy * vy);
    let x = a.x + u * vx;
    let y = a.y + u * vy;
    let dx = x - p.x;
    let dy = y - p.y;
    dx * dx + dy * dy
}

fn split_cubic(
    p0: Vec2,
    c1: Vec2,
    c2: Vec2,
    p3: Vec2,
) -> (Vec2, Vec2, Vec2, Vec2, Vec2, Vec2, Vec2, Vec2) {
    let m1 = mid(p0, c1);
    let m2 = mid(c1, c2);
    let m3 = mid(c2, p3);
    let m4 = mid(m1, m2);
    let m5 = mid(m2, m3);
    let m6 = mid(m4, m5);
    (
        p0, m1, m4, m6, // first half
        m6, m5, m3, p3, // second half
    )
}

fn mid(a: Vec2, b: Vec2) -> Vec2 {
    Vec2 {
        x: (a.x + b.x) * 0.5,
        y: (a.y + b.y) * 0.5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_build_and_flatten() {
        let mut path = Path::new();
        path.move_to(Vec2 { x: 0.0, y: 0.0 });
        path.line_to(Vec2 { x: 1.0, y: 0.0 });
        path.cubic_to(
            Vec2 { x: 1.0, y: 1.0 },
            Vec2 { x: 0.0, y: 1.0 },
            Vec2 { x: 0.0, y: 0.0 },
        );
        path.close();
        let segs = path.flatten(0.01);
        assert!(segs.len() >= 2);
        assert_eq!(segs.first().unwrap().from, Vec2 { x: 0.0, y: 0.0 });
        assert_eq!(segs.first().unwrap().to, Vec2 { x: 1.0, y: 0.0 });
    }
}
