# abi_cross_scalar (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_scatter_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_scatter_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_scatter_null_entry dominates: 54179% faster than the next best (abi_cross_scalar_scatter_inproc_direct)

abi_cross_scalar_scatter_null_entry (4.00 us) leads abi_cross_scalar_scatter_inproc_direct (2.17 ms) by 54179%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_scatter_null_entry beats baseline by 100% (significant)

abi_cross_scalar_scatter_null_entry is -2.17 ms (100%) faster than baseline abi_cross_scalar_scatter_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_scatter_inproc_fnptr is an outlier: 545.8x slower than the field

abi_cross_scalar_scatter_inproc_fnptr (2.18 ms) is 545.8x the fastest (4.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_scatter_inproc_direct shows alternating (throttle bounce) (autocorr -0.58)

abi_cross_scalar_scatter_inproc_direct's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_scatter_null_entry} vs {abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_fnptr} (54179% apart)

The field splits into a fast tier {abi_cross_scalar_scatter_null_entry} and a slow tier {abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_fnptr} with a 54179% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 545.8x the fastest

Fastest abi_cross_scalar_scatter_null_entry (4.00 us) to slowest abi_cross_scalar_scatter_inproc_fnptr (2.18 ms): 545.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_scatter_null_entry** at 3997.7 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 545.79x (fastest 3997.7 ns, slowest 2181899.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2198132ns | 2182427ns | 2177070ns | 2181421ns | 2233731ns | +0.31% |
| abi_cross_scalar_scatter_inproc_direct | 2191446ns | 2173206ns | 2161373ns | 2170269ns | 2238248ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2255217ns | 2185247ns | 2181456ns | 2184425ns | 2398284ns | +2.91% |
| abi_cross_scalar_scatter_null_entry | 6318ns | 6277ns | 6194ns | 6251ns | 6480ns | -99.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2194574ns | 2173858ns | 2229771ns | +0.30% | 0.000 |
| abi_cross_scalar_scatter_inproc_direct | 2188004ns | 2158108ns | 2234373ns | base | 0.000 |
| abi_cross_scalar_scatter_inproc_fnptr | 2251614ns | 2178363ns | 2394001ns | +2.91% | 0.000 |
| abi_cross_scalar_scatter_null_entry | 4017ns | 3938ns | 4101ns | -99.82% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 67577.6 | 2191898.9 | 2194574.4 | n/a |
| abi_cross_scalar_scatter_inproc_direct | 7939.3 | 2189388.5 | 2188004.0 | n/a |
| abi_cross_scalar_scatter_inproc_fnptr | 8285.1 | 2222957.1 | 2251614.2 | n/a |
| abi_cross_scalar_scatter_null_entry | 27040.1 | 4112.7 | 4017.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_scalar_scatter_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_scatter_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_scatter_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_scatter_null_entry | 0.001 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2198132ns | 2198132ns | +0.31% |
| abi_cross_scalar_scatter_inproc_direct | 2191446ns | 2191446ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2255217ns | 2255217ns | +2.91% |
| abi_cross_scalar_scatter_null_entry | 6318ns | 6318ns | -99.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_inproc_direct | 2169910ns | base | --- | [2159729, 2234373] | --- | --- | --- | --- |
| abi_cross_scalar_scatter_ffi_batched_scalar | 2179005ns | no significant difference | [-17949, +22180]ns | [2174947, 2229771] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_scatter_inproc_fnptr | 2181899ns | no significant difference | [-10997, +183954]ns | [2178943, 2394001] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_scatter_null_entry | 3998ns | -2165937.9ns (-99.8%) | [-2230342, -2155680]ns | [3953, 4101] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_scatter_inproc_direct | abi_cross_scalar_scatter_ffi_batched_scalar | abi_cross_scalar_scatter_inproc_fnptr | abi_cross_scalar_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2158108ns | +1.3% | +1.0% | -99.8% |
| 2 | 2210002ns | -1.6% | -1.3% | -99.8% |
| 3 | 2161350ns | +0.8% | +1.6% | -99.8% |
| 4 | 2258744ns | +0.7% | +14.7% | -99.8% |
| 5 | 2164030ns | +0.7% | +0.7% | -99.8% |
| 6 | 2175789ns | +0.0% | +0.3% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | -0.220 | moderate- |
| abi_cross_scalar_scatter_inproc_direct | -0.580 | HIGH- (thermal bounce) |
| abi_cross_scalar_scatter_inproc_fnptr | -0.213 | moderate- |
| abi_cross_scalar_scatter_null_entry | -0.282 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_scatter_ffi_batched_scalar**: won 1/6, lost 4/6
- **abi_cross_scalar_scatter_inproc_fnptr**: won 1/6, lost 5/6
- **abi_cross_scalar_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 6648224.3ns | 2194574.4ns | 302.9% | HIGH |
| abi_cross_scalar_scatter_inproc_direct | 6580074.9ns | 2188004.0ns | 300.7% | HIGH |
| abi_cross_scalar_scatter_inproc_fnptr | 6662836.6ns | 2251614.2ns | 295.9% | HIGH |
| abi_cross_scalar_scatter_null_entry | 121804.6ns | 4017.2ns | 3032.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_scatter_ffi_batched_scalar (n=6, range 2173857.5-2229770.9 ns)
  2173857.5 |########################################
  2176653.2 |####################
  2179448.8 |####################
  2182244.5 |
  2185040.2 |####################
  2187835.8 |
  2190631.5 |
  2193427.2 |
  2196222.8 |
  2199018.5 |
  2201814.2 |
  2204609.8 |
  2207405.5 |
  2210201.2 |
  2212996.8 |
  2215792.5 |
  2218588.2 |
  2221383.8 |
  2224179.5 |
  2226975.2 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_direct (n=6, range 2158108.3-2234373.1 ns)
  2158108.3 |########################################
  2161921.5 |####################
  2165734.8 |
  2169548.0 |
  2173361.3 |####################
  2177174.5 |
  2180987.8 |
  2184801.0 |
  2188614.2 |
  2192427.5 |
  2196240.7 |
  2200054.0 |
  2203867.2 |
  2207680.5 |####################
  2211493.7 |
  2215306.9 |
  2219120.2 |
  2222933.4 |
  2226746.7 |
  2230559.9 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_fnptr (n=6, range 2178363.3-2394001.0 ns)
  2178363.3 |########################################
  2189145.2 |##########
  2199927.1 |
  2210709.0 |
  2221490.8 |
  2232272.7 |
  2243054.6 |
  2253836.5 |
  2264618.4 |
  2275400.3 |
  2286182.2 |
  2296964.1 |
  2307745.9 |
  2318527.8 |
  2329309.7 |
  2340091.6 |
  2350873.5 |
  2361655.4 |
  2372437.3 |
  2383219.2 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_null_entry (n=6, range 3937.5-4100.9 ns)
   3937.5 |########################################
   3945.7 |
   3953.8 |
   3962.0 |########################################
   3970.2 |########################################
   3978.3 |
   3986.5 |
   3994.7 |
   4002.8 |
   4011.0 |
   4019.2 |########################################
   4027.3 |
   4035.5 |
   4043.7 |
   4051.8 |
   4060.0 |
   4068.2 |
   4076.3 |########################################
   4084.5 |
   4092.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_scatter_ffi_batched_scalar**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_direct**: bridge=300.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_fnptr**: bridge=300.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_null_entry**: bridge=3034.3% of algo (FFI overhead may distort results)
