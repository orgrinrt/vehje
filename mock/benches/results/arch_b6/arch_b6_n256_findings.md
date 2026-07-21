# Per-branch strategy: archetype 6 (ifchain4_blocks), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b6_table**

## Highlights

Baseline for all deltas below: **ab_b6_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b6_tree, ab_b6_prof) are a dead heat (<1%)

ab_b6_tree (83.90 us) and ab_b6_prof (84.07 us) differ by 0.20%, inside the noise, even though the wider field spreads 84.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b6_tree shows alternating (throttle bounce) (autocorr -0.76)

ab_b6_tree's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {ab_b6_tree, ab_b6_prof, ab_b6_seq, ab_b6_table} vs {ab_b6_pred} (81% apart)

The field splits into a fast tier {ab_b6_tree, ab_b6_prof, ab_b6_seq, ab_b6_table} and a slow tier {ab_b6_pred} with a 81% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader ab_b6_tree vs stability leader ab_b6_seq (+0% speed for 2.2x steadier)

ab_b6_tree is fastest (83.90 us, CV 5.7%); ab_b6_seq gives up 0.3% median for 2.2x lower variance (CV 2.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: ab_b6_tree** at 83902.9 ns median (-2.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.84x (fastest 83902.9 ns, slowest 154622.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b6_pred | 159264ns | 156906ns | 153111ns | 155972ns | 167278ns | +80.04% |
| ab_b6_prof | 89093ns | 86362ns | 83778ns | 86135ns | 96188ns | +0.72% |
| ab_b6_seq | 87127ns | 86521ns | 83565ns | 86381ns | 90026ns | -1.51% |
| ab_b6_table | 88460ns | 88042ns | 81877ns | 86401ns | 94841ns | base |
| ab_b6_tree | 87188ns | 86252ns | 81743ns | 85031ns | 93147ns | -1.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b6_pred | 156819ns | 150742ns | 164612ns | +82.27% | 0.002 |
| ab_b6_prof | 86738ns | 81489ns | 93679ns | +0.81% | 0.003 |
| ab_b6_seq | 84812ns | 81408ns | 87665ns | -1.43% | 0.003 |
| ab_b6_table | 86039ns | 79699ns | 92226ns | base | 0.003 |
| ab_b6_tree | 84810ns | 79541ns | 90592ns | -1.43% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b6_tree; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b6_pred | 0.002 | 51.4% |
| ab_b6_prof | 0.003 | 94.6% |
| ab_b6_seq | 0.003 | 94.5% |
| ab_b6_table | 0.003 | 92.9% |
| ab_b6_tree | 0.003 | 94.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b6_pred | 159264ns | 159264ns | +80.04% |
| ab_b6_prof | 89093ns | 89093ns | +0.72% |
| ab_b6_seq | 87127ns | 87127ns | -1.51% |
| ab_b6_table | 88460ns | 88460ns | base |
| ab_b6_tree | 87188ns | 87188ns | -1.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b6_table | 85616ns | base | --- | [80274, 92226] | --- | --- | --- | --- |
| ab_b6_pred | 154622ns | +72278.6ns (+84.4%) | [+65919, +74143]ns | [151223, 164612] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b6_prof | 84072ns | no significant difference | [-6412, +7000]ns | [82463, 93679] | no | 0.9167 | 0.6875 | 0 |
| ab_b6_seq | 84184ns | no significant difference | [-4562, +2343]ns | [82589, 87665] | no | 1.0000 | 1.0000 | 0 |
| ab_b6_tree | 83903ns | -1360.4ns (-1.6%) | [-2165, -160]ns | [79936, 90592] | YES (adj: no) | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b6_table | ab_b6_pred | ab_b6_prof | ab_b6_seq | ab_b6_tree |
|---|---|---|---|---|---|
| 1 | 79699ns | +90.3% | +2.2% | +2.1% | +1.1% |
| 2 | 88426ns | +84.3% | -5.6% | -4.4% | -1.3% |
| 3 | 82805ns | +82.0% | +1.5% | +1.2% | -3.0% |
| 4 | 91942ns | +69.5% | -8.5% | -4.6% | -2.0% |
| 5 | 80849ns | +89.7% | +14.7% | +3.7% | -1.6% |
| 6 | 92511ns | +79.7% | +2.3% | -5.3% | -1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b6_pred | -0.452 | moderate- |
| ab_b6_prof | 0.430 | moderate+ |
| ab_b6_seq | -0.248 | moderate- |
| ab_b6_table | -0.663 | HIGH- (thermal bounce) |
| ab_b6_tree | -0.760 | HIGH- (thermal bounce) |

**Consistency summary:**

- **ab_b6_pred**: won 0/6, lost 6/6
- **ab_b6_prof**: won 2/6, lost 4/6
- **ab_b6_seq**: won 3/6, lost 3/6
- **ab_b6_tree**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b6_pred | 5.2ns | 156819.2ns | 0.0% |  |
| ab_b6_prof | 5.6ns | 86738.3ns | 0.0% |  |
| ab_b6_seq | 4.2ns | 84812.3ns | 0.0% |  |
| ab_b6_table | 5.1ns | 86038.7ns | 0.0% |  |
| ab_b6_tree | 4.9ns | 84810.4ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b6_pred (n=6, range 150742.1-164611.9 ns)
  150742.1 |########################################
  151435.6 |########################################
  152129.1 |
  152822.6 |########################################
  153516.1 |
  154209.5 |
  154903.0 |
  155596.5 |########################################
  156290.0 |
  156983.5 |
  157677.0 |
  158370.5 |
  159064.0 |
  159757.4 |
  160450.9 |
  161144.4 |
  161837.9 |
  162531.4 |########################################
  163224.9 |
  163918.4 |
  (0 below, 1 above range)

ab_b6_prof (n=6, range 81488.8-93679.4 ns)
  81488.8 |####################
  82098.3 |
  82707.9 |
  83317.4 |####################
  83926.9 |########################################
  84536.4 |
  85146.0 |
  85755.5 |
  86365.0 |
  86974.6 |
  87584.1 |
  88193.6 |
  88803.2 |
  89412.7 |
  90022.2 |
  90631.8 |
  91241.3 |
  91850.8 |
  92460.3 |####################
  93069.9 |
  (0 below, 1 above range)

ab_b6_seq (n=6, range 81407.5-87664.8 ns)
  81407.5 |####################
  81720.4 |
  82033.2 |
  82346.1 |
  82658.9 |
  82971.8 |
  83284.7 |
  83597.5 |########################################
  83910.4 |
  84223.3 |
  84536.1 |####################
  84849.0 |
  85161.9 |
  85474.7 |
  85787.6 |
  86100.4 |
  86413.3 |
  86726.2 |
  87039.0 |
  87351.9 |####################
  (0 below, 1 above range)

ab_b6_table (n=6, range 79699.2-92226.5 ns)
  79699.2 |########################################
  80325.6 |########################################
  80951.9 |
  81578.3 |
  82204.6 |########################################
  82831.0 |
  83457.4 |
  84083.7 |
  84710.1 |
  85336.5 |
  85962.8 |
  86589.2 |
  87215.6 |
  87841.9 |########################################
  88468.3 |
  89094.6 |
  89721.0 |
  90347.4 |
  90973.7 |
  91600.1 |########################################
  (0 below, 1 above range)

ab_b6_tree (n=6, range 79540.8-90591.9 ns)
  79540.8 |####################
  80093.4 |########################################
  80645.9 |
  81198.5 |
  81751.0 |
  82303.6 |
  82856.1 |
  83408.7 |
  83961.2 |
  84513.8 |
  85066.3 |
  85618.9 |
  86171.4 |
  86724.0 |####################
  87276.5 |
  87829.1 |
  88381.6 |
  88934.2 |
  89486.7 |
  90039.3 |####################
  (0 below, 1 above range)

```
