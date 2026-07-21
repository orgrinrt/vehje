# Branch strategies, heavy-arm, pred95: ~95% taken, predictable (b<243)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_pred95**

## Key findings

- **Baseline (br_branch_h_pred95) is the fastest** at 162919.0 ns median
- 2 variants significantly slower than baseline
- Spread: 1.61x (fastest 162919.0 ns, slowest 262983.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_pred95 | 165055ns | 165151ns | 162922ns | 164549ns | 166880ns | base |
| br_predicate_h_pred95 | 265894ns | 265461ns | 261575ns | 264628ns | 269952ns | +61.09% |
| br_profiled_hot_h_pred95 | 174615ns | 172917ns | 171150ns | 172504ns | 179515ns | +5.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_pred95 | 162705ns | 160554ns | 164425ns | base | 0.101 |
| br_predicate_h_pred95 | 263458ns | 258948ns | 267585ns | +61.92% | 0.062 |
| br_profiled_hot_h_pred95 | 172154ns | 168725ns | 177110ns | +5.81% | 0.095 |

## Performance model

- Peak throughput: **0.102 Gops/s** (br_branch_h_pred95; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_pred95 | 0.101 | 98.5% |
| br_predicate_h_pred95 | 0.062 | 61.1% |
| br_profiled_hot_h_pred95 | 0.096 | 94.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_pred95 | 165055ns | 165055ns | base |
| br_predicate_h_pred95 | 265894ns | 265894ns | +61.09% |
| br_profiled_hot_h_pred95 | 174615ns | 174615ns | +5.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_pred95 | 162919ns | base | --- | [160771, 164425] | --- | --- | --- | --- |
| br_predicate_h_pred95 | 262983ns | +100636.4ns (+61.8%) | [+97373, +104251]ns | [259807, 267585] | YES | 0.0313 | 0.0313 | 0 |
| br_profiled_hot_h_pred95 | 170292ns | +9403.8ns (+5.8%) | [+5953, +12990]ns | [169058, 177110] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_pred95 | br_predicate_h_pred95 | br_profiled_hot_h_pred95 |
|---|---|---|---|
| 1 | 163705ns | +61.8% | +6.5% |
| 2 | 160988ns | +62.2% | +6.2% |
| 3 | 164314ns | +58.6% | +2.7% |
| 4 | 162132ns | +65.8% | +4.6% |
| 5 | 160554ns | +61.3% | +5.5% |
| 6 | 164536ns | +61.9% | +9.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_pred95 | -0.546 | HIGH- (thermal bounce) |
| br_predicate_h_pred95 | -0.673 | HIGH- (thermal bounce) |
| br_profiled_hot_h_pred95 | -0.046 | ok |

**Consistency summary:**

- **br_predicate_h_pred95**: won 0/6, lost 6/6
- **br_profiled_hot_h_pred95**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_pred95 | 4.0ns | 162704.9ns | 0.0% |  |
| br_predicate_h_pred95 | 6.9ns | 263458.5ns | 0.0% |  |
| br_profiled_hot_h_pred95 | 5.3ns | 172153.6ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_pred95 (n=6, range 160553.8-164424.8 ns)
  160553.8 |########################################
  160747.3 |
  160940.9 |########################################
  161134.4 |
  161328.0 |
  161521.5 |
  161715.1 |
  161908.6 |
  162102.2 |########################################
  162295.8 |
  162489.3 |
  162682.8 |
  162876.4 |
  163069.9 |
  163263.5 |
  163457.0 |
  163650.6 |########################################
  163844.1 |
  164037.7 |
  164231.2 |########################################
  (0 below, 1 above range)

br_predicate_h_pred95 (n=6, range 258947.9-267585.0 ns)
  258947.9 |########################################
  259379.8 |
  259811.6 |
  260243.5 |########################################
  260675.3 |########################################
  261107.2 |
  261539.0 |
  261970.9 |
  262402.7 |
  262834.6 |
  263266.5 |
  263698.3 |
  264130.2 |
  264562.0 |########################################
  264993.9 |
  265425.7 |
  265857.6 |
  266289.4 |########################################
  266721.3 |
  267153.1 |
  (0 below, 1 above range)

br_profiled_hot_h_pred95 (n=6, range 168724.6-177110.2 ns)
  168724.6 |########################################
  169143.9 |########################################
  169563.2 |########################################
  169982.4 |
  170401.7 |
  170821.0 |########################################
  171240.3 |
  171659.6 |
  172078.8 |
  172498.1 |
  172917.4 |
  173336.7 |
  173756.0 |
  174175.2 |########################################
  174594.5 |
  175013.8 |
  175433.1 |
  175852.4 |
  176271.6 |
  176690.9 |
  (0 below, 1 above range)

```
