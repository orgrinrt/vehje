# Project field access, monomorphic site: direct offset vs inline cache vs hash vs linear

4 variants, 6 samples per variant.
Baseline: **project_mono_direct**

## Highlights

Baseline for all deltas below: **project_mono_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### project_mono_direct dominates: 25% faster than the next best (project_mono_ic)

project_mono_direct (2.87 us) leads project_mono_ic (3.59 us) by 25%, a clear separation rather than a photo finish. CV 7.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### project_mono_linear is an outlier: 3.9x slower than the field

project_mono_linear (11.09 us) is 3.9x the fastest (2.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### project_mono_direct is fastest but the noisiest (CV 7.9%)

project_mono_direct wins on median (2.87 us) yet has the highest variance (CV 7.9%), while project_mono_ic is the steadiest (CV 7.0%, 3.59 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (project_mono_direct)

The baseline project_mono_direct is the fastest (2.87 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {project_mono_direct, project_mono_ic} vs {project_mono_hash, project_mono_linear} (162% apart)

The field splits into a fast tier {project_mono_direct, project_mono_ic} and a slow tier {project_mono_hash, project_mono_linear} with a 162% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.9x the fastest

Fastest project_mono_direct (2.87 us) to slowest project_mono_linear (11.09 us): 3.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (project_mono_direct) is the fastest** at 2866.2 ns median
- 3 variants significantly slower than baseline
- Spread: 3.87x (fastest 2866.2 ns, slowest 11093.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| project_mono_direct | 5315ns | 5110ns | 4931ns | 5061ns | 5886ns | base |
| project_mono_hash | 11424ns | 11946ns | 10173ns | 11374ns | 12124ns | +114.96% |
| project_mono_ic | 5930ns | 6118ns | 5324ns | 5871ns | 6321ns | +11.57% |
| project_mono_linear | 12983ns | 13711ns | 11498ns | 12976ns | 13736ns | +144.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| project_mono_direct | 2976ns | 2757ns | 3291ns | base | 1.377 |
| project_mono_hash | 8975ns | 7985ns | 9519ns | +201.62% | 0.456 |
| project_mono_ic | 3487ns | 3140ns | 3715ns | +17.17% | 1.175 |
| project_mono_linear | 10506ns | 9306ns | 11118ns | +253.08% | 0.390 |

## Performance model

- Peak throughput: **1.486 Gops/s** (project_mono_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| project_mono_direct | 1.429 | 96.2% |
| project_mono_hash | 0.436 | 29.3% |
| project_mono_ic | 1.140 | 76.7% |
| project_mono_linear | 0.369 | 24.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| project_mono_direct | 5315ns | 5315ns | base |
| project_mono_hash | 11424ns | 11424ns | +114.96% |
| project_mono_ic | 5930ns | 5930ns | +11.57% |
| project_mono_linear | 12983ns | 12983ns | +144.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| project_mono_direct | 2866ns | base | --- | [2769, 3291] | --- | --- | --- | --- |
| project_mono_hash | 9403ns | +6225.4ns (+217.2%) | [+5137, +6636]ns | [8004, 9519] | YES | 0.0313 | 0.0313 | 0 |
| project_mono_ic | 3593ns | +424.0ns (+14.8%) | [+285, +824]ns | [3151, 3715] | YES | 0.0313 | 0.0313 | 0 |
| project_mono_linear | 11093ns | +7804.6ns (+272.3%) | [+6441, +8346]ns | [9308, 11118] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | project_mono_direct | project_mono_hash | project_mono_ic | project_mono_linear |
|---|---|---|---|---|
| 1 | 2757ns | +237.0% | +34.6% | +302.4% |
| 2 | 3292ns | +189.2% | +12.9% | +237.0% |
| 3 | 3291ns | +189.1% | +12.9% | +237.2% |
| 4 | 2866ns | +178.6% | +10.3% | +224.7% |
| 5 | 2866ns | +179.9% | +9.6% | +224.8% |
| 6 | 2782ns | +242.2% | +25.0% | +300.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| project_mono_direct | 0.094 | ok |
| project_mono_hash | 0.123 | ok |
| project_mono_ic | 0.381 | moderate+ |
| project_mono_linear | 0.154 | ok |

**Consistency summary:**

- **project_mono_hash**: won 0/6, lost 6/6
- **project_mono_ic**: won 0/6, lost 6/6
- **project_mono_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| project_mono_direct | 2.4ns | 2975.6ns | 0.1% |  |
| project_mono_hash | 3.5ns | 8975.2ns | 0.0% |  |
| project_mono_ic | 3.2ns | 3486.5ns | 0.1% |  |
| project_mono_linear | 3.2ns | 10506.2ns | 0.0% |  |

## Distribution (algo ns)

```
project_mono_direct (n=6, range 2756.7-3291.2 ns)
   2756.7 |########################################
   2783.4 |
   2810.2 |
   2836.9 |
   2863.6 |########################################
   2890.3 |
   2917.1 |
   2943.8 |
   2970.5 |
   2997.2 |
   3024.0 |
   3050.7 |
   3077.4 |
   3104.2 |
   3130.9 |
   3157.6 |
   3184.3 |
   3211.1 |
   3237.8 |
   3264.5 |####################
  (0 below, 1 above range)

project_mono_hash (n=6, range 7984.6-9519.1 ns)
   7984.6 |########################################
   8061.3 |
   8138.1 |
   8214.8 |
   8291.5 |
   8368.2 |
   8445.0 |
   8521.7 |
   8598.4 |
   8675.1 |
   8751.9 |
   8828.6 |
   8905.3 |
   8982.1 |
   9058.8 |
   9135.5 |
   9212.2 |
   9289.0 |####################
   9365.7 |
   9442.4 |########################################
  (0 below, 1 above range)

project_mono_ic (n=6, range 3140.4-3715.2 ns)
   3140.4 |########################################
   3169.1 |
   3197.9 |
   3226.6 |
   3255.4 |
   3284.1 |
   3312.8 |
   3341.6 |
   3370.3 |
   3399.1 |
   3427.8 |
   3456.5 |####################
   3485.3 |
   3514.0 |
   3542.8 |
   3571.5 |
   3600.2 |
   3629.0 |
   3657.7 |
   3686.5 |########################################
  (0 below, 1 above range)

project_mono_linear (n=6, range 9305.8-11117.9 ns)
   9305.8 |##########################
   9396.4 |
   9487.0 |
   9577.6 |
   9668.2 |
   9758.8 |
   9849.4 |
   9940.0 |
  10030.6 |
  10121.2 |
  10211.8 |
  10302.5 |
  10393.1 |
  10483.7 |
  10574.3 |
  10664.9 |
  10755.5 |
  10846.1 |
  10936.7 |
  11027.3 |########################################
  (0 below, 1 above range)

```
