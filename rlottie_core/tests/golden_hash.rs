use std::collections::HashMap;
use std::fs;
use std::path::Path;

use rlottie_core::loader::json;
use sha2::{Digest, Sha256};
use util::{load_reference_png, pixel_diff_count, render_frame, render_png, rmse};

mod util;

fn load_hashes() -> HashMap<String, String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/assets/hashes.json");
    let data = fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}

/// Compare rendered frames with C++ reference hashes.
#[test]
#[ignore]
fn golden_hash_corpus() {
    let hashes = load_hashes();
    let corpus_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/assets/corpus");
    let frames = [0u32, 30, 60];

    for entry in fs::read_dir(corpus_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let data = fs::read(&path).unwrap();
        let comp = json::from_slice(&data).unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        for &frame in &frames {
            let buf = render_frame(&comp, frame);
            let png_bytes = render_png(&comp, frame);
            let digest = Sha256::digest(&png_bytes);
            let key = format!("tests/assets/corpus/{file_name}_{frame}.png");
            if let Some(expect) = hashes.get(&key) {
                if hex::encode(digest) != *expect {
                    let png_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join(&key);
                    let ref_buf = load_reference_png(&png_path);
                    let diff = pixel_diff_count(&buf, &ref_buf);
                    if diff <= 5 {
                        let err = rmse(&buf, &ref_buf);
                        assert!(err < 1.0, "RMSE {} for {}", err, key);
                    } else {
                        panic!("hash mismatch for {key}; diff_pixels={diff}");
                    }
                }
            } else {
                panic!("missing hash entry for {key}");
            }
        }
    }
}
