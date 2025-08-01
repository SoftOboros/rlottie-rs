// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
//! Module: animation timeline primitives
//! Mirrors: rlottie/src/lottie/lottiemodel.h

use crate::types::Vec2;

const LUT_SIZE: usize = 256;
const SAMPLE_STEP: f32 = 1.0 / (LUT_SIZE as f32 - 1.0);
const NEWTON_ITERATIONS: usize = 4;
const NEWTON_MIN_SLOPE: f32 = 0.02;
const SUBDIVISION_PRECISION: f32 = 1e-7;
const SUBDIVISION_MAX_ITERATIONS: usize = 10;

/// Cubic Bézier easing curve defined by two control points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubicBezier {
    /// First control point
    pub c1: Vec2,
    /// Second control point
    pub c2: Vec2,
    samples: [f32; LUT_SIZE],
}

impl CubicBezier {
    /// Create a new cubic Bézier and precompute a lookup table.
    pub fn new(c1: Vec2, c2: Vec2) -> Self {
        let mut bez = Self {
            c1,
            c2,
            samples: [0.0; LUT_SIZE],
        };
        bez.calc_samples();
        bez
    }

    fn calc_samples(&mut self) {
        for i in 0..LUT_SIZE {
            let t = i as f32 * SAMPLE_STEP;
            self.samples[i] = Self::calc_bezier(t, self.c1.x, self.c2.x);
        }
    }

    fn calc_bezier(t: f32, a1: f32, a2: f32) -> f32 {
        ((Self::coeff_a(a1, a2) * t + Self::coeff_b(a1, a2)) * t + Self::coeff_c(a1)) * t
    }

    fn get_slope(t: f32, a1: f32, a2: f32) -> f32 {
        3.0 * Self::coeff_a(a1, a2) * t * t + 2.0 * Self::coeff_b(a1, a2) * t + Self::coeff_c(a1)
    }

    const fn coeff_a(a1: f32, a2: f32) -> f32 {
        1.0 - 3.0 * a2 + 3.0 * a1
    }
    const fn coeff_b(a1: f32, a2: f32) -> f32 {
        3.0 * a2 - 6.0 * a1
    }
    const fn coeff_c(a1: f32) -> f32 {
        3.0 * a1
    }

    fn binary_subdivide(&self, x: f32, mut a: f32, mut b: f32) -> f32 {
        let mut current_t = 0.0;
        for _ in 0..SUBDIVISION_MAX_ITERATIONS {
            current_t = a + (b - a) / 2.0;
            let current_x = Self::calc_bezier(current_t, self.c1.x, self.c2.x) - x;
            if current_x.abs() <= SUBDIVISION_PRECISION {
                break;
            }
            if current_x > 0.0 {
                b = current_t;
            } else {
                a = current_t;
            }
        }
        current_t
    }

    fn get_t_for_x(&self, x: f32) -> f32 {
        let mut interval_start = 0.0;
        let mut current_sample = 1;
        while current_sample < LUT_SIZE - 1 && self.samples[current_sample] <= x {
            current_sample += 1;
            interval_start += SAMPLE_STEP;
        }
        current_sample -= 1;
        let dist = (x - self.samples[current_sample])
            / (self.samples[current_sample + 1] - self.samples[current_sample]);
        let mut guess_t = interval_start + dist * SAMPLE_STEP;
        let initial_slope = Self::get_slope(guess_t, self.c1.x, self.c2.x);
        if initial_slope >= NEWTON_MIN_SLOPE {
            for _ in 0..NEWTON_ITERATIONS {
                let current_x = Self::calc_bezier(guess_t, self.c1.x, self.c2.x) - x;
                let current_slope = Self::get_slope(guess_t, self.c1.x, self.c2.x);
                if current_slope == 0.0 {
                    return guess_t;
                }
                guess_t -= current_x / current_slope;
            }
            guess_t
        } else if initial_slope == 0.0 {
            guess_t
        } else {
            self.binary_subdivide(x, interval_start, interval_start + SAMPLE_STEP)
        }
    }

