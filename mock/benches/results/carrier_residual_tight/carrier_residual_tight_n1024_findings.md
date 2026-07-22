# Residual encoding: register/SSA vs stack bytecode, tight profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_tight_register**

## Highlights

Baseline for all deltas below: **carrier_res_tight_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_tight_register dominates: 127% faster than the next best (carrier_res_tight_stack)

carrier_res_tight_register (42.48 us) leads carrier_res_tight_stack (96.24 us) by 127%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_tight_register)

The baseline carrier_res_tight_register is the fastest (42.48 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_tight_register) is the fastest** at 42479.8 ns median
- 1 variant significantly slower than baseline
- Spread: 2.27x (fastest 42479.8 ns, slowest 96235.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_tight_register | 45413ns | 44650ns | 44397ns | 44582ns | 47166ns | base |
| carrier_res_tight_stack | 97832ns | 98675ns | 94153ns | 97251ns | 100544ns | +115.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_tight_register | 43153ns | 42193ns | 44785ns | base | 0.024 |
| carrier_res_tight_stack | 95519ns | 91990ns | 98207ns | +121.35% | 0.011 |

## Performance model

- Peak throughput: **0.024 Gops/s** (carrier_res_tight_register; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_tight_register | 0.024 | 99.3% |
| carrier_res_tight_stack | 0.011 | 43.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_tight_register | 45413ns | 45413ns | base |
| carrier_res_tight_stack | 97832ns | 97832ns | +115.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_tight_register | 42480ns | base | --- | [42194, 44785] | --- | --- | --- | --- |
| carrier_res_tight_stack | 96235ns | +52536.2ns (+123.7%) | [+49834, +54727]ns | [92114, 98207] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_tight_register | carrier_res_tight_stack |
|---|---|---|
| 1 | 45205ns | +114.7% |
| 2 | 42364ns | +117.1% |
| 3 | 42595ns | +130.1% |
| 4 | 44365ns | +121.8% |
| 5 | 42195ns | +118.6% |
| 6 | 42193ns | +126.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_tight_register | -0.248 | moderate- |
| carrier_res_tight_stack | -0.403 | moderate- |

**Consistency summary:**

- **carrier_res_tight_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_tight_register | 110327.8ns | 43152.8ns | 255.7% | HIGH |
| carrier_res_tight_stack | 95525.6ns | 95518.5ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_res_tight_register (n=6, range 42193.3-44784.8 ns)
  42193.3 |########################################
  42322.9 |####################
  42452.5 |
  42582.0 |####################
  42711.6 |
  42841.2 |
  42970.8 |
  43100.3 |
  43229.9 |
  43359.5 |
  43489.1 |
  43618.6 |
  43748.2 |
  43877.8 |
  44007.4 |
  44136.9 |
  44266.5 |####################
  44396.1 |
  44525.7 |
  44655.2 |
  (0 below, 1 above range)

carrier_res_tight_stack (n=6, range 91990.0-98206.9 ns)
  91990.0 |########################################
  92300.8 |
  92611.7 |
  92922.5 |
  93233.4 |
  93544.2 |
  93855.1 |
  94165.9 |
  94476.8 |
  94787.6 |
  95098.4 |
  95409.3 |####################
  95720.1 |
  96031.0 |
  96341.8 |
  96652.7 |
  96963.5 |####################
  97274.4 |
  97585.2 |
  97896.1 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_tight_register**: bridge=269.1% of algo (FFI overhead may distort results)
- **carrier_res_tight_stack**: bridge=99.9% of algo (FFI overhead may distort results)
