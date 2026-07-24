# abi_cross_scalar (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_tight_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_tight_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_tight_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_tight_inproc_direct has the worst median (2.19 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_tight_null_entry at 2.86 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_tight_null_entry dominates: 74762% faster than the next best (abi_cross_scalar_tight_ffi_batched_scalar)

abi_cross_scalar_tight_null_entry (2.86 us) leads abi_cross_scalar_tight_ffi_batched_scalar (2.14 ms) by 74762%, a clear separation rather than a photo finish. CV 4.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_tight_null_entry beats baseline by 100% (significant)

abi_cross_scalar_tight_null_entry is -2.18 ms (100%) faster than baseline abi_cross_scalar_tight_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_tight_inproc_direct is an outlier: 764.2x slower than the field

abi_cross_scalar_tight_inproc_direct (2.19 ms) is 764.2x the fastest (2.86 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_tight_null_entry} vs {abi_cross_scalar_tight_ffi_batched_scalar, abi_cross_scalar_tight_inproc_fnptr, abi_cross_scalar_tight_inproc_direct} (74762% apart)

The field splits into a fast tier {abi_cross_scalar_tight_null_entry} and a slow tier {abi_cross_scalar_tight_ffi_batched_scalar, abi_cross_scalar_tight_inproc_fnptr, abi_cross_scalar_tight_inproc_direct} with a 74762% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 764.2x the fastest

