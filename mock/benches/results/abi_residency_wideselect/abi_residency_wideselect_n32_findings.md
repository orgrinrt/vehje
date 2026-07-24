# abi_residency (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_residency_wideselect_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_wideselect_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_wideselect_null_entry dominates: 86946% faster than the next best (abi_residency_wideselect_reused_buffer)

abi_residency_wideselect_null_entry (2.36 us) leads abi_residency_wideselect_reused_buffer (2.05 ms) by 86946%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_wideselect_null_entry beats baseline by 100% (significant)

abi_residency_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_residency_wideselect_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_wideselect_fresh_alloc is an outlier: 870.6x slower than the field

abi_residency_wideselect_fresh_alloc (2.05 ms) is 870.6x the fastest (2.36 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 870.6x the fastest

Fastest abi_residency_wideselect_null_entry (2.36 us) to slowest abi_residency_wideselect_fresh_alloc (2.05 ms): 870.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_wideselect_null_entry** at 2356.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 870.59x (fastest 2356.4 ns, slowest 2051500.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2055129ns | 2054025ns | 2049892ns | 2053710ns | 2059877ns | +0.17% |
| abi_residency_wideselect_null_entry | 4705ns | 4708ns | 4542ns | 4684ns | 4817ns | -99.77% |
| abi_residency_wideselect_reused_buffer | 2051613ns | 2053773ns | 2037571ns | 2051455ns | 2058872ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2052521ns | 2047258ns | 2057247ns | +0.17% | 0.000 |
| abi_residency_wideselect_null_entry | 2354ns | 2267ns | 2415ns | -99.89% | 0.014 |
| abi_residency_wideselect_reused_buffer | 2049052ns | 2035166ns | 2056249ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 42612.3 | 2050699.5 | 2052520.7 | n/a |
| abi_residency_wideselect_null_entry | 29630.3 | 2486.7 | 2354.3 | n/a |
| abi_residency_wideselect_reused_buffer | 40618.9 | 2049095.1 | 2049052.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_residency_wideselect_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | 0.000 | 0.1% |
| abi_residency_wideselect_null_entry | 0.014 | 96.2% |
| abi_residency_wideselect_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2055129ns | 2055129ns | +0.17% |
| abi_residency_wideselect_null_entry | 4705ns | 4705ns | -99.77% |
| abi_residency_wideselect_reused_buffer | 2051613ns | 2051613ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_wideselect_reused_buffer | 2051188ns | base | --- | [2039721, 2056249] | --- | --- | --- | --- |
| abi_residency_wideselect_fresh_alloc | 2051500ns | no significant difference | [-2415, +10202]ns | [2048815, 2057247] | no | 1.0000 | 1.0000 | 0 |
| abi_residency_wideselect_null_entry | 2356ns | -2048878.9ns (-99.9%) | [-2053909, -2037306]ns | [2292, 2415] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_wideselect_reused_buffer | abi_residency_wideselect_fresh_alloc | abi_residency_wideselect_null_entry |
|---|---|---|---|
| 1 | 2049306ns | +0.4% | -99.9% |
| 2 | 2053069ns | -0.0% | -99.9% |
| 3 | 2059313ns | -0.1% | -99.9% |
| 4 | 2053184ns | -0.1% | -99.9% |
| 5 | 2044276ns | +0.3% | -99.9% |
| 6 | 2035166ns | +0.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | 0.057 | ok |
| abi_residency_wideselect_null_entry | 0.048 | ok |
| abi_residency_wideselect_reused_buffer | 0.370 | moderate+ |

**Consistency summary:**

- **abi_residency_wideselect_fresh_alloc**: won 2/6, lost 3/6
- **abi_residency_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 6198953.6ns | 2052520.7ns | 302.0% | HIGH |
| abi_residency_wideselect_null_entry | 118358.2ns | 2354.3ns | 5027.2% | HIGH |
| abi_residency_wideselect_reused_buffer | 6193680.6ns | 2049052.5ns | 302.3% | HIGH |

## Distribution (algo ns)

```
abi_residency_wideselect_fresh_alloc (n=6, range 2047257.9-2057247.1 ns)
  2047257.9 |########################################
  2047757.4 |
  2048256.8 |
  2048756.3 |
  2049255.7 |
  2049755.2 |
  2050254.6 |########################################
  2050754.1 |########################################
  2051253.6 |
  2051753.0 |########################################
  2052252.5 |
  2052751.9 |
  2053251.4 |
  2053750.8 |
  2054250.3 |
  2054749.8 |
  2055249.2 |
  2055748.7 |
  2056248.1 |
  2056747.6 |########################################
  (0 below, 1 above range)

abi_residency_wideselect_null_entry (n=6, range 2267.1-2414.9 ns)
   2267.1 |########################################
   2274.5 |
   2281.9 |
   2289.3 |
   2296.7 |
   2304.1 |
   2311.5 |########################################
   2318.8 |
   2326.2 |
   2333.6 |
   2341.0 |
   2348.4 |########################################
   2355.8 |########################################
   2363.2 |
   2370.6 |
   2378.0 |
   2385.4 |
   2392.8 |
   2400.2 |########################################
   2407.6 |
  (0 below, 1 above range)

abi_residency_wideselect_reused_buffer (n=6, range 2035165.8-2056248.8 ns)
  2035165.8 |########################################
  2036219.9 |
  2037274.1 |
  2038328.2 |
  2039382.4 |
  2040436.5 |
  2041490.7 |
  2042544.8 |
  2043599.0 |########################################
  2044653.1 |
  2045707.3 |
  2046761.4 |
  2047815.6 |
  2048869.7 |########################################
  2049923.9 |
  2050978.0 |
  2052032.2 |########################################
  2053086.3 |########################################
  2054140.5 |
  2055194.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_wideselect_fresh_alloc**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_null_entry**: bridge=5001.0% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_reused_buffer**: bridge=302.3% of algo (FFI overhead may distort results)
