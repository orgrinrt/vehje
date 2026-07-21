# Per-branch strategy: archetype 1 (match4_nested), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b1_table**

## Highlights

Baseline for all deltas below: **ab_b1_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_b1_table is fastest but the noisiest (CV 6.4%)

ab_b1_table wins on median (23.34 us) yet has the highest variance (CV 6.4%), while ab_b1_pred is the steadiest (CV 3.0%, 28.51 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ab_b1_pred shows alternating (throttle bounce) (autocorr -0.60)

ab_b1_pred's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_b1_table)

The baseline ab_b1_table is the fastest (23.34 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (ab_b1_table) is the fastest** at 23337.7 ns median
- 2 variants significantly slower than baseline
- Spread: 1.22x (fastest 23337.7 ns, slowest 28513.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b1_pred | 30667ns | 31044ns | 29450ns | 30517ns | 31502ns | +20.25% |
| ab_b1_prof | 26191ns | 26803ns | 22464ns | 26733ns | 27242ns | +2.70% |
| ab_b1_seq | 26116ns | 26825ns | 22457ns | 26809ns | 26907ns | +2.41% |
| ab_b1_table | 25503ns | 25846ns | 22367ns | 25523ns | 27040ns | base |
| ab_b1_tree | 25612ns | 26249ns | 22114ns | 25982ns | 26807ns | +0.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b1_pred | 28146ns | 26982ns | 28912ns | +22.17% | 0.002 |
| ab_b1_prof | 23649ns | 20306ns | 24583ns | +2.65% | 0.003 |
| ab_b1_seq | 23602ns | 20304ns | 24337ns | +2.45% | 0.003 |
| ab_b1_table | 23037ns | 20205ns | 24433ns | base | 0.003 |
| ab_b1_tree | 23156ns | 20005ns | 24240ns | +0.51% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b1_tree; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b1_pred | 0.002 | 70.2% |
| ab_b1_prof | 0.003 | 82.7% |
| ab_b1_seq | 0.003 | 82.5% |
| ab_b1_table | 0.003 | 85.7% |
| ab_b1_tree | 0.003 | 84.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b1_pred | 30667ns | 30667ns | +20.25% |
| ab_b1_prof | 26191ns | 26191ns | +2.70% |
| ab_b1_seq | 26116ns | 26116ns | +2.41% |
| ab_b1_table | 25503ns | 25503ns | base |
| ab_b1_tree | 25612ns | 25612ns | +0.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b1_table | 23338ns | base | --- | [21341, 24433] | --- | --- | --- | --- |
| ab_b1_pred | 28514ns | +4749.6ns (+20.4%) | [+4006, +6569]ns | [27011, 28912] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b1_prof | 24193ns | +234.4ns (+1.0%) | [+42, +1558]ns | [22170, 24583] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| ab_b1_seq | 24241ns | no significant difference | [-192, +1782]ns | [22228, 24337] | no | 0.2917 | 0.2188 | 0 |
| ab_b1_tree | 23725ns | no significant difference | [-786, +1046]ns | [21504, 24240] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b1_table | ab_b1_pred | ab_b1_prof | ab_b1_seq | ab_b1_tree |
|---|---|---|---|---|---|
| 1 | 20205ns | +33.8% | +0.5% | +0.5% | -1.0% |
| 2 | 22632ns | +27.8% | +6.2% | +7.5% | +1.6% |
| 3 | 24654ns | +14.2% | +1.0% | -1.6% | -5.6% |
| 4 | 24044ns | +20.1% | +0.9% | +0.4% | +1.0% |
| 5 | 22478ns | +20.0% | +7.6% | +8.3% | +7.7% |
| 6 | 24212ns | +19.3% | -0.1% | +0.1% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b1_pred | -0.598 | HIGH- (thermal bounce) |
| ab_b1_prof | 0.043 | ok |
| ab_b1_seq | -0.051 | ok |
| ab_b1_table | 0.067 | ok |
| ab_b1_tree | 0.213 | moderate+ |

**Consistency summary:**

- **ab_b1_pred**: won 0/6, lost 6/6
- **ab_b1_prof**: won 0/6, lost 5/6
- **ab_b1_seq**: won 1/6, lost 4/6
- **ab_b1_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b1_pred | 3.9ns | 28145.6ns | 0.0% |  |
| ab_b1_prof | 4.4ns | 23648.8ns | 0.0% |  |
| ab_b1_seq | 4.6ns | 23602.0ns | 0.0% |  |
| ab_b1_table | 2.8ns | 23037.4ns | 0.0% |  |
| ab_b1_tree | 3.5ns | 23156.1ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b1_pred (n=6, range 26982.1-28912.1 ns)
  26982.1 |########################################
  27078.6 |
  27175.1 |
  27271.6 |
  27368.1 |
  27464.6 |
  27561.1 |
  27657.6 |
  27754.1 |
  27850.6 |
  27947.1 |
  28043.6 |
  28140.1 |####################
  28236.6 |
  28333.1 |
  28429.6 |
  28526.1 |
  28622.6 |
  28719.1 |
  28815.6 |########################################
  (0 below, 1 above range)

ab_b1_prof (n=6, range 20306.2-24583.1 ns)
  20306.2 |#############
  20520.0 |
  20733.9 |
  20947.7 |
  21161.6 |
  21375.4 |
  21589.3 |
  21803.1 |
  22017.0 |
  22230.8 |
  22444.7 |
  22658.5 |
  22872.3 |
  23086.2 |
  23300.0 |
  23513.9 |
  23727.7 |
  23941.6 |#############
  24155.4 |########################################
  24369.3 |
  (0 below, 1 above range)

ab_b1_seq (n=6, range 20304.2-24336.8 ns)
  20304.2 |##########
  20505.8 |
  20707.5 |
  20909.1 |
  21110.7 |
  21312.4 |
  21514.0 |
  21715.6 |
  21917.3 |
  22118.9 |
  22320.5 |
  22522.2 |
  22723.8 |
  22925.4 |
  23127.1 |
  23328.7 |
  23530.3 |
  23732.0 |
  23933.6 |
  24135.2 |########################################
  (0 below, 1 above range)

ab_b1_table (n=6, range 20204.6-24433.2 ns)
  20204.6 |####################
  20416.0 |
  20627.5 |
  20838.9 |
  21050.3 |
  21261.7 |
  21473.2 |
  21684.6 |
  21896.0 |
  22107.4 |
  22318.9 |####################
  22530.3 |####################
  22741.7 |
  22953.2 |
  23164.6 |
  23376.0 |
  23587.4 |
  23798.9 |
  24010.3 |########################################
  24221.7 |
  (0 below, 1 above range)

ab_b1_tree (n=6, range 20005.4-24239.8 ns)
  20005.4 |####################
  20217.1 |
  20428.8 |
  20640.6 |
  20852.3 |
  21064.0 |
  21275.7 |
  21487.4 |
  21699.2 |
  21910.9 |
  22122.6 |
  22334.3 |
  22546.0 |
  22757.8 |
  22969.5 |####################
  23181.2 |####################
  23392.9 |
  23604.6 |
  23816.4 |
  24028.1 |########################################
  (0 below, 1 above range)

```
