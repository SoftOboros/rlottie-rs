#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all -- --check
cargo check --all-features --workspace
cargo clippy --all-features -- -D warnings
cargo test --all-features
cargo +nightly doc --all-features --no-deps
if command -v cargo-bloat >/dev/null; then
    cargo bloat -n 0 | grep TOTAL
else
    echo "warning: cargo-bloat not installed" >&2
fi
if [ -f tests/golden_hash.rs ]; then
    "$(dirname "$0")/setup_corpus.sh"
    cargo test --test golden_hash
fi
