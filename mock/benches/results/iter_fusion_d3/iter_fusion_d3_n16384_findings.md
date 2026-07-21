# Iterator fusion (depth 3): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull3**

## Highlights

Baseline for all deltas below: **iterfuse_pull3**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (iterfuse_pull3, iterfuse_push3) are a dead heat (<1%)

iterfuse_pull3 (889.32 us) and iterfuse_push3 (894.55 us) differ by 0.59%, inside the noise, even though the wider field spreads 31.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (iterfuse_pull3)

The baseline iterfuse_pull3 is the fastest (889.32 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (iterfuse_pull3) is the fastest** at 889323.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.32x (fastest 889323.9 ns, slowest 1170335.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat3 | 1169339ns | 1172740ns | 1129012ns | 1167581ns | 1192138ns | +31.38% |
| iterfuse_pull3 | 890060ns | 892168ns | 874025ns | 890775ns | 897006ns | base |
| iterfuse_push3 | 893757ns | 897778ns | 872649ns | 894406ns | 903337ns | +0.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat3 | 1166931ns | 1126604ns | 1189689ns | +31.54% | 0.014 |
| iterfuse_pull3 | 887115ns | 871771ns | 893585ns | base | 0.018 |
| iterfuse_push3 | 890638ns | 870211ns | 900246ns | +0.40% | 0.018 |

## Performance model

- Peak throughput: **0.019 Gops/s** (iterfuse_push3; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat3 | 0.014 | 74.4% |
| iterfuse_pull3 | 0.018 | 97.9% |
| iterfuse_push3 | 0.018 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat3 | 1169339ns | 1169339ns | +31.38% |
| iterfuse_pull3 | 890060ns | 890060ns | base |
| iterfuse_push3 | 893757ns | 893757ns | +0.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull3 | 889324ns | base | --- | [878437, 893585] | --- | --- | --- | --- |
| iterfuse_mat3 | 1170335ns | +289501.5ns (+32.6%) | [+247183, +302762]ns | [1140769, 1189689] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| iterfuse_push3 | 894552ns | no significant difference | [-1837, +10922]ns | [877116, 900246] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull3 | iterfuse_mat3 | iterfuse_push3 |
|---|---|---|---|
| 1 | 891175ns | +29.6% | +0.5% |
| 2 | 888752ns | +34.1% | +1.5% |
| 3 | 885102ns | +34.2% | -0.1% |
| 4 | 871771ns | +33.2% | -0.2% |
| 5 | 889896ns | +32.6% | +1.0% |
| 6 | 895995ns | +25.7% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat3 | -0.164 | ok |
| iterfuse_pull3 | 0.047 | ok |
| iterfuse_push3 | -0.039 | ok |

**Consistency summary:**

- **iterfuse_mat3**: won 0/6, lost 6/6
- **iterfuse_push3**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat3 | 5080.0ns | 1166931.0ns | 0.4% |  |
| iterfuse_pull3 | 7.6ns | 887115.4ns | 0.0% |  |
| iterfuse_push3 | 11.2ns | 890637.9ns | 0.0% |  |

## Distribution (algo ns)

```
iterfuse_mat3 (n=6, range 1126604.2-1189688.9 ns)
  1126604.2 |########################################
  1129758.4 |
  1132912.7 |
  1136066.9 |
  1139221.1 |
  1142375.4 |
  1145529.6 |
  1148683.9 |
  1151838.1 |########################################
  1154992.3 |
  1158146.6 |########################################
  1161300.8 |
  1164455.1 |
  1167609.3 |
  1170763.5 |
  1173917.8 |
  1177072.0 |########################################
  1180226.2 |
  1183380.5 |
  1186534.7 |########################################
  (0 below, 1 above range)

iterfuse_pull3 (n=6, range 871771.2-893585.4 ns)
  871771.2 |########################################
  872861.9 |
  873952.6 |
  875043.3 |
  876134.0 |
  877224.8 |
  878315.5 |
  879406.2 |
  880496.9 |
  881587.6 |
  882678.3 |
  883769.0 |
  884859.7 |########################################
  885950.4 |
  887041.1 |
  888131.8 |########################################
  889222.6 |########################################
  890313.3 |########################################
  891404.0 |
  892494.7 |
  (0 below, 1 above range)

iterfuse_push3 (n=6, range 870210.8-900246.2 ns)
  870210.8 |########################################
  871712.6 |
  873214.3 |
  874716.1 |
  876217.9 |
  877719.7 |
  879221.4 |
  880723.2 |
  882225.0 |
  883726.8 |########################################
  885228.5 |
  886730.3 |
  888232.1 |
  889733.8 |
  891235.6 |
  892737.4 |########################################
  894239.2 |########################################
  895740.9 |
  897242.7 |########################################
  898744.5 |
  (0 below, 1 above range)

```
