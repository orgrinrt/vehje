# Per-branch strategy: archetype 0 (match4_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b0_table**

## Highlights

Baseline for all deltas below: **ab_b0_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_b0_table shows alternating (throttle bounce) (autocorr -0.52)

ab_b0_table's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader ab_b0_seq vs stability leader ab_b0_tree (+2% speed for 1.3x steadier)

ab_b0_seq is fastest (23.58 us, CV 7.5%); ab_b0_tree gives up 2.2% median for 1.3x lower variance (CV 5.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### ab_b0_prof is inconsistent: worst-20% is 1.6x its best-20%

ab_b0_prof's best 20% of batches run at 19.56 us but its worst 20% at 31.70 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ab_b0_seq** at 23577.5 ns median (-2.2% vs baseline)
- Spread: 1.13x (fastest 23577.5 ns, slowest 26719.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b0_pred | 29220ns | 29042ns | 26358ns | 28538ns | 31674ns | +10.46% |
| ab_b0_prof | 27862ns | 26752ns | 21663ns | 25420ns | 34623ns | +5.33% |
| ab_b0_seq | 25502ns | 26091ns | 22539ns | 25156ns | 27504ns | -3.59% |
| ab_b0_table | 26453ns | 26763ns | 22501ns | 26099ns | 28959ns | base |
| ab_b0_tree | 25885ns | 26692ns | 22587ns | 26199ns | 27062ns | -2.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b0_pred | 26785ns | 24194ns | 29024ns | +11.97% | 0.002 |
| ab_b0_prof | 25306ns | 19557ns | 31705ns | +5.79% | 0.003 |
| ab_b0_seq | 23024ns | 20351ns | 24805ns | -3.75% | 0.003 |
| ab_b0_table | 23922ns | 20313ns | 26301ns | base | 0.003 |
| ab_b0_tree | 23376ns | 20402ns | 24406ns | -2.28% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b0_prof; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b0_pred | 0.002 | 73.2% |
| ab_b0_prof | 0.003 | 80.9% |
| ab_b0_seq | 0.003 | 82.9% |
| ab_b0_table | 0.003 | 81.1% |
| ab_b0_tree | 0.003 | 81.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b0_pred | 29220ns | 29220ns | +10.46% |
| ab_b0_prof | 27862ns | 27862ns | +5.33% |
| ab_b0_seq | 25502ns | 25502ns | -3.59% |
| ab_b0_table | 26453ns | 26453ns | base |
| ab_b0_tree | 25885ns | 25885ns | -2.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b0_table | 24115ns | base | --- | [21349, 26301] | --- | --- | --- | --- |
| ab_b0_pred | 26720ns | no significant difference | [-868, +5140]ns | [24612, 29024] | no | 0.2917 | 0.2188 | 0 |
| ab_b0_prof | 24182ns | no significant difference | [-2460, +6474]ns | [20031, 31705] | no | 0.2917 | 0.2188 | 0 |
| ab_b0_seq | 23578ns | no significant difference | [-3506, +701]ns | [20689, 24805] | no | 0.2917 | 0.2188 | 0 |
| ab_b0_tree | 24087ns | no significant difference | [-2793, +1066]ns | [21636, 24406] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b0_table | ab_b0_pred | ab_b0_prof | ab_b0_seq | ab_b0_tree |
|---|---|---|---|---|---|
| 1 | 28078ns | -8.0% | +38.8% | -25.1% | -18.5% |
| 2 | 20313ns | +19.1% | +0.9% | +0.2% | +0.4% |
| 3 | 24525ns | +2.1% | -20.3% | +0.4% | -1.5% |
| 4 | 24188ns | +19.7% | +0.2% | +3.3% | +0.4% |
| 5 | 24041ns | +21.1% | +0.4% | +0.5% | +2.0% |
| 6 | 22384ns | +23.3% | +9.2% | +2.7% | +7.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b0_pred | 0.473 | moderate+ |
| ab_b0_prof | -0.120 | ok |
| ab_b0_seq | 0.338 | moderate+ |
| ab_b0_table | -0.519 | HIGH- (thermal bounce) |
| ab_b0_tree | 0.139 | ok |

**Consistency summary:**

- **ab_b0_pred**: won 1/6, lost 5/6
- **ab_b0_prof**: won 1/6, lost 5/6
- **ab_b0_seq**: won 1/6, lost 5/6
- **ab_b0_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b0_pred | 3.6ns | 26785.1ns | 0.0% |  |
| ab_b0_prof | 4.8ns | 25305.9ns | 0.0% |  |
| ab_b0_seq | 4.3ns | 23024.0ns | 0.0% |  |
| ab_b0_table | 4.0ns | 23921.5ns | 0.0% |  |
| ab_b0_tree | 4.2ns | 23376.3ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b0_pred (n=6, range 24193.7-29024.0 ns)
  24193.7 |########################################
  24435.2 |
  24676.7 |
  24918.2 |########################################
  25159.8 |
  25401.3 |
  25642.8 |########################################
  25884.3 |
  26125.8 |
  26367.3 |
  26608.8 |
  26850.3 |
  27091.8 |
  27333.4 |
  27574.9 |########################################
  27816.4 |
  28057.9 |
  28299.4 |
  28540.9 |
  28782.4 |########################################
  (0 below, 1 above range)

ab_b0_prof (n=6, range 19556.7-31704.8 ns)
  19556.7 |####################
  20164.1 |####################
  20771.5 |
  21378.9 |
  21986.3 |
  22593.7 |
  23201.1 |
  23808.5 |########################################
  24415.9 |####################
  25023.3 |
  25630.8 |
  26238.2 |
  26845.6 |
  27453.0 |
  28060.4 |
  28667.8 |
  29275.2 |
  29882.6 |
  30490.0 |
  31097.4 |
  (0 below, 1 above range)

ab_b0_seq (n=6, range 20350.8-24805.2 ns)
  20350.8 |########################################
  20573.5 |
  20796.2 |
  21019.0 |########################################
  21241.7 |
  21464.4 |
  21687.1 |
  21909.8 |
  22132.6 |
  22355.3 |
  22578.0 |
  22800.7 |########################################
  23023.4 |
  23246.2 |
  23468.9 |
  23691.6 |
  23914.3 |
  24137.0 |########################################
  24359.8 |
  24582.5 |########################################
  (0 below, 1 above range)

ab_b0_table (n=6, range 20313.3-26301.5 ns)
  20313.3 |####################
  20612.7 |
  20912.1 |
  21211.5 |
  21510.9 |
  21810.3 |
  22109.7 |####################
  22409.2 |
  22708.6 |
  23008.0 |
  23307.4 |
  23606.8 |
  23906.2 |########################################
  24205.6 |
  24505.0 |####################
  24804.4 |
  25103.8 |
  25403.2 |
  25702.6 |
  26002.0 |
  (0 below, 1 above range)

ab_b0_tree (n=6, range 20402.5-24406.5 ns)
  20402.5 |####################
  20602.7 |
  20802.9 |
  21003.1 |
  21203.3 |
  21403.5 |
  21603.7 |
  21803.9 |
  22004.1 |
  22204.3 |
  22404.5 |
  22604.7 |
  22804.9 |####################
  23005.1 |
  23205.3 |
  23405.5 |
  23605.7 |
  23805.9 |
  24006.1 |########################################
  24206.3 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **ab_b0_prof**: CV=25.3% (high variance, measurements may be unstable)
