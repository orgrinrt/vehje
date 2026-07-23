# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), real profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_real_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_real_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_real_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_real_scalar has the worst median (294.84 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_real_vert8 at 116.59 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_real_vert8 dominates: 127% faster than the next best (carrier_vert_real_vert4)

carrier_vert_real_vert8 (116.59 us) leads carrier_vert_real_vert4 (264.91 us) by 127%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_real_vert8 beats baseline by 61% (significant)

carrier_vert_real_vert8 is -179.72 us (61%) faster than baseline carrier_vert_real_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_real_scalar is an outlier: 2.5x slower than the field

carrier_vert_real_scalar (294.84 us) is 2.5x the fastest (116.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_real_vert8** at 116586.2 ns median (-60.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.53x (fastest 116586.2 ns, slowest 294839.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 296275ns | 297108ns | 287625ns | 295086ns | 302383ns | base |
| carrier_vert_real_vert4 | 266932ns | 267514ns | 252412ns | 266820ns | 274361ns | -9.90% |
| carrier_vert_real_vert8 | 119378ns | 119025ns | 114712ns | 118520ns | 122998ns | -59.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_real_scalar | 293953ns | 285320ns | 300039ns | base | 0.003 |
| carrier_vert_real_vert4 | 264293ns | 249860ns | 271555ns | -10.09% | 0.004 |
| carrier_vert_real_vert8 | 116914ns | 112252ns | 120450ns | -60.23% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_real_scalar | 1838131 | 5273788 | 0.349 | 1.00× |
| carrier_vert_real_vert4 | 1660588 | 2002954 | 0.829 | 0.90× |
| carrier_vert_real_vert8 | 730006 | 1412155 | 0.517 | 0.40× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_real_vert8; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_real_scalar | 0.003 | 38.1% |
| carrier_vert_real_vert4 | 0.004 | 42.4% |
| carrier_vert_real_vert8 | 0.009 | 96.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_real_scalar | 296275ns | 296275ns | base |
| carrier_vert_real_vert4 | 266932ns | 266932ns | -9.90% |
| carrier_vert_real_vert8 | 119378ns | 119378ns | -59.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 294839ns | base | --- | [286982, 300039] | --- | --- | --- | --- |
| carrier_vert_real_vert4 | 264907ns | -30818.3ns (-10.5%) | [-41748, -16415]ns | [256417, 271555] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_real_vert8 | 116586ns | -179716.2ns (-61.0%) | [-184782, -166620]ns | [113706, 120450] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_real_scalar | carrier_vert_real_vert4 | carrier_vert_real_vert8 |
|---|---|---|---|
| 1 | 300577ns | -16.9% | -61.7% |
| 2 | 295382ns | -10.3% | -60.1% |
| 3 | 288645ns | -8.3% | -59.2% |
| 4 | 299501ns | -10.9% | -61.5% |
| 5 | 294296ns | -10.6% | -61.9% |
| 6 | 285320ns | -3.1% | -56.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_real_scalar | -0.159 | ok |
| carrier_vert_real_vert4 | -0.080 | ok |
| carrier_vert_real_vert8 | -0.353 | moderate- |

**Consistency summary:**

- **carrier_vert_real_vert4**: won 6/6, lost 0/6
- **carrier_vert_real_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_real_scalar | 298756.8ns | 293953.5ns | 101.6% | HIGH |
| carrier_vert_real_vert4 | 271109.0ns | 264293.0ns | 102.6% | HIGH |
| carrier_vert_real_vert8 | 119057.2ns | 116914.1ns | 101.8% | HIGH |

## Distribution (algo ns)

```
carrier_vert_real_scalar (n=6, range 285320.0-300038.8 ns)
  285320.0 |########################################
  286055.9 |
  286791.9 |
  287527.8 |
  288263.8 |########################################
  288999.7 |
  289735.6 |
  290471.6 |
  291207.5 |
  291943.4 |
  292679.4 |
  293415.3 |
  294151.2 |########################################
  294887.2 |########################################
  295623.1 |
  296359.1 |
  297095.0 |
  297830.9 |
  298566.9 |
  299302.8 |########################################
  (0 below, 1 above range)

carrier_vert_real_vert4 (n=6, range 249859.6-271555.0 ns)
  249859.6 |########################################
  250944.4 |
  252029.1 |
  253113.9 |
  254198.7 |
  255283.5 |
  256368.2 |
  257453.0 |
  258537.8 |
  259622.5 |
  260707.3 |
  261792.1 |
  262876.8 |########################################
  263961.6 |########################################
  265046.4 |########################################
  266131.2 |########################################
  267215.9 |
  268300.7 |
  269385.5 |
  270470.2 |
  (0 below, 1 above range)

carrier_vert_real_vert8 (n=6, range 112252.1-120449.9 ns)
  112252.1 |####################
  112662.0 |
  113071.9 |
  113481.8 |
  113891.7 |
  114301.6 |
  114711.5 |
  115121.3 |########################################
  115531.2 |
  115941.1 |
  116351.0 |
  116760.9 |
  117170.8 |
  117580.7 |####################
  117990.6 |####################
  118400.5 |
  118810.4 |
  119220.3 |
  119630.2 |
  120040.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_real_scalar**: bridge=101.2% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert4**: bridge=102.2% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert8**: bridge=100.2% of algo (FFI overhead may distort results)
