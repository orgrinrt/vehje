# Dedup vs reuse composition, 60% shared: reuse's adversarial regime

4 variants, 6 samples per variant.
Baseline: **dru_s_plain**

## Highlights

Baseline for all deltas below: **dru_s_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dru_s_reuse dominates: 40% faster than the next best (dru_s_plain)

dru_s_reuse (24.78 us) leads dru_s_plain (34.63 us) by 40%, a clear separation rather than a photo finish. CV 4.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dru_s_reuse beats baseline by 28% (significant)

dru_s_reuse is -9.70 us (28%) faster than baseline dru_s_plain, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### dru_s_dedup is an outlier: 9.5x slower than the field

dru_s_dedup (234.98 us) is 9.5x the fastest (24.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dru_s_both shows alternating (throttle bounce) (autocorr -0.60)

dru_s_both's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {dru_s_reuse, dru_s_plain} vs {dru_s_both, dru_s_dedup} (312% apart)

The field splits into a fast tier {dru_s_reuse, dru_s_plain} and a slow tier {dru_s_both, dru_s_dedup} with a 312% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 9.5x the fastest

Fastest dru_s_reuse (24.78 us) to slowest dru_s_dedup (234.98 us): 9.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dru_s_reuse** at 24777.1 ns median (-28.5% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 9.48x (fastest 24777.1 ns, slowest 234978.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dru_s_both | 146851ns | 145234ns | 144400ns | 144992ns | 150864ns | +300.19% |
| dru_s_dedup | 236590ns | 237370ns | 232027ns | 236468ns | 239053ns | +544.74% |
| dru_s_plain | 36695ns | 36957ns | 35658ns | 36683ns | 37232ns | base |
| dru_s_reuse | 27488ns | 27071ns | 26526ns | 26966ns | 28753ns | -25.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dru_s_both | 144371ns | 142011ns | 148263ns | +319.72% | 0.007 |
| dru_s_dedup | 233998ns | 229472ns | 236277ns | +580.29% | 0.004 |
| dru_s_plain | 34397ns | 33400ns | 34900ns | base | 0.030 |
| dru_s_reuse | 25171ns | 24248ns | 26356ns | -26.82% | 0.041 |

## Performance model

- Peak throughput: **0.042 Gops/s** (dru_s_reuse; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dru_s_both | 0.007 | 17.0% |
| dru_s_dedup | 0.004 | 10.3% |
| dru_s_plain | 0.030 | 70.0% |
| dru_s_reuse | 0.041 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dru_s_both | 146851ns | 146851ns | +300.19% |
| dru_s_dedup | 236590ns | 236590ns | +544.74% |
| dru_s_plain | 36695ns | 36695ns | base |
| dru_s_reuse | 27488ns | 27488ns | -25.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dru_s_plain | 34634ns | base | --- | [33656, 34900] | --- | --- | --- | --- |
| dru_s_both | 142740ns | +108812.7ns (+314.2%) | [+107641, +113470]ns | [142110, 148263] | YES | 0.0313 | 0.0313 | 0 |
| dru_s_dedup | 234979ns | +200445.4ns (+578.8%) | [+196712, +201646]ns | [230738, 236277] | YES | 0.0313 | 0.0313 | 0 |
| dru_s_reuse | 24777ns | -9697.1ns (-28.0%) | [-9963, -8017]ns | [24379, 26356] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | dru_s_plain | dru_s_both | dru_s_dedup | dru_s_reuse |
|---|---|---|---|---|
| 1 | 33400ns | +327.9% | +587.0% | -26.6% |
| 2 | 34617ns | +311.8% | +577.4% | -28.5% |
| 3 | 34937ns | +318.8% | +575.3% | -27.8% |
| 4 | 33912ns | +318.8% | +594.3% | -28.5% |
| 5 | 34650ns | +333.5% | +569.6% | -20.6% |
| 6 | 34863ns | +307.9% | +578.7% | -28.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dru_s_both | -0.595 | HIGH- (thermal bounce) |
| dru_s_dedup | -0.177 | ok |
| dru_s_plain | -0.198 | ok |
| dru_s_reuse | -0.400 | moderate- |

**Consistency summary:**

- **dru_s_both**: won 0/6, lost 6/6
- **dru_s_dedup**: won 0/6, lost 6/6
- **dru_s_reuse**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dru_s_both | 14320.5ns | 144371.1ns | 9.9% | HIGH |
| dru_s_dedup | 14054.9ns | 233997.9ns | 6.0% | HIGH |
| dru_s_plain | 16635.7ns | 34396.7ns | 48.4% | HIGH |
| dru_s_reuse | 16345.5ns | 25170.8ns | 64.9% | HIGH |

## Distribution (algo ns)

```
dru_s_both (n=6, range 142011.2-148263.1 ns)
  142011.2 |########################################
  142323.8 |####################
  142636.4 |####################
  142949.0 |
  143261.6 |
  143574.2 |
  143886.8 |
  144199.4 |
  144512.0 |
  144824.6 |
  145137.2 |
  145449.8 |
  145762.4 |
  146075.0 |####################
  146387.6 |
  146700.2 |
  147012.8 |
  147325.4 |
  147638.0 |
  147950.6 |
  (0 below, 1 above range)

dru_s_dedup (n=6, range 229472.1-236277.3 ns)
  229472.1 |########################################
  229812.4 |
  230152.6 |
  230492.9 |
  230833.1 |
  231173.4 |
  231513.7 |
  231853.9 |########################################
  232194.2 |
  232534.4 |
  232874.7 |
  233215.0 |
  233555.2 |
  233895.5 |
  234235.7 |########################################
  234576.0 |
  234916.3 |
  235256.5 |########################################
  235596.8 |########################################
  235937.0 |
  (0 below, 1 above range)

dru_s_plain (n=6, range 33400.4-34900.0 ns)
  33400.4 |####################
  33475.4 |
  33550.4 |
  33625.3 |
  33700.3 |
  33775.3 |
  33850.3 |####################
  33925.3 |
  34000.2 |
  34075.2 |
  34150.2 |
  34225.2 |
  34300.2 |
  34375.1 |
  34450.1 |
  34525.1 |
  34600.1 |########################################
  34675.1 |
  34750.0 |
  34825.0 |####################
  (0 below, 1 above range)

dru_s_reuse (n=6, range 24247.5-26356.0 ns)
  24247.5 |########################################
  24352.9 |
  24458.4 |########################################
  24563.8 |
  24669.2 |########################################
  24774.6 |########################################
  24880.1 |
  24985.5 |
  25090.9 |
  25196.3 |########################################
  25301.8 |
  25407.2 |
  25512.6 |
  25618.1 |
  25723.5 |
  25828.9 |
  25934.3 |
  26039.8 |
  26145.2 |
  26250.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **dru_s_both**: bridge=10.2% of algo (FFI overhead may distort results)
- **dru_s_plain**: bridge=47.9% of algo (FFI overhead may distort results)
- **dru_s_reuse**: bridge=66.1% of algo (FFI overhead may distort results)
