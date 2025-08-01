// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
use proptest::prelude::*;
use rlottie_core::types::{Transform, Vec2};

pub fn vec2_strategy() -> impl Strategy<Value = Vec2> {
    (-1000.0f32..1000.0f32, -1000.0f32..1000.0f32).prop_map(|(x, y)| Vec2 { x, y })
}

pub fn vec2_positive_strategy() -> impl Strategy<Value = Vec2> {
    (0.0f32..100.0f32, 0.0f32..100.0f32).prop_map(|(x, y)| Vec2 { x, y })
}

pub fn transform_strategy() -> impl Strategy<Value = Transform> {
    (
        vec2_strategy(),
        vec2_strategy(),
        vec2_strategy(),
        -360.0f32..360.0f32,
        0.0f32..1.0f32,
    )
        .prop_map(|(anchor, position, scale, rotation, opacity)| Transform {
            anchor,
            position,
            scale,
            rotation,
            opacity,
            animators: std::collections::HashMap::new(),
        })
}
