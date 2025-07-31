# rlottie-rs — Test‑Coverage TODO

> **Purpose**  Achieve high‑confidence functional equivalence to Samsung **rlottie**.  
> **Philosophy**  All reference data originate from the *C++ engine*. Tests must *never* bake in Rust‑side behaviour.
>
> **Conventions**  
> • Each bite ≤ 150 LOC.  
> • Put all test helpers in the `tests/` or `rlottie_core::testutil` module.  
> • Golden assets live in `tests/assets/` (PNG, JSON, etc.).
>
> **Legend**  
> `✔` column for manual check‑off.

---
## 0 Corpus & Fixtures
| ID | ✔ | Detailed instruction |
|----|----|---------------------|
|0.1| | **Collect corpus** – download 25 public Lottie files (≤ 50 KB each) from *lottiefiles.com* into `tests/assets/corpus/`.|
|0.2| | Write `scripts/gen_png.sh` – uses **C++ rlottie CLI** to render each file at 240×240, frame 0/30/60 → save as PNG next to JSON.|  
|0.3| | Hash each PNG with SHA‑256; store map in `tests/assets/hashes.json`.|

---
## 1 Golden Frame Hash Tests
| ID | ✔ | Instruction |
|----|----|-------------|
|1.1| | Add `tests/golden_hash.rs` that iterates corpus, renders same frames via **Rust** engine, converts to PNG (or raw RGBA) and hashes.|
|1.2| | Assert computed SHA‑256 matches reference hash ± optional tolerance (allow ≤ 5 pixel diff → fallback to RMSE check).|
|1.3| | Bite‑size: implement helper `fn render_hash(anim:&Composition, frame:u32)->[u8;32]`.|

---
## 2 Visual Diff CLI
| ID | ✔ | Instruction |
|----|----|-------------|
|2.1| | Create `bin/rlottie-diff.rs` CLI: `diff <json> --frame <n> --output diff.png`.|
|2.2| | Use C++ frame as baseline; overlay difference heat‑map (red alpha) for pixels above threshold.|
|2.3| | CI uses CLI to store diff artefacts if hash mismatch.|

---
## 3 Property‑Based Tests
| ID | ✔ | Instruction |
|----|----|-------------|
|3.1| | Integrate `proptest` crate (dev‑dependency).|
|3.2| | Generate random `Transform` sequences; ensure round‑trip JSON ↔ IR ↔ JSON preserves numeric tolerance < 1e‑3.|
|3.3| | Random path commands → ensure `flatten(tol)` output ≤ expected segment count bound (property: complexity grows ≤ 2ⁿ).|

---
## 4 Fuzzing
| ID | ✔ | Instruction |
|----|----|-------------|
|4.1| | Add `cargo fuzz` target `fuzz_json`: takes arbitrary bytes, attempts `Composition::from_slice()`.|
|4.2| | Enable ASan & UBSan (`-Zsanitizer=address`) in fuzz config.|
|4.3| | Corpus starts with 5 real Lottie files + dictionary of JSON tokens.|

---
## 5 Performance Benchmarks
| ID | ✔ | Instruction |
|----|----|-------------|
|5.1| | `benches/render.rs` (`criterion`) renders frame 0..N of `demo.json`; record ms/frame.|
|5.2| | Export HTML report via `criterion --message-format json`.|
|5.3| | Fail CI if p95 > 120 % of baseline stored in `bench_baseline.json`.|

---
## 6 Size Regression
| ID | ✔ | Instruction |
|----|----|-------------|
|6.1| | `tests/size.rs` builds `rlottie_core` for `thumbv7em-none-eabihf --release`; capture `.rlib` size.|
|6.2| | Assert total < 600 kB; warn at 550 kB.|

---
## 7 Coverage Instrumentation
| ID | ✔ | Instruction |
|----|----|-------------|
|7.1| | Enable `cargo llvm-cov` in CI (`--all-features --workspace`).|
|7.2| | Minimum line coverage target 80 % for `rlottie_core`. Fail if below.|
|7.3| | Upload HTML report artifact.

---
## 8 CI Wiring
| ID | ✔ | Instruction |
|----|----|-------------|
|8.1| | Extend **Workflow** job `test` → runs: unit tests, golden hash test, size regression.|
|8.2| | Add `fuzz` job (nightly, cron) to run 30 min of `cargo fuzz`.|
|8.3| | Publish diff images & coverage reports as artifacts.|

---
*(All items reference C++ rlottie outputs; Rust results are validated *against* those, not themselves.)

