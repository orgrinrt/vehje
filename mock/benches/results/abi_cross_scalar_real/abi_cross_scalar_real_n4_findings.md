# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 53428% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (4.06 us) leads abi_cross_scalar_real_inproc_direct (2.17 ms) by 53428%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.17 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_ffi_batched_scalar is an outlier: 543.4x slower than the field

abi_cross_scalar_real_ffi_batched_scalar (2.21 ms) is 543.4x the fastest (4.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_real_null_entry shows alternating (throttle bounce) (autocorr -0.58)

abi_cross_scalar_real_null_entry's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_ffi_batched_scalar} (53428% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_ffi_batched_scalar} with a 53428% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 543.4x the fastest

Fastest abi_cross_scalar_real_null_entry (4.06 us) to slowest abi_cross_scalar_real_ffi_batched_scalar (2.21 ms): 543.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 4062.7 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 543.39x (fastest 4062.7 ns, slowest 2207642.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2208013ns | 2211590ns | 2184410ns | 2209722ns | 2217252ns | -1.65% |
| abi_cross_scalar_real_inproc_direct | 2245080ns | 2178071ns | 2164817ns | 2177344ns | 2386814ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2203734ns | 2198182ns | 2186657ns | 2196692ns | 2222836ns | -1.84% |
| abi_cross_scalar_real_null_entry | 6373ns | 6365ns | 6202ns | 6362ns | 6473ns | -99.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2204154ns | 2180694ns | 2213330ns | -1.66% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2241452ns | 2161373ns | 2382858ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2200268ns | 2183344ns | 2219397ns | -1.84% | 0.000 |
| abi_cross_scalar_real_null_entry | 4037ns | 3938ns | 4091ns | -99.82% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 79845.5 | 2204087.0 | 2204154.0 | 2 |
| abi_cross_scalar_real_inproc_direct | 11659.3 | 2205727.4 | 2241451.6 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 11356.0 | 2205762.0 | 2200268.2 | n/a |
| abi_cross_scalar_real_null_entry | 28742.2 | 4181.7 | 4037.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_real_null_entry | 0.001 | 96.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2208013ns | 2208013ns | -1.65% |
| abi_cross_scalar_real_inproc_direct | 2245080ns | 2245080ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2203734ns | 2203734ns | -1.84% |
| abi_cross_scalar_real_null_entry | 6373ns | 6373ns | -99.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2174665ns | base | --- | [2166832, 2382858] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2207642ns | no significant difference | [-180409, +37524]ns | [2191490, 2213330] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2194609ns | no significant difference | [-165664, +24131]ns | [2186798, 2219397] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_real_null_entry | 4063ns | -2170647.7ns (-99.8%) | [-2378827, -2162768]ns | [3958, 4091] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2178143ns | +1.4% | +0.8% | -99.8% |
| 2 | 2174173ns | +1.6% | +1.2% | -99.8% |
| 3 | 2172290ns | +0.4% | +0.8% | -99.8% |
| 4 | 2161373ns | +1.9% | +1.0% | -99.8% |
| 5 | 2175158ns | +1.5% | +0.8% | -99.8% |
| 6 | 2587573ns | -14.3% | -13.5% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.004 | ok |
| abi_cross_scalar_real_inproc_direct | -0.022 | ok |
| abi_cross_scalar_real_inproc_fnptr | 0.009 | ok |
| abi_cross_scalar_real_null_entry | -0.577 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 1/6, lost 5/6
- **abi_cross_scalar_real_inproc_fnptr**: won 1/6, lost 5/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 6706695.5ns | 2204154.0ns | 304.3% | HIGH |
| abi_cross_scalar_real_inproc_direct | 6624178.1ns | 2241451.6ns | 295.5% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 6624940.5ns | 2200268.2ns | 301.1% | HIGH |
| abi_cross_scalar_real_null_entry | 123183.0ns | 4037.3ns | 3051.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2180694.2-2213329.6 ns)
  2180694.2 |#############
  2182326.0 |
  2183957.7 |
  2185589.5 |
  2187221.3 |
  2188853.1 |
  2190484.8 |
  2192116.6 |
  2193748.4 |
  2195380.1 |
  2197011.9 |
  2198643.7 |
  2200275.4 |
  2201907.2 |#############
  2203539.0 |
  2205170.8 |
  2206802.5 |########################################
  2208434.3 |
  2210066.1 |
  2211697.8 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2161373.3-2382857.9 ns)
  2161373.3 |##########################
  2172447.5 |########################################
  2183521.8 |
  2194596.0 |
  2205670.2 |
  2216744.4 |
  2227818.7 |
  2238892.9 |
  2249967.1 |
  2261041.4 |
  2272115.6 |
  2283189.8 |
  2294264.1 |
  2305338.3 |
  2316412.5 |
  2327486.8 |
  2338561.0 |
  2349635.2 |
  2360709.4 |
  2371783.7 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2183343.8-2219396.9 ns)
  2183343.8 |########################################
  2185146.5 |
  2186949.1 |
  2188751.8 |########################################
  2190554.4 |
  2192357.1 |########################################
  2194159.7 |
  2195962.4 |########################################
  2197765.0 |
  2199567.7 |########################################
  2201370.3 |
  2203173.0 |
  2204975.7 |
  2206778.3 |
  2208581.0 |
  2210383.6 |
  2212186.3 |
  2213988.9 |
  2215791.6 |
  2217594.2 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 3938.3-4090.9 ns)
   3938.3 |####################
   3945.9 |
   3953.6 |
   3961.2 |
   3968.8 |
   3976.4 |####################
   3984.1 |
   3991.7 |
   3999.3 |
   4006.9 |
   4014.6 |
   4022.2 |
   4029.8 |
   4037.5 |####################
   4045.1 |
   4052.7 |
   4060.3 |
   4068.0 |
   4075.6 |
   4083.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=300.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=300.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=3004.9% of algo (FFI overhead may distort results)
