#!/usr/bin/env bash
# Build the Zig dispatch variants into the harness variant paths. Zig cdylibs
# are first-class harness variants (they export the same plain C ABI as the
# Rust #[bench_variant] macro), but they are not cargo crates, so `cargo mock
# bench run` skips them; build them with this script before running the bench.
set -euo pipefail
cd "$(dirname "$0")"
for v in switch tail; do
  out="../variants/zig_$v/target/release/libzig_$v.dylib"
  mkdir -p "$(dirname "$out")"
  zig build-lib -dynamic -OReleaseFast -femit-bin="$out" "$v.zig"
  echo "built $out"
done
