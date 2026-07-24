# abi_cross_scalar (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_scatter_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_scatter_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_scatter_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_scatter_inproc_direct has the worst median (2.19 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_scatter_null_entry at 2.69 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_scatter_null_entry dominates: 80879% faster than the next best (abi_cross_scalar_scatter_inproc_fnptr)

abi_cross_scalar_scatter_null_entry (2.69 us) leads abi_cross_scalar_scatter_inproc_fnptr (2.18 ms) by 80879%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_scatter_null_entry beats baseline by 100% (significant)

abi_cross_scalar_scatter_null_entry is -2.19 ms (100%) faster than baseline abi_cross_scalar_scatter_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_scatter_inproc_direct is an outlier: 813.5x slower than the field

abi_cross_scalar_scatter_inproc_direct (2.19 ms) is 813.5x the fastest (2.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_scatter_null_entry} vs {abi_cross_scalar_scatter_inproc_fnptr, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_direct} (80879% apart)

The field splits into a fast tier {abi_cross_scalar_scatter_null_entry} and a slow tier {abi_cross_scalar_scatter_inproc_fnptr, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_direct} with a 80879% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 813.5x the fastest

Fastest abi_cross_scalar_scatter_null_entry (2.69 us) to slowest abi_cross_scalar_scatter_inproc_direct (2.19 ms): 813.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_scatter_null_entry** at 2690.0 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 813.48x (fastest 2690.0 ns, slowest 2188255.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2241896ns | 2191421ns | 2170687ns | 2185188ns | 2362563ns | +1.08% |
| abi_cross_scalar_scatter_inproc_direct | 2218023ns | 2191435ns | 2159259ns | 2181383ns | 2302364ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2181262ns | 2181765ns | 2167759ns | 2178935ns | 2191503ns | -1.66% |
| abi_cross_scalar_scatter_null_entry | 4944ns | 4939ns | 4835ns | 4922ns | 5030ns | -99.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2238093ns | 2167356ns | 2358265ns | +1.06% | 0.000 |
| abi_cross_scalar_scatter_inproc_direct | 2214527ns | 2156074ns | 2298280ns | base | 0.000 |
| abi_cross_scalar_scatter_inproc_fnptr | 2177927ns | 2164784ns | 2188052ns | -1.65% | 0.000 |
| abi_cross_scalar_scatter_null_entry | 2683ns | 2644ns | 2711ns | -99.88% | 0.048 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 73063.8 | 2284255.6 | 2238092.7 | n/a |
| abi_cross_scalar_scatter_inproc_direct | 7915.2 | 2199942.4 | 2214526.8 | n/a |
| abi_cross_scalar_scatter_inproc_fnptr | 7962.2 | 2178943.0 | 2177926.8 | 0 |
| abi_cross_scalar_scatter_null_entry | 27206.0 | 2744.7 | 2683.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_cross_scalar_scatter_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_scatter_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_scatter_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_scatter_null_entry | 0.048 | 98.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2241896ns | 2241896ns | +1.08% |
| abi_cross_scalar_scatter_inproc_direct | 2218023ns | 2218023ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2181262ns | 2181262ns | -1.66% |
| abi_cross_scalar_scatter_null_entry | 4944ns | 4944ns | -99.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_inproc_direct | 2188255ns | base | --- | [2157046, 2298280] | --- | --- | --- | --- |
| abi_cross_scalar_scatter_ffi_batched_scalar | 2187619ns | no significant difference | [-54157, +120902]ns | [2168395, 2358265] | no | 1.0000 | 0.6875 | 0 |
| abi_cross_scalar_scatter_inproc_fnptr | 2178323ns | no significant difference | [-122382, +21550]ns | [2167405, 2188052] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_scatter_null_entry | 2690ns | -2185580.0ns (-99.9%) | [-2295612, -2154338]ns | [2648, 2711] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_scatter_inproc_direct | abi_cross_scalar_scatter_ffi_batched_scalar | abi_cross_scalar_scatter_inproc_fnptr | abi_cross_scalar_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2158018ns | +10.6% | +0.8% | -99.9% |
| 2 | 2156074ns | +0.6% | +1.2% | -99.9% |
| 3 | 2162424ns | +0.2% | +0.1% | -99.9% |
| 4 | 2214086ns | -0.9% | -0.9% | -99.9% |
| 5 | 2327095ns | +0.1% | -6.2% | -99.9% |
| 6 | 2269464ns | -3.9% | -4.4% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | -0.252 | moderate- |
| abi_cross_scalar_scatter_inproc_direct | 0.500 | moderate+ |
| abi_cross_scalar_scatter_inproc_fnptr | -0.470 | moderate- |
| abi_cross_scalar_scatter_null_entry | 0.276 | moderate+ |

**Consistency summary:**

- **abi_cross_scalar_scatter_ffi_batched_scalar**: won 2/6, lost 4/6
- **abi_cross_scalar_scatter_inproc_fnptr**: won 3/6, lost 3/6
- **abi_cross_scalar_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 6827068.3ns | 2238092.7ns | 305.0% | HIGH |
| abi_cross_scalar_scatter_inproc_direct | 6625672.8ns | 2214526.8ns | 299.2% | HIGH |
| abi_cross_scalar_scatter_inproc_fnptr | 6547652.3ns | 2177926.8ns | 300.6% | HIGH |
| abi_cross_scalar_scatter_null_entry | 118731.7ns | 2683.2ns | 4425.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_scatter_ffi_batched_scalar (n=6, range 2167356.2-2358264.5 ns)
  2167356.2 |########################################
  2176901.6 |####################
  2186447.0 |####################
  2195992.5 |
  2205537.9 |
  2215083.3 |
  2224628.7 |
  2234174.1 |
  2243719.5 |
  2253265.0 |
  2262810.4 |
  2272355.8 |
  2281901.2 |
  2291446.6 |
  2300992.0 |
  2310537.5 |
  2320082.9 |
  2329628.3 |####################
  2339173.7 |
  2348719.1 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_direct (n=6, range 2156073.7-2298279.6 ns)
  2156073.7 |########################################
  2163184.0 |
  2170294.3 |
  2177404.6 |
  2184514.9 |
  2191625.2 |
  2198735.5 |
  2205845.8 |
  2212956.1 |#############
  2220066.4 |
  2227176.7 |
  2234286.9 |
  2241397.2 |
  2248507.5 |
  2255617.8 |
  2262728.1 |#############
  2269838.4 |
  2276948.7 |
  2284059.0 |
  2291169.3 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_fnptr (n=6, range 2164784.2-2188052.1 ns)
  2164784.2 |########################################
  2165947.6 |
  2167111.0 |
  2168274.4 |
  2169437.8 |########################################
  2170601.2 |
  2171764.6 |
  2172928.0 |
  2174091.4 |########################################
  2175254.8 |
  2176418.2 |
  2177581.5 |
  2178744.9 |
  2179908.3 |
  2181071.7 |########################################
  2182235.1 |########################################
  2183398.5 |
  2184561.9 |
  2185725.3 |
  2186888.7 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_null_entry (n=6, range 2644.2-2711.2 ns)
   2644.2 |########################################
   2647.5 |
   2650.9 |########################################
   2654.2 |
   2657.6 |
   2660.9 |
   2664.3 |
   2667.6 |
   2671.0 |
   2674.3 |
   2677.7 |
   2681.0 |########################################
   2684.4 |
   2687.8 |
   2691.1 |
   2694.4 |
   2697.8 |########################################
   2701.1 |
   2704.5 |########################################
   2707.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_scatter_ffi_batched_scalar**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_direct**: bridge=299.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_fnptr**: bridge=300.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_null_entry**: bridge=4407.8% of algo (FFI overhead may distort results)
