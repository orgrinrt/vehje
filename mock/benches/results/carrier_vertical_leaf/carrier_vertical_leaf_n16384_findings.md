# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_leaf_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_leaf_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_leaf_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_leaf_scalar has the worst median (6.59 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_leaf_vert8 at 2.22 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_leaf_vert8 dominates: 33% faster than the next best (carrier_vert_leaf_vert4)

carrier_vert_leaf_vert8 (2.22 ms) leads carrier_vert_leaf_vert4 (2.95 ms) by 33%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_leaf_vert8 beats baseline by 66% (significant)

carrier_vert_leaf_vert8 is -4.33 ms (66%) faster than baseline carrier_vert_leaf_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_leaf_scalar is an outlier: 3.0x slower than the field

carrier_vert_leaf_scalar (6.59 ms) is 3.0x the fastest (2.22 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_leaf_vert8** at 2215456.8 ns median (-66.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.97x (fastest 2215456.8 ns, slowest 6589996.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 6570122ns | 6594175ns | 6501211ns | 6564972ns | 6612302ns | base |
| carrier_vert_leaf_vert4 | 2962456ns | 2953548ns | 2933828ns | 2949519ns | 2996176ns | -54.91% |
| carrier_vert_leaf_vert8 | 2229065ns | 2219135ns | 2198286ns | 2215425ns | 2264915ns | -66.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 6566153ns | 6497882ns | 6608023ns | base | 0.002 |
| carrier_vert_leaf_vert4 | 2958592ns | 2929997ns | 2992061ns | -54.94% | 0.006 |
| carrier_vert_leaf_vert8 | 2225431ns | 2194662ns | 2261320ns | -66.11% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 40598165 | 77901302 | 0.521 | 1.00× |
| carrier_vert_leaf_vert4 | 18280623 | 30513538 | 0.599 | 0.45× |
| carrier_vert_leaf_vert8 | 13873597 | 20663992 | 0.671 | 0.34× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_vert_leaf_vert8; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_leaf_scalar | 0.002 | 33.3% |
| carrier_vert_leaf_vert4 | 0.006 | 74.4% |
| carrier_vert_leaf_vert8 | 0.007 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_leaf_scalar | 6570122ns | 6570122ns | base |
| carrier_vert_leaf_vert4 | 2962456ns | 2962456ns | -54.91% |
| carrier_vert_leaf_vert8 | 2229065ns | 2229065ns | -66.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 6589997ns | base | --- | [6500439, 6608023] | --- | --- | --- | --- |
| carrier_vert_leaf_vert4 | 2949788ns | -3638529.0ns (-55.2%) | [-3662645, -3521510]ns | [2933926, 2992061] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_leaf_vert8 | 2215457ns | -4331290.8ns (-65.7%) | [-4405892, -4284983]ns | [2199516, 2261320] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_leaf_scalar | carrier_vert_leaf_vert4 | carrier_vert_leaf_vert8 |
|---|---|---|---|
| 1 | 6609119ns | -55.3% | -66.1% |
| 2 | 6576104ns | -55.1% | -65.3% |
| 3 | 6606927ns | -55.5% | -66.6% |
| 4 | 6497882ns | -54.9% | -66.0% |
| 5 | 6502997ns | -53.4% | -65.8% |
| 6 | 6603890ns | -55.4% | -66.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_leaf_scalar | -0.002 | ok |
| carrier_vert_leaf_vert4 | -0.320 | moderate- |
| carrier_vert_leaf_vert8 | 0.044 | ok |

**Consistency summary:**

- **carrier_vert_leaf_vert4**: won 6/6, lost 0/6
- **carrier_vert_leaf_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 6622942.5ns | 6566153.0ns | 100.9% | HIGH |
| carrier_vert_leaf_vert4 | 2959125.6ns | 2958591.8ns | 100.0% | HIGH |
| carrier_vert_leaf_vert8 | 2258453.9ns | 2225431.2ns | 101.5% | HIGH |

## Distribution (algo ns)

```
carrier_vert_leaf_scalar (n=6, range 6497882.1-6608022.8 ns)
  6497882.1 |########################################
  6503389.1 |
  6508896.2 |
  6514403.2 |
  6519910.2 |
  6525417.3 |
  6530924.3 |
  6536431.3 |
  6541938.4 |
  6547445.4 |
  6552952.4 |
  6558459.5 |
  6563966.5 |
  6569473.5 |
  6574980.6 |####################
  6580487.6 |
  6585994.6 |
  6591501.7 |
  6597008.7 |
  6602515.7 |########################################
  (0 below, 1 above range)

carrier_vert_leaf_vert4 (n=6, range 2929997.1-2992061.5 ns)
  2929997.1 |########################################
  2933100.3 |
  2936203.5 |########################################
  2939306.8 |
  2942410.0 |
  2945513.2 |########################################
  2948616.4 |
  2951719.6 |########################################
  2954822.8 |########################################
  2957926.1 |
  2961029.3 |
  2964132.5 |
  2967235.7 |
  2970338.9 |
  2973442.1 |
  2976545.4 |
  2979648.6 |
  2982751.8 |
  2985855.0 |
  2988958.2 |
  (0 below, 1 above range)

carrier_vert_leaf_vert8 (n=6, range 2194662.5-2261320.5 ns)
  2194662.5 |########################################
  2197995.4 |
  2201328.3 |########################################
  2204661.2 |
  2207994.1 |########################################
  2211327.0 |
  2214659.9 |
  2217992.8 |########################################
  2221325.7 |
  2224658.6 |
  2227991.5 |
  2231324.4 |
  2234657.3 |
  2237990.2 |########################################
  2241323.1 |
  2244656.0 |
  2247988.9 |
  2251321.8 |
  2254654.7 |
  2257987.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_leaf_scalar**: bridge=101.1% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert4**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert8**: bridge=101.4% of algo (FFI overhead may distort results)
