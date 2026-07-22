# Residual encoding: register/SSA vs stack bytecode, scatter profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_scatter_register**

## Highlights

Baseline for all deltas below: **carrier_res_scatter_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_scatter_register dominates: 205% faster than the next best (carrier_res_scatter_stack)

carrier_res_scatter_register (2.33 us) leads carrier_res_scatter_stack (7.09 us) by 205%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_scatter_register)

The baseline carrier_res_scatter_register is the fastest (2.33 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.0x the fastest

Fastest carrier_res_scatter_register (2.33 us) to slowest carrier_res_scatter_stack (7.09 us): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_scatter_register) is the fastest** at 2325.0 ns median
- 1 variant significantly slower than baseline
- Spread: 3.05x (fastest 2325.0 ns, slowest 7088.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 4667ns | 4680ns | 4474ns | 4655ns | 4780ns | base |
| carrier_res_scatter_stack | 9435ns | 9502ns | 9147ns | 9393ns | 9642ns | +102.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_scatter_register | 2323ns | 2216ns | 2398ns | base | 0.028 |
| carrier_res_scatter_stack | 7084ns | 6879ns | 7266ns | +204.99% | 0.009 |

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_res_scatter_register; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_scatter_register | 0.028 | 95.3% |
| carrier_res_scatter_stack | 0.009 | 31.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_scatter_register | 4667ns | 4667ns | base |
| carrier_res_scatter_stack | 9435ns | 9435ns | +102.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 2325ns | base | --- | [2246, 2398] | --- | --- | --- | --- |
| carrier_res_scatter_stack | 7088ns | +4803.5ns (+206.6%) | [+4554, +4927]ns | [6899, 7266] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_scatter_register | carrier_res_scatter_stack |
|---|---|---|
| 1 | 2216ns | +214.1% |
| 2 | 2415ns | +184.8% |
| 3 | 2353ns | +206.7% |
| 4 | 2380ns | +207.2% |
| 5 | 2297ns | +214.3% |
| 6 | 2275ns | +204.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_scatter_register | -0.208 | moderate- |
| carrier_res_scatter_stack | 0.213 | moderate+ |

**Consistency summary:**

- **carrier_res_scatter_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_scatter_register | 86361.4ns | 2322.8ns | 3718.0% | HIGH |
| carrier_res_scatter_stack | 92137.8ns | 7084.3ns | 1300.6% | HIGH |

## Distribution (algo ns)

```
carrier_res_scatter_register (n=6, range 2216.2-2397.5 ns)
   2216.2 |########################################
   2225.3 |
   2234.3 |
   2243.4 |
   2252.5 |
   2261.5 |
   2270.6 |########################################
   2279.7 |
   2288.7 |########################################
   2297.8 |
   2306.8 |
   2315.9 |
   2325.0 |
   2334.0 |
   2343.1 |
   2352.2 |########################################
   2361.2 |
   2370.3 |
   2379.4 |########################################
   2388.4 |
  (0 below, 1 above range)

carrier_res_scatter_stack (n=6, range 6878.8-7265.8 ns)
   6878.8 |####################
   6898.2 |
   6917.5 |####################
   6936.9 |
   6956.2 |####################
   6975.6 |
   6994.9 |
   7014.2 |
   7033.6 |
   7052.9 |
   7072.3 |
   7091.6 |
   7111.0 |
   7130.3 |
   7149.7 |
   7169.0 |
   7188.4 |
   7207.7 |########################################
   7227.1 |
   7246.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_scatter_register**: bridge=3710.9% of algo (FFI overhead may distort results)
- **carrier_res_scatter_stack**: bridge=1295.4% of algo (FFI overhead may distort results)
