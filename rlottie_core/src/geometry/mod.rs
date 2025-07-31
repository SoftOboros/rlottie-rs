// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
//! Module: geometry primitives
//! Mirrors: rlottie/src/vector/vpath.h

mod path;
mod tess;

pub use path::{LineSegment, Path, PathSeg};
pub use tess::{tessellate, Mesh};
