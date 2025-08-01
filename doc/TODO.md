# rlottie-rs — Road‑to‑Parity TODO

> **Goal**  Build a pure‑Rust runtime that loads **.lottie / Bodymovin JSON** and rasterises each frame with parity to Samsung rlottie.
>
> **Codex‑friendly conventions**  
> • Each task is ≤ 200 LOC.  
> • Always place new code under the `rlottie_core` crate unless stated.  
> • All public items **must** have doc‑comments.  
> • Unit tests live next to source with `#[cfg(test)]`.

---
## 0 TOML Dependency Setup
| Step | ✔ | Detailed instruction |
|------|---|---------------------|
|0.1|✔| Create a root `Cargo.toml` **workspace** with members `rlottie_core/` and `examples/`.|
|0.2|✔| Add optional **feature flags**: `simd`, `wasm`, `embedded`, `bench`.  Each toggles a `[features]` block.|
|0.3|✔| Gate deps:<br>• `lyon` → `feature = "simd"`<br>• `packed_simd_2` → `simd`<br>• `wasm-bindgen` → `wasm`|
|0.4|✔| Set release profile:<br>`opt-level="z"`, `lto=true`, `codegen-units=1`, `panic="abort"`, `strip=true`.|
|0.5|✔| Add docs.rs metadata:<br>`all-features=true` and `rustdoc-args=["--cfg","docsrs","--cfg","nightly"]`.|

---
## 1 JSON → IR Parser
| ID | ✔ | Instruction |
|----|---|-------------|
|1.1|✔| Create `rlottie_core::loader::json` module. Implement `Composition::from_reader<R: Read>(reader) -> Result<Composition>`.|
|1.2|✔| Define `struct Composition { width:u32,height:u32,fps:f32,layers:Vec<Layer> }` in `types.rs`.|
|1.3|✔| Add `enum Layer { Shape(ShapeLayer), Image(ImageLayer), PreComp(PreCompLayer), Text(TextLayer) }`.|
|1.4|✔| Implement transform struct: `Transform { anchor:Vec2, position:Vec2, scale:Vec2, rotation:f32, opacity:f32 }` + `serde` derives.|
|1.5|✔| Parse **shape paths** only: support `m,l,c,o` path verbs. Map to proto‑`PathCommand` enum.|
|1.6|✔| Add **unit test** fixture `tests/data/min_shape.json` and assert layer count = 1.|

---
## 2 Vector Primitives & Path Engine
|ID| ✔ | Instruction|
|--|---|-----------|
|2.1|✔| Create `geometry::Path` with `Vec<PathSeg>`, where `PathSeg` = `MoveTo(Vec2)`, `LineTo(Vec2)`, `Cubic(Vec2,Vec2,Vec2)`, `Close`.|
|2.2|✔| Implement `fn flatten(&self, tolerance:f32) -> SmallVec<[LineSegment;32]>` using recursive subdivision of cubics.|
|2.3|✔| Feature‑gate integration with `lyon` tessellator (`simd` feature). Provide blanket fallback tessellator on `no_std`.|
|2.4|✔| Provide `no_std` fixed‑point `Vec2Fx` type (Q16.16).|

---
## 3 Timeline & Interpolator
|ID| ✔ | Instruction|
|--|---|-----------|
|3.1|✔| `struct Keyframe<T> { start:u32,end:u32,start_v:T,end_v:T,ease:CubicBezier }`.|
|3.2|✔| `fn sample(&self, frame:f32) -> T` using ease LUT (256 entries).|
|3.3|✔| `Animator<T>` stores `Vec<Keyframe<T>>`. Implement `Animator::value(frame)`.|
|3.4|✔| Attach animators to `Transform` and fill/style props within each `Layer` via `HashMap<&'static str, Animator<Value>>`.|

---
## 4 Raster Back‑Ends
|ID| ✔ | Instruction|
|--|---|-----------|
|4.1|✔| `renderer::cpu` → implement `draw_path(Path, Paint, &mut [u8], w,h,stride)` in RGBA8888.|
|4.2|⚠️ Needs upstream clarification| Enable SIMD span‑fill with `packed_simd_2` behind `simd` feature; provide scalar fallback.|
|4.3|⚠️ Needs upstream clarification| `renderer::embedded_graphics` adapter that implements `embedded_graphics::Drawable` for `Composition`.|
|4.4|✔| `renderer::wasm` produce `ImageData` via wasm‑bindgen; compile under `wasm32-unknown-unknown`.|

---
## 5 Feature‑Parity Checklist
- [x] Shape solid fills & strokes → uses engine 2 + renderer 4
- [ ] Gradients (linear, radial) → extend Paint enum
- [ ] TrimPaths → apply length mask during tessellation
- [ ] Masks / Mattes → layer compositing with stencil buffer
- [ ] Rounded corners → path boolean ops or explicit arc segments
- [ ] Pre‑comp layers → recursive `Composition` render
- [ ] Image assets → decode PNG/JPEG via `image-rs`
- [ ] Text layers → raster glyphs via `fontdue`
- [ ] Repeater → dup path with transform per copy
- [x] Time‑remap / loop → adjust frame sampling in `Composition::render(frame)`

---
## 6 Performance & Size
- [ ] Add `benches/render.rs` using `criterion`; measure ms/frame 240×240 @ 60 fps.
- [ ] Integrate `cargo bloat --release` in CI; fail if binary > 600 kB (embedded profile).
- [ ] SIMD vs scalar regression → bench group tags.
- [ ] Tile cache: static layer bitmaps reused across frames.

---
## 7 Docs & Examples
- [ ] Inline API docs via `#![doc = include_str!("../README.md")]`.
- [ ] `examples/viewer.rs` using `pixels + winit` — press space to restart anim.
- [ ] `examples/embedded_stm32.rs` displays animation on SPI `st7789`.

---
## 8 Validation
- [ ] Port rlottie C++ unit tests: render PNG of frame 0/30/60, hash.
- [ ] CLI tool `rlottie-diff` to compare two PNGs (root‑mean‑square error).
- [ ] `cargo fuzz run fuzz_json` corpus from lottiefiles.zip.

---
## 9 Workflow
- [ ] **GitHub Action** matrix: `stable`, `nightly`, `wasm32`, `thumbv7em`; run `cargo check --all-features` + `cargo test`.
- [ ] After CI success, build docs: `cargo +nightly doc --all-features --no-deps` and push to `gh-pages`.
- [ ] Size & bench artifacts uploaded via `actions/upload-artifact`.

---
*(Testing roadmap will be authored in **/doc/TODO-TESTING.doc**.)

