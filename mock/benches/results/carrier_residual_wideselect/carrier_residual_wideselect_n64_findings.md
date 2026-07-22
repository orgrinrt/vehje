# Residual encoding: register/SSA vs stack bytecode, wideselect profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_wideselect_register**

## Highlights

Baseline for all deltas below: **carrier_res_wideselect_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_wideselect_register dominates: 156% faster than the next best (carrier_res_wideselect_stack)

carrier_res_wideselect_register (2.18 us) leads carrier_res_wideselect_stack (5.57 us) by 156%, a clear separation rather than a photo finish. CV 6.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_wideselect_register is fastest but the noisiest (CV 6.9%)

carrier_res_wideselect_register wins on median (2.18 us) yet has the highest variance (CV 6.9%), while carrier_res_wideselect_stack is the steadiest (CV 5.3%, 5.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (carrier_res_wideselect_register)

The baseline carrier_res_wideselect_register is the fastest (2.18 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_wideselect_register) is the fastest** at 2179.6 ns median
- 1 variant significantly slower than baseline
- Spread: 2.56x (fastest 2179.6 ns, slowest 5572.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 4572ns | 4454ns | 4292ns | 4409ns | 4957ns | base |
| carrier_res_wideselect_stack | 8064ns | 7956ns | 7570ns | 7876ns | 8593ns | +76.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_wideselect_register | 2236ns | 2096ns | 2425ns | base | 0.029 |
| carrier_res_wideselect_stack | 5682ns | 5358ns | 6065ns | +154.12% | 0.011 |

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_res_wideselect_register; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_wideselect_register | 0.029 | 96.2% |
| carrier_res_wideselect_stack | 0.011 | 37.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_wideselect_register | 4572ns | 4572ns | base |
| carrier_res_wideselect_stack | 8064ns | 8064ns | +76.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 2180ns | base | --- | [2103, 2425] | --- | --- | --- | --- |
| carrier_res_wideselect_stack | 5573ns | +3441.2ns (+157.9%) | [+3256, +3640]ns | [5407, 6065] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_wideselect_register | carrier_res_wideselect_stack |
|---|---|---|
| 1 | 2524ns | +145.9% |
| 2 | 2096ns | +155.6% |
| 3 | 2154ns | +155.4% |
| 4 | 2326ns | +154.6% |
| 5 | 2110ns | +167.6% |
| 6 | 2205ns | +147.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_wideselect_register | -0.326 | moderate- |
| carrier_res_wideselect_stack | -0.297 | moderate- |

**Consistency summary:**

- **carrier_res_wideselect_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_wideselect_register | 86227.8ns | 2235.8ns | 3856.6% | HIGH |
| carrier_res_wideselect_stack | 83146.4ns | 5681.6ns | 1463.4% | HIGH |

## Distribution (algo ns)

```
carrier_res_wideselect_register (n=6, range 2096.2-2425.0 ns)
   2096.2 |########################################
   2112.6 |
   2129.1 |
   2145.5 |####################
   2162.0 |
   2178.4 |
   2194.8 |####################
   2211.3 |
   2227.7 |
   2244.2 |
   2260.6 |
   2277.0 |
   2293.5 |
   2309.9 |####################
   2326.4 |
   2342.8 |
   2359.2 |
   2375.7 |
   2392.1 |
   2408.6 |
  (0 below, 1 above range)

carrier_res_wideselect_stack (n=6, range 5357.9-6064.6 ns)
   5357.9 |########################################
   5393.2 |
   5428.6 |########################################
   5463.9 |
   5499.2 |########################################
   5534.6 |
   5569.9 |
   5605.2 |
   5640.6 |########################################
   5675.9 |
   5711.2 |
   5746.6 |
   5781.9 |
   5817.3 |
   5852.6 |
   5887.9 |########################################
   5923.3 |
   5958.6 |
   5993.9 |
   6029.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_wideselect_register**: bridge=3956.4% of algo (FFI overhead may distort results)
- **carrier_res_wideselect_stack**: bridge=1495.7% of algo (FFI overhead may distort results)
