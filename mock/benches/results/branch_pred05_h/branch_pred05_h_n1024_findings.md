# Branch strategies, heavy-arm, pred05: ~5% taken, predictable (b<13)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_pred05**

## Key findings

- **Baseline (br_branch_h_pred05) is the fastest** at 7595.6 ns median
- 1 variant significantly slower than baseline
- Spread: 2.54x (fastest 7595.6 ns, slowest 19311.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_pred05 | 9938ns | 10199ns | 8632ns | 10178ns | 10231ns | base |
| br_predicate_h_pred05 | 21549ns | 21892ns | 19270ns | 21535ns | 22709ns | +116.82% |
| br_profiled_hot_h_pred05 | 10133ns | 10401ns | 9200ns | 10225ns | 10460ns | +1.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_pred05 | 7412ns | 6452ns | 7639ns | base | 0.138 |
| br_predicate_h_pred05 | 19024ns | 17011ns | 20064ns | +156.67% | 0.054 |
| br_profiled_hot_h_pred05 | 7625ns | 6943ns | 7868ns | +2.88% | 0.134 |

## Performance model

- Peak throughput: **0.159 Gops/s** (br_branch_h_pred05; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_pred05 | 0.135 | 84.9% |
| br_predicate_h_pred05 | 0.053 | 33.4% |
| br_profiled_hot_h_pred05 | 0.131 | 82.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_pred05 | 9938ns | 9938ns | base |
| br_predicate_h_pred05 | 21549ns | 21549ns | +116.82% |
| br_profiled_hot_h_pred05 | 10133ns | 10133ns | +1.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_pred05 | 7596ns | base | --- | [7001, 7639] | --- | --- | --- | --- |
| br_predicate_h_pred05 | 19311ns | +11717.1ns (+154.3%) | [+10652, +12468]ns | [17697, 20064] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| br_profiled_hot_h_pred05 | 7817ns | no significant difference | [-3, +380]ns | [7191, 7868] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_pred05 | br_predicate_h_pred05 | br_profiled_hot_h_pred05 |
|---|---|---|---|
| 1 | 6452ns | +163.7% | +7.6% |
| 2 | 7639ns | +140.6% | +2.5% |
| 3 | 7638ns | +155.3% | -2.6% |
| 4 | 7639ns | +155.8% | +3.5% |
| 5 | 7553ns | +172.5% | +3.5% |
| 6 | 7550ns | +153.3% | +3.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_pred05 | -0.057 | ok |
| br_predicate_h_pred05 | 0.296 | moderate+ |
| br_profiled_hot_h_pred05 | -0.204 | moderate- |

**Consistency summary:**

- **br_predicate_h_pred05**: won 0/6, lost 6/6
- **br_profiled_hot_h_pred05**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_pred05 | 4.1ns | 7411.9ns | 0.1% |  |
| br_predicate_h_pred05 | 3.0ns | 19024.3ns | 0.0% |  |
| br_profiled_hot_h_pred05 | 3.5ns | 7625.3ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_pred05 (n=6, range 6451.7-7639.2 ns)
   6451.7 |####################
   6511.1 |
   6570.4 |
   6629.8 |
   6689.2 |
   6748.6 |
   6807.9 |
   6867.3 |
   6926.7 |
   6986.1 |
   7045.4 |
   7104.8 |
   7164.2 |
   7223.6 |
   7282.9 |
   7342.3 |
   7401.7 |
   7461.1 |
   7520.4 |########################################
   7579.8 |####################
  (0 below, 2 above range)

br_predicate_h_pred05 (n=6, range 17011.2-20064.4 ns)
  17011.2 |####################
  17163.9 |
  17316.5 |
  17469.2 |
  17621.8 |
  17774.5 |
  17927.2 |
  18079.8 |
  18232.5 |####################
  18385.1 |
  18537.8 |
  18690.5 |
  18843.1 |
  18995.8 |####################
  19148.4 |
  19301.1 |
  19453.8 |########################################
  19606.4 |
  19759.1 |
  19911.7 |
  (0 below, 1 above range)

br_profiled_hot_h_pred05 (n=6, range 6943.3-7867.7 ns)
   6943.3 |####################
   6989.5 |
   7035.7 |
   7082.0 |
   7128.2 |
   7174.4 |
   7220.6 |
   7266.8 |
   7313.1 |
   7359.3 |
   7405.5 |####################
   7451.7 |
   7497.9 |
   7544.2 |
   7590.4 |
   7636.6 |
   7682.8 |
   7729.0 |
   7775.3 |########################################
   7821.5 |####################
  (0 below, 1 above range)

```