    /// Evaluate the easing curve at position `x` in the range `0..=1`.
    pub fn value(&self, x: f32) -> f32 {
        if (self.c1.x - self.c1.y).abs() < f32::EPSILON
            && (self.c2.x - self.c2.y).abs() < f32::EPSILON
        {
            return x;
        }
        let t = self.get_t_for_x(x);
        Self::calc_bezier(t, self.c1.y, self.c2.y)
    }
}

/// Keyframe describing a value interpolation over a frame range.
#[derive(Debug, Clone)]
pub struct Keyframe<T> {
    /// Start frame inclusive
    pub start: u32,
    /// End frame exclusive
    pub end: u32,
    /// Value at the start frame
    pub start_v: T,
    /// Value at the end frame
    pub end_v: T,
    /// Easing curve applied between frames
    pub ease: CubicBezier,
}

/// Trait for values that can be linearly interpolated.
pub trait Lerp: Sized + Copy {
    /// Interpolate between `self` and `other` with factor `t`.
    fn lerp(self, other: Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(self, other: Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Lerp for Vec2 {
    fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
}

impl<T: Lerp> Keyframe<T> {
    /// Sample the interpolated value at the given frame as a floating point frame index.
    pub fn sample(&self, frame: f32) -> T {
        if frame <= self.start as f32 {
            return self.start_v;
        }
        if frame >= self.end as f32 {
            return self.end_v;
        }
        let progress = (frame - self.start as f32) / (self.end as f32 - self.start as f32);
        let eased = self.ease.value(progress);
        self.start_v.lerp(self.end_v, eased)
    }
}

/// Sequence of [`Keyframe`]s describing an animated property.
#[derive(Debug, Clone, Default)]
pub struct Animator<T> {
    /// Ordered list of keyframes
    pub frames: Vec<Keyframe<T>>,
}

impl<T: Lerp + Default> Animator<T> {
    /// Sample the animated value at the given frame.
    pub fn value(&self, frame: f32) -> T {
        if self.frames.is_empty() {
            return T::default();
        }
        let first = &self.frames[0];
        if frame <= first.start as f32 {
            return first.start_v;
        }
        let last = &self.frames[self.frames.len() - 1];
        if frame >= last.end as f32 {
            return last.end_v;
        }
        for kf in &self.frames {
            if frame >= kf.start as f32 && frame < kf.end as f32 {
                return kf.sample(frame);
            }
        }
        T::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_keyframe() {
        let kf = Keyframe {
            start: 0,
            end: 10,
            start_v: 1.0f32,
            end_v: 2.0,
            ease: CubicBezier::new(Vec2 { x: 0.0, y: 0.0 }, Vec2 { x: 1.0, y: 1.0 }),
        };
        assert_eq!(kf.start, 0);
        assert_eq!(kf.end, 10);
    }

    #[test]
    fn bezier_value_matches_cpp() {
        let bez = CubicBezier::new(Vec2 { x: 0.42, y: 0.0 }, Vec2 { x: 0.58, y: 1.0 });
        let v = bez.value(0.25);
        // Expected value from rlottie C++ vinterpolator
        assert!((v - 0.129162).abs() < 0.0001);
    }

    #[test]
    fn keyframe_sample() {
        let kf = Keyframe {
            start: 0,
            end: 10,
            start_v: 0.0f32,
            end_v: 1.0,
            ease: CubicBezier::new(Vec2 { x: 0.42, y: 0.0 }, Vec2 { x: 0.58, y: 1.0 }),
        };
        let v = kf.sample(2.5);
        assert!((v - 0.129162).abs() < 0.0001);
    }

    #[test]
    fn animator_value() {
        let kf = Keyframe {
            start: 0,
            end: 10,
            start_v: 0.0f32,
            end_v: 1.0,
            ease: CubicBezier::new(Vec2 { x: 0.42, y: 0.0 }, Vec2 { x: 0.58, y: 1.0 }),
        };
        let anim = Animator {
            frames: vec![kf.clone()],
        };
        let v = anim.value(2.5);
        assert!((v - 0.129162).abs() < 0.0001);
        assert_eq!(anim.value(-1.0), 0.0);
        assert_eq!(anim.value(20.0), 1.0);
    }
}
