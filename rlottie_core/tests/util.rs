use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};
use rlottie_core::types::Composition;
use sha2::{Digest, Sha256};
use std::path::Path;

pub fn render_hash(anim: &Composition, frame: u32) -> [u8; 32] {
    let png = render_png(anim, frame);
    let digest = Sha256::digest(&png);
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

pub fn render_frame(anim: &Composition, frame: u32) -> Vec<u8> {
    let width = 240usize;
    let height = 240usize;
    let mut buf = vec![0u8; width * height * 4];
    anim.render_sync(frame, &mut buf, width, height, width * 4);
    buf
}

pub fn render_png(anim: &Composition, frame: u32) -> Vec<u8> {
    let width = 240usize;
    let height = 240usize;
    let mut buf = vec![0u8; width * height * 4];
    anim.render_sync(frame, &mut buf, width, height, width * 4);
    let mut out = Vec::new();
    PngEncoder::new(&mut out)
        .write_image(&buf, width as u32, height as u32, ColorType::Rgba8.into())
        .unwrap();
    out
}

pub fn load_reference_png(path: &Path) -> Vec<u8> {
    let img = ImageReader::open(path)
        .expect("open png")
        .decode()
        .expect("decode png")
        .to_rgba8();
    img.into_raw()
}

pub fn pixel_diff_count(a: &[u8], b: &[u8]) -> usize {
    a.chunks_exact(4)
        .zip(b.chunks_exact(4))
        .filter(|(x, y)| x != y)
        .count()
}

pub fn rmse(a: &[u8], b: &[u8]) -> f64 {
    assert_eq!(a.len(), b.len());
    let sum: f64 = a
        .iter()
        .zip(b.iter())
        .map(|(&x, &y)| {
            let d = x as f64 - y as f64;
            d * d
        })
        .sum();
    (sum / a.len() as f64).sqrt()
}
