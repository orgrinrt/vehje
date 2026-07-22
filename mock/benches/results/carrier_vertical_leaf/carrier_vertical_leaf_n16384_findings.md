# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_leaf_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_leaf_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_leaf_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_leaf_scalar has the worst median (8.48 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_leaf_vert8 at 2.18 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_leaf_vert8 dominates: 33% faster than the next best (carrier_vert_leaf_vert4)

carrier_vert_leaf_vert8 (2.18 ms) leads carrier_vert_leaf_vert4 (2.89 ms) by 33%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_leaf_vert8 beats baseline by 74% (significant)

carrier_vert_leaf_vert8 is -6.31 ms (74%) faster than baseline carrier_vert_leaf_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_leaf_scalar is an outlier: 3.9x slower than the field

carrier_vert_leaf_scalar (8.48 ms) is 3.9x the fastest (2.18 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.9x the fastest

Fastest carrier_vert_leaf_vert8 (2.18 ms) to slowest carrier_vert_leaf_scalar (8.48 ms): 3.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_leaf_vert8** at 2177633.4 ns median (-74.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.90x (fastest 2177633.4 ns, slowest 8482392.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 8503158ns | 8486658ns | 8465570ns | 8482677ns | 8552672ns | base |
| carrier_vert_leaf_vert4 | 2900738ns | 2897028ns | 2891347ns | 2896446ns | 2911870ns | -65.89% |
| carrier_vert_leaf_vert8 | 2178418ns | 2180511ns | 2154072ns | 2178449ns | 2190543ns | -74.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 8499070ns | 8460924ns | 8548743ns | base | 0.002 |
| carrier_vert_leaf_vert4 | 2897154ns | 2887052ns | 2908799ns | -65.91% | 0.006 |
| carrier_vert_leaf_vert8 | 2174986ns | 2149977ns | 2187002ns | -74.41% | 0.008 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_vert_leaf_vert8; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_leaf_scalar | 0.002 | 25.3% |
| carrier_vert_leaf_vert4 | 0.006 | 74.3% |
| carrier_vert_leaf_vert8 | 0.008 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_leaf_scalar | 8503158ns | 8503158ns | base |
| carrier_vert_leaf_vert4 | 2900738ns | 2900738ns | -65.89% |
| carrier_vert_leaf_vert8 | 2178418ns | 2178418ns | -74.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 8482392ns | base | --- | [8466076, 8548743] | --- | --- | --- | --- |
| carrier_vert_leaf_vert4 | 2893487ns | -5580824.4ns (-65.8%) | [-5650694, -5574232]ns | [2889175, 2908799] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_leaf_vert8 | 2177633ns | -6306838.5ns (-74.4%) | [-6373580, -6291833]ns | [2160324, 2187002] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_leaf_scalar | carrier_vert_leaf_vert4 | carrier_vert_leaf_vert8 |
|---|---|---|---|
| 1 | 8460924ns | -65.9% | -74.6% |
| 2 | 8488192ns | -65.9% | -74.3% |
| 3 | 8598294ns | -66.3% | -74.8% |
| 4 | 8476592ns | -65.9% | -74.2% |
| 5 | 8499191ns | -65.6% | -74.4% |
| 6 | 8471229ns | -65.8% | -74.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_leaf_scalar | -0.228 | moderate- |
| carrier_vert_leaf_vert4 | -0.020 | ok |
| carrier_vert_leaf_vert8 | -0.312 | moderate- |

**Consistency summary:**

- **carrier_vert_leaf_vert4**: won 6/6, lost 0/6
- **carrier_vert_leaf_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 8800518.5ns | 8499070.4ns | 103.5% | HIGH |
| carrier_vert_leaf_vert4 | 2902744.2ns | 2897153.7ns | 100.2% | HIGH |
| carrier_vert_leaf_vert8 | 2203499.0ns | 2174986.5ns | 101.3% | HIGH |

## Distribution (algo ns)

```
carrier_vert_leaf_scalar (n=6, range 8460924.2-8548742.7 ns)
  8460924.2 |########################################
  8465315.1 |
  8469706.0 |########################################
  8474097.0 |########################################
  8478487.9 |
  8482878.8 |
  8487269.8 |########################################
  8491660.7 |
  8496051.6 |########################################
  8500442.5 |
  8504833.4 |
  8509224.4 |
  8513615.3 |
  8518006.2 |
  8522397.1 |
  8526788.1 |
  8531179.0 |
  8535569.9 |
  8539960.8 |
  8544351.8 |
  (0 below, 1 above range)

carrier_vert_leaf_vert4 (n=6, range 2887052.1-2908799.0 ns)
  2887052.1 |########################################
  2888139.4 |
  2889226.8 |
  2890314.1 |########################################
  2891401.5 |
  2892488.8 |########################################
  2893576.2 |########################################
  2894663.5 |
  2895750.8 |########################################
  2896838.2 |
  2897925.5 |
  2899012.9 |
  2900100.2 |
  2901187.6 |
  2902274.9 |
  2903362.2 |
  2904449.6 |
  2905536.9 |
  2906624.3 |
  2907711.6 |
  (0 below, 1 above range)

carrier_vert_leaf_vert8 (n=6, range 2149976.7-2187002.3 ns)
  2149976.7 |########################################
  2151828.0 |
  2153679.3 |
  2155530.5 |
  2157381.8 |
  2159233.1 |
  2161084.4 |
  2162935.7 |
  2164786.9 |
  2166638.2 |
  2168489.5 |
  2170340.8 |########################################
  2172192.1 |
  2174043.3 |########################################
  2175894.6 |
  2177745.9 |
  2179597.2 |########################################
  2181448.5 |
  2183299.7 |
  2185151.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_leaf_scalar**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert4**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert8**: bridge=101.3% of algo (FFI overhead may distort results)
