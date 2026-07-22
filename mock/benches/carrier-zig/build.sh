#!/usr/bin/env bash
# Build the carrier Zig interpreter artifacts.
#
#   zigcheck            cross-validation CLI (reads [seed][program] on stdin,
#                       writes the checksum; the Rust test spawns it).
#   libcarrier_zig.*    cdylib exporting carrier_zig_switch / carrier_zig_tail
#                       for the eventual harness variant (consumes identical
#                       program bytes; the harness passes them at run time).
#
# The Zig cell is the one dispatch shape a Rust-only carrier cannot ask: a
# guaranteed-tail-call (@call(.always_tail)) token-threaded loop, stable in Zig,
# and the shipped vehje runtime is Zig. Run this before the Rust cross-validation
# test and before benching the Zig variant.
set -euo pipefail
cd "$(dirname "$0")"

zig build-exe -OReleaseFast -lc -femit-bin=zigcheck zigcheck.zig
echo "built zigcheck"

zig build-lib -dynamic -OReleaseFast -femit-bin=libcarrier_zig.dylib interp.zig
echo "built libcarrier_zig.dylib"
