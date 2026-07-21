# Per-branch strategy: archetype 5 (ifchain4_nested), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b5_table**

## Highlights

Baseline for all deltas below: **ab_b5_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (ab_b5_tree, ab_b5_table) are a dead heat (<1%)

ab_b5_tree (83.14 us) and ab_b5_table (83.36 us) differ by 0.25%, inside the noise, even though the wider field spreads 15.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### ab_b5_seq shows alternating (throttle bounce) (autocorr -0.75)

ab_b5_seq's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: ab_b5_tree** at 83144.6 ns median (-0.3% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.15x (fastest 83144.6 ns, slowest 95632.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b5_pred | 99370ns | 97960ns | 94193ns | 97319ns | 105036ns | +13.62% |
| ab_b5_prof | 88111ns | 86979ns | 85627ns | 86701ns | 91468ns | +0.74% |
| ab_b5_seq | 90445ns | 88459ns | 85536ns | 87686ns | 97040ns | +3.41% |
| ab_b5_table | 87461ns | 85639ns | 82000ns | 84969ns | 93931ns | base |
| ab_b5_tree | 86368ns | 85463ns | 82648ns | 85264ns | 89885ns | -1.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b5_pred | 97050ns | 92057ns | 102601ns | +14.01% | 0.003 |
| ab_b5_prof | 85747ns | 83390ns | 89054ns | +0.73% | 0.003 |
| ab_b5_seq | 88043ns | 83369ns | 94472ns | +3.43% | 0.003 |
| ab_b5_table | 85127ns | 79832ns | 91451ns | base | 0.003 |
| ab_b5_tree | 84028ns | 80387ns | 87469ns | -1.29% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b5_table; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b5_pred | 0.003 | 83.5% |
| ab_b5_prof | 0.003 | 94.4% |
| ab_b5_seq | 0.003 | 92.8% |
| ab_b5_table | 0.003 | 95.8% |
| ab_b5_tree | 0.003 | 96.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b5_pred | 99370ns | 99370ns | +13.62% |
| ab_b5_prof | 88111ns | 88111ns | +0.74% |
| ab_b5_seq | 90445ns | 90445ns | +3.41% |
| ab_b5_table | 87461ns | 87461ns | base |
| ab_b5_tree | 86368ns | 86368ns | -1.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b5_table | 83355ns | base | --- | [80573, 91451] | --- | --- | --- | --- |
| ab_b5_pred | 95632ns | +11992.1ns (+14.4%) | [+7811, +15966]ns | [92916, 102601] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b5_prof | 84597ns | no significant difference | [-7393, +6191]ns | [83591, 89054] | no | 0.6875 | 0.6875 | 0 |
| ab_b5_seq | 86046ns | +2331.0ns (+2.8%) | [+944, +5476]ns | [83612, 94472] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| ab_b5_tree | 83145ns | no significant difference | [-5997, +3033]ns | [81471, 87469] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b5_table | ab_b5_pred | ab_b5_prof | ab_b5_seq | ab_b5_tree |
|---|---|---|---|---|---|
| 1 | 79832ns | +15.3% | +4.5% | +9.0% | +3.6% |
| 2 | 81314ns | +18.2% | +4.4% | +2.5% | +3.9% |
| 3 | 90948ns | +5.0% | -7.9% | +4.1% | -0.6% |
| 4 | 84022ns | +14.0% | +10.5% | -0.2% | -4.3% |
| 5 | 91955ns | +18.6% | -8.3% | +2.5% | -9.1% |
| 6 | 82688ns | +13.4% | +3.1% | +2.9% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b5_pred | -0.251 | moderate- |
| ab_b5_prof | -0.310 | moderate- |
| ab_b5_seq | -0.748 | HIGH- (thermal bounce) |
| ab_b5_table | -0.251 | moderate- |
| ab_b5_tree | -0.318 | moderate- |

**Consistency summary:**

- **ab_b5_pred**: won 0/6, lost 6/6
- **ab_b5_prof**: won 2/6, lost 4/6
- **ab_b5_seq**: won 1/6, lost 5/6
- **ab_b5_tree**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b5_pred | 4.3ns | 97049.7ns | 0.0% |  |
| ab_b5_prof | 5.4ns | 85747.2ns | 0.0% |  |
| ab_b5_seq | 5.8ns | 88043.5ns | 0.0% |  |
| ab_b5_table | 5.4ns | 85126.6ns | 0.0% |  |
| ab_b5_tree | 6.5ns | 84028.3ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b5_pred (n=6, range 92056.7-102601.1 ns)
  92056.7 |####################
  92583.9 |
  93111.1 |
  93638.4 |####################
  94165.6 |
  94692.8 |
  95220.0 |####################
  95747.2 |########################################
  96274.4 |
  96801.7 |
  97328.9 |
  97856.1 |
  98383.3 |
  98910.5 |
  99437.7 |
  99965.0 |
  100492.2 |
  101019.4 |
  101546.6 |
  102073.8 |
  (0 below, 1 above range)

ab_b5_prof (n=6, range 83390.0-89053.8 ns)
  83390.0 |########################################
  83673.2 |########################################
  83956.4 |
  84239.6 |########################################
  84522.8 |
  84805.9 |########################################
  85089.1 |########################################
  85372.3 |
  85655.5 |
  85938.7 |
  86221.9 |
  86505.1 |
  86788.2 |
  87071.4 |
  87354.6 |
  87637.8 |
  87921.0 |
  88204.2 |
  88487.4 |
  88770.6 |
  (0 below, 1 above range)

ab_b5_seq (n=6, range 83369.2-94472.1 ns)
  83369.2 |########################################
  83924.3 |
  84479.5 |
  85034.6 |####################
  85589.8 |
  86144.9 |
  86700.1 |####################
  87255.2 |
  87810.4 |
  88365.5 |
  88920.6 |
  89475.8 |
  90030.9 |
  90586.1 |
  91141.2 |
  91696.4 |
  92251.5 |
  92806.7 |
  93361.8 |
  93917.0 |####################
  (0 below, 1 above range)

ab_b5_table (n=6, range 79832.1-91451.4 ns)
  79832.1 |########################################
  80413.1 |
  80994.0 |########################################
  81575.0 |
  82156.0 |########################################
  82736.9 |
  83317.9 |
  83898.9 |########################################
  84479.8 |
  85060.8 |
  85641.8 |
  86222.7 |
  86803.7 |
  87384.7 |
  87965.6 |
  88546.6 |
  89127.6 |
  89708.5 |
  90289.5 |
  90870.5 |########################################
  (0 below, 1 above range)

ab_b5_tree (n=6, range 80387.1-87469.4 ns)
  80387.1 |####################
  80741.2 |
  81095.3 |
  81449.4 |
  81803.6 |
  82157.7 |
  82511.8 |########################################
  82865.9 |
  83220.0 |
  83574.1 |####################
  83928.2 |
  84282.4 |####################
  84636.5 |
  84990.6 |
  85344.7 |
  85698.8 |
  86052.9 |
  86407.1 |
  86761.2 |
  87115.3 |
  (0 below, 1 above range)

```
