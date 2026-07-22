# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_tight_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_tight_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_tight_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_tight_scalar has the worst median (340.27 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_tight_vert8 at 123.85 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_tight_vert8 dominates: 31% faster than the next best (carrier_vert_tight_vert4)

carrier_vert_tight_vert8 (123.85 us) leads carrier_vert_tight_vert4 (162.03 us) by 31%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_tight_vert8 beats baseline by 63% (significant)

carrier_vert_tight_vert8 is -216.06 us (63%) faster than baseline carrier_vert_tight_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_tight_scalar is an outlier: 2.7x slower than the field

carrier_vert_tight_scalar (340.27 us) is 2.7x the fastest (123.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_tight_vert8** at 123852.1 ns median (-63.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.75x (fastest 123852.1 ns, slowest 340274.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 343276ns | 343585ns | 340320ns | 342519ns | 345891ns | base |
| carrier_vert_tight_vert4 | 165235ns | 164681ns | 162499ns | 164130ns | 168261ns | -51.87% |
| carrier_vert_tight_vert8 | 127180ns | 126554ns | 122855ns | 125384ns | 132038ns | -62.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 340271ns | 337143ns | 343126ns | base | 0.003 |
| carrier_vert_tight_vert4 | 162634ns | 160120ns | 165643ns | -52.20% | 0.006 |
| carrier_vert_tight_vert8 | 124538ns | 120474ns | 129282ns | -63.40% | 0.008 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_vert_tight_vert8; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_tight_scalar | 0.003 | 35.4% |
| carrier_vert_tight_vert4 | 0.006 | 74.4% |
| carrier_vert_tight_vert8 | 0.008 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_tight_scalar | 343276ns | 343276ns | base |
| carrier_vert_tight_vert4 | 165235ns | 165235ns | -51.87% |
| carrier_vert_tight_vert8 | 127180ns | 127180ns | -62.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 340275ns | base | --- | [337412, 343126] | --- | --- | --- | --- |
| carrier_vert_tight_vert4 | 162032ns | -177681.3ns (-52.2%) | [-180695, -174534]ns | [160228, 165643] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_tight_vert8 | 123852ns | -216057.9ns (-63.5%) | [-220278, -210862]ns | [120481, 129282] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_tight_scalar | carrier_vert_tight_vert4 | carrier_vert_tight_vert8 |
|---|---|---|---|
| 1 | 337143ns | -52.4% | -61.4% |
| 2 | 343107ns | -52.3% | -64.8% |
| 3 | 337680ns | -52.5% | -64.3% |
| 4 | 338616ns | -50.9% | -64.4% |
| 5 | 343145ns | -51.9% | -62.5% |
| 6 | 341933ns | -53.2% | -62.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_tight_scalar | -0.311 | moderate- |
| carrier_vert_tight_vert4 | -0.269 | moderate- |
| carrier_vert_tight_vert8 | 0.047 | ok |

**Consistency summary:**

- **carrier_vert_tight_vert4**: won 6/6, lost 0/6
- **carrier_vert_tight_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 340048.1ns | 340270.8ns | 99.9% | HIGH |
| carrier_vert_tight_vert4 | 163205.5ns | 162634.1ns | 100.4% | HIGH |
| carrier_vert_tight_vert8 | 126022.2ns | 124538.1ns | 101.2% | HIGH |

## Distribution (algo ns)

```
carrier_vert_tight_scalar (n=6, range 337142.9-343126.1 ns)
  337142.9 |########################################
  337442.1 |########################################
  337741.2 |
  338040.4 |
  338339.5 |########################################
  338638.7 |
  338937.8 |
  339237.0 |
  339536.2 |
  339835.3 |
  340134.5 |
  340433.6 |
  340732.8 |
  341031.9 |
  341331.1 |
  341630.3 |
  341929.4 |########################################
  342228.6 |
  342527.7 |
  342826.9 |########################################
  (0 below, 1 above range)

carrier_vert_tight_vert4 (n=6, range 160120.0-165643.1 ns)
  160120.0 |########################################
  160396.2 |####################
  160672.3 |
  160948.5 |
  161224.6 |
  161500.8 |
  161776.9 |
  162053.1 |
  162329.2 |
  162605.4 |
  162881.5 |
  163157.7 |
  163433.9 |####################
  163710.0 |
  163986.2 |
  164262.3 |
  164538.5 |
  164814.6 |
  165090.8 |####################
  165366.9 |
  (0 below, 1 above range)

carrier_vert_tight_vert8 (n=6, range 120473.7-129281.6 ns)
  120473.7 |########################################
  120914.1 |
  121354.5 |
  121794.9 |
  122235.3 |
  122675.7 |
  123116.1 |
  123556.5 |
  123996.9 |
  124437.3 |
  124877.7 |
  125318.1 |
  125758.5 |
  126198.9 |
  126639.3 |#############
  127079.7 |
  127520.1 |
  127960.5 |
  128400.9 |#############
  128841.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_tight_scalar**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert4**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert8**: bridge=101.3% of algo (FFI overhead may distort results)
