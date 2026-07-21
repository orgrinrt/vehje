# Branch strategies, heavy-arm, pred95: ~95% taken, predictable (b<243)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_pred95**

## Key findings

- **Fastest: br_profiled_hot_h_pred95** at 11897.7 ns median (-1.5% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.64x (fastest 11897.7 ns, slowest 19555.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_pred95 | 14410ns | 14670ns | 12840ns | 14625ns | 14874ns | base |
| br_predicate_h_pred95 | 22659ns | 22175ns | 20705ns | 22165ns | 24376ns | +57.24% |
| br_profiled_hot_h_pred95 | 14370ns | 14324ns | 12679ns | 14319ns | 15291ns | -0.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_pred95 | 11860ns | 10585ns | 12219ns | base | 0.086 |
| br_predicate_h_pred95 | 20041ns | 18280ns | 21656ns | +68.98% | 0.051 |
| br_profiled_hot_h_pred95 | 11927ns | 10497ns | 12691ns | +0.57% | 0.086 |

## Performance model

- Peak throughput: **0.098 Gops/s** (br_profiled_hot_h_pred95; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_pred95 | 0.085 | 86.9% |
| br_predicate_h_pred95 | 0.052 | 53.7% |
| br_profiled_hot_h_pred95 | 0.086 | 88.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_pred95 | 14410ns | 14410ns | base |
| br_predicate_h_pred95 | 22659ns | 22659ns | +57.24% |
| br_profiled_hot_h_pred95 | 14370ns | 14370ns | -0.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_pred95 | 12082ns | base | --- | [11279, 12219] | --- | --- | --- | --- |
| br_predicate_h_pred95 | 19555ns | +7531.9ns (+62.3%) | [+6768, +10242]ns | [18912, 21656] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| br_profiled_hot_h_pred95 | 11898ns | no significant difference | [-950, +1025]ns | [11194, 12691] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_pred95 | br_predicate_h_pred95 | br_profiled_hot_h_pred95 |
|---|---|---|---|
| 1 | 12031ns | +51.9% | -12.7% |
| 2 | 12182ns | +94.5% | +4.1% |
| 3 | 11972ns | +63.8% | +6.1% |
| 4 | 12256ns | +59.5% | -3.0% |
| 5 | 12133ns | +61.2% | -2.0% |
| 6 | 10585ns | +84.7% | +12.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_pred95 | -0.052 | ok |
| br_predicate_h_pred95 | -0.421 | moderate- |
| br_profiled_hot_h_pred95 | -0.160 | ok |

**Consistency summary:**

- **br_predicate_h_pred95**: won 0/6, lost 6/6
- **br_profiled_hot_h_pred95**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_pred95 | 3.8ns | 11859.9ns | 0.0% |  |
| br_predicate_h_pred95 | 11.9ns | 20040.8ns | 0.1% |  |
| br_profiled_hot_h_pred95 | 3.0ns | 11927.4ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_pred95 (n=6, range 10585.4-12219.0 ns)
  10585.4 |########################################
  10667.1 |
  10748.8 |
  10830.4 |
  10912.1 |
  10993.8 |
  11075.5 |
  11157.1 |
  11238.8 |
  11320.5 |
  11402.2 |
  11483.9 |
  11565.5 |
  11647.2 |
  11728.9 |
  11810.6 |
  11892.2 |########################################
  11973.9 |########################################
  12055.6 |########################################
  12137.3 |########################################
  (0 below, 1 above range)

br_predicate_h_pred95 (n=6, range 18280.4-21655.6 ns)
  18280.4 |##########
  18449.2 |
  18617.9 |
  18786.7 |
  18955.4 |
  19124.2 |
  19293.0 |
  19461.7 |########################################
  19630.5 |
  19799.2 |
  19968.0 |
  20136.8 |
  20305.5 |
  20474.3 |
  20643.0 |
  20811.8 |
  20980.6 |
  21149.3 |
  21318.1 |
  21486.8 |
  (0 below, 1 above range)

br_profiled_hot_h_pred95 (n=6, range 10497.1-12690.8 ns)
  10497.1 |#############
  10606.8 |
  10716.5 |
  10826.2 |
  10935.8 |
  11045.5 |
  11155.2 |
  11264.9 |
  11374.6 |
  11484.3 |
  11594.0 |
  11703.6 |
  11813.3 |########################################
  11923.0 |
  12032.7 |
  12142.4 |
  12252.1 |
  12361.7 |
  12471.4 |
  12581.1 |#############
  (0 below, 1 above range)

```
