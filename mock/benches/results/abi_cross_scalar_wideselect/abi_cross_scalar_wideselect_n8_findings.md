# abi_cross_scalar (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_wideselect_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_wideselect_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_wideselect_null_entry dominates: 69159% faster than the next best (abi_cross_scalar_wideselect_inproc_direct)

abi_cross_scalar_wideselect_null_entry (3.01 us) leads abi_cross_scalar_wideselect_inproc_direct (2.08 ms) by 69159%, a clear separation rather than a photo finish. CV 0.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_wideselect_null_entry beats baseline by 100% (significant)

abi_cross_scalar_wideselect_null_entry is -2.08 ms (100%) faster than baseline abi_cross_scalar_wideselect_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_wideselect_inproc_fnptr is an outlier: 696.8x slower than the field

abi_cross_scalar_wideselect_inproc_fnptr (2.09 ms) is 696.8x the fastest (3.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_wideselect_null_entry} vs {abi_cross_scalar_wideselect_inproc_direct, abi_cross_scalar_wideselect_ffi_batched_scalar, abi_cross_scalar_wideselect_inproc_fnptr} (69159% apart)

The field splits into a fast tier {abi_cross_scalar_wideselect_null_entry} and a slow tier {abi_cross_scalar_wideselect_inproc_direct, abi_cross_scalar_wideselect_ffi_batched_scalar, abi_cross_scalar_wideselect_inproc_fnptr} with a 69159% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 696.8x the fastest

