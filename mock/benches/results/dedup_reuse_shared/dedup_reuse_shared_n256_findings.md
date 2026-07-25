# Dedup vs reuse composition, 60% shared: reuse's adversarial regime

4 variants, 6 samples per variant.
Baseline: **dru_s_plain**

## Highlights

Baseline for all deltas below: **dru_s_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dru_s_reuse dominates: 39% faster than the next best (dru_s_plain)

dru_s_reuse (6.10 us) leads dru_s_plain (8.46 us) by 39%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dru_s_reuse beats baseline by 28% (significant)

dru_s_reuse is -2.36 us (28%) faster than baseline dru_s_plain, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### dru_s_dedup is an outlier: 8.4x slower than the field

dru_s_dedup (51.34 us) is 8.4x the fastest (6.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {dru_s_reuse, dru_s_plain} vs {dru_s_both, dru_s_dedup} (226% apart)

The field splits into a fast tier {dru_s_reuse, dru_s_plain} and a slow tier {dru_s_both, dru_s_dedup} with a 226% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.4x the fastest

Fastest dru_s_reuse (6.10 us) to slowest dru_s_dedup (51.34 us): 8.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dru_s_reuse** at 6100.6 ns median (-27.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 8.42x (fastest 6100.6 ns, slowest 51337.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dru_s_both | 30449ns | 29837ns | 29645ns | 29808ns | 31814ns | +184.35% |
| dru_s_dedup | 53718ns | 53593ns | 52156ns | 53507ns | 54815ns | +401.65% |
| dru_s_plain | 10708ns | 10722ns | 10509ns | 10701ns | 10819ns | base |
| dru_s_reuse | 8456ns | 8366ns | 8319ns | 8358ns | 8673ns | -21.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dru_s_both | 28112ns | 27322ns | 29402ns | +231.51% | 0.009 |
| dru_s_dedup | 51383ns | 49968ns | 52268ns | +505.93% | 0.005 |
| dru_s_plain | 8480ns | 8308ns | 8605ns | base | 0.030 |
| dru_s_reuse | 6140ns | 6037ns | 6270ns | -27.59% | 0.042 |

## Performance model

- Peak throughput: **0.042 Gops/s** (dru_s_reuse; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dru_s_both | 0.009 | 21.9% |
| dru_s_dedup | 0.005 | 11.8% |
| dru_s_plain | 0.030 | 71.3% |
| dru_s_reuse | 0.042 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dru_s_both | 30449ns | 30449ns | +184.35% |
| dru_s_dedup | 53718ns | 53718ns | +401.65% |
| dru_s_plain | 10708ns | 10708ns | base |
| dru_s_reuse | 8456ns | 8456ns | -21.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dru_s_plain | 8461ns | base | --- | [8373, 8605] | --- | --- | --- | --- |
| dru_s_both | 27550ns | +19027.9ns (+224.9%) | [+18944, +20923]ns | [27382, 29402] | YES | 0.0313 | 0.0313 | 0 |
| dru_s_dedup | 51338ns | +42897.7ns (+507.0%) | [+41937, +43874]ns | [50542, 52268] | YES | 0.0313 | 0.0313 | 0 |
| dru_s_reuse | 6101ns | -2358.5ns (-27.9%) | [-2516, -2145]ns | [6050, 6270] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | dru_s_plain | dru_s_both | dru_s_dedup | dru_s_reuse |
|---|---|---|---|---|
| 1 | 8516ns | +261.2% | +486.7% | -29.1% |
| 2 | 8695ns | +217.7% | +487.9% | -29.4% |
| 3 | 8438ns | +225.7% | +509.8% | -27.7% |
| 4 | 8442ns | +232.1% | +506.7% | -24.2% |
| 5 | 8480ns | +223.6% | +513.6% | -28.0% |
| 6 | 8308ns | +228.9% | +531.9% | -27.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dru_s_both | -0.043 | ok |
| dru_s_dedup | 0.256 | moderate+ |
| dru_s_plain | 0.003 | ok |
| dru_s_reuse | -0.208 | moderate- |

**Consistency summary:**

- **dru_s_both**: won 0/6, lost 6/6
- **dru_s_dedup**: won 0/6, lost 6/6
- **dru_s_reuse**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dru_s_both | 13657.7ns | 28111.7ns | 48.6% | HIGH |
| dru_s_dedup | 14007.0ns | 51382.8ns | 27.3% | HIGH |
| dru_s_plain | 16919.0ns | 8480.0ns | 199.5% | HIGH |
| dru_s_reuse | 16595.3ns | 6140.1ns | 270.3% | HIGH |

## Distribution (algo ns)

```
dru_s_both (n=6, range 27322.5-29402.5 ns)
  27322.5 |####################
  27426.5 |########################################
  27530.5 |####################
  27634.5 |
  27738.5 |
  27842.5 |
  27946.5 |####################
  28050.5 |
  28154.5 |
  28258.5 |
  28362.5 |
  28466.5 |
  28570.5 |
  28674.5 |
  28778.5 |
  28882.5 |
  28986.5 |
  29090.5 |
  29194.5 |
  29298.5 |
  (0 below, 1 above range)

dru_s_dedup (n=6, range 49968.3-52267.9 ns)
  49968.3 |########################################
  50083.3 |
  50198.3 |
  50313.2 |
  50428.2 |
  50543.2 |
  50658.2 |
  50773.2 |
  50888.1 |
  51003.1 |########################################
  51118.1 |########################################
  51233.1 |
  51348.1 |########################################
  51463.0 |
  51578.0 |
  51693.0 |
  51808.0 |
  51923.0 |########################################
  52037.9 |
  52152.9 |
  (0 below, 1 above range)

dru_s_plain (n=6, range 8308.3-8605.4 ns)
   8308.3 |########################################
   8323.2 |
   8338.0 |
   8352.9 |
   8367.7 |
   8382.6 |
   8397.4 |
   8412.3 |
   8427.1 |########################################
   8442.0 |########################################
   8456.9 |
   8471.7 |########################################
   8486.6 |
   8501.4 |########################################
   8516.3 |
   8531.1 |
   8546.0 |
   8560.8 |
   8575.7 |
   8590.5 |
  (0 below, 1 above range)

dru_s_reuse (n=6, range 6037.1-6269.6 ns)
   6037.1 |####################
   6048.7 |
   6060.4 |####################
   6072.0 |
   6083.6 |
   6095.2 |########################################
   6106.9 |
   6118.5 |
   6130.1 |
   6141.7 |####################
   6153.4 |
   6165.0 |
   6176.6 |
   6188.2 |
   6199.9 |
   6211.5 |
   6223.1 |
   6234.7 |
   6246.4 |
   6258.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **dru_s_both**: bridge=49.5% of algo (FFI overhead may distort results)
- **dru_s_dedup**: bridge=28.0% of algo (FFI overhead may distort results)
- **dru_s_plain**: bridge=200.2% of algo (FFI overhead may distort results)
- **dru_s_reuse**: bridge=269.3% of algo (FFI overhead may distort results)
