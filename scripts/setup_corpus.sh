#!/usr/bin/env bash
# Copyright © SoftOboros Technology, Inc.
# SPDX-License-Identifier: MIT
set -euo pipefail
ROOT="$(git rev-parse --show-toplevel)"
BUILD_DIR="$ROOT/rlottie/build"
# Ensure runtime can find rlottie shared library
export LD_LIBRARY_PATH="$BUILD_DIR:${LD_LIBRARY_PATH:-}"
# Build rlottie if not already built
if [ ! -f "$BUILD_DIR/librlottie.so" ] && [ ! -f "$BUILD_DIR/librlottie.a" ]; then
  cmake -S "$ROOT/rlottie" -B "$BUILD_DIR" -DCMAKE_BUILD_TYPE=Release
  cmake --build "$BUILD_DIR" --target rlottie
fi
# Compile helper
c++ -std=c++11 -I"$ROOT/rlottie/inc" -L"$BUILD_DIR" "$ROOT/scripts/lottie2png.cpp" \
    -o "$ROOT/scripts/lottie2png" -Wl,-rpath,"$BUILD_DIR" -lrlottie
# Generate PNG frames
"$ROOT/scripts/gen_png.sh"
# Compute hashes
python3 - <<'PY' "$ROOT"/tests/assets/corpus "$ROOT"/tests/assets/hashes.json "$ROOT"
import sys, hashlib, json, pathlib
corpus=pathlib.Path(sys.argv[1])
out=pathlib.Path(sys.argv[2])
root=pathlib.Path(sys.argv[3])
result={}
for json_file in corpus.glob('*.json'):
    for frame in (0,30,60):
        png=json_file.with_name(json_file.name+f'_{frame}.png')
        with open(png,'rb') as f:
            result[str(png.relative_to(root))] = hashlib.sha256(f.read()).hexdigest()
with open(out,'w') as f:
    json.dump(result,f,indent=2,sort_keys=True)
PY
