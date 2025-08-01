// Copyright © SoftOboros Technology, Inc.
// SPDX-License-Identifier: MIT
use proptest::prelude::*;
use rlottie_core::{geometry::Path, geometry::PathSeg, types::Transform};
mod testutil;

proptest! {
    #[test]
    fn transform_roundtrip(t in testutil::transform_strategy()) {
        let json = serde_json::to_string(&t).unwrap();
        let parsed: Transform = serde_json::from_str(&json).unwrap();
        let json2 = serde_json::to_string(&parsed).unwrap();
        let reparsed: Transform = serde_json::from_str(&json2).unwrap();
        assert!((t.anchor.x - reparsed.anchor.x).abs() < 1e-3);
        assert!((t.anchor.y - reparsed.anchor.y).abs() < 1e-3);
        assert!((t.position.x - reparsed.position.x).abs() < 1e-3);
        assert!((t.position.y - reparsed.position.y).abs() < 1e-3);
        assert!((t.scale.x - reparsed.scale.x).abs() < 1e-3);
        assert!((t.scale.y - reparsed.scale.y).abs() < 1e-3);
        assert!((t.rotation - reparsed.rotation).abs() < 1e-3);
        assert!((t.opacity - reparsed.opacity).abs() < 1e-3);
    }
}

proptest! {
    #[test]
    fn path_flatten_bound(cmds in proptest::collection::vec(path_seg_strategy(), 1..8)) {
        let mut path = Path::new();
        let mut started = false;
        for seg in &cmds {
            match seg {
                PathSeg::MoveTo(p) => {
                    path.move_to(*p);
                    started = true;
                }
                PathSeg::LineTo(p) => if started { path.line_to(*p); },
                PathSeg::Cubic(c1, c2, p) => if started { path.cubic_to(*c1, *c2, *p); },
                PathSeg::Close => if started { path.close(); started = false; },
            }
        }
        let _count_input = path.segments.len();
        let segs = path.flatten(1.0);
        let bound = 4096usize;
        assert!(segs.len() <= bound);
    }
}

fn path_seg_strategy() -> impl Strategy<Value = PathSeg> {
    prop_oneof![
        testutil::vec2_strategy().prop_map(PathSeg::MoveTo),
        testutil::vec2_strategy().prop_map(PathSeg::LineTo),
        (
            testutil::vec2_strategy(),
            testutil::vec2_strategy(),
            testutil::vec2_strategy()
        )
            .prop_map(|(c1, c2, p)| PathSeg::Cubic(c1, c2, p)),
        Just(PathSeg::Close),
    ]
}
