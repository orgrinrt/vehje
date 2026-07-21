# Per-branch strategy (NATIVE tier): archetype 0

5 variants, 6 samples per variant.
Baseline: **an_b0_table**

## Highlights

Baseline for all deltas below: **an_b0_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b0_prof is fastest but the noisiest (CV 8.6%)

an_b0_prof wins on median (7.94 us) yet has the highest variance (CV 8.6%), while an_b0_seq is the steadiest (CV 5.4%, 8.44 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Speed leader an_b0_prof vs stability leader an_b0_seq (+6% speed for 1.6x steadier)

an_b0_prof is fastest (7.94 us, CV 8.6%); an_b0_seq gives up 6.3% median for 1.6x lower variance (CV 5.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### an_b0_prof's edge over baseline is significant but tiny (14 ns, 0.17%)

an_b0_prof differs from baseline an_b0_table by 14 ns (0.17%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_b0_prof** at 7941.4 ns median (-5.6% vs baseline)
- Spread: 1.10x (fastest 7941.4 ns, slowest 8730.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b0_pred | 11043ns | 11210ns | 9831ns | 11093ns | 11574ns | +4.43% |
| an_b0_prof | 10331ns | 10289ns | 9390ns | 10013ns | 11280ns | -2.30% |
| an_b0_seq | 10826ns | 10952ns | 9751ns | 10831ns | 11356ns | +2.38% |
| an_b0_table | 10575ns | 10931ns | 9505ns | 10472ns | 11264ns | base |
| an_b0_tree | 10785ns | 11260ns | 9398ns | 10795ns | 11465ns | +1.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b0_pred | 8604ns | 7660ns | 9019ns | +6.02% | 0.119 |
| an_b0_prof | 7976ns | 7256ns | 8704ns | -1.72% | 0.128 |
| an_b0_seq | 8347ns | 7496ns | 8765ns | +2.85% | 0.123 |
| an_b0_table | 8116ns | 7229ns | 8652ns | base | 0.126 |
| an_b0_tree | 8328ns | 7225ns | 8872ns | +2.61% | 0.123 |

## Performance model

- Peak throughput: **0.142 Gops/s** (an_b0_tree; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b0_pred | 0.117 | 82.8% |
| an_b0_prof | 0.129 | 91.0% |
| an_b0_seq | 0.121 | 85.6% |
| an_b0_table | 0.122 | 85.9% |
| an_b0_tree | 0.118 | 83.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b0_pred | 11043ns | 11043ns | +4.43% |
| an_b0_prof | 10331ns | 10331ns | -2.30% |
| an_b0_seq | 10826ns | 10826ns | +2.38% |
| an_b0_table | 10575ns | 10575ns | base |
| an_b0_tree | 10785ns | 10785ns | +1.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b0_table | 8410ns | base | --- | [7285, 8652] | --- | --- | --- | --- |
| an_b0_pred | 8730ns | no significant difference | [-64, +1287]ns | [8064, 9019] | no | 0.4375 | 0.2188 | 0 |
| an_b0_prof | 7941ns | no significant difference | [-642, +208]ns | [7282, 8704] | no | 0.6875 | 0.6875 | 0 |
| an_b0_seq | 8442ns | no significant difference | [-175, +715]ns | [7834, 8765] | no | 0.4375 | 0.2188 | 0 |
| an_b0_tree | 8685ns | no significant difference | [-536, +1042]ns | [7427, 8872] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b0_table | an_b0_pred | an_b0_prof | an_b0_seq | an_b0_tree |
|---|---|---|---|---|---|
| 1 | 7229ns | +26.7% | +0.4% | +13.0% | +22.7% |
| 2 | 8649ns | -2.1% | +0.0% | +0.4% | +2.6% |
| 3 | 8655ns | +0.6% | +1.2% | +1.8% | +0.4% |
| 4 | 8235ns | +7.8% | +3.8% | +5.9% | +5.4% |
| 5 | 8585ns | +1.9% | -14.6% | -4.5% | -11.1% |
| 6 | 7342ns | +4.3% | -0.5% | +2.1% | -1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b0_pred | -0.121 | ok |
| an_b0_prof | 0.199 | ok |
| an_b0_seq | 0.272 | moderate+ |
| an_b0_table | -0.195 | ok |
| an_b0_tree | 0.449 | moderate+ |

**Consistency summary:**

- **an_b0_pred**: won 1/6, lost 5/6
- **an_b0_prof**: won 2/6, lost 3/6
- **an_b0_seq**: won 1/6, lost 5/6
- **an_b0_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b0_pred | 5.0ns | 8604.4ns | 0.1% |  |
| an_b0_prof | 5.8ns | 7975.9ns | 0.1% |  |
| an_b0_seq | 6.2ns | 8347.3ns | 0.1% |  |
| an_b0_table | 5.2ns | 8115.8ns | 0.1% |  |
| an_b0_tree | 6.0ns | 8328.0ns | 0.1% |  |

## Distribution (algo ns)

```
an_b0_pred (n=6, range 7659.6-9018.8 ns)
   7659.6 |########################################
   7727.6 |
   7795.5 |
   7863.5 |
   7931.4 |
   7999.4 |
   8067.3 |
   8135.3 |
   8203.3 |
   8271.2 |
   8339.2 |
   8407.1 |########################################
   8475.1 |
   8543.0 |
   8611.0 |
   8679.0 |########################################
   8746.9 |########################################
   8814.9 |########################################
   8882.8 |
   8950.8 |
  (0 below, 1 above range)

an_b0_prof (n=6, range 7255.8-8704.4 ns)
   7255.8 |########################################
   7328.2 |####################
   7400.7 |
   7473.1 |
   7545.5 |
   7617.9 |
   7690.4 |
   7762.8 |
   7835.2 |
   7907.6 |
   7980.1 |
   8052.5 |
   8124.9 |
   8197.4 |
   8269.8 |
   8342.2 |
   8414.6 |
   8487.1 |####################
   8559.5 |
   8631.9 |####################
  (0 below, 1 above range)

an_b0_seq (n=6, range 7496.2-8765.2 ns)
   7496.2 |########################################
   7559.6 |
   7623.1 |
   7686.6 |
   7750.0 |
   7813.4 |
   7876.9 |
   7940.4 |
   8003.8 |
   8067.2 |
   8130.7 |########################################
   8194.1 |########################################
   8257.6 |
   8321.1 |
   8384.5 |
   8448.0 |
   8511.4 |
   8574.9 |
   8638.3 |########################################
   8701.8 |########################################
  (0 below, 1 above range)

an_b0_table (n=6, range 7229.2-8652.3 ns)
   7229.2 |####################
   7300.4 |####################
   7371.5 |
   7442.7 |
   7513.8 |
   7585.0 |
   7656.1 |
   7727.3 |
   7798.4 |
   7869.6 |
   7940.8 |
   8011.9 |
   8083.1 |
   8154.2 |
   8225.4 |####################
   8296.5 |
   8367.7 |
   8438.8 |
   8510.0 |
   8581.1 |########################################
  (0 below, 1 above range)

an_b0_tree (n=6, range 7224.6-8872.5 ns)
   7224.6 |####################
   7307.0 |
   7389.4 |
   7471.8 |
   7554.2 |####################
   7636.6 |
   7719.0 |
   7801.4 |
   7883.8 |
   7966.2 |
   8048.6 |
   8130.9 |
   8213.3 |
   8295.7 |
   8378.1 |
   8460.5 |
   8542.9 |
   8625.3 |########################################
   8707.7 |
   8790.1 |####################
  (0 below, 1 above range)

```
