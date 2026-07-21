# Node record width: 16B pool-spill vs 24B inline operands

2 variants, 6 samples per variant.
Baseline: **hx_recwidth__rec24**

## Highlights

Baseline for all deltas below: **hx_recwidth__rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_recwidth__rec24 dominates: 20% faster than the next best (hx_recwidth__rec16)

hx_recwidth__rec24 (4.98 us) leads hx_recwidth__rec16 (5.96 us) by 20%, a clear separation rather than a photo finish. CV 7.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_recwidth__rec24 is fastest but the noisiest (CV 7.3%)

hx_recwidth__rec24 wins on median (4.98 us) yet has the highest variance (CV 7.3%), while hx_recwidth__rec16 is the steadiest (CV 3.1%, 5.96 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_recwidth__rec24)

The baseline hx_recwidth__rec24 is the fastest (4.98 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_recwidth__rec24) is the fastest** at 4984.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.20x (fastest 4984.1 ns, slowest 5959.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_recwidth__rec16 | 8588ns | 8621ns | 8169ns | 8571ns | 8823ns | +16.45% |
| hx_recwidth__rec24 | 7375ns | 7522ns | 6324ns | 7402ns | 7861ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_recwidth__rec16 | 6002ns | 5753ns | 6238ns | +22.74% | 0.682 |
| hx_recwidth__rec24 | 4890ns | 4188ns | 5218ns | base | 0.838 |

## Performance model

- Peak throughput: **0.978 Gops/s** (hx_recwidth__rec24; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_recwidth__rec16 | 0.687 | 70.3% |
| hx_recwidth__rec24 | 0.822 | 84.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_recwidth__rec16 | 8588ns | 8588ns | +16.45% |
| hx_recwidth__rec24 | 7375ns | 7375ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_recwidth__rec24 | 4984ns | base | --- | [4468, 5218] | --- | --- | --- | --- |
| hx_recwidth__rec16 | 5959ns | +1125.2ns (+22.6%) | [+854, +1357]ns | [5808, 6238] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_recwidth__rec24 | hx_recwidth__rec16 |
|---|---|---|
| 1 | 4188ns | +37.4% |
| 2 | 5080ns | +15.4% |
| 3 | 4748ns | +24.2% |
| 4 | 5093ns | +21.9% |
| 5 | 4888ns | +23.2% |
| 6 | 5342ns | +17.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_recwidth__rec16 | 0.180 | ok |
| hx_recwidth__rec24 | -0.239 | moderate- |

**Consistency summary:**

- **hx_recwidth__rec16**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_recwidth__rec16 | 3.1ns | 6001.9ns | 0.1% |  |
| hx_recwidth__rec24 | 3.1ns | 4889.8ns | 0.1% |  |

## Distribution (algo ns)

```
hx_recwidth__rec16 (n=6, range 5753.3-6238.1 ns)
   5753.3 |########################################
   5777.5 |
   5801.8 |
   5826.0 |
   5850.3 |########################################
   5874.5 |########################################
   5898.7 |
   5923.0 |
   5947.2 |
   5971.5 |
   5995.7 |
   6019.9 |########################################
   6044.2 |
   6068.4 |
   6092.7 |
   6116.9 |
   6141.1 |
   6165.4 |
   6189.6 |########################################
   6213.9 |
  (0 below, 1 above range)

hx_recwidth__rec24 (n=6, range 4187.5-5217.5 ns)
   4187.5 |####################
   4239.0 |
   4290.5 |
   4342.0 |
   4393.5 |
   4445.0 |
   4496.5 |
   4548.0 |
   4599.5 |
   4651.0 |
   4702.5 |####################
   4754.0 |
   4805.5 |
   4857.0 |####################
   4908.5 |
   4960.0 |
   5011.5 |
   5063.0 |########################################
   5114.5 |
   5166.0 |
  (0 below, 1 above range)

```
