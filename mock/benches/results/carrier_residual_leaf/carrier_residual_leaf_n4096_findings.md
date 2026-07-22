# Residual encoding: register/SSA vs stack bytecode, leaf profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_leaf_register**

## Highlights

Baseline for all deltas below: **carrier_res_leaf_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_leaf_register dominates: 13% faster than the next best (carrier_res_leaf_stack)

carrier_res_leaf_register (223.52 us) leads carrier_res_leaf_stack (253.15 us) by 13%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_leaf_register)

The baseline carrier_res_leaf_register is the fastest (223.52 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_leaf_register) is the fastest** at 223524.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.13x (fastest 223524.0 ns, slowest 253148.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 225510ns | 225970ns | 217193ns | 223138ns | 233226ns | base |
| carrier_res_leaf_stack | 255659ns | 255421ns | 248839ns | 254987ns | 260078ns | +13.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_leaf_register | 223233ns | 215024ns | 231043ns | base | 0.018 |
| carrier_res_leaf_stack | 253414ns | 246648ns | 257815ns | +13.52% | 0.016 |

## Performance model

- Peak throughput: **0.019 Gops/s** (carrier_res_leaf_register; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_leaf_register | 0.018 | 96.2% |
| carrier_res_leaf_stack | 0.016 | 84.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_leaf_register | 225510ns | 225510ns | base |
| carrier_res_leaf_stack | 255659ns | 255659ns | +13.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 223524ns | base | --- | [215132, 231043] | --- | --- | --- | --- |
| carrier_res_leaf_stack | 253148ns | +31038.5ns (+13.9%) | [+24223, +35283]ns | [249280, 257815] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_leaf_register | carrier_res_leaf_stack |
|---|---|---|
| 1 | 228178ns | +11.1% |
| 2 | 230998ns | +13.2% |
| 3 | 218870ns | +15.1% |
| 4 | 231088ns | +10.0% |
| 5 | 215024ns | +14.7% |
| 6 | 215240ns | +17.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_leaf_register | -0.096 | ok |
| carrier_res_leaf_stack | -0.115 | ok |

**Consistency summary:**

- **carrier_res_leaf_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_leaf_register | 224072.5ns | 223233.0ns | 100.4% | HIGH |
| carrier_res_leaf_stack | 298001.4ns | 253414.4ns | 117.6% | HIGH |

## Distribution (algo ns)

```
carrier_res_leaf_register (n=6, range 215024.2-231042.7 ns)
  215024.2 |########################################
  215825.1 |
  216626.1 |
  217427.0 |
  218227.9 |####################
  219028.8 |
  219829.8 |
  220630.7 |
  221431.6 |
  222232.5 |
  223033.5 |
  223834.4 |
  224635.3 |
  225436.2 |
  226237.2 |
  227038.1 |
  227839.0 |####################
  228639.9 |
  229440.9 |
  230241.8 |####################
  (0 below, 1 above range)

carrier_res_leaf_stack (n=6, range 246648.3-257814.5 ns)
  246648.3 |########################################
  247206.6 |
  247764.9 |
  248323.2 |
  248881.5 |
  249439.9 |
  249998.2 |
  250556.5 |
  251114.8 |
  251673.1 |########################################
  252231.4 |########################################
  252789.7 |
  253348.0 |########################################
  253906.4 |########################################
  254464.7 |
  255023.0 |
  255581.3 |
  256139.6 |
  256697.9 |
  257256.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_leaf_register**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_res_leaf_stack**: bridge=117.8% of algo (FFI overhead may distort results)
