# Branch strategies, heavy-arm, alt: strict alternation (i&1)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_alt**

## Key findings

- **Baseline (br_branch_h_alt) is the fastest** at 620.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.94x (fastest 620.0 ns, slowest 1203.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_alt | 3131ns | 3123ns | 3009ns | 3090ns | 3255ns | base |
| br_predicate_h_alt | 3671ns | 3725ns | 3185ns | 3681ns | 3899ns | +17.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_alt | 619ns | 595ns | 641ns | base | 0.103 |
| br_predicate_h_alt | 1182ns | 1035ns | 1244ns | +91.00% | 0.054 |

## Performance model

- Peak throughput: **0.107 Gops/s** (br_branch_h_alt; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_alt | 0.103 | 96.0% |
| br_predicate_h_alt | 0.053 | 49.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_alt | 3131ns | 3131ns | base |
| br_predicate_h_alt | 3671ns | 3671ns | +17.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_alt | 620ns | base | --- | [595, 641] | --- | --- | --- | --- |
| br_predicate_h_alt | 1203ns | +582.9ns (+94.0%) | [+482, +625]ns | [1099, 1244] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_alt | br_predicate_h_alt |
|---|---|---|
| 1 | 595ns | +73.8% |
| 2 | 639ns | +82.1% |
| 3 | 601ns | +94.1% |
| 4 | 595ns | +108.9% |
| 5 | 640ns | +93.8% |
| 6 | 642ns | +93.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_alt | -0.144 | ok |
| br_predicate_h_alt | 0.277 | moderate+ |

**Consistency summary:**

- **br_predicate_h_alt**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_alt | 4.2ns | 618.8ns | 0.7% |  |
| br_predicate_h_alt | 3.3ns | 1182.0ns | 0.3% |  |

## Distribution (algo ns)

```
br_branch_h_alt (n=6, range 595.4-641.0 ns)
    595.4 |########################################
    597.7 |
    600.0 |####################
    602.2 |
    604.5 |
    606.8 |
    609.1 |
    611.4 |
    613.7 |
    615.9 |
    618.2 |
    620.5 |
    622.8 |
    625.1 |
    627.4 |
    629.6 |
    631.9 |
    634.2 |
    636.5 |
    638.8 |########################################
  (0 below, 1 above range)

br_predicate_h_alt (n=6, range 1034.6-1243.5 ns)
   1034.6 |####################
   1045.0 |
   1055.5 |
   1065.9 |
   1076.4 |
   1086.8 |
   1097.3 |
   1107.7 |
   1118.2 |
   1128.6 |
   1139.1 |
   1149.5 |
   1160.0 |########################################
   1170.4 |
   1180.9 |
   1191.3 |
   1201.8 |
   1212.2 |
   1222.7 |
   1233.1 |########################################
  (0 below, 1 above range)

```
