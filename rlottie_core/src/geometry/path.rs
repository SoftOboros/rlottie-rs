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

impl LineSegment {
    /// Calculate the Euclidean length of the segment.
    pub fn length(&self) -> f32 {
        let dx = self.to.x - self.from.x;
        let dy = self.to.y - self.from.y;
        (dx * dx + dy * dy).sqrt()
    }
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

    /// Approximate path length by summing flattened segment lengths.
    pub fn length(&self, tolerance: f32) -> f32 {
        self.flatten(tolerance)
            .iter()
            .map(LineSegment::length)
            .sum()
    }

    /// Return a new path trimmed between `start` and `end` fractions.
    /// Values are normalized to `[0,1]` and treat `start > end` as a loop.
    pub fn trim(&self, start: f32, end: f32, tolerance: f32) -> Self {
        if (start - end).abs() < f32::EPSILON {
            return Self::new();
        }
        if ((start <= 0.0 && end >= 1.0) || (start >= 1.0 && end <= 0.0)) && start != end {
            return self.clone();
        }

        let segs = self.flatten(tolerance);
        if segs.is_empty() {
            return Self::new();
        }
        let total: f32 = segs.iter().map(LineSegment::length).sum();
        let s = start.clamp(0.0, 1.0) * total;
        let e = end.clamp(0.0, 1.0) * total;

        if start < end {
            extract_range(&segs, s, e)
        } else {
            let mut first = extract_range(&segs, s, total);
            let second = extract_range(&segs, 0.0, e);
            first.segments.extend(second.segments);
            first
        }
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

fn lerp(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    Vec2 {
        x: a.x + (b.x - a.x) * t,
        y: a.y + (b.y - a.y) * t,
    }
}

fn extract_range(segs: &[LineSegment], from: f32, to: f32) -> Path {
    let mut result = Path::new();
    if from >= to {
        return result;
    }
    let mut pos = 0.0f32;
    let mut started = false;
    for seg in segs {
        let len = seg.length();
        let next = pos + len;
        if next <= from {
            pos = next;
            continue;
        }
        if pos >= to {
            break;
        }
        let start_t = if from > pos { (from - pos) / len } else { 0.0 };
        let end_t = if to < next { (to - pos) / len } else { 1.0 };
        let start_pt = lerp(seg.from, seg.to, start_t);
        let end_pt = lerp(seg.from, seg.to, end_t);
        if !started {
            result.move_to(start_pt);
            started = true;
        } else if start_t > 0.0 {
            result.move_to(start_pt);
        } else {
            result.line_to(start_pt);
        }
        result.line_to(end_pt);
        pos = next;
        if next >= to {
            break;
        }
    }
    result
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

    #[test]
    fn path_trim_half() {
        let mut path = Path::new();
        path.move_to(Vec2 { x: 0.0, y: 0.0 });
        path.line_to(Vec2 { x: 10.0, y: 0.0 });
        let trimmed = path.trim(0.0, 0.5, 0.01);
        let segs = trimmed.flatten(0.01);
        assert_eq!(segs.len(), 1);
        assert!((segs[0].to.x - 5.0).abs() < 1e-5);
    }

    #[test]
    fn path_trim_loop() {
        let mut path = Path::new();
        path.move_to(Vec2 { x: 0.0, y: 0.0 });
        path.line_to(Vec2 { x: 10.0, y: 0.0 });
        let trimmed = path.trim(0.8, 0.2, 0.01);
        let segs = trimmed.flatten(0.01);
        assert_eq!(segs.len(), 2);
        assert!((segs[0].from.x - 8.0).abs() < 1e-5);
        assert!((segs[1].to.x - 2.0).abs() < 1e-5);
    }
}
