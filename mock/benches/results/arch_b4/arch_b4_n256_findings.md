# Per-branch strategy: archetype 4 (ifchain4_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b4_table**

## Highlights

Baseline for all deltas below: **ab_b4_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b4_tree, ab_b4_table) are a dead heat (<1%)

ab_b4_tree (85.83 us) and ab_b4_table (86.04 us) differ by 0.25%, inside the noise, even though the wider field spreads 16.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: ab_b4_tree** at 85827.9 ns median (-0.2% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.16x (fastest 85827.9 ns, slowest 99954.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b4_pred | 102777ns | 102287ns | 96701ns | 100596ns | 109086ns | +15.91% |
| ab_b4_prof | 91992ns | 92010ns | 85445ns | 89865ns | 98456ns | +3.74% |
| ab_b4_seq | 90970ns | 90414ns | 84379ns | 88668ns | 97717ns | +2.59% |
| ab_b4_table | 88673ns | 88472ns | 81965ns | 86703ns | 94982ns | base |
| ab_b4_tree | 88781ns | 88184ns | 82568ns | 86727ns | 94969ns | +0.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b4_pred | 100406ns | 94578ns | 106633ns | +16.36% | 0.003 |
| ab_b4_prof | 89564ns | 83178ns | 95828ns | +3.80% | 0.003 |
| ab_b4_seq | 88544ns | 82077ns | 95041ns | +2.61% | 0.003 |
| ab_b4_table | 86288ns | 79777ns | 92449ns | base | 0.003 |
| ab_b4_tree | 86358ns | 80258ns | 92321ns | +0.08% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b4_table; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b4_pred | 0.003 | 79.8% |
| ab_b4_prof | 0.003 | 89.0% |
| ab_b4_seq | 0.003 | 90.6% |
| ab_b4_table | 0.003 | 92.7% |
| ab_b4_tree | 0.003 | 92.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b4_pred | 102777ns | 102777ns | +15.91% |
| ab_b4_prof | 91992ns | 91992ns | +3.74% |
| ab_b4_seq | 90970ns | 90970ns | +2.59% |
| ab_b4_table | 88673ns | 88673ns | base |
| ab_b4_tree | 88781ns | 88781ns | +0.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b4_table | 86041ns | base | --- | [80375, 92449] | --- | --- | --- | --- |
| ab_b4_pred | 99955ns | +14183.8ns (+16.5%) | [+12904, +15266]ns | [94631, 106633] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_b4_prof | 89646ns | +3793.0ns (+4.4%) | [+1111, +4924]ns | [83219, 95828] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_b4_seq | 88098ns | +2309.2ns (+2.7%) | [+797, +3662]ns | [82494, 95041] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| ab_b4_tree | 85828ns | no significant difference | [-2361, +2020]ns | [80925, 92321] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b4_table | ab_b4_pred | ab_b4_prof | ab_b4_seq | ab_b4_tree |
|---|---|---|---|---|---|
| 1 | 79777ns | +18.7% | +4.4% | +2.9% | +0.6% |
| 2 | 93082ns | +14.8% | +0.2% | -0.2% | -4.3% |
| 3 | 80973ns | +19.3% | +6.2% | +2.9% | +0.8% |
| 4 | 81140ns | +16.6% | +2.5% | +2.2% | +1.8% |
| 5 | 90943ns | +13.6% | +4.5% | +5.4% | +2.8% |
| 6 | 91816ns | +15.9% | +5.2% | +2.6% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b4_pred | -0.229 | moderate- |
| ab_b4_prof | -0.055 | ok |
| ab_b4_seq | -0.102 | ok |
| ab_b4_table | -0.262 | moderate- |
| ab_b4_tree | -0.025 | ok |

**Consistency summary:**

- **ab_b4_pred**: won 0/6, lost 6/6
- **ab_b4_prof**: won 0/6, lost 6/6
- **ab_b4_seq**: won 1/6, lost 5/6
- **ab_b4_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b4_pred | 5.4ns | 100406.2ns | 0.0% |  |
| ab_b4_prof | 5.5ns | 89564.1ns | 0.0% |  |
| ab_b4_seq | 4.8ns | 88544.4ns | 0.0% |  |
| ab_b4_table | 4.5ns | 86288.4ns | 0.0% |  |
| ab_b4_tree | 4.3ns | 86358.1ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b4_pred (n=6, range 94578.3-106632.7 ns)
  94578.3 |########################################
  95181.0 |
  95783.7 |
  96386.5 |####################
  96989.2 |
  97591.9 |
  98194.6 |
  98797.3 |
  99400.1 |
  100002.8 |
  100605.5 |
  101208.2 |
  101810.9 |
  102413.7 |
  103016.4 |####################
  103619.1 |
  104221.8 |
  104824.5 |
  105427.3 |
  106030.0 |####################
  (0 below, 1 above range)

ab_b4_prof (n=6, range 83178.3-95827.5 ns)
  83178.3 |########################################
  83810.8 |
  84443.2 |
  85075.7 |
  85708.1 |####################
  86340.6 |
  86973.1 |
  87605.5 |
  88238.0 |
  88870.4 |
  89502.9 |
  90135.4 |
  90767.8 |
  91400.3 |
  92032.7 |
  92665.2 |####################
  93297.7 |
  93930.1 |
  94562.6 |####################
  95195.0 |
  (0 below, 1 above range)

ab_b4_seq (n=6, range 82077.1-95041.4 ns)
  82077.1 |####################
  82725.3 |########################################
  83373.5 |
  84021.8 |
  84670.0 |
  85318.2 |
  85966.4 |
  86614.6 |
  87262.8 |
  87911.1 |
  88559.3 |
  89207.5 |
  89855.7 |
  90503.9 |
  91152.1 |
  91800.4 |
  92448.6 |####################
  93096.8 |
  93745.0 |####################
  94393.2 |
  (0 below, 1 above range)

ab_b4_table (n=6, range 79776.7-92449.0 ns)
  79776.7 |########################################
  80410.3 |########################################
  81043.9 |########################################
  81677.5 |
  82311.1 |
  82944.8 |
  83578.4 |
  84212.0 |
  84845.6 |
  85479.2 |
  86112.8 |
  86746.4 |
  87380.1 |
  88013.7 |
  88647.3 |
  89280.9 |
  89914.5 |
  90548.1 |########################################
  91181.7 |
  91815.3 |########################################
  (0 below, 1 above range)

ab_b4_tree (n=6, range 80257.9-92321.0 ns)
  80257.9 |########################################
  80861.1 |
  81464.2 |########################################
  82067.4 |########################################
  82670.5 |
  83273.7 |
  83876.8 |
  84480.0 |
  85083.2 |
  85686.3 |
  86289.5 |
  86892.6 |
  87495.8 |
  88098.9 |
  88702.1 |########################################
  89305.3 |
  89908.4 |
  90511.6 |
  91114.7 |########################################
  91717.9 |
  (0 below, 1 above range)

```
