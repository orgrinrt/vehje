# Project field access, polymorphic site: inline-cache hazard vs hash vs linear

3 variants, 6 samples per variant.
Baseline: **project_poly_hash**

## Highlights

Baseline for all deltas below: **project_poly_hash**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (project_poly_hash) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline project_poly_hash has the worst median (124.35 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest project_poly_ic at 117.93 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (project_poly_ic, project_poly_linear) are a dead heat (<1%)

project_poly_ic (117.93 us) and project_poly_linear (118.66 us) differ by 0.62%, inside the noise, even though the wider field spreads 5.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: project_poly_ic** at 117927.7 ns median (-5.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.05x (fastest 117927.7 ns, slowest 124348.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| project_poly_hash | 128131ns | 126704ns | 124280ns | 126077ns | 133136ns | base |
| project_poly_ic | 120975ns | 120378ns | 116400ns | 120099ns | 124576ns | -5.58% |
| project_poly_linear | 119796ns | 121107ns | 111369ns | 117898ns | 126858ns | -6.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| project_poly_hash | 125637ns | 121840ns | 130498ns | base | 0.130 |
| project_poly_ic | 118535ns | 114136ns | 122121ns | -5.65% | 0.138 |
| project_poly_linear | 117362ns | 109055ns | 124299ns | -6.59% | 0.140 |

## Performance model

- Peak throughput: **0.150 Gops/s** (project_poly_linear; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| project_poly_hash | 0.132 | 87.7% |
| project_poly_ic | 0.139 | 92.5% |
| project_poly_linear | 0.138 | 91.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| project_poly_hash | 128131ns | 128131ns | base |
| project_poly_ic | 120975ns | 120975ns | -5.58% |
| project_poly_linear | 119796ns | 119796ns | -6.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| project_poly_hash | 124349ns | base | --- | [122064, 130498] | --- | --- | --- | --- |
| project_poly_ic | 117928ns | -6878.2ns (-5.5%) | [-13488, -940]ns | [115556, 122121] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| project_poly_linear | 118661ns | no significant difference | [-15500, +204]ns | [109125, 124299] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | project_poly_hash | project_poly_ic | project_poly_linear |
|---|---|---|---|
| 1 | 126839ns | -6.2% | -14.0% |
| 2 | 121840ns | -4.0% | -1.9% |
| 3 | 134157ns | -12.0% | -9.9% |
| 4 | 124954ns | -8.7% | +2.2% |
| 5 | 122288ns | +2.4% | -10.7% |
| 6 | 123744ns | -4.8% | -4.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| project_poly_hash | -0.329 | moderate- |
| project_poly_ic | -0.465 | moderate- |
| project_poly_linear | -0.235 | moderate- |

**Consistency summary:**

- **project_poly_ic**: won 5/6, lost 1/6
- **project_poly_linear**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| project_poly_hash | 3.3ns | 125637.0ns | 0.0% |  |
| project_poly_ic | 4.4ns | 118534.8ns | 0.0% |  |
| project_poly_linear | 5.5ns | 117361.9ns | 0.0% |  |

## Distribution (algo ns)

```
project_poly_hash (n=6, range 121840.0-130498.1 ns)
  121840.0 |########################################
  122272.9 |########################################
  122705.8 |
  123138.7 |
  123571.6 |########################################
  124004.5 |
  124437.4 |
  124870.4 |########################################
  125303.3 |
  125736.2 |
  126169.1 |
  126602.0 |########################################
  127034.9 |
  127467.8 |
  127900.7 |
  128333.6 |
  128766.5 |
  129199.4 |
  129632.3 |
  130065.2 |
  (0 below, 1 above range)

project_poly_ic (n=6, range 114135.8-122120.6 ns)
  114135.8 |####################
  114535.0 |
  114934.3 |
  115333.5 |
  115732.8 |
  116132.0 |
  116531.2 |
  116930.5 |####################
  117329.7 |
  117729.0 |########################################
  118128.2 |
  118527.4 |
  118926.7 |####################
  119325.9 |
  119725.2 |
  120124.4 |
  120523.6 |
  120922.9 |
  121322.1 |
  121721.4 |
  (0 below, 1 above range)

project_poly_linear (n=6, range 109055.0-124299.4 ns)
  109055.0 |########################################
  109817.2 |
  110579.4 |
  111341.7 |
  112103.9 |
  112866.1 |
  113628.3 |
  114390.5 |
  115152.7 |
  115915.0 |
  116677.2 |
  117439.4 |####################
  118201.6 |
  118963.8 |####################
  119726.0 |
  120488.3 |####################
  121250.5 |
  122012.7 |
  122774.9 |
  123537.1 |
  (0 below, 1 above range)

```
