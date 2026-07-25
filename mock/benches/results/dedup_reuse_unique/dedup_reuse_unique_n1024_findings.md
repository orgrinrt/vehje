# Dedup vs reuse composition, 20% shared: the templating norm

4 variants, 6 samples per variant.
Baseline: **dru_u_plain**

## Highlights

Baseline for all deltas below: **dru_u_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dru_u_reuse dominates: 171% faster than the next best (dru_u_plain)

dru_u_reuse (12.64 us) leads dru_u_plain (34.29 us) by 171%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dru_u_reuse beats baseline by 63% (significant)

dru_u_reuse is -21.58 us (63%) faster than baseline dru_u_plain, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### dru_u_dedup is an outlier: 18.8x slower than the field

dru_u_dedup (237.21 us) is 18.8x the fastest (12.64 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {dru_u_reuse, dru_u_plain, dru_u_both} vs {dru_u_dedup} (375% apart)

The field splits into a fast tier {dru_u_reuse, dru_u_plain, dru_u_both} and a slow tier {dru_u_dedup} with a 375% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 18.8x the fastest

Fastest dru_u_reuse (12.64 us) to slowest dru_u_dedup (237.21 us): 18.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dru_u_reuse** at 12635.6 ns median (-63.1% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 18.77x (fastest 12635.6 ns, slowest 237210.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dru_u_both | 53402ns | 52334ns | 51003ns | 52155ns | 56472ns | +44.96% |
| dru_u_dedup | 238785ns | 239580ns | 235372ns | 238676ns | 240656ns | +548.18% |
| dru_u_plain | 36840ns | 36573ns | 35915ns | 36527ns | 37770ns | base |
| dru_u_reuse | 15137ns | 14847ns | 14522ns | 14844ns | 15883ns | -58.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dru_u_both | 51024ns | 48781ns | 53924ns | +47.76% | 0.020 |
| dru_u_dedup | 236349ns | 232911ns | 238218ns | +584.43% | 0.004 |
| dru_u_plain | 34532ns | 33605ns | 35432ns | base | 0.030 |
| dru_u_reuse | 12876ns | 12342ns | 13507ns | -62.71% | 0.080 |

## Performance model

- Peak throughput: **0.083 Gops/s** (dru_u_reuse; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dru_u_both | 0.020 | 24.7% |
| dru_u_dedup | 0.004 | 5.2% |
| dru_u_plain | 0.030 | 36.0% |
| dru_u_reuse | 0.081 | 97.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dru_u_both | 53402ns | 53402ns | +44.96% |
| dru_u_dedup | 238785ns | 238785ns | +548.18% |
| dru_u_plain | 36840ns | 36840ns | base |
| dru_u_reuse | 15137ns | 15137ns | -58.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dru_u_plain | 34288ns | base | --- | [33876, 35432] | --- | --- | --- | --- |
| dru_u_both | 49983ns | +15765.2ns (+46.0%) | [+14675, +19036]ns | [49166, 53924] | YES | 0.0313 | 0.0313 | 0 |
| dru_u_dedup | 237211ns | +202585.2ns (+590.8%) | [+199401, +203465]ns | [233619, 238218] | YES | 0.0313 | 0.0313 | 0 |
| dru_u_reuse | 12636ns | -21583.1ns (-62.9%) | [-22154, -21233]ns | [12484, 13507] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | dru_u_plain | dru_u_both | dru_u_dedup | dru_u_reuse |
|---|---|---|---|---|
| 1 | 34288ns | +42.3% | +579.3% | -61.8% |
| 2 | 36170ns | +52.2% | +558.6% | -61.5% |
| 3 | 34148ns | +47.6% | +586.2% | -63.0% |
| 4 | 34694ns | +42.8% | +586.6% | -63.6% |
| 5 | 34288ns | +44.5% | +593.3% | -63.2% |
| 6 | 33605ns | +57.1% | +604.4% | -63.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dru_u_both | -0.380 | moderate- |
| dru_u_dedup | -0.443 | moderate- |
| dru_u_plain | -0.236 | moderate- |
| dru_u_reuse | 0.141 | ok |

**Consistency summary:**

- **dru_u_both**: won 0/6, lost 6/6
- **dru_u_dedup**: won 0/6, lost 6/6
- **dru_u_reuse**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dru_u_both | 14272.7ns | 51024.2ns | 28.0% | HIGH |
| dru_u_dedup | 14216.6ns | 236349.3ns | 6.0% | HIGH |
| dru_u_plain | 16608.5ns | 34532.3ns | 48.1% | HIGH |
| dru_u_reuse | 16787.8ns | 12875.6ns | 130.4% | HIGH |

## Distribution (algo ns)

```
dru_u_both (n=6, range 48780.8-53923.6 ns)
  48780.8 |########################################
  49037.9 |
  49295.1 |########################################
  49552.2 |########################################
  49809.4 |
  50066.5 |
  50323.6 |########################################
  50580.8 |
  50837.9 |
  51095.0 |
  51352.2 |
  51609.3 |
  51866.5 |
  52123.6 |
  52380.7 |
  52637.9 |########################################
  52895.0 |
  53152.1 |
  53409.3 |
  53666.4 |
  (0 below, 1 above range)

dru_u_dedup (n=6, range 232910.8-238218.4 ns)
  232910.8 |########################################
  233176.2 |
  233441.6 |
  233706.9 |
  233972.3 |
  234237.7 |########################################
  234503.1 |
  234768.4 |
  235033.8 |
  235299.2 |
  235564.6 |
  235830.0 |
  236095.3 |
  236360.7 |
  236626.1 |########################################
  236891.5 |
  237156.8 |
  237422.2 |
  237687.6 |########################################
  237953.0 |########################################
  (0 below, 1 above range)

dru_u_plain (n=6, range 33605.0-35432.1 ns)
  33605.0 |####################
  33696.4 |
  33787.7 |
  33879.1 |
  33970.4 |
  34061.8 |####################
  34153.1 |
  34244.5 |########################################
  34335.8 |
  34427.2 |
  34518.6 |
  34609.9 |####################
  34701.3 |
  34792.6 |
  34884.0 |
  34975.3 |
  35066.7 |
  35158.0 |
  35249.4 |
  35340.7 |
  (0 below, 1 above range)

dru_u_reuse (n=6, range 12341.7-13506.6 ns)
  12341.7 |####################
  12399.9 |
  12458.2 |
  12516.4 |
  12574.7 |########################################
  12632.9 |####################
  12691.2 |
  12749.4 |
  12807.7 |
  12865.9 |
  12924.2 |
  12982.4 |
  13040.7 |####################
  13098.9 |
  13157.2 |
  13215.4 |
  13273.7 |
  13331.9 |
  13390.2 |
  13448.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **dru_u_both**: bridge=28.8% of algo (FFI overhead may distort results)
- **dru_u_plain**: bridge=48.2% of algo (FFI overhead may distort results)
- **dru_u_reuse**: bridge=132.1% of algo (FFI overhead may distort results)
