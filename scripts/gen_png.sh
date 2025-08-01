#!/usr/bin/env bash
## Copyright © SoftOboros Technology, Inc.
## SPDX-License-Identifier: MIT
set -euo pipefail
BIN_DIR=$(dirname "$0")
L2P="$BIN_DIR/lottie2png"
CORPUS_DIR="$(git rev-parse --show-toplevel)/tests/assets/corpus"
for json in "$CORPUS_DIR"/*.json; do
  for frame in 0 30 60; do
    "$L2P" "$json" 240 240 "$frame" >/dev/null
done
done
