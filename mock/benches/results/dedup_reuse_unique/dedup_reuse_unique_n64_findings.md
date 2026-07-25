# Dedup vs reuse composition, 20% shared: the templating norm

4 variants, 6 samples per variant.
Baseline: **dru_u_plain**

## Highlights

Baseline for all deltas below: **dru_u_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dru_u_reuse dominates: 141% faster than the next best (dru_u_plain)

dru_u_reuse (636 ns) leads dru_u_plain (1.53 us) by 141%, a clear separation rather than a photo finish. CV 14.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dru_u_reuse beats baseline by 58% (significant)

dru_u_reuse is -896 ns (58%) faster than baseline dru_u_plain, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### dru_u_dedup is an outlier: 16.1x slower than the field

dru_u_dedup (10.23 us) is 16.1x the fastest (636 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dru_u_plain shows alternating (throttle bounce) (autocorr -0.56)

dru_u_plain's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {dru_u_reuse, dru_u_plain, dru_u_both} vs {dru_u_dedup} (422% apart)

The field splits into a fast tier {dru_u_reuse, dru_u_plain, dru_u_both} and a slow tier {dru_u_dedup} with a 422% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 16.1x the fastest

Fastest dru_u_reuse (636 ns) to slowest dru_u_dedup (10.23 us): 16.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dru_u_reuse** at 635.7 ns median (-58.5% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 16.10x (fastest 635.7 ns, slowest 10234.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dru_u_both | 4682ns | 4186ns | 4095ns | 4180ns | 5727ns | +22.73% |
| dru_u_dedup | 12742ns | 12720ns | 11130ns | 12303ns | 14205ns | +234.02% |
| dru_u_plain | 3815ns | 3775ns | 3650ns | 3766ns | 3970ns | base |
| dru_u_reuse | 3186ns | 2895ns | 2867ns | 2893ns | 3783ns | -16.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dru_u_both | 2192ns | 1884ns | 2722ns | +41.39% | 0.029 |
| dru_u_dedup | 10297ns | 8858ns | 11620ns | +564.28% | 0.006 |
| dru_u_plain | 1550ns | 1475ns | 1616ns | base | 0.041 |
| dru_u_reuse | 682ns | 601ns | 803ns | -55.99% | 0.094 |

## Performance model

- Peak throughput: **0.107 Gops/s** (dru_u_reuse; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dru_u_both | 0.033 | 30.6% |
| dru_u_dedup | 0.006 | 5.9% |
| dru_u_plain | 0.042 | 39.2% |
| dru_u_reuse | 0.101 | 94.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dru_u_both | 4682ns | 4682ns | +22.73% |
| dru_u_dedup | 12742ns | 12742ns | +234.02% |
| dru_u_plain | 3815ns | 3815ns | base |
| dru_u_reuse | 3186ns | 3186ns | -16.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dru_u_plain | 1533ns | base | --- | [1501, 1616] | --- | --- | --- | --- |
| dru_u_both | 1961ns | +428.1ns (+27.9%) | [+330, +1166]ns | [1892, 2722] | YES | 0.0313 | 0.0313 | 0 |
| dru_u_dedup | 10234ns | +8672.9ns (+565.7%) | [+7476, +10092]ns | [9038, 11620] | YES | 0.0313 | 0.0313 | 0 |
| dru_u_reuse | 636ns | -896.5ns (-58.5%) | [-940, -767]ns | [608, 803] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | dru_u_plain | dru_u_both | dru_u_dedup | dru_u_reuse |
|---|---|---|---|---|
| 1 | 1528ns | +103.0% | +718.7% | -43.2% |
| 2 | 1585ns | +47.9% | +459.0% | -61.2% |
| 3 | 1528ns | +28.1% | +602.5% | -57.8% |
| 4 | 1538ns | +27.8% | +499.2% | -59.2% |
| 5 | 1648ns | +15.3% | +533.3% | -55.2% |
| 6 | 1475ns | +27.7% | +580.3% | -59.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dru_u_both | 0.274 | moderate+ |
| dru_u_dedup | -0.532 | HIGH- (thermal bounce) |
| dru_u_plain | -0.558 | HIGH- (thermal bounce) |
| dru_u_reuse | -0.291 | moderate- |

**Consistency summary:**

- **dru_u_both**: won 0/6, lost 6/6
- **dru_u_dedup**: won 0/6, lost 6/6
- **dru_u_reuse**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dru_u_both | 17859.9ns | 2191.7ns | 814.9% | HIGH |
| dru_u_dedup | 18184.8ns | 10297.2ns | 176.6% | HIGH |
| dru_u_plain | 17509.7ns | 1550.1ns | 1129.6% | HIGH |
| dru_u_reuse | 19082.5ns | 682.1ns | 2797.4% | HIGH |

## Distribution (algo ns)

```
dru_u_both (n=6, range 1884.2-2722.2 ns)
   1884.2 |########################################
   1926.1 |########################################
   1968.0 |
   2009.9 |
   2051.8 |
   2093.7 |
   2135.6 |
   2177.5 |
   2219.4 |
   2261.3 |
   2303.2 |####################
   2345.1 |
   2387.0 |
   2428.9 |
   2470.8 |
   2512.7 |
   2554.6 |
   2596.5 |
   2638.4 |
   2680.3 |
  (0 below, 1 above range)

dru_u_dedup (n=6, range 8857.9-11619.6 ns)
   8857.9 |########################################
   8996.0 |
   9134.1 |########################################
   9272.2 |
   9410.2 |
   9548.3 |
   9686.4 |
   9824.5 |
   9962.6 |########################################
  10100.7 |
  10238.8 |
  10376.8 |########################################
  10514.9 |
  10653.0 |########################################
  10791.1 |
  10929.2 |
  11067.3 |
  11205.3 |
  11343.4 |
  11481.5 |
  (0 below, 1 above range)

dru_u_plain (n=6, range 1475.0-1616.0 ns)
   1475.0 |####################
   1482.1 |
   1489.1 |
   1496.2 |
   1503.2 |
   1510.3 |
   1517.3 |
   1524.4 |########################################
   1531.4 |####################
   1538.5 |
   1545.5 |
   1552.6 |
   1559.6 |
   1566.7 |
   1573.7 |
   1580.8 |####################
   1587.8 |
   1594.9 |
   1601.9 |
   1609.0 |
  (0 below, 1 above range)

dru_u_reuse (n=6, range 600.8-802.7 ns)
    600.8 |########################################
    610.9 |########################################
    621.0 |########################################
    631.1 |
    641.2 |########################################
    651.3 |
    661.4 |
    671.5 |
    681.6 |
    691.7 |
    701.8 |
    711.8 |
    721.9 |
    732.0 |########################################
    742.1 |
    752.2 |
    762.3 |
    772.4 |
    782.5 |
    792.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **dru_u_both**: bridge=848.1% of algo (FFI overhead may distort results)
- **dru_u_dedup**: bridge=166.1% of algo (FFI overhead may distort results)
- **dru_u_plain**: bridge=1161.6% of algo (FFI overhead may distort results)
- **dru_u_reuse**: bridge=2946.3% of algo (FFI overhead may distort results)
