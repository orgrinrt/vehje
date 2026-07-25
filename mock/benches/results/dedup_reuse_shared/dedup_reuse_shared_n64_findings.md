# Dedup vs reuse composition, 60% shared: reuse's adversarial regime

4 variants, 6 samples per variant.
Baseline: **dru_s_plain**

## Highlights

Baseline for all deltas below: **dru_s_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dru_s_reuse dominates: 34% faster than the next best (dru_s_plain)

dru_s_reuse (1.11 us) leads dru_s_plain (1.48 us) by 34%, a clear separation rather than a photo finish. CV 5.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dru_s_reuse beats baseline by 25% (significant)

dru_s_reuse is -376 ns (25%) faster than baseline dru_s_plain, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### dru_s_dedup is an outlier: 7.8x slower than the field

dru_s_dedup (8.63 us) is 7.8x the fastest (1.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dru_s_reuse is fastest but the noisiest (CV 5.5%)

dru_s_reuse wins on median (1.11 us) yet has the highest variance (CV 5.5%), while dru_s_both is the steadiest (CV 1.1%, 4.51 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### dru_s_both shows alternating (throttle bounce) (autocorr -0.61)

dru_s_both's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {dru_s_reuse, dru_s_plain} vs {dru_s_both, dru_s_dedup} (204% apart)

The field splits into a fast tier {dru_s_reuse, dru_s_plain} and a slow tier {dru_s_both, dru_s_dedup} with a 204% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.8x the fastest

Fastest dru_s_reuse (1.11 us) to slowest dru_s_dedup (8.63 us): 7.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dru_s_reuse** at 1105.2 ns median (-25.5% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 7.80x (fastest 1105.2 ns, slowest 8625.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dru_s_both | 6765ns | 6766ns | 6662ns | 6740ns | 6853ns | +84.15% |
| dru_s_dedup | 10800ns | 10873ns | 10344ns | 10818ns | 11002ns | +194.00% |
| dru_s_plain | 3673ns | 3651ns | 3562ns | 3626ns | 3800ns | base |
| dru_s_reuse | 3332ns | 3276ns | 3193ns | 3252ns | 3522ns | -9.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dru_s_both | 4520ns | 4445ns | 4584ns | +204.08% | 0.014 |
| dru_s_dedup | 8599ns | 8179ns | 8835ns | +478.54% | 0.007 |
| dru_s_plain | 1486ns | 1430ns | 1537ns | base | 0.043 |
| dru_s_reuse | 1126ns | 1074ns | 1197ns | -24.26% | 0.057 |

## Performance model

- Peak throughput: **0.060 Gops/s** (dru_s_reuse; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dru_s_both | 0.014 | 23.8% |
| dru_s_dedup | 0.007 | 12.5% |
| dru_s_plain | 0.043 | 72.4% |
| dru_s_reuse | 0.058 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dru_s_both | 6765ns | 6765ns | +84.15% |
| dru_s_dedup | 10800ns | 10800ns | +194.00% |
| dru_s_plain | 3673ns | 3673ns | base |
| dru_s_reuse | 3332ns | 3332ns | -9.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dru_s_plain | 1484ns | base | --- | [1439, 1537] | --- | --- | --- | --- |
| dru_s_both | 4509ns | +3054.1ns (+205.8%) | [+2965, +3081]ns | [4466, 4584] | YES | 0.0313 | 0.0313 | 0 |
| dru_s_dedup | 8625ns | +7160.0ns (+482.5%) | [+6801, +7378]ns | [8338, 8835] | YES | 0.0313 | 0.0313 | 0 |
| dru_s_reuse | 1105ns | -376.0ns (-25.3%) | [-413, -293]ns | [1075, 1197] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | dru_s_plain | dru_s_both | dru_s_dedup | dru_s_reuse |
|---|---|---|---|---|
| 1 | 1447ns | +210.8% | +491.9% | -25.8% |
| 2 | 1485ns | +207.8% | +495.8% | -26.2% |
| 3 | 1483ns | +204.7% | +485.5% | -15.6% |
| 4 | 1552ns | +186.5% | +427.1% | -28.2% |
| 5 | 1522ns | +202.3% | +458.4% | -24.9% |
| 6 | 1430ns | +213.8% | +517.2% | -24.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dru_s_both | -0.605 | HIGH- (thermal bounce) |
| dru_s_dedup | -0.012 | ok |
| dru_s_plain | 0.018 | ok |
| dru_s_reuse | -0.210 | moderate- |

**Consistency summary:**

- **dru_s_both**: won 0/6, lost 6/6
- **dru_s_dedup**: won 0/6, lost 6/6
- **dru_s_reuse**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dru_s_both | 15195.1ns | 4519.9ns | 336.2% | HIGH |
| dru_s_dedup | 13734.0ns | 8599.4ns | 159.7% | HIGH |
| dru_s_plain | 16445.8ns | 1486.4ns | 1106.4% | HIGH |
| dru_s_reuse | 17107.8ns | 1125.8ns | 1519.7% | HIGH |

## Distribution (algo ns)

```
dru_s_both (n=6, range 4445.0-4584.4 ns)
   4445.0 |########################################
   4452.0 |
   4458.9 |
   4465.9 |
   4472.9 |
   4479.9 |
   4486.8 |########################################
   4493.8 |########################################
   4500.8 |
   4507.7 |
   4514.7 |########################################
   4521.7 |
   4528.6 |
   4535.6 |
   4542.6 |
   4549.5 |
   4556.5 |
   4563.5 |########################################
   4570.5 |
   4577.4 |
  (0 below, 1 above range)

dru_s_dedup (n=6, range 8178.7-8835.0 ns)
   8178.7 |########################################
   8211.5 |
   8244.3 |
   8277.1 |
   8310.0 |
   8342.8 |
   8375.6 |
   8408.4 |
   8441.2 |
   8474.0 |########################################
   8506.9 |
   8539.7 |########################################
   8572.5 |
   8605.3 |
   8638.1 |
   8670.9 |########################################
   8703.7 |
   8736.6 |
   8769.4 |
   8802.2 |########################################
  (0 below, 1 above range)

dru_s_plain (n=6, range 1430.0-1536.7 ns)
   1430.0 |########################################
   1435.3 |
   1440.7 |
   1446.0 |########################################
   1451.3 |
   1456.7 |
   1462.0 |
   1467.3 |
   1472.7 |
   1478.0 |########################################
   1483.3 |########################################
   1488.7 |
   1494.0 |
   1499.4 |
   1504.7 |
   1510.0 |
   1515.4 |
   1520.7 |########################################
   1526.0 |
   1531.4 |
  (0 below, 1 above range)

dru_s_reuse (n=6, range 1074.2-1197.1 ns)
   1074.2 |########################################
   1080.3 |
   1086.5 |
   1092.6 |####################
   1098.8 |
   1104.9 |
   1111.1 |####################
   1117.2 |
   1123.4 |
   1129.5 |
   1135.7 |
   1141.8 |####################
   1147.9 |
   1154.1 |
   1160.2 |
   1166.4 |
   1172.5 |
   1178.7 |
   1184.8 |
   1191.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **dru_s_both**: bridge=334.4% of algo (FFI overhead may distort results)
- **dru_s_dedup**: bridge=156.0% of algo (FFI overhead may distort results)
- **dru_s_plain**: bridge=1099.1% of algo (FFI overhead may distort results)
- **dru_s_reuse**: bridge=1570.0% of algo (FFI overhead may distort results)
