# Residual encoding: register/SSA vs stack bytecode, leaf profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_leaf_register**

## Highlights

Baseline for all deltas below: **carrier_res_leaf_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_leaf_register dominates: 55% faster than the next best (carrier_res_leaf_stack)

carrier_res_leaf_register (44.46 us) leads carrier_res_leaf_stack (68.71 us) by 55%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_leaf_register)

The baseline carrier_res_leaf_register is the fastest (44.46 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_leaf_register) is the fastest** at 44463.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.55x (fastest 44463.1 ns, slowest 68707.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 47654ns | 47055ns | 45700ns | 46667ns | 50112ns | base |
| carrier_res_leaf_stack | 77022ns | 71159ns | 68320ns | 70415ns | 91284ns | +61.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_leaf_register | 45120ns | 43365ns | 47441ns | base | 0.023 |
| carrier_res_leaf_stack | 69844ns | 66052ns | 74468ns | +54.80% | 0.015 |

## Performance model

- Peak throughput: **0.024 Gops/s** (carrier_res_leaf_register; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_leaf_register | 0.023 | 97.5% |
| carrier_res_leaf_stack | 0.015 | 63.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_leaf_register | 47654ns | 47654ns | base |
| carrier_res_leaf_stack | 77022ns | 77022ns | +61.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 44463ns | base | --- | [43456, 47441] | --- | --- | --- | --- |
| carrier_res_leaf_stack | 68707ns | +24244.2ns (+54.5%) | [+22900, +27027]ns | [66356, 74468] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_leaf_register | carrier_res_leaf_stack |
|---|---|---|
| 1 | 47119ns | +59.6% |
| 2 | 43548ns | +51.7% |
| 3 | 43365ns | +53.7% |
| 4 | 47762ns | +54.4% |
| 5 | 44848ns | +52.2% |
| 6 | 44078ns | +56.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_leaf_register | -0.308 | moderate- |
| carrier_res_leaf_stack | -0.360 | moderate- |

**Consistency summary:**

- **carrier_res_leaf_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_leaf_register | 107777.1ns | 45120.1ns | 238.9% | HIGH |
| carrier_res_leaf_stack | 138474.6ns | 69843.6ns | 198.3% | HIGH |

## Distribution (algo ns)

```
carrier_res_leaf_register (n=6, range 43364.6-47440.6 ns)
  43364.6 |########################################
  43568.4 |
  43772.2 |
  43976.0 |####################
  44179.8 |
  44383.6 |
  44587.4 |
  44791.2 |####################
  44995.0 |
  45198.8 |
  45402.6 |
  45606.4 |
  45810.2 |
  46014.0 |
  46217.8 |
  46421.6 |
  46625.4 |
  46829.2 |
  47033.0 |####################
  47236.8 |
  (0 below, 1 above range)

carrier_res_leaf_stack (n=6, range 66052.5-74467.5 ns)
  66052.5 |########################################
  66473.2 |########################################
  66894.0 |
  67314.8 |
  67735.5 |
  68156.2 |########################################
  68577.0 |
  68997.8 |########################################
  69418.5 |
  69839.2 |
  70260.0 |
  70680.8 |
  71101.5 |
  71522.2 |
  71943.0 |
  72363.8 |
  72784.5 |
  73205.2 |
  73626.0 |########################################
  74046.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_leaf_register**: bridge=243.9% of algo (FFI overhead may distort results)
- **carrier_res_leaf_stack**: bridge=199.8% of algo (FFI overhead may distort results)
