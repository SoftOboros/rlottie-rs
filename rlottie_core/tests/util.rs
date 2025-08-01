use rlottie_core::types::Composition;
use sha2::{Digest, Sha256};

pub fn render_hash(anim: &Composition, frame: u32) -> [u8; 32] {
    let width = 240usize;
    let height = 240usize;
    let mut buf = vec![0u8; width * height * 4];
    anim.render_sync(frame, &mut buf, width, height, width * 4);
    let mut hasher = Sha256::new();
    hasher.update(&buf);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}
