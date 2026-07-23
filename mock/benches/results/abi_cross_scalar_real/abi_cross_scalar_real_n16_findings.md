# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 85004% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (2.54 us) leads abi_cross_scalar_real_inproc_direct (2.16 ms) by 85004%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.16 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_inproc_fnptr is an outlier: 858.4x slower than the field

abi_cross_scalar_real_inproc_fnptr (2.18 ms) is 858.4x the fastest (2.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_real_null_entry shows alternating (throttle bounce) (autocorr -0.80)

abi_cross_scalar_real_null_entry's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} (85004% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} with a 85004% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 858.4x the fastest

Fastest abi_cross_scalar_real_null_entry (2.54 us) to slowest abi_cross_scalar_real_inproc_fnptr (2.18 ms): 858.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 2536.2 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 858.36x (fastest 2536.2 ns, slowest 2176979.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2172951ns | 2171786ns | 2168015ns | 2171342ns | 2177833ns | +0.50% |
| abi_cross_scalar_real_inproc_direct | 2162087ns | 2161457ns | 2159131ns | 2160841ns | 2165434ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2182247ns | 2180119ns | 2170634ns | 2179819ns | 2191696ns | +0.93% |
| abi_cross_scalar_real_null_entry | 4827ns | 4834ns | 4720ns | 4806ns | 4912ns | -99.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2169669ns | 2164892ns | 2174301ns | +0.49% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2159000ns | 2156300ns | 2162256ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2179050ns | 2167550ns | 2188314ns | +0.93% | 0.000 |
| abi_cross_scalar_real_null_entry | 2529ns | 2486ns | 2556ns | -99.88% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 65543.9 | 2168808.8 | 2169669.2 | n/a |
| abi_cross_scalar_real_inproc_direct | 9781.5 | 2159494.9 | 2159000.3 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 10137.6 | 2185560.2 | 2179049.8 | n/a |
| abi_cross_scalar_real_null_entry | 28381.0 | 2636.3 | 2529.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_real_null_entry | 0.006 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2172951ns | 2172951ns | +0.50% |
| abi_cross_scalar_real_inproc_direct | 2162087ns | 2162087ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2182247ns | 2182247ns | +0.93% |
| abi_cross_scalar_real_null_entry | 4827ns | 4827ns | -99.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2158396ns | base | --- | [2156349, 2162256] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2168589ns | +9190.6ns (+0.4%) | [+4864, +17953]ns | [2166118, 2174301] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2176980ns | +16073.6ns (+0.7%) | [+12110, +31965]ns | [2171856, 2188314] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_real_null_entry | 2536ns | -2155872.8ns (-99.9%) | [-2159728, -2153812]ns | [2496, 2556] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2156300ns | +0.6% | +1.6% | -99.9% |
| 2 | 2157732ns | +0.4% | +0.5% | -99.9% |
| 3 | 2162671ns | +0.3% | +0.7% | -99.9% |
| 4 | 2161842ns | +0.1% | +0.7% | -99.9% |
| 5 | 2159060ns | +0.4% | +0.8% | -99.9% |
| 6 | 2156397ns | +1.1% | +1.4% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | -0.048 | ok |
| abi_cross_scalar_real_inproc_direct | 0.247 | moderate+ |
| abi_cross_scalar_real_inproc_fnptr | -0.348 | moderate- |
| abi_cross_scalar_real_null_entry | -0.803 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 0/6, lost 6/6
- **abi_cross_scalar_real_inproc_fnptr**: won 0/6, lost 6/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 6579151.6ns | 2169669.2ns | 303.2% | HIGH |
| abi_cross_scalar_real_inproc_direct | 6489902.6ns | 2159000.3ns | 300.6% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 6571367.0ns | 2179049.8ns | 301.6% | HIGH |
| abi_cross_scalar_real_null_entry | 119139.1ns | 2529.4ns | 4710.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2164892.5-2174301.2 ns)
  2164892.5 |####################
  2165362.9 |
  2165833.4 |
  2166303.8 |
  2166774.2 |
  2167244.7 |####################
  2167715.1 |####################
  2168185.6 |
  2168656.0 |
  2169126.4 |########################################
  2169596.9 |
  2170067.3 |
  2170537.8 |
  2171008.2 |
  2171478.6 |
  2171949.1 |
  2172419.5 |
  2172889.9 |
  2173360.4 |
  2173830.8 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2156300.4-2162256.5 ns)
  2156300.4 |########################################
  2156598.2 |
  2156896.0 |
  2157193.8 |
  2157491.6 |####################
  2157789.4 |
  2158087.2 |
  2158385.0 |
  2158682.8 |
  2158980.6 |####################
  2159278.4 |
  2159576.2 |
  2159874.0 |
  2160171.8 |
  2160469.6 |
  2160767.4 |
  2161065.2 |
  2161363.0 |
  2161660.8 |####################
  2161958.6 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2167550.0-2188313.5 ns)
  2167550.0 |####################
  2168588.2 |
  2169626.4 |
  2170664.5 |
  2171702.7 |
  2172740.9 |
  2173779.1 |
  2174817.2 |
  2175855.4 |########################################
  2176893.6 |####################
  2177931.8 |
  2178970.0 |
  2180008.1 |
  2181046.3 |
  2182084.5 |
  2183122.7 |
  2184160.8 |
  2185199.0 |
  2186237.2 |####################
  2187275.4 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 2486.2-2556.4 ns)
   2486.2 |########################################
   2489.7 |
   2493.2 |
   2496.7 |
   2500.2 |
   2503.8 |########################################
   2507.3 |
   2510.8 |
   2514.3 |
   2517.8 |
   2521.3 |
   2524.8 |
   2528.3 |########################################
   2531.9 |
   2535.4 |
   2538.9 |########################################
   2542.4 |########################################
   2545.9 |
   2549.4 |
   2552.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=300.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=300.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=4699.7% of algo (FFI overhead may distort results)
