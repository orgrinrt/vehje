# Per-branch strategy (NATIVE tier): archetype 6

5 variants, 6 samples per variant.
Baseline: **an_b6_table**

## Highlights

Baseline for all deltas below: **an_b6_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b6_tree shows alternating (throttle bounce) (autocorr -0.55)

an_b6_tree's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (an_b6_table)

The baseline an_b6_table is the fastest (52.88 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (an_b6_table) is the fastest** at 52882.3 ns median
- 2 variants significantly slower than baseline
- Spread: 1.42x (fastest 52882.3 ns, slowest 74867.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b6_pred | 76726ns | 77245ns | 68165ns | 76702ns | 81041ns | +38.96% |
| an_b6_prof | 61850ns | 62660ns | 50759ns | 61922ns | 67288ns | +12.02% |
| an_b6_seq | 65522ns | 65893ns | 62008ns | 64635ns | 68608ns | +18.66% |
| an_b6_table | 55216ns | 55236ns | 50370ns | 54935ns | 58060ns | base |
| an_b6_tree | 55926ns | 57168ns | 48523ns | 54506ns | 61756ns | +1.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b6_pred | 74337ns | 65843ns | 78716ns | +40.62% | 0.055 |
| an_b6_prof | 59517ns | 48541ns | 64852ns | +12.59% | 0.069 |
| an_b6_seq | 63152ns | 59779ns | 66131ns | +19.46% | 0.065 |
| an_b6_table | 52863ns | 48111ns | 55674ns | base | 0.077 |
| an_b6_tree | 53603ns | 46297ns | 59373ns | +1.40% | 0.076 |

## Performance model

- Peak throughput: **0.088 Gops/s** (an_b6_tree; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b6_pred | 0.055 | 61.8% |
| an_b6_prof | 0.068 | 76.8% |
| an_b6_seq | 0.065 | 73.0% |
| an_b6_table | 0.077 | 87.5% |
| an_b6_tree | 0.075 | 84.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b6_pred | 76726ns | 76726ns | +38.96% |
| an_b6_prof | 61850ns | 61850ns | +12.02% |
| an_b6_seq | 65522ns | 65522ns | +18.66% |
| an_b6_table | 55216ns | 55216ns | base |
| an_b6_tree | 55926ns | 55926ns | +1.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b6_table | 52882ns | base | --- | [50033, 55674] | --- | --- | --- | --- |
| an_b6_pred | 74868ns | +23689.7ns (+44.8%) | [+14516, +26215]ns | [69428, 78716] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b6_prof | 60288ns | no significant difference | [-1681, +14819]ns | [53411, 64852] | no | 0.2917 | 0.2188 | 0 |
| an_b6_seq | 63457ns | +9431.5ns (+17.8%) | [+5958, +15475]ns | [59867, 66131] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b6_tree | 54793ns | no significant difference | [-8020, +9302]ns | [46644, 59373] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b6_table | an_b6_pred | an_b6_prof | an_b6_seq | an_b6_tree |
|---|---|---|---|---|---|
| 1 | 54585ns | +20.6% | -11.1% | +9.5% | +5.2% |
| 2 | 52564ns | +45.9% | +10.9% | +14.1% | -11.9% |
| 3 | 48111ns | +51.8% | +35.5% | +35.5% | +19.2% |
| 4 | 53200ns | +51.7% | +14.9% | +26.1% | -1.8% |
| 5 | 56764ns | +31.3% | +4.7% | +11.8% | -17.2% |
| 6 | 51955ns | +44.7% | +24.1% | +22.1% | +18.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b6_pred | -0.250 | moderate- |
| an_b6_prof | 0.084 | ok |
| an_b6_seq | 0.331 | moderate+ |
| an_b6_table | -0.070 | ok |
| an_b6_tree | -0.546 | HIGH- (thermal bounce) |

**Consistency summary:**

- **an_b6_pred**: won 0/6, lost 6/6
- **an_b6_prof**: won 1/6, lost 5/6
- **an_b6_seq**: won 0/6, lost 6/6
- **an_b6_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b6_pred | 4.5ns | 74337.1ns | 0.0% |  |
| an_b6_prof | 6.2ns | 59516.9ns | 0.0% |  |
| an_b6_seq | 7.5ns | 63151.6ns | 0.0% |  |
| an_b6_table | 5.9ns | 52863.2ns | 0.0% |  |
| an_b6_tree | 5.5ns | 53603.3ns | 0.0% |  |

## Distribution (algo ns)

```
an_b6_pred (n=6, range 65842.9-78715.9 ns)
  65842.9 |########################################
  66486.5 |
  67130.2 |
  67773.8 |
  68417.5 |
  69061.1 |
  69704.8 |
  70348.4 |
  70992.1 |
  71635.7 |
  72279.4 |
  72923.0 |########################################
  73566.7 |
  74210.3 |########################################
  74854.0 |########################################
  75497.6 |
  76141.3 |########################################
  76784.9 |
  77428.6 |
  78072.2 |
  (0 below, 1 above range)

an_b6_prof (n=6, range 48541.2-64852.3 ns)
  48541.2 |########################################
  49356.8 |
  50172.3 |
  50987.9 |
  51803.4 |
  52619.0 |
  53434.5 |
  54250.1 |
  55065.6 |
  55881.2 |
  56696.8 |
  57512.3 |########################################
  58327.9 |
  59143.4 |########################################
  59959.0 |
  60774.5 |########################################
  61590.1 |
  62405.6 |
  63221.2 |
  64036.7 |########################################
  (0 below, 1 above range)

an_b6_seq (n=6, range 59778.8-66130.9 ns)
  59778.8 |########################################
  60096.4 |
  60414.0 |
  60731.6 |
  61049.2 |
  61366.8 |
  61684.4 |
  62002.0 |
  62319.6 |
  62637.2 |
  62954.8 |
  63272.4 |########################################
  63590.0 |
  63907.6 |
  64225.2 |
  64542.8 |
  64860.4 |####################
  65178.0 |
  65495.6 |
  65813.2 |
  (0 below, 1 above range)

an_b6_table (n=6, range 48110.8-55674.3 ns)
  48110.8 |########################################
  48489.0 |
  48867.2 |
  49245.3 |
  49623.5 |
  50001.7 |
  50379.9 |
  50758.0 |
  51136.2 |
  51514.4 |
  51892.6 |########################################
  52270.8 |########################################
  52648.9 |
  53027.1 |########################################
  53405.3 |
  53783.5 |
  54161.6 |
  54539.8 |########################################
  54918.0 |
  55296.2 |
  (0 below, 1 above range)

an_b6_tree (n=6, range 46297.1-59372.7 ns)
  46297.1 |####################
  46950.9 |####################
  47604.7 |
  48258.4 |
  48912.2 |
  49566.0 |
  50219.8 |
  50873.6 |
  51527.3 |
  52181.1 |####################
  52834.9 |
  53488.7 |
  54142.5 |
  54796.2 |
  55450.0 |
  56103.8 |
  56757.6 |########################################
  57411.4 |
  58065.1 |
  58718.9 |
  (0 below, 1 above range)

```
