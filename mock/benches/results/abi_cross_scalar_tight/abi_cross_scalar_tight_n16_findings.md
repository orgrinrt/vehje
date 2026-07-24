# abi_cross_scalar (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_tight_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_tight_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_tight_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_tight_inproc_direct has the worst median (2.27 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_tight_null_entry at 2.70 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_tight_null_entry dominates: 82680% faster than the next best (abi_cross_scalar_tight_ffi_batched_scalar)

abi_cross_scalar_tight_null_entry (2.70 us) leads abi_cross_scalar_tight_ffi_batched_scalar (2.23 ms) by 82680%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_tight_null_entry beats baseline by 100% (significant)

abi_cross_scalar_tight_null_entry is -2.27 ms (100%) faster than baseline abi_cross_scalar_tight_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_tight_inproc_direct is an outlier: 840.9x slower than the field

abi_cross_scalar_tight_inproc_direct (2.27 ms) is 840.9x the fastest (2.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_tight_ffi_batched_scalar shows alternating (throttle bounce) (autocorr -0.67)

abi_cross_scalar_tight_ffi_batched_scalar's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_tight_null_entry} vs {abi_cross_scalar_tight_ffi_batched_scalar, abi_cross_scalar_tight_inproc_fnptr, abi_cross_scalar_tight_inproc_direct} (82680% apart)

The field splits into a fast tier {abi_cross_scalar_tight_null_entry} and a slow tier {abi_cross_scalar_tight_ffi_batched_scalar, abi_cross_scalar_tight_inproc_fnptr, abi_cross_scalar_tight_inproc_direct} with a 82680% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 840.9x the fastest

Fastest abi_cross_scalar_tight_null_entry (2.70 us) to slowest abi_cross_scalar_tight_inproc_direct (2.27 ms): 840.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_tight_null_entry** at 2697.7 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 840.95x (fastest 2697.7 ns, slowest 2268629.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2243086ns | 2238240ns | 2114192ns | 2212055ns | 2354079ns | -6.24% |
| abi_cross_scalar_tight_inproc_direct | 2392419ns | 2273780ns | 2093452ns | 2257063ns | 2744935ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2213130ns | 2238165ns | 2075750ns | 2204560ns | 2294676ns | -7.49% |
| abi_cross_scalar_tight_null_entry | 5326ns | 5167ns | 4828ns | 5133ns | 5863ns | -99.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2238306ns | 2110479ns | 2348753ns | -6.24% | 0.000 |
| abi_cross_scalar_tight_inproc_direct | 2387354ns | 2089610ns | 2738885ns | base | 0.000 |
| abi_cross_scalar_tight_inproc_fnptr | 2208652ns | 2072161ns | 2289631ns | -7.49% | 0.000 |
| abi_cross_scalar_tight_null_entry | 2700ns | 2561ns | 2814ns | -99.89% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 96871.4 | 2246525.0 | 2238306.1 | 1 |
| abi_cross_scalar_tight_inproc_direct | 11420.2 | 2386096.5 | 2387354.3 | n/a |
| abi_cross_scalar_tight_inproc_fnptr | 10265.4 | 2214874.4 | 2208651.5 | n/a |
| abi_cross_scalar_tight_null_entry | 32968.9 | 2814.6 | 2699.9 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_cross_scalar_tight_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_tight_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_tight_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_tight_null_entry | 0.006 | 94.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2243086ns | 2243086ns | -6.24% |
| abi_cross_scalar_tight_inproc_direct | 2392419ns | 2392419ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2213130ns | 2213130ns | -7.49% |
| abi_cross_scalar_tight_null_entry | 5326ns | 5326ns | -99.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_inproc_direct | 2268629ns | base | --- | [2154549, 2738885] | --- | --- | --- | --- |
| abi_cross_scalar_tight_ffi_batched_scalar | 2233154ns | no significant difference | [-406617, +54993]ns | [2133011, 2348753] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_tight_inproc_fnptr | 2233502ns | no significant difference | [-467230, +14033]ns | [2102822, 2289631] | no | 0.3281 | 0.2188 | 0 |
| abi_cross_scalar_tight_null_entry | 2698ns | -2265840.2ns (-99.9%) | [-2736193, -2151930]ns | [2588, 2814] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_tight_inproc_direct | abi_cross_scalar_tight_ffi_batched_scalar | abi_cross_scalar_tight_inproc_fnptr | abi_cross_scalar_tight_null_entry |
|---|---|---|---|---|
| 1 | 2242550ns | -2.3% | -0.7% | -99.9% |
| 2 | 2639533ns | -9.5% | -15.1% | -99.9% |
| 3 | 2294708ns | -6.1% | -0.8% | -99.9% |
| 4 | 2838236ns | -19.8% | -18.9% | -99.9% |
| 5 | 2089610ns | +1.0% | +2.1% | -99.9% |
| 6 | 2219488ns | +4.0% | -6.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | -0.670 | HIGH- (thermal bounce) |
| abi_cross_scalar_tight_inproc_direct | -0.450 | moderate- |
| abi_cross_scalar_tight_inproc_fnptr | 0.313 | moderate+ |
| abi_cross_scalar_tight_null_entry | -0.184 | ok |

**Consistency summary:**

- **abi_cross_scalar_tight_ffi_batched_scalar**: won 4/6, lost 2/6
- **abi_cross_scalar_tight_inproc_fnptr**: won 5/6, lost 1/6
- **abi_cross_scalar_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 6846944.2ns | 2238306.1ns | 305.9% | HIGH |
| abi_cross_scalar_tight_inproc_direct | 7147404.9ns | 2387354.3ns | 299.4% | HIGH |
| abi_cross_scalar_tight_inproc_fnptr | 6658695.5ns | 2208651.5ns | 301.5% | HIGH |
| abi_cross_scalar_tight_null_entry | 124441.7ns | 2699.9ns | 4609.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_tight_ffi_batched_scalar (n=6, range 2110479.2-2348752.7 ns)
  2110479.2 |########################################
  2122392.9 |
  2134306.6 |
  2146220.2 |########################################
  2158133.9 |
  2170047.6 |
  2181961.2 |########################################
  2193874.9 |
  2205788.6 |
  2217702.3 |
  2229616.0 |
  2241529.6 |
  2253443.3 |
  2265357.0 |########################################
  2277270.7 |
  2289184.3 |
  2301098.0 |########################################
  2313011.7 |
  2324925.4 |
  2336839.0 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_direct (n=6, range 2089610.0-2738884.8 ns)
  2089610.0 |####################
  2122073.7 |
  2154537.5 |
  2187001.2 |
  2219465.0 |########################################
  2251928.7 |
  2284392.4 |####################
  2316856.2 |
  2349319.9 |
  2381783.6 |
  2414247.4 |
  2446711.1 |
  2479174.9 |
  2511638.6 |
  2544102.3 |
  2576566.1 |
  2609029.8 |####################
  2641493.5 |
  2673957.3 |
  2706421.0 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_fnptr (n=6, range 2072160.8-2289630.6 ns)
  2072160.8 |########################################
  2083034.3 |
  2093907.8 |
  2104781.3 |
  2115654.8 |
  2126528.2 |########################################
  2137401.7 |
  2148275.2 |
  2159148.7 |
  2170022.2 |
  2180895.7 |
  2191769.2 |
  2202642.7 |
  2213516.2 |
  2224389.7 |########################################
  2235263.1 |########################################
  2246136.6 |
  2257010.1 |
  2267883.6 |########################################
  2278757.1 |
  (0 below, 1 above range)

abi_cross_scalar_tight_null_entry (n=6, range 2561.2-2813.9 ns)
   2561.2 |########################################
   2573.8 |
   2586.5 |
   2599.1 |
   2611.8 |########################################
   2624.4 |
   2637.0 |
   2649.7 |
   2662.3 |
   2674.9 |########################################
   2687.6 |
   2700.2 |
   2712.8 |########################################
   2725.5 |
   2738.1 |
   2750.8 |
   2763.4 |########################################
   2776.0 |
   2788.7 |
   2801.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_tight_ffi_batched_scalar**: bridge=304.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_direct**: bridge=300.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_fnptr**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_null_entry**: bridge=4578.2% of algo (FFI overhead may distort results)
