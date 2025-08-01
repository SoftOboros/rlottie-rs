// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
//! Module: path tessellation helpers
//! Mirrors: rlottie/src/vector/vdrawhelper.cpp (approx)

#[cfg(feature = "simd")]
use super::Path;
#[cfg(not(feature = "simd"))]
use super::{LineSegment, Path};
use crate::types::Vec2;

/// A simple triangle mesh produced by tessellation.
#[derive(Debug, Default, Clone)]
pub struct Mesh {
    /// Vertex positions
    pub vertices: Vec<Vec2>,
    /// Index buffer (triples)
    pub indices: Vec<u32>,
}

/// Tessellate a [`Path`] into triangles using the lyon tessellator when
/// the `simd` feature is enabled. A very naive fan triangulator is used
/// as a fallback for `no_std` or when lyon is disabled.
/// Tessellate a [`Path`] into triangles, optionally trimming the length to
/// the range `[start, end]` before tessellation.
pub fn tessellate(path: &Path, tolerance: f32, mask: Option<(f32, f32)>) -> Mesh {
    let tmp;
    let src = if let Some((s, e)) = mask {
        tmp = path.trim(s, e, tolerance);
        &tmp
    } else {
        path
    };
    tessellate_impl(src, tolerance)
}

#[cfg(feature = "simd")]
fn tessellate_impl(path: &Path, tolerance: f32) -> Mesh {
    use lyon::math::Point;
    use lyon::path::Path as LyonPath;
    use lyon::tessellation::{
        BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers,
    };

    let mut builder = LyonPath::builder();
    for seg in &path.segments {
        match *seg {
            super::PathSeg::MoveTo(p) => {
                builder.begin(Point::new(p.x, p.y));
            }
            super::PathSeg::LineTo(p) => {
                builder.line_to(Point::new(p.x, p.y));
            }
            super::PathSeg::Cubic(c1, c2, p) => {
                builder.cubic_bezier_to(
                    Point::new(c1.x, c1.y),
                    Point::new(c2.x, c2.y),
                    Point::new(p.x, p.y),
                );
            }
            super::PathSeg::Close => {
                builder.close();
            }
        }
    }
    let lyon_path = builder.build();
    let mut tess = FillTessellator::new();
    let mut buffers: VertexBuffers<Vec2, u32> = VertexBuffers::new();
    tess.tessellate_path(
        &lyon_path,
        &FillOptions::tolerance(tolerance),
        &mut BuffersBuilder::new(&mut buffers, |v: FillVertex| {
            let p = v.position();
            Vec2 { x: p.x, y: p.y }
        }),
    )
    .unwrap();
    Mesh {
        vertices: buffers.vertices,
        indices: buffers.indices,
    }
}

#[cfg(not(feature = "simd"))]
fn tessellate_impl(path: &Path, tolerance: f32) -> Mesh {
    use smallvec::SmallVec;
    let segs: SmallVec<[LineSegment; 32]> = path.flatten(tolerance);
    if segs.is_empty() {
        return Mesh::default();
    }
    let mut vertices = Vec::new();
    vertices.push(segs[0].from);
    for seg in &segs {
        vertices.push(seg.to);
    }
    if vertices.len() > 1 && vertices.last() == vertices.first() {
        vertices.pop();
    }
    let mut indices = Vec::new();
    for i in 1..vertices.len() - 1 {
        indices.push(0);
        indices.push(i as u32);
        indices.push(i as u32 + 1);
    }
    Mesh { vertices, indices }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangulate_rectangle() {
        let mut path = Path::new();
        path.move_to(Vec2 { x: 0.0, y: 0.0 });
        path.line_to(Vec2 { x: 1.0, y: 0.0 });
        path.line_to(Vec2 { x: 1.0, y: 1.0 });
        path.line_to(Vec2 { x: 0.0, y: 1.0 });
        path.close();
        let mesh = tessellate(&path, 0.1, None);
        assert_eq!(mesh.indices.len(), 6);
        assert!(mesh.vertices.len() >= 4);
    }
}
