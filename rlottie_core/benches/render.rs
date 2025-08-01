use criterion::{criterion_group, criterion_main, Criterion};
use rlottie_core::loader::json;
use std::path::Path;

fn bench_render(c: &mut Criterion) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../tests/assets/corpus/1643-exploding-star.json");
    let data = std::fs::read(path).unwrap();
    let comp = json::from_slice(&data).unwrap();
    let width = 240usize;
    let height = 240usize;
    let mut buf = vec![0u8; width * height * 4];
    c.bench_function("render_60_frames", |b| {
        b.iter(|| {
            for frame in 0..60u32 {
                comp.render_sync(frame, &mut buf, width, height, width * 4);
            }
        });
    });
}

criterion_group!(benches, bench_render);
criterion_main!(benches);
