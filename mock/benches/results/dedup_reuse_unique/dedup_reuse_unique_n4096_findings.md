# Dedup vs reuse composition, 20% shared: the templating norm

4 variants, 6 samples per variant.
Baseline: **dru_u_plain**

## Highlights

Baseline for all deltas below: **dru_u_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dru_u_reuse dominates: 170% faster than the next best (dru_u_plain)

dru_u_reuse (51.88 us) leads dru_u_plain (140.19 us) by 170%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dru_u_reuse beats baseline by 63% (significant)

dru_u_reuse is -88.69 us (63%) faster than baseline dru_u_plain, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### dru_u_dedup is an outlier: 18.8x slower than the field

dru_u_dedup (973.38 us) is 18.8x the fastest (51.88 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {dru_u_reuse, dru_u_plain, dru_u_both} vs {dru_u_dedup} (360% apart)

The field splits into a fast tier {dru_u_reuse, dru_u_plain, dru_u_both} and a slow tier {dru_u_dedup} with a 360% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 18.8x the fastest

Fastest dru_u_reuse (51.88 us) to slowest dru_u_dedup (973.38 us): 18.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dru_u_reuse** at 51881.2 ns median (-63.0% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 18.76x (fastest 51881.2 ns, slowest 973377.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dru_u_both | 214439ns | 214330ns | 211682ns | 213626ns | 217036ns | +50.21% |
| dru_u_dedup | 976179ns | 976432ns | 966909ns | 974561ns | 983242ns | +583.77% |
| dru_u_plain | 142763ns | 142690ns | 141414ns | 142301ns | 144132ns | base |
| dru_u_reuse | 54555ns | 54303ns | 53307ns | 54032ns | 55962ns | -61.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dru_u_both | 211929ns | 209119ns | 214571ns | +51.06% | 0.019 |
| dru_u_dedup | 973255ns | 964042ns | 980577ns | +593.74% | 0.004 |
| dru_u_plain | 140292ns | 138911ns | 141681ns | base | 0.029 |
| dru_u_reuse | 52093ns | 51043ns | 53270ns | -62.87% | 0.079 |

## Performance model

- Peak throughput: **0.080 Gops/s** (dru_u_reuse; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dru_u_both | 0.019 | 24.1% |
| dru_u_dedup | 0.004 | 5.2% |
| dru_u_plain | 0.029 | 36.4% |
| dru_u_reuse | 0.079 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dru_u_both | 214439ns | 214439ns | +50.21% |
| dru_u_dedup | 976179ns | 976179ns | +583.77% |
| dru_u_plain | 142763ns | 142763ns | base |
| dru_u_reuse | 54555ns | 54555ns | -61.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dru_u_plain | 140186ns | base | --- | [139009, 141681] | --- | --- | --- | --- |
| dru_u_both | 211783ns | +70993.9ns (+50.6%) | [+68591, +75325]ns | [209431, 214571] | YES | 0.0313 | 0.0313 | 0 |
| dru_u_dedup | 973377ns | +833526.6ns (+594.6%) | [+825492, +839871]ns | [965812, 980577] | YES | 0.0313 | 0.0313 | 0 |
| dru_u_reuse | 51881ns | -88691.2ns (-63.3%) | [-90110, -85795]ns | [51128, 53270] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | dru_u_plain | dru_u_both | dru_u_dedup | dru_u_reuse |
|---|---|---|---|---|
| 1 | 141533ns | +49.4% | +581.1% | -63.8% |
| 2 | 138911ns | +53.0% | +602.1% | -60.7% |
| 3 | 139107ns | +50.8% | +595.6% | -62.7% |
| 4 | 139581ns | +55.2% | +603.7% | -63.4% |
| 5 | 140790ns | +50.7% | +590.0% | -63.1% |
| 6 | 141830ns | +47.4% | +590.2% | -63.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dru_u_both | -0.330 | moderate- |
| dru_u_dedup | -0.453 | moderate- |
| dru_u_plain | 0.148 | ok |
| dru_u_reuse | -0.291 | moderate- |

**Consistency summary:**

- **dru_u_both**: won 0/6, lost 6/6
- **dru_u_dedup**: won 0/6, lost 6/6
- **dru_u_reuse**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dru_u_both | 14175.8ns | 211928.5ns | 6.7% | HIGH |
| dru_u_dedup | 14306.9ns | 973255.5ns | 1.5% |  |
| dru_u_plain | 16896.6ns | 140291.9ns | 12.0% | HIGH |
| dru_u_reuse | 16697.2ns | 52093.0ns | 32.1% | HIGH |

## Distribution (algo ns)

```
dru_u_both (n=6, range 209119.2-214571.2 ns)
  209119.2 |########################################
  209391.8 |
  209664.4 |########################################
  209937.0 |
  210209.6 |
  210482.2 |
  210754.8 |
  211027.4 |
  211300.0 |########################################
  211572.6 |
  211845.2 |
  212117.8 |########################################
  212390.4 |########################################
  212663.0 |
  212935.6 |
  213208.2 |
  213480.8 |
  213753.4 |
  214026.0 |
  214298.6 |
  (0 below, 1 above range)

dru_u_dedup (n=6, range 964042.5-980576.7 ns)
  964042.5 |########################################
  964869.2 |
  965695.9 |
  966522.6 |
  967349.3 |########################################
  968176.0 |
  969002.7 |
  969829.5 |
  970656.2 |########################################
  971482.9 |
  972309.6 |
  973136.3 |
  973963.0 |
  974789.7 |########################################
  975616.4 |
  976443.1 |
  977269.8 |
  978096.5 |
  978923.2 |########################################
  979749.9 |
  (0 below, 1 above range)

dru_u_plain (n=6, range 138911.2-141681.2 ns)
  138911.2 |########################################
  139049.7 |########################################
  139188.2 |
  139326.7 |
  139465.2 |########################################
  139603.7 |
  139742.2 |
  139880.7 |
  140019.2 |
  140157.7 |
  140296.2 |
  140434.7 |
  140573.2 |
  140711.7 |########################################
  140850.2 |
  140988.7 |
  141127.2 |
  141265.7 |
  141404.2 |########################################
  141542.7 |
  (0 below, 1 above range)

dru_u_reuse (n=6, range 51043.3-53269.8 ns)
  51043.3 |####################
  51154.6 |####################
  51266.0 |
  51377.3 |
  51488.6 |
  51599.9 |
  51711.2 |
  51822.6 |########################################
  51933.9 |####################
  52045.2 |
  52156.6 |
  52267.9 |
  52379.2 |
  52490.5 |
  52601.9 |
  52713.2 |
  52824.5 |
  52935.8 |
  53047.2 |
  53158.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **dru_u_plain**: bridge=12.0% of algo (FFI overhead may distort results)
- **dru_u_reuse**: bridge=32.0% of algo (FFI overhead may distort results)
