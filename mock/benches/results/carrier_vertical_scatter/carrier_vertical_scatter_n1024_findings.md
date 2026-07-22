# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_scatter_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_scatter_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_scatter_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_scatter_scalar has the worst median (327.82 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_scatter_vert8 at 113.56 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_scatter_vert8 dominates: 138% faster than the next best (carrier_vert_scatter_vert4)

carrier_vert_scatter_vert8 (113.56 us) leads carrier_vert_scatter_vert4 (270.33 us) by 138%, a clear separation rather than a photo finish. CV 8.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_scatter_vert8 beats baseline by 65% (significant)

carrier_vert_scatter_vert8 is -213.20 us (65%) faster than baseline carrier_vert_scatter_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_scatter_scalar is an outlier: 2.9x slower than the field

carrier_vert_scatter_scalar (327.82 us) is 2.9x the fastest (113.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_scatter_vert8 is fastest but the noisiest (CV 8.5%)

carrier_vert_scatter_vert8 wins on median (113.56 us) yet has the highest variance (CV 8.5%), while carrier_vert_scatter_scalar is the steadiest (CV 0.8%, 327.82 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: carrier_vert_scatter_vert8** at 113559.8 ns median (-65.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.89x (fastest 113559.8 ns, slowest 327821.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 331556ns | 330736ns | 328902ns | 330554ns | 334387ns | base |
| carrier_vert_scatter_vert4 | 272883ns | 273039ns | 258239ns | 269326ns | 285540ns | -17.70% |
| carrier_vert_scatter_vert8 | 120758ns | 116039ns | 114200ns | 115682ns | 131650ns | -63.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 328802ns | 326104ns | 331829ns | base | 0.003 |
| carrier_vert_scatter_vert4 | 270124ns | 256081ns | 282368ns | -17.85% | 0.004 |
| carrier_vert_scatter_vert8 | 118304ns | 111761ns | 129230ns | -64.02% | 0.009 |

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_scatter_vert8; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_scatter_scalar | 0.003 | 34.1% |
| carrier_vert_scatter_vert4 | 0.004 | 41.3% |
| carrier_vert_scatter_vert8 | 0.009 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_scatter_scalar | 331556ns | 331556ns | base |
| carrier_vert_scatter_vert4 | 272883ns | 272883ns | -17.70% |
| carrier_vert_scatter_vert8 | 120758ns | 120758ns | -63.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 327822ns | base | --- | [326756, 331829] | --- | --- | --- | --- |
| carrier_vert_scatter_vert4 | 270329ns | -60423.1ns (-18.4%) | [-70146, -45465]ns | [257676, 282368] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_scatter_vert8 | 113560ns | -213196.2ns (-65.0%) | [-219706, -198592]ns | [112123, 129230] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_scatter_scalar | carrier_vert_scatter_vert4 | carrier_vert_scatter_vert8 |
|---|---|---|---|
| 1 | 326104ns | -14.9% | -65.2% |
| 2 | 334096ns | -18.4% | -66.5% |
| 3 | 328164ns | -21.0% | -57.6% |
| 4 | 327408ns | -18.1% | -65.3% |
| 5 | 329562ns | -12.8% | -65.9% |
| 6 | 327480ns | -21.8% | -63.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_scatter_scalar | -0.471 | moderate- |
| carrier_vert_scatter_vert4 | -0.391 | moderate- |
| carrier_vert_scatter_vert8 | -0.319 | moderate- |

**Consistency summary:**

- **carrier_vert_scatter_vert4**: won 6/6, lost 0/6
- **carrier_vert_scatter_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 328166.2ns | 328802.2ns | 99.8% | HIGH |
| carrier_vert_scatter_vert4 | 266413.6ns | 270124.3ns | 98.6% | HIGH |
| carrier_vert_scatter_vert8 | 122807.6ns | 118304.0ns | 103.8% | HIGH |

## Distribution (algo ns)

```
carrier_vert_scatter_scalar (n=6, range 326104.2-331828.9 ns)
  326104.2 |####################
  326390.4 |
  326676.7 |
  326962.9 |
  327249.2 |########################################
  327535.4 |
  327821.6 |
  328107.9 |####################
  328394.1 |
  328680.3 |
  328966.6 |
  329252.8 |
  329539.0 |####################
  329825.3 |
  330111.5 |
  330397.8 |
  330684.0 |
  330970.2 |
  331256.5 |
  331542.7 |
  (0 below, 1 above range)

carrier_vert_scatter_vert4 (n=6, range 256081.2-282368.2 ns)
  256081.2 |########################################
  257395.5 |
  258709.9 |########################################
  260024.2 |
  261338.6 |
  262652.9 |
  263967.3 |
  265281.6 |
  266596.0 |
  267910.3 |########################################
  269224.7 |
  270539.0 |
  271853.4 |########################################
  273167.7 |
  274482.1 |
  275796.4 |
  277110.8 |########################################
  278425.1 |
  279739.5 |
  281053.8 |
  (0 below, 1 above range)

carrier_vert_scatter_vert8 (n=6, range 111761.2-129229.5 ns)
  111761.2 |########################################
  112634.6 |####################
  113508.0 |####################
  114381.5 |
  115254.9 |
  116128.3 |
  117001.7 |
  117875.1 |
  118748.5 |####################
  119622.0 |
  120495.4 |
  121368.8 |
  122242.2 |
  123115.6 |
  123989.0 |
  124862.5 |
  125735.9 |
  126609.3 |
  127482.7 |
  128356.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_scatter_scalar**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert4**: bridge=97.7% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert8**: bridge=102.7% of algo (FFI overhead may distort results)