Fastest abi_cross_scalar_wideselect_null_entry (3.01 us) to slowest abi_cross_scalar_wideselect_inproc_fnptr (2.09 ms): 696.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_wideselect_null_entry** at 3005.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 696.76x (fastest 3005.6 ns, slowest 2094231.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2091318ns | 2092041ns | 2078627ns | 2090929ns | 2098247ns | +0.07% |
| abi_cross_scalar_wideselect_inproc_direct | 2089904ns | 2084755ns | 2073426ns | 2084166ns | 2106752ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2100051ns | 2097391ns | 2090722ns | 2096917ns | 2109418ns | +0.49% |
| abi_cross_scalar_wideselect_null_entry | 5298ns | 5314ns | 5185ns | 5290ns | 5366ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2088082ns | 2075611ns | 2094923ns | +0.07% | 0.000 |
| abi_cross_scalar_wideselect_inproc_direct | 2086704ns | 2070555ns | 2103241ns | base | 0.000 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2096767ns | 2087785ns | 2105892ns | +0.48% | 0.000 |
| abi_cross_scalar_wideselect_null_entry | 3004ns | 2966ns | 3026ns | -99.86% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 64777.1 | 2089259.0 | 2088081.5 | n/a |
| abi_cross_scalar_wideselect_inproc_direct | 10258.1 | 2084753.9 | 2086704.2 | n/a |
| abi_cross_scalar_wideselect_inproc_fnptr | 10113.0 | 2089002.4 | 2096766.5 | 0 |
| abi_cross_scalar_wideselect_null_entry | 29975.0 | 3122.5 | 3004.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_cross_scalar_wideselect_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_null_entry | 0.003 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2091318ns | 2091318ns | +0.07% |
| abi_cross_scalar_wideselect_inproc_direct | 2089904ns | 2089904ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2100051ns | 2100051ns | +0.49% |
| abi_cross_scalar_wideselect_null_entry | 5298ns | 5298ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_inproc_direct | 2081686ns | base | --- | [2075185, 2103241] | --- | --- | --- | --- |
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2088811ns | no significant difference | [-11752, +12010]ns | [2080510, 2094923] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2094231ns | no significant difference | [-7726, +23025]ns | [2090176, 2105892] | no | 0.3281 | 0.2188 | 0 |
| abi_cross_scalar_wideselect_null_entry | 3006ns | -2078660.6ns (-99.9%) | [-2100236, -2072204]ns | [2981, 3026] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_wideselect_inproc_direct | abi_cross_scalar_wideselect_ffi_batched_scalar | abi_cross_scalar_wideselect_inproc_fnptr | abi_cross_scalar_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2079815ns | +0.3% | +0.4% | -99.9% |
| 2 | 2090492ns | +0.1% | +0.9% | -99.9% |
| 3 | 2080325ns | -0.2% | +1.1% | -99.9% |
| 4 | 2083048ns | +0.4% | +0.5% | -99.9% |
| 5 | 2070555ns | +0.8% | +1.1% | -99.9% |
| 6 | 2115989ns | -0.9% | -1.1% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | -0.459 | moderate- |
| abi_cross_scalar_wideselect_inproc_direct | -0.357 | moderate- |
| abi_cross_scalar_wideselect_inproc_fnptr | -0.107 | ok |
| abi_cross_scalar_wideselect_null_entry | -0.035 | ok |

**Consistency summary:**

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: won 2/6, lost 4/6
- **abi_cross_scalar_wideselect_inproc_fnptr**: won 1/6, lost 5/6
- **abi_cross_scalar_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 6338411.9ns | 2088081.5ns | 303.6% | HIGH |
| abi_cross_scalar_wideselect_inproc_direct | 6271208.8ns | 2086704.2ns | 300.5% | HIGH |
| abi_cross_scalar_wideselect_inproc_fnptr | 6283406.2ns | 2096766.5ns | 299.7% | HIGH |
| abi_cross_scalar_wideselect_null_entry | 121269.4ns | 3004.1ns | 4036.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_wideselect_ffi_batched_scalar (n=6, range 2075610.8-2094922.7 ns)
  2075610.8 |########################################
  2076576.4 |
  2077542.0 |
  2078507.6 |
  2079473.2 |
  2080438.8 |
  2081404.4 |
  2082370.0 |
  2083335.6 |
  2084301.2 |
  2085266.8 |########################################
  2086232.3 |########################################
  2087197.9 |
  2088163.5 |
  2089129.1 |
  2090094.7 |
  2091060.3 |########################################
  2092025.9 |########################################
  2092991.5 |
  2093957.1 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_direct (n=6, range 2070555.4-2103240.6 ns)
  2070555.4 |####################
  2072189.7 |
  2073823.9 |
  2075458.2 |
  2077092.4 |
  2078726.7 |########################################
  2080361.0 |
  2081995.2 |####################
  2083629.5 |
  2085263.8 |
  2086898.0 |
  2088532.3 |
  2090166.5 |####################
  2091800.8 |
  2093435.1 |
  2095069.3 |
  2096703.6 |
  2098337.9 |
  2099972.1 |
  2101606.4 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_fnptr (n=6, range 2087785.4-2105892.0 ns)
  2087785.4 |########################################
  2088690.7 |
  2089596.1 |
  2090501.4 |
  2091406.7 |
  2092312.1 |########################################
  2093217.4 |########################################
  2094122.7 |########################################
  2095028.1 |
  2095933.4 |
  2096838.7 |
  2097744.1 |
  2098649.4 |
  2099554.7 |
  2100460.1 |
  2101365.4 |
  2102270.7 |########################################
  2103176.1 |
  2104081.4 |
  2104986.7 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_null_entry (n=6, range 2966.2-3025.8 ns)
   2966.2 |####################
   2969.2 |
   2972.2 |
   2975.1 |
   2978.1 |
   2981.1 |
   2984.1 |
   2987.1 |
   2990.1 |
   2993.0 |####################
   2996.0 |####################
   2999.0 |
   3002.0 |
   3005.0 |
   3008.0 |
   3010.9 |
   3013.9 |########################################
   3016.9 |
   3019.9 |
   3022.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_direct**: bridge=301.0% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_fnptr**: bridge=300.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_null_entry**: bridge=4035.4% of algo (FFI overhead may distort results)
