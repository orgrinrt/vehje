# abi_cross_scalar (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_wideselect_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_wideselect_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_wideselect_null_entry dominates: 51829% faster than the next best (abi_cross_scalar_wideselect_ffi_batched_scalar)

abi_cross_scalar_wideselect_null_entry (4.04 us) leads abi_cross_scalar_wideselect_ffi_batched_scalar (2.10 ms) by 51829%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_wideselect_null_entry beats baseline by 100% (significant)

abi_cross_scalar_wideselect_null_entry is -2.09 ms (100%) faster than baseline abi_cross_scalar_wideselect_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_wideselect_inproc_fnptr is an outlier: 519.5x slower than the field

abi_cross_scalar_wideselect_inproc_fnptr (2.10 ms) is 519.5x the fastest (4.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_wideselect_null_entry} vs {abi_cross_scalar_wideselect_ffi_batched_scalar, abi_cross_scalar_wideselect_inproc_direct, abi_cross_scalar_wideselect_inproc_fnptr} (51829% apart)

The field splits into a fast tier {abi_cross_scalar_wideselect_null_entry} and a slow tier {abi_cross_scalar_wideselect_ffi_batched_scalar, abi_cross_scalar_wideselect_inproc_direct, abi_cross_scalar_wideselect_inproc_fnptr} with a 51829% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 519.5x the fastest

Fastest abi_cross_scalar_wideselect_null_entry (4.04 us) to slowest abi_cross_scalar_wideselect_inproc_fnptr (2.10 ms): 519.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_wideselect_null_entry** at 4038.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 519.54x (fastest 4038.8 ns, slowest 2098311.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2102459ns | 2100745ns | 2088108ns | 2100493ns | 2112582ns | +0.32% |
| abi_cross_scalar_wideselect_inproc_direct | 2095679ns | 2100816ns | 2076178ns | 2096936ns | 2103543ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2102480ns | 2101671ns | 2079871ns | 2098303ns | 2120049ns | +0.32% |
| abi_cross_scalar_wideselect_null_entry | 6343ns | 6339ns | 6265ns | 6321ns | 6415ns | -99.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2099059ns | 2085091ns | 2109073ns | +0.32% | 0.000 |
| abi_cross_scalar_wideselect_inproc_direct | 2092432ns | 2073053ns | 2100069ns | base | 0.000 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2099083ns | 2076811ns | 2116358ns | +0.32% | 0.000 |
| abi_cross_scalar_wideselect_null_entry | 4034ns | 3985ns | 4075ns | -99.81% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 69055.8 | 2099639.2 | 2099059.0 | n/a |
| abi_cross_scalar_wideselect_inproc_direct | 10511.8 | 2096510.0 | 2092432.3 | n/a |
| abi_cross_scalar_wideselect_inproc_fnptr | 10648.9 | 2102798.8 | 2099083.5 | n/a |
| abi_cross_scalar_wideselect_null_entry | 29785.5 | 4205.8 | 4033.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_scalar_wideselect_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_wideselect_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_wideselect_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_wideselect_null_entry | 0.001 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2102459ns | 2102459ns | +0.32% |
| abi_cross_scalar_wideselect_inproc_direct | 2095679ns | 2095679ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2102480ns | 2102480ns | +0.32% |
| abi_cross_scalar_wideselect_null_entry | 6343ns | 6343ns | -99.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_inproc_direct | 2097688ns | base | --- | [2079540, 2100069] | --- | --- | --- | --- |
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2097281ns | no significant difference | [-6780, +21727]ns | [2090823, 2109073] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2098311ns | no significant difference | [-8329, +23103]ns | [2082581, 2116358] | no | 1.0000 | 0.6875 | 0 |
| abi_cross_scalar_wideselect_null_entry | 4039ns | -2093678.1ns (-99.8%) | [-2095994, -2075524]ns | [3987, 4075] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_wideselect_inproc_direct | abi_cross_scalar_wideselect_ffi_batched_scalar | abi_cross_scalar_wideselect_inproc_fnptr | abi_cross_scalar_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2073053ns | +1.2% | +0.2% | -99.8% |
| 2 | 2099655ns | -0.0% | +0.3% | -99.8% |
| 3 | 2097778ns | -0.6% | -0.4% | -99.8% |
| 4 | 2097599ns | -0.0% | -0.3% | -99.8% |
| 5 | 2100483ns | +0.9% | +0.3% | -99.8% |
| 6 | 2086026ns | +0.5% | +1.9% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | -0.104 | ok |
| abi_cross_scalar_wideselect_inproc_direct | -0.142 | ok |
| abi_cross_scalar_wideselect_inproc_fnptr | 0.002 | ok |
| abi_cross_scalar_wideselect_null_entry | -0.150 | ok |

**Consistency summary:**

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: won 1/6, lost 3/6
- **abi_cross_scalar_wideselect_inproc_fnptr**: won 2/6, lost 4/6
- **abi_cross_scalar_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 6375026.4ns | 2099059.0ns | 303.7% | HIGH |
| abi_cross_scalar_wideselect_inproc_direct | 6301272.3ns | 2092432.3ns | 301.1% | HIGH |
| abi_cross_scalar_wideselect_inproc_fnptr | 6312271.8ns | 2099083.5ns | 300.7% | HIGH |
| abi_cross_scalar_wideselect_null_entry | 125144.1ns | 4033.6ns | 3102.5% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_wideselect_ffi_batched_scalar (n=6, range 2085090.8-2109073.4 ns)
  2085090.8 |####################
  2086289.9 |
  2087489.1 |
  2088688.2 |
  2089887.3 |
  2091086.4 |
  2092285.6 |
  2093484.7 |
  2094683.8 |
  2095882.9 |########################################
  2097082.1 |####################
  2098281.2 |####################
  2099480.3 |
  2100679.5 |
  2101878.6 |
  2103077.7 |
  2104276.8 |
  2105476.0 |
  2106675.1 |
  2107874.2 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_direct (n=6, range 2073053.3-2100069.0 ns)
  2073053.3 |####################
  2074404.1 |
  2075754.9 |
  2077105.6 |
  2078456.4 |
  2079807.2 |
  2081158.0 |
  2082508.8 |
  2083859.6 |
  2085210.3 |####################
  2086561.1 |
  2087911.9 |
  2089262.7 |
  2090613.5 |
  2091964.3 |
  2093315.0 |
  2094665.8 |
  2096016.6 |
  2097367.4 |########################################
  2098718.2 |####################
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_fnptr (n=6, range 2076810.8-2116358.1 ns)
  2076810.8 |########################################
  2078788.2 |
  2080765.5 |
  2082742.9 |
  2084720.3 |
  2086697.6 |########################################
  2088675.0 |########################################
  2090652.4 |
  2092629.7 |
  2094607.1 |
  2096584.4 |
  2098561.8 |
  2100539.2 |
  2102516.5 |
  2104493.9 |########################################
  2106471.3 |########################################
  2108448.6 |
  2110426.0 |
  2112403.4 |
  2114380.7 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_null_entry (n=6, range 3985.0-4075.2 ns)
   3985.0 |########################################
   3989.5 |
   3994.0 |
   3998.5 |
   4003.1 |
   4007.6 |
   4012.1 |
   4016.6 |
   4021.1 |
   4025.6 |
   4030.1 |
   4034.6 |####################
   4039.2 |####################
   4043.7 |
   4048.2 |
   4052.7 |####################
   4057.2 |
   4061.7 |
   4066.2 |
   4070.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_direct**: bridge=300.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_fnptr**: bridge=300.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_null_entry**: bridge=3101.9% of algo (FFI overhead may distort results)
