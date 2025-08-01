
<p align="center">
  <img src="./rlottie-rs-logo.png" alt="rlottie-rs" />
</p>

<span style="font-size:26px"><b>rlottie-rs</b></span> is a pure‑Rust, no‑std‑friendly re‑implementation of Samsung’s *[*`rlottie`*](https://github.com/Samsung/rlottie)* animation engine.

---

## ✨ Why?

Lottie (Bodymovin) is a ubiquitous vector‑animation format used on the web, Android, iOS—and even micro‑controllers.\
The original reference library **rlottie** is written in C++.  `rlottie‑rs` brings the same feature‑set to safe, idiomatic Rust with first‑class support for:

- ``** / embedded** targets (ARM Cortex‑M, RISC‑V)
- **WASM** playback in the browser
- Desktop SIMD acceleration (SSE2 / NEON)

Our long‑term target is **frame‑perfect parity** with upstream `rlottie` while maintaining a < 600 kB `rlib` for micro‑controllers.

---

## 🚀 Features

| Status | Feature                             |
| ------ | ----------------------------------- |
| ✅      | JSON / .lottie archive loader       |
| ✅      | Shape layers: solid fills & strokes |
| 🛠     | Gradient fills (linear / radial)    |
| 🛠     | Masks & mattes                      |
| 🛠     | Image & text layers                 |
| 🛠     | Repeater / Trim‑Paths               |

*Check the *[*Road‑to‑Parity TODO*](./doc/TODO.md)* for the full progress board.*

---

## 🗂 Project structure

```text
rlottie-rs/
├─ core/           ← rlottie_core crate (rendering engine)
├─ examples/       ← desktop & embedded demos
├─ rlottie/        ← C++ rlottie **submodule (reference only)**
├─ doc/            ← project & testing TODOs
└─ Cargo.toml      ← workspace manifest
```

> **Note**   The `rlottie/` submodule is *not* compiled or linked; it exists solely for test‑vector generation and behaviour comparison.

---

## 🔧 Quick‑start

```bash
# Desktop preview
cargo run --example viewer ./tests/assets/corpus/hello_lottie.json

# Build docs (needs nightly)
cargo +nightly doc --all-features --no-deps

# Embedded (STM32H7, ST7789 display)
cargo build --release \
    --no-default-features \
    --features embedded \
    --target thumbv7em-none-eabihf
```

See `examples/` for more entry‑points.

---

## 📚 Documentation

| Resource             | Link                                                                                       |
| -------------------- | ------------------------------------------------------------------------------------------ |
| API docs (nightly)   | [https://softoboros.github.io/rlottie-rs/](https://softoboros.github.io/rlottie-rs/dev) |
| Road‑to‑Parity board | [`doc/TODO.md`](./doc/TODO.md)                                                             |
| Test‑coverage plan   | [`doc/TODO-TESTING.md`](./doc/TODO-TESTING.md)                                             |

---

## 🤝 Contributing

Bug reports, feature requests, and pull requests are welcome on GitHub:\
[https://github.com/SoftOboros/rlottie-rs](https://github.com/SoftOboros/rlottie-rs)

Please read `CONTRIBUTING.md` (TBD) before submitting code.

---

## 📄 License

`rlottie‑rs` is distributed under the MIT License—see the [LICENSE](./LICENSE) file.\
© SoftOboros Technology, Inc. 2025.

