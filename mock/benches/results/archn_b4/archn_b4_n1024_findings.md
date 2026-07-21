# Per-branch strategy (NATIVE tier): archetype 4

5 variants, 6 samples per variant.
Baseline: **an_b4_table**

## Highlights

Baseline for all deltas below: **an_b4_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b4_table is fastest but the noisiest (CV 9.6%)

an_b4_table wins on median (8.46 us) yet has the highest variance (CV 9.6%), while an_b4_prof is the steadiest (CV 5.5%, 8.79 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (an_b4_table)

The baseline an_b4_table is the fastest (8.46 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader an_b4_table vs stability leader an_b4_prof (+4% speed for 1.8x steadier)

an_b4_table is fastest (8.46 us, CV 9.6%); an_b4_prof gives up 3.8% median for 1.8x lower variance (CV 5.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### an_b4_tree's edge over baseline is significant but tiny (12 ns, 0.14%)

an_b4_tree differs from baseline an_b4_table by 12 ns (0.14%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (an_b4_table) is the fastest** at 8461.8 ns median
- 2 variants significantly slower than baseline
- Spread: 1.19x (fastest 8461.8 ns, slowest 10037.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b4_pred | 12251ns | 12550ns | 10966ns | 12151ns | 13045ns | +14.51% |
| an_b4_prof | 11131ns | 11277ns | 10188ns | 11005ns | 11791ns | +4.03% |
| an_b4_seq | 11315ns | 11811ns | 9873ns | 11197ns | 12211ns | +5.75% |
| an_b4_table | 10699ns | 10987ns | 9415ns | 10480ns | 11669ns | base |
| an_b4_tree | 10881ns | 11202ns | 9441ns | 10717ns | 11848ns | +1.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b4_pred | 9810ns | 8764ns | 10451ns | +18.22% | 0.104 |
| an_b4_prof | 8696ns | 8024ns | 9195ns | +4.80% | 0.118 |
| an_b4_seq | 8874ns | 7717ns | 9641ns | +6.95% | 0.115 |
| an_b4_table | 8298ns | 7268ns | 9152ns | base | 0.123 |
| an_b4_tree | 8384ns | 7291ns | 9150ns | +1.04% | 0.122 |

## Performance model

- Peak throughput: **0.141 Gops/s** (an_b4_table; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b4_pred | 0.102 | 72.4% |
| an_b4_prof | 0.117 | 82.7% |
| an_b4_seq | 0.111 | 78.8% |
| an_b4_table | 0.121 | 85.9% |
| an_b4_tree | 0.119 | 84.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b4_pred | 12251ns | 12251ns | +14.51% |
| an_b4_prof | 11131ns | 11131ns | +4.03% |
| an_b4_seq | 11315ns | 11315ns | +5.75% |
| an_b4_table | 10699ns | 10699ns | base |
| an_b4_tree | 10881ns | 10881ns | +1.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b4_table | 8462ns | base | --- | [7279, 9152] | --- | --- | --- | --- |
| an_b4_pred | 10037ns | +1714.2ns (+20.3%) | [+847, +1975]ns | [8941, 10451] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b4_prof | 8786ns | no significant difference | [-650, +1342]ns | [8107, 9195] | no | 0.6875 | 0.6875 | 0 |
| an_b4_seq | 9224ns | +536.4ns (+6.3%) | [+141, +1052]ns | [7756, 9641] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| an_b4_tree | 8615ns | no significant difference | [-486, +733]ns | [7386, 9150] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b4_table | an_b4_pred | an_b4_prof | an_b4_seq | an_b4_tree |
|---|---|---|---|---|---|
| 1 | 8313ns | +25.5% | -3.5% | +9.8% | +15.0% |
| 2 | 8611ns | +20.1% | +7.8% | +14.9% | +0.3% |
| 3 | 9532ns | +2.1% | -10.6% | -1.5% | -8.3% |
| 4 | 7290ns | +25.1% | +12.3% | +5.9% | +0.0% |
| 5 | 7268ns | +20.6% | +24.6% | +7.3% | +2.9% |
| 6 | 8772ns | +19.3% | +3.8% | +6.2% | -2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b4_pred | 0.142 | ok |
| an_b4_prof | -0.321 | moderate- |
| an_b4_seq | 0.237 | moderate+ |
| an_b4_table | -0.077 | ok |
| an_b4_tree | 0.218 | moderate+ |

**Consistency summary:**

- **an_b4_pred**: won 0/6, lost 6/6
- **an_b4_prof**: won 2/6, lost 4/6
- **an_b4_seq**: won 1/6, lost 5/6
- **an_b4_tree**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b4_pred | 3.0ns | 9809.7ns | 0.0% |  |
| an_b4_prof | 6.3ns | 8695.9ns | 0.1% |  |
| an_b4_seq | 6.0ns | 8874.0ns | 0.1% |  |
| an_b4_table | 6.5ns | 8297.6ns | 0.1% |  |
| an_b4_tree | 6.4ns | 8383.8ns | 0.1% |  |

## Distribution (algo ns)

```
an_b4_pred (n=6, range 8763.8-10451.0 ns)
   8763.8 |########################################
   8848.2 |
   8932.5 |
   9016.9 |
   9101.2 |########################################
   9185.6 |
   9270.0 |
   9354.3 |
   9438.7 |
   9523.1 |
   9607.4 |
   9691.8 |########################################
   9776.1 |
   9860.5 |
   9944.9 |
  10029.2 |
  10113.6 |
  10198.0 |
  10282.3 |########################################
  10366.7 |########################################
  (0 below, 1 above range)

an_b4_prof (n=6, range 8023.7-9194.6 ns)
   8023.7 |########################################
   8082.2 |
   8140.8 |########################################
   8199.3 |
   8257.9 |
   8316.4 |
   8375.0 |
   8433.5 |
   8492.1 |########################################
   8550.6 |
   8609.1 |
   8667.7 |
   8726.2 |
   8784.8 |
   8843.3 |
   8901.9 |
   8960.4 |
   9019.0 |########################################
   9077.5 |########################################
   9136.1 |
  (0 below, 1 above range)

an_b4_seq (n=6, range 7717.1-9641.5 ns)
   7717.1 |########################################
   7813.3 |
   7909.5 |
   8005.8 |
   8102.0 |
   8198.2 |
   8294.4 |
   8390.6 |
   8486.8 |
   8583.1 |
   8679.3 |
   8775.5 |
   8871.7 |
   8967.9 |
   9064.1 |####################
   9160.4 |
   9256.6 |####################
   9352.8 |####################
   9449.0 |
   9545.2 |
  (0 below, 1 above range)

an_b4_table (n=6, range 7267.5-9152.1 ns)
   7267.5 |########################################
   7361.7 |
   7456.0 |
   7550.2 |
   7644.4 |
   7738.6 |
   7832.9 |
   7927.1 |
   8021.3 |
   8115.6 |
   8209.8 |
   8304.0 |####################
   8398.3 |
   8492.5 |
   8586.7 |####################
   8681.0 |####################
   8775.2 |
   8869.4 |
   8963.6 |
   9057.9 |
  (0 below, 1 above range)

an_b4_tree (n=6, range 7290.8-9150.5 ns)
   7290.8 |####################
   7383.8 |
   7476.8 |####################
   7569.7 |
   7662.7 |
   7755.7 |
   7848.7 |
   7941.7 |
   8034.7 |
   8127.6 |
   8220.6 |
   8313.6 |
   8406.6 |
   8499.6 |
   8592.6 |########################################
   8685.5 |####################
   8778.5 |
   8871.5 |
   8964.5 |
   9057.5 |
  (0 below, 1 above range)

```
