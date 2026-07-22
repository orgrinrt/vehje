# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_tight_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_tight_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_tight_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_tight_scalar has the worst median (17.01 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_tight_vert8 at 7.16 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_tight_vert8 dominates: 24% faster than the next best (carrier_vert_tight_vert4)

carrier_vert_tight_vert8 (7.16 us) leads carrier_vert_tight_vert4 (8.89 us) by 24%, a clear separation rather than a photo finish. CV 10.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_tight_vert8 beats baseline by 55% (significant)

carrier_vert_tight_vert8 is -9.37 us (55%) faster than baseline carrier_vert_tight_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_tight_scalar is an outlier: 2.4x slower than the field

carrier_vert_tight_scalar (17.01 us) is 2.4x the fastest (7.16 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_tight_vert8** at 7164.8 ns median (-57.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.37x (fastest 7164.8 ns, slowest 17006.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 20824ns | 19345ns | 18605ns | 19133ns | 24470ns | base |
| carrier_vert_tight_vert4 | 11232ns | 11409ns | 10238ns | 11082ns | 11954ns | -46.06% |
| carrier_vert_tight_vert8 | 10081ns | 9424ns | 9395ns | 9415ns | 11424ns | -51.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 18341ns | 16418ns | 21561ns | base | 0.003 |
| carrier_vert_tight_vert4 | 8695ns | 7985ns | 9127ns | -52.59% | 0.007 |
| carrier_vert_tight_vert8 | 7627ns | 7030ns | 8677ns | -58.42% | 0.008 |

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_tight_vert8; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_tight_scalar | 0.004 | 41.3% |
| carrier_vert_tight_vert4 | 0.007 | 79.1% |
| carrier_vert_tight_vert8 | 0.009 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_tight_scalar | 20824ns | 20824ns | base |
| carrier_vert_tight_vert4 | 11232ns | 11232ns | -46.06% |
| carrier_vert_tight_vert8 | 10081ns | 10081ns | -51.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 17006ns | base | --- | [16454, 21561] | --- | --- | --- | --- |
| carrier_vert_tight_vert4 | 8890ns | -8572.7ns (-50.4%) | [-12853, -7513]ns | [8067, 9127] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_tight_vert8 | 7165ns | -9367.0ns (-55.1%) | [-14445, -8329]ns | [7039, 8677] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_tight_scalar | carrier_vert_tight_vert4 | carrier_vert_tight_vert8 |
|---|---|---|---|
| 1 | 16490ns | -51.6% | -57.4% |
| 2 | 16418ns | -45.4% | -56.5% |
| 3 | 16561ns | -45.8% | -49.8% |
| 4 | 17451ns | -49.5% | -48.2% |
| 5 | 25112ns | -67.6% | -71.4% |
| 6 | 18011ns | -48.5% | -60.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_tight_scalar | 0.005 | ok |
| carrier_vert_tight_vert4 | -0.352 | moderate- |
| carrier_vert_tight_vert8 | 0.158 | ok |

**Consistency summary:**

- **carrier_vert_tight_vert4**: won 6/6, lost 0/6
- **carrier_vert_tight_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 98644.4ns | 18340.7ns | 537.8% | HIGH |
| carrier_vert_tight_vert4 | 91888.9ns | 8694.5ns | 1056.9% | HIGH |
| carrier_vert_tight_vert8 | 91680.7ns | 7626.9ns | 1202.1% | HIGH |

## Distribution (algo ns)

```
carrier_vert_tight_scalar (n=6, range 16418.3-21561.5 ns)
  16418.3 |########################################
  16675.5 |
  16932.6 |
  17189.8 |
  17446.9 |#############
  17704.1 |
  17961.2 |#############
  18218.4 |
  18475.6 |
  18732.7 |
  18989.9 |
  19247.0 |
  19504.2 |
  19761.3 |
  20018.5 |
  20275.7 |
  20532.8 |
  20790.0 |
  21047.1 |
  21304.3 |
  (0 below, 1 above range)

carrier_vert_tight_vert4 (n=6, range 7985.4-9126.7 ns)
   7985.4 |####################
   8042.5 |
   8099.5 |####################
   8156.6 |
   8213.7 |
   8270.7 |
   8327.8 |
   8384.9 |
   8441.9 |
   8499.0 |
   8556.0 |
   8613.1 |
   8670.2 |
   8727.2 |
   8784.3 |####################
   8841.4 |
   8898.4 |
   8955.5 |########################################
   9012.6 |
   9069.6 |
  (0 below, 1 above range)

carrier_vert_tight_vert8 (n=6, range 7030.4-8676.9 ns)
   7030.4 |########################################
   7112.7 |########################################
   7195.0 |
   7277.4 |
   7359.7 |
   7442.0 |
   7524.3 |
   7606.7 |
   7689.0 |
   7771.3 |
   7853.6 |
   7935.9 |
   8018.3 |
   8100.6 |
   8182.9 |
   8265.2 |####################
   8347.6 |
   8429.9 |
   8512.2 |
   8594.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_tight_scalar**: bridge=576.0% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert4**: bridge=1032.1% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert8**: bridge=1277.1% of algo (FFI overhead may distort results)
