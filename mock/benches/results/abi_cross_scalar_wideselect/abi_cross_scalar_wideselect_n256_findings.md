# abi_cross_scalar (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_wideselect_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_wideselect_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_wideselect_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_wideselect_inproc_direct has the worst median (2.16 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_wideselect_null_entry at 3.19 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_wideselect_null_entry dominates: 65039% faster than the next best (abi_cross_scalar_wideselect_inproc_fnptr)

abi_cross_scalar_wideselect_null_entry (3.19 us) leads abi_cross_scalar_wideselect_inproc_fnptr (2.08 ms) by 65039%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_wideselect_null_entry beats baseline by 100% (significant)

abi_cross_scalar_wideselect_null_entry is -2.16 ms (100%) faster than baseline abi_cross_scalar_wideselect_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_wideselect_inproc_direct is an outlier: 675.5x slower than the field

abi_cross_scalar_wideselect_inproc_direct (2.16 ms) is 675.5x the fastest (3.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_wideselect_ffi_batched_scalar shows alternating (throttle bounce) (autocorr -0.61)

abi_cross_scalar_wideselect_ffi_batched_scalar's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_wideselect_null_entry} vs {abi_cross_scalar_wideselect_inproc_fnptr, abi_cross_scalar_wideselect_ffi_batched_scalar, abi_cross_scalar_wideselect_inproc_direct} (65039% apart)

The field splits into a fast tier {abi_cross_scalar_wideselect_null_entry} and a slow tier {abi_cross_scalar_wideselect_inproc_fnptr, abi_cross_scalar_wideselect_ffi_batched_scalar, abi_cross_scalar_wideselect_inproc_direct} with a 65039% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 675.5x the fastest

Fastest abi_cross_scalar_wideselect_null_entry (3.19 us) to slowest abi_cross_scalar_wideselect_inproc_direct (2.16 ms): 675.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_wideselect_null_entry** at 3195.0 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 675.52x (fastest 3195.0 ns, slowest 2158276.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2162665ns | 2120262ns | 2089947ns | 2113585ns | 2272644ns | -0.89% |
| abi_cross_scalar_wideselect_inproc_direct | 2181991ns | 2162000ns | 2082807ns | 2137203ns | 2298765ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2217934ns | 2084335ns | 2082852ns | 2083896ns | 2486533ns | +1.65% |
| abi_cross_scalar_wideselect_null_entry | 5637ns | 5486ns | 5383ns | 5459ns | 6031ns | -99.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2157107ns | 2086862ns | 2262806ns | -0.98% | 0.000 |
| abi_cross_scalar_wideselect_inproc_direct | 2178536ns | 2079940ns | 2295159ns | base | 0.000 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2214251ns | 2079664ns | 2481851ns | +1.64% | 0.000 |
| abi_cross_scalar_wideselect_null_entry | 3257ns | 3145ns | 3428ns | -99.85% | 0.079 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 75471.3 | 2157447.8 | 2157106.9 | 3 |
| abi_cross_scalar_wideselect_inproc_direct | 11346.3 | 2139963.5 | 2178535.8 | n/a |
| abi_cross_scalar_wideselect_inproc_fnptr | 11520.0 | 2197148.7 | 2214251.4 | n/a |
| abi_cross_scalar_wideselect_null_entry | 31343.4 | 3258.2 | 3257.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.081 Gops/s** (abi_cross_scalar_wideselect_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_wideselect_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_wideselect_null_entry | 0.080 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2162665ns | 2162665ns | -0.89% |
| abi_cross_scalar_wideselect_inproc_direct | 2181991ns | 2181991ns | base |
| abi_cross_scalar_wideselect_inproc_fnptr | 2217934ns | 2217934ns | +1.65% |
| abi_cross_scalar_wideselect_null_entry | 5637ns | 5637ns | -99.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_wideselect_inproc_direct | 2158276ns | base | --- | [2082172, 2295159] | --- | --- | --- | --- |
| abi_cross_scalar_wideselect_ffi_batched_scalar | 2116718ns | no significant difference | [-100786, +29579]ns | [2091797, 2262806] | no | 1.0000 | 0.6875 | 0 |
| abi_cross_scalar_wideselect_inproc_fnptr | 2081192ns | no significant difference | [-92677, +201950]ns | [2079711, 2481851] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_wideselect_null_entry | 3195ns | -2155127.9ns (-99.9%) | [-2291781, -2078927]ns | [3149, 3428] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_wideselect_inproc_direct | abi_cross_scalar_wideselect_ffi_batched_scalar | abi_cross_scalar_wideselect_inproc_fnptr | abi_cross_scalar_wideselect_null_entry |
|---|---|---|---|---|
| 1 | 2084404ns | +0.6% | -0.2% | -99.8% |
| 2 | 2339348ns | -3.5% | +10.1% | -99.8% |
| 3 | 2250970ns | -5.4% | -7.6% | -99.9% |
| 4 | 2096098ns | +0.3% | -0.7% | -99.8% |
| 5 | 2220455ns | +2.1% | +7.5% | -99.9% |
| 6 | 2079940ns | +0.3% | +0.0% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | -0.609 | HIGH- (thermal bounce) |
| abi_cross_scalar_wideselect_inproc_direct | -0.293 | moderate- |
| abi_cross_scalar_wideselect_inproc_fnptr | -0.541 | HIGH- (thermal bounce) |
| abi_cross_scalar_wideselect_null_entry | -0.231 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: won 2/6, lost 4/6
- **abi_cross_scalar_wideselect_inproc_fnptr**: won 3/6, lost 2/6
- **abi_cross_scalar_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_wideselect_ffi_batched_scalar | 6554862.3ns | 2157106.9ns | 303.9% | HIGH |
| abi_cross_scalar_wideselect_inproc_direct | 6525828.5ns | 2178535.8ns | 299.6% | HIGH |
| abi_cross_scalar_wideselect_inproc_fnptr | 6614405.5ns | 2214251.4ns | 298.7% | HIGH |
| abi_cross_scalar_wideselect_null_entry | 124460.3ns | 3257.2ns | 3821.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_wideselect_ffi_batched_scalar (n=6, range 2086861.7-2262806.0 ns)
  2086861.7 |####################
  2095658.9 |########################################
  2104456.1 |
  2113253.4 |
  2122050.6 |####################
  2130847.8 |
  2139645.0 |
  2148442.2 |
  2157239.4 |
  2166036.7 |
  2174833.9 |
  2183631.1 |
  2192428.3 |
  2201225.5 |
  2210022.7 |
  2218820.0 |
  2227617.2 |
  2236414.4 |
  2245211.6 |
  2254008.8 |####################
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_direct (n=6, range 2079940.4-2295158.8 ns)
  2079940.4 |########################################
  2090701.3 |####################
  2101462.2 |
  2112223.2 |
  2122984.1 |
  2133745.0 |
  2144505.9 |
  2155266.8 |
  2166027.7 |
  2176788.7 |
  2187549.6 |
  2198310.5 |
  2209071.4 |
  2219832.3 |####################
  2230593.2 |
  2241354.2 |####################
  2252115.1 |
  2262876.0 |
  2273636.9 |
  2284397.8 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_inproc_fnptr (n=6, range 2079663.8-2481850.9 ns)
  2079663.8 |########################################
  2099773.2 |
  2119882.5 |
  2139991.9 |
  2160101.2 |
  2180210.6 |
  2200319.9 |
  2220429.3 |
  2240538.6 |
  2260648.0 |
  2280757.3 |
  2300866.7 |
  2320976.0 |
  2341085.4 |
  2361194.7 |
  2381304.1 |##########
  2401413.4 |
  2421522.8 |
  2441632.1 |
  2461741.5 |
  (0 below, 1 above range)

abi_cross_scalar_wideselect_null_entry (n=6, range 3145.4-3428.2 ns)
   3145.4 |########################################
   3159.5 |####################
   3173.7 |
   3187.8 |
   3202.0 |
   3216.1 |####################
   3230.2 |
   3244.4 |
   3258.5 |
   3272.6 |
   3286.8 |
   3300.9 |
   3315.1 |####################
   3329.2 |
   3343.3 |
   3357.5 |
   3371.6 |
   3385.7 |
   3399.9 |
   3414.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_wideselect_ffi_batched_scalar**: bridge=307.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_direct**: bridge=300.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_inproc_fnptr**: bridge=301.0% of algo (FFI overhead may distort results)
- **abi_cross_scalar_wideselect_null_entry**: bridge=3867.4% of algo (FFI overhead may distort results)
