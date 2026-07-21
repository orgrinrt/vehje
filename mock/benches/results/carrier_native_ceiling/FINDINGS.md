# carrier_native_ceiling: interp vs shape-specialized native, opaque program

**Strength: measurement** (wall-clock via CNTVCT_EL0 @ 24 MHz; 6 passes; cross-validated byte-exact).
**Audit defect fixed: C1.** The old interp-vs-native "1.2x ceiling" measured native-versus-native:
its two-instruction program was a comptime-visible constant the optimizer partially evaluated the
interpreter into. Here the program is a multiply-add chain that crosses into every variant as wire
bytes (FFI-opaque), so no variant's optimizer can see it and fold it.

## Result

Ratio = variant median / native median (warm), Apple M1, per-size:

| n     | native | switch | fntable |
|-------|--------|--------|---------|
| 64    | 1.00x  | 2.05x  | 2.11x   |
| 256   | 1.00x  | 2.01x  | 2.16x   |
| 1024  | 1.00x  | 2.05x  | 2.21x   |
| 4096  | 1.00x  | 2.02x  | 2.12x   |
| 16384 | 1.00x  | 2.04x  | 2.22x   |

The switch interpreter is ~2.0x slower than shape-specialized native; the function-pointer-table
interpreter ~2.1-2.2x. The ratio is stable across three orders of magnitude of program size, which
is itself evidence the measurement is clean (no size-dependent artifact). fntable is consistently
~5-10% slower than switch: on a maximally predictable opcode stream the switch's jump table plus a
correctly-predicted indirect branch beats the fntable's pointer load + indirect call.

## Cost-model sanity line

native, per work-node (a const fold or a chain mul/add fold), converting ns via M1 Firestorm 3.2 GHz:

| n     | ns/work-node | cyc/work-node |
|-------|--------------|---------------|
| 64    | 1.47         | 4.71          |
| 256   | 1.23         | 3.92          |
| 1024  | 1.13         | 3.62          |
| 4096  | 1.07         | 3.42          |
| 16384 | 1.01         | 3.22          |

~3.2-4.7 cycles per simple memory-touching work unit is ~1-1.5 IPC, well under the M1's ~8-wide
retire. The number is physically real, not a partial-eval artifact (an artifact would imply an
impossible >30 IPC). cyc/work-node falls with n as fixed per-call overhead amortizes.

## What this ratio is, and is not

This is the interpreter's **best case**, a lower bound on interp/native overhead, for two reasons:

1. The madd stream is perfectly predictable (MUL, ADD, MUL, ADD, ...), so the switch's indirect
   branch essentially never mispredicts. On an unpredictable opcode stream dispatch misprediction
   dominates and the ratio grows (that dependence is measured directly in `carrier_dispatch`, not
   here; the native ceiling needs a hand-compilable shape and so must use a regular program).
2. The madd program is ~2/3 constant-declaration nodes whose folding both variants share, diluting
   the pure compute-dispatch difference. Native folds consts inline from the wire (an indexed load,
   the same the interpreter's CONST handler does), so the comparison stays fair.

The native baseline was strengthened after first run: it previously allocated a `Vec` and did nconst
stores per call, penalizing native and understating the ratio (~1.8x). Reading consts inline from
the wire removed that asymmetry; the honest ratio is ~2.0x.

## Cross-validation

All three variants fold the identical rolling-hash checksum over the same opaque program; the
`native_madd_matches_interp` unit test asserts byte-exact agreement across seeds. So the ratio is
dispatch-and-decode overhead, not a difference in what was computed.

## Bottom line

On a program maximally friendly to interpretation (predictable dispatch, no cache pressure), a
switch interpreter costs ~2x a compiled native loop. Real programs are less friendly; ~2x is the
floor, not the typical case. The "1.2x native ceiling" headline is retracted: it measured the
optimizer, not an interpreter.
