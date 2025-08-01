//! Module: rendering backends
//! Mirrors: rlottie/src/vpainter.cpp (simplified)

pub mod cpu;
pub use cpu::*;

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub mod wasm;
