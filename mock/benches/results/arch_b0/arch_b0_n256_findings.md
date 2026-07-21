# Per-branch strategy: archetype 0 (match4_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b0_table**

## Highlights

Baseline for all deltas below: **ab_b0_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_b0_tree is fastest but the noisiest (CV 6.8%)

ab_b0_tree wins on median (86.07 us) yet has the highest variance (CV 6.8%), while ab_b0_table is the steadiest (CV 5.4%, 86.14 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (ab_b0_tree, ab_b0_table) are a dead heat (<1%)

ab_b0_tree (86.07 us) and ab_b0_table (86.14 us) differ by 0.08%, inside the noise, even though the wider field spreads 17.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b0_table shows alternating (throttle bounce) (autocorr -0.54)

ab_b0_table's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader ab_b0_tree vs stability leader ab_b0_table (+0% speed for 1.3x steadier)

ab_b0_tree is fastest (86.07 us, CV 6.8%); ab_b0_table gives up 0.1% median for 1.3x lower variance (CV 5.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: ab_b0_tree** at 86074.8 ns median (-0.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.17x (fastest 86074.8 ns, slowest 100913.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b0_pred | 105562ns | 103311ns | 99645ns | 102492ns | 113126ns | +18.83% |
| ab_b0_prof | 90440ns | 90998ns | 82479ns | 88768ns | 96929ns | +1.81% |
| ab_b0_seq | 89893ns | 90288ns | 81940ns | 88922ns | 95327ns | +1.20% |
| ab_b0_table | 88831ns | 88614ns | 82838ns | 87295ns | 94133ns | base |
| ab_b0_tree | 89660ns | 88443ns | 82639ns | 86879ns | 97344ns | +0.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b0_pred | 103108ns | 97331ns | 110495ns | +19.37% | 0.002 |
| ab_b0_prof | 88002ns | 80300ns | 94306ns | +1.88% | 0.003 |
| ab_b0_seq | 87463ns | 79728ns | 92806ns | +1.26% | 0.003 |
| ab_b0_table | 86376ns | 80451ns | 91530ns | base | 0.003 |
| ab_b0_tree | 87206ns | 80372ns | 94620ns | +0.96% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b0_seq; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b0_pred | 0.003 | 79.0% |
| ab_b0_prof | 0.003 | 90.1% |
| ab_b0_seq | 0.003 | 90.8% |
| ab_b0_table | 0.003 | 92.6% |
| ab_b0_tree | 0.003 | 92.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b0_pred | 105562ns | 105562ns | +18.83% |
| ab_b0_prof | 90440ns | 90440ns | +1.81% |
| ab_b0_seq | 89893ns | 89893ns | +1.20% |
| ab_b0_table | 88831ns | 88831ns | base |
| ab_b0_tree | 89660ns | 89660ns | +0.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b0_table | 86144ns | base | --- | [81453, 91530] | --- | --- | --- | --- |
| ab_b0_pred | 100913ns | +15398.4ns (+17.9%) | [+9786, +25013]ns | [97916, 110495] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b0_prof | 88501ns | no significant difference | [-316, +2824]ns | [81200, 94306] | no | 0.9167 | 0.6875 | 0 |
| ab_b0_seq | 87778ns | no significant difference | [-3274, +6013]ns | [81805, 92806] | no | 1.0000 | 1.0000 | 0 |
| ab_b0_tree | 86075ns | no significant difference | [-4155, +4638]ns | [80924, 94620] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b0_table | ab_b0_pred | ab_b0_prof | ab_b0_seq | ab_b0_tree |
|---|---|---|---|---|---|
| 1 | 80451ns | +38.1% | -0.2% | -0.9% | -0.1% |
| 2 | 90515ns | +21.4% | +3.2% | +1.2% | +2.9% |
| 3 | 89708ns | +14.5% | +3.1% | -6.5% | -9.2% |
| 4 | 82455ns | +19.5% | +2.5% | +1.8% | +1.7% |
| 5 | 92546ns | +7.1% | +2.9% | -0.0% | +3.8% |
| 6 | 82580ns | +17.9% | -0.6% | +12.7% | +6.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b0_pred | 0.526 | HIGH+ (drift/warm-up) |
| ab_b0_prof | -0.485 | moderate- |
| ab_b0_seq | -0.147 | ok |
| ab_b0_table | -0.544 | HIGH- (thermal bounce) |
| ab_b0_tree | -0.366 | moderate- |

**Consistency summary:**

- **ab_b0_pred**: won 0/6, lost 6/6
- **ab_b0_prof**: won 2/6, lost 4/6
- **ab_b0_seq**: won 2/6, lost 3/6
- **ab_b0_tree**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b0_pred | 5.1ns | 103108.3ns | 0.0% |  |
| ab_b0_prof | 5.6ns | 88002.4ns | 0.0% |  |
| ab_b0_seq | 5.0ns | 87462.8ns | 0.0% |  |
| ab_b0_table | 5.8ns | 86375.9ns | 0.0% |  |
| ab_b0_tree | 4.6ns | 87206.2ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b0_pred (n=6, range 97330.8-110495.4 ns)
  97330.8 |########################################
  97989.0 |########################################
  98647.3 |########################################
  99305.5 |
  99963.7 |
  100621.9 |
  101280.2 |
  101938.4 |
  102596.6 |########################################
  103254.9 |
  103913.1 |
  104571.3 |
  105229.6 |
  105887.8 |
  106546.0 |
  107204.2 |
  107862.5 |
  108520.7 |
  109178.9 |
  109837.2 |########################################
  (0 below, 1 above range)

ab_b0_prof (n=6, range 80300.0-94306.4 ns)
  80300.0 |########################################
  81000.3 |
  81700.6 |########################################
  82401.0 |
  83101.3 |
  83801.6 |
  84501.9 |########################################
  85202.3 |
  85902.6 |
  86602.9 |
  87303.2 |
  88003.5 |
  88703.9 |
  89404.2 |
  90104.5 |
  90804.8 |
  91505.2 |
  92205.5 |########################################
  92905.8 |########################################
  93606.1 |
  (0 below, 1 above range)

ab_b0_seq (n=6, range 79727.9-92805.9 ns)
  79727.9 |####################
  80381.8 |
  81035.7 |
  81689.6 |
  82343.5 |
  82997.4 |
  83651.3 |########################################
  84305.2 |
  84959.1 |
  85613.0 |
  86266.9 |
  86920.8 |
  87574.7 |
  88228.6 |
  88882.5 |
  89536.4 |
  90190.3 |
  90844.2 |
  91498.1 |####################
  92152.0 |####################
  (0 below, 1 above range)

ab_b0_table (n=6, range 80450.8-91530.4 ns)
  80450.8 |####################
  81004.8 |
  81558.8 |
  82112.7 |########################################
  82666.7 |
  83220.7 |
  83774.7 |
  84328.7 |
  84882.6 |
  85436.6 |
  85990.6 |
  86544.6 |
  87098.6 |
  87652.5 |
  88206.5 |
  88760.5 |
  89314.5 |####################
  89868.5 |
  90422.4 |####################
  90976.4 |
  (0 below, 1 above range)

ab_b0_tree (n=6, range 80371.7-94619.5 ns)
  80371.7 |########################################
  81084.1 |########################################
  81796.5 |
  82508.9 |
  83221.3 |########################################
  83933.7 |
  84646.1 |
  85358.4 |
  86070.8 |
  86783.2 |
  87495.6 |
  88208.0 |########################################
  88920.4 |
  89632.8 |
  90345.2 |
  91057.6 |
  91770.0 |
  92482.4 |########################################
  93194.8 |
  93907.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **ab_b0_pred**: autocorrelation=0.53 (measurement drift or warm-up artifact)
