# Dedup vs reuse composition, 60% shared: reuse's adversarial regime

4 variants, 6 samples per variant.
Baseline: **dru_s_plain**

## Highlights

Baseline for all deltas below: **dru_s_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dru_s_reuse dominates: 38% faster than the next best (dru_s_plain)

dru_s_reuse (101.02 us) leads dru_s_plain (139.13 us) by 38%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dru_s_reuse beats baseline by 27% (significant)

dru_s_reuse is -37.88 us (27%) faster than baseline dru_s_plain, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### dru_s_dedup is an outlier: 9.7x slower than the field

dru_s_dedup (977.74 us) is 9.7x the fastest (101.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dru_s_dedup shows alternating (throttle bounce) (autocorr -0.59)

dru_s_dedup's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {dru_s_reuse, dru_s_plain} vs {dru_s_both, dru_s_dedup} (327% apart)

The field splits into a fast tier {dru_s_reuse, dru_s_plain} and a slow tier {dru_s_both, dru_s_dedup} with a 327% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 9.7x the fastest

Fastest dru_s_reuse (101.02 us) to slowest dru_s_dedup (977.74 us): 9.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dru_s_reuse** at 101019.1 ns median (-27.4% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 9.68x (fastest 101019.1 ns, slowest 977737.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dru_s_both | 596749ns | 596870ns | 581612ns | 593969ns | 608487ns | +321.74% |
| dru_s_dedup | 981328ns | 980406ns | 970636ns | 979504ns | 989411ns | +593.54% |
| dru_s_plain | 141496ns | 141625ns | 137014ns | 141303ns | 144028ns | base |
| dru_s_reuse | 104188ns | 103359ns | 102104ns | 103016ns | 106988ns | -26.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dru_s_both | 593952ns | 578407ns | 605465ns | +327.40% | 0.007 |
| dru_s_dedup | 978252ns | 967515ns | 986277ns | +603.94% | 0.004 |
| dru_s_plain | 138968ns | 134434ns | 141396ns | base | 0.029 |
| dru_s_reuse | 101750ns | 99871ns | 104318ns | -26.78% | 0.040 |

## Performance model

- Peak throughput: **0.041 Gops/s** (dru_s_reuse; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dru_s_both | 0.007 | 16.8% |
| dru_s_dedup | 0.004 | 10.2% |
| dru_s_plain | 0.029 | 71.8% |
| dru_s_reuse | 0.041 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dru_s_both | 596749ns | 596749ns | +321.74% |
| dru_s_dedup | 981328ns | 981328ns | +593.54% |
| dru_s_plain | 141496ns | 141496ns | base |
| dru_s_reuse | 104188ns | 104188ns | -26.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dru_s_plain | 139134ns | base | --- | [136375, 141396] | --- | --- | --- | --- |
| dru_s_both | 594356ns | +453893.8ns (+326.2%) | [+444726, +466331]ns | [582035, 605465] | YES | 0.0313 | 0.0313 | 0 |
| dru_s_dedup | 977737ns | +840416.7ns (+604.0%) | [+830090, +847345]ns | [970742, 986277] | YES | 0.0313 | 0.0313 | 0 |
| dru_s_reuse | 101019ns | -37879.8ns (-27.2%) | [-41157, -32620]ns | [99912, 104318] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | dru_s_plain | dru_s_both | dru_s_dedup | dru_s_reuse |
|---|---|---|---|---|
| 1 | 138826ns | +331.7% | +606.0% | -28.1% |
| 2 | 134434ns | +330.3% | +624.5% | -23.9% |
| 3 | 140184ns | +317.8% | +595.8% | -28.7% |
| 4 | 139441ns | +338.6% | +611.7% | -23.8% |
| 5 | 142608ns | +319.3% | +578.4% | -29.5% |
| 6 | 138317ns | +327.1% | +608.5% | -26.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dru_s_both | -0.065 | ok |
| dru_s_dedup | -0.593 | HIGH- (thermal bounce) |
| dru_s_plain | -0.138 | ok |
| dru_s_reuse | -0.534 | HIGH- (thermal bounce) |

**Consistency summary:**

- **dru_s_both**: won 0/6, lost 6/6
- **dru_s_dedup**: won 0/6, lost 6/6
- **dru_s_reuse**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dru_s_both | 14713.8ns | 593951.8ns | 2.5% |  |
| dru_s_dedup | 14395.8ns | 978252.1ns | 1.5% |  |
| dru_s_plain | 16613.1ns | 138968.3ns | 12.0% | HIGH |
| dru_s_reuse | 16672.4ns | 101749.5ns | 16.4% | HIGH |

## Distribution (algo ns)

```
dru_s_both (n=6, range 578406.7-605464.8 ns)
  578406.7 |########################################
  579759.6 |
  581112.5 |
  582465.4 |
  583818.3 |
  585171.2 |########################################
  586524.1 |
  587877.0 |
  589229.9 |
  590582.8 |########################################
  591935.8 |
  593288.7 |
  594641.6 |
  595994.5 |
  597347.4 |########################################
  598700.3 |########################################
  600053.2 |
  601406.1 |
  602759.0 |
  604111.9 |
  (0 below, 1 above range)

dru_s_dedup (n=6, range 967515.4-986276.9 ns)
  967515.4 |####################
  968453.5 |
  969391.6 |
  970329.6 |
  971267.7 |
  972205.8 |
  973143.8 |####################
  974081.9 |
  975020.0 |####################
  975958.1 |
  976896.1 |
  977834.2 |
  978772.3 |
  979710.4 |########################################
  980648.4 |
  981586.5 |
  982524.6 |
  983462.7 |
  984400.7 |
  985338.8 |
  (0 below, 1 above range)

dru_s_plain (n=6, range 134434.2-141395.9 ns)
  134434.2 |########################################
  134782.3 |
  135130.4 |
  135478.4 |
  135826.5 |
  136174.6 |
  136522.7 |
  136870.8 |
  137218.9 |
  137566.9 |
  137915.0 |
  138263.1 |########################################
  138611.2 |########################################
  138959.3 |
  139307.4 |########################################
  139655.4 |
  140003.5 |########################################
  140351.6 |
  140699.7 |
  141047.8 |
  (0 below, 1 above range)

dru_s_reuse (n=6, range 99871.2-104317.5 ns)
  99871.2 |########################################
  100093.5 |
  100315.8 |####################
  100538.1 |
  100760.5 |
  100982.8 |
  101205.1 |
  101427.4 |####################
  101649.7 |
  101872.0 |
  102094.4 |
  102316.7 |####################
  102539.0 |
  102761.3 |
  102983.6 |
  103205.9 |
  103428.2 |
  103650.6 |
  103872.9 |
  104095.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **dru_s_plain**: bridge=11.9% of algo (FFI overhead may distort results)
- **dru_s_reuse**: bridge=16.4% of algo (FFI overhead may distort results)