Fastest abi_cross_scalar_tight_null_entry (2.86 us) to slowest abi_cross_scalar_tight_inproc_direct (2.19 ms): 764.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_tight_null_entry** at 2860.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 764.20x (fastest 2860.4 ns, slowest 2185908.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2148990ns | 2145760ns | 2095345ns | 2129868ns | 2204498ns | -3.00% |
| abi_cross_scalar_tight_inproc_direct | 2215347ns | 2189999ns | 2071685ns | 2182696ns | 2336154ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2186360ns | 2154474ns | 2082333ns | 2141498ns | 2305669ns | -1.31% |
| abi_cross_scalar_tight_null_entry | 5331ns | 5278ns | 5116ns | 5226ns | 5595ns | -99.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2144629ns | 2091349ns | 2200048ns | -3.00% | 0.000 |
| abi_cross_scalar_tight_inproc_direct | 2211032ns | 2067661ns | 2331350ns | base | 0.000 |
| abi_cross_scalar_tight_inproc_fnptr | 2182060ns | 2078347ns | 2300961ns | -1.31% | 0.000 |
| abi_cross_scalar_tight_null_entry | 2898ns | 2741ns | 3065ns | -99.87% | 0.044 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 91179.8 | 2154478.8 | 2144629.2 | 1 |
| abi_cross_scalar_tight_inproc_direct | 9915.4 | 2203272.7 | 2211031.6 | n/a |
| abi_cross_scalar_tight_inproc_fnptr | 9990.5 | 2179007.8 | 2182059.6 | 0 |
| abi_cross_scalar_tight_null_entry | 31503.3 | 2940.2 | 2898.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.047 Gops/s** (abi_cross_scalar_tight_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_tight_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_tight_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_tight_null_entry | 0.045 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2148990ns | 2148990ns | -3.00% |
| abi_cross_scalar_tight_inproc_direct | 2215347ns | 2215347ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2186360ns | 2186360ns | -1.31% |
| abi_cross_scalar_tight_null_entry | 5331ns | 5331ns | -99.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_inproc_direct | 2185908ns | base | --- | [2115836, 2331350] | --- | --- | --- | --- |
| abi_cross_scalar_tight_ffi_batched_scalar | 2141366ns | no significant difference | [-188548, +27575]ns | [2092474, 2200048] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_tight_inproc_fnptr | 2150252ns | no significant difference | [-83955, +41338]ns | [2094966, 2300961] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_tight_null_entry | 2860ns | -2183080.4ns (-99.9%) | [-2328431, -2112888]ns | [2770, 3065] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_tight_inproc_direct | abi_cross_scalar_tight_ffi_batched_scalar | abi_cross_scalar_tight_inproc_fnptr | abi_cross_scalar_tight_null_entry |
|---|---|---|---|---|
| 1 | 2437610ns | -11.7% | -3.9% | -99.9% |
| 2 | 2187486ns | -2.7% | +3.3% | -99.9% |
| 3 | 2184330ns | -4.3% | -3.3% | -99.9% |
| 4 | 2067661ns | +1.3% | +0.5% | -99.9% |
| 5 | 2225091ns | -0.8% | -2.9% | -99.9% |
| 6 | 2164011ns | +1.3% | -1.1% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 0.271 | moderate+ |
| abi_cross_scalar_tight_inproc_direct | -0.047 | ok |
| abi_cross_scalar_tight_inproc_fnptr | 0.351 | moderate+ |
| abi_cross_scalar_tight_null_entry | 0.043 | ok |

**Consistency summary:**

- **abi_cross_scalar_tight_ffi_batched_scalar**: won 4/6, lost 2/6
- **abi_cross_scalar_tight_inproc_fnptr**: won 4/6, lost 2/6
- **abi_cross_scalar_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 6525969.2ns | 2144629.2ns | 304.3% | HIGH |
| abi_cross_scalar_tight_inproc_direct | 6637202.6ns | 2211031.6ns | 300.2% | HIGH |
| abi_cross_scalar_tight_inproc_fnptr | 6550139.6ns | 2182059.6ns | 300.2% | HIGH |
| abi_cross_scalar_tight_null_entry | 124287.2ns | 2898.4ns | 4288.2% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_tight_ffi_batched_scalar (n=6, range 2091349.2-2200048.1 ns)
  2091349.2 |########################################
  2096784.1 |
  2102219.1 |
  2107654.0 |
  2113089.0 |
  2118523.9 |
  2123958.9 |####################
  2129393.8 |
  2134828.8 |
  2140263.7 |
  2145698.7 |
  2151133.6 |####################
  2156568.6 |
  2162003.5 |
  2167438.5 |
  2172873.4 |
  2178308.4 |
  2183743.3 |
  2189178.3 |####################
  2194613.2 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_direct (n=6, range 2067660.8-2331350.4 ns)
  2067660.8 |########################################
  2080845.3 |
  2094029.8 |
  2107214.2 |
  2120398.7 |
  2133583.2 |
  2146767.7 |
  2159952.2 |########################################
  2173136.6 |########################################
  2186321.1 |########################################
  2199505.6 |
  2212690.1 |########################################
  2225874.6 |
  2239059.0 |
  2252243.5 |
  2265428.0 |
  2278612.5 |
  2291797.0 |
  2304981.4 |
  2318165.9 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_fnptr (n=6, range 2078346.7-2300960.7 ns)
  2078346.7 |########################################
  2089477.4 |
  2100608.1 |########################################
  2111738.8 |
  2122869.5 |
  2134000.2 |########################################
  2145130.9 |
  2156261.6 |########################################
  2167392.3 |
  2178523.0 |
  2189653.7 |
  2200784.4 |
  2211915.1 |
  2223045.8 |
  2234176.5 |
  2245307.2 |
  2256437.9 |########################################
  2267568.6 |
  2278699.3 |
  2289830.0 |
  (0 below, 1 above range)

abi_cross_scalar_tight_null_entry (n=6, range 2741.2-3065.2 ns)
   2741.2 |########################################
   2757.4 |
   2773.6 |
   2789.8 |########################################
   2806.0 |########################################
   2822.2 |
   2838.4 |
   2854.6 |
   2870.8 |
   2887.0 |
   2903.2 |########################################
   2919.4 |
   2935.6 |
   2951.8 |
   2968.0 |
   2984.2 |
   3000.4 |
   3016.6 |
   3032.8 |########################################
   3049.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_tight_ffi_batched_scalar**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_direct**: bridge=300.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_fnptr**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_null_entry**: bridge=4372.3% of algo (FFI overhead may distort results)
