# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 65263% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (3.44 us) leads abi_cross_scalar_real_inproc_direct (2.25 ms) by 65263%, a clear separation rather than a photo finish. CV 13.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.25 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_ffi_batched_scalar is an outlier: 668.9x slower than the field

abi_cross_scalar_real_ffi_batched_scalar (2.30 ms) is 668.9x the fastest (3.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_real_null_entry is fastest but the noisiest (CV 13.7%)

abi_cross_scalar_real_null_entry wins on median (3.44 us) yet has the highest variance (CV 13.7%), while abi_cross_scalar_real_ffi_batched_scalar is the steadiest (CV 3.6%, 2.30 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### abi_cross_scalar_real_inproc_direct shows warm-up / thermal drift (autocorr +0.57)

abi_cross_scalar_real_inproc_direct's per-pass series has lag-1 autocorrelation +0.57, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_ffi_batched_scalar} (65263% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_ffi_batched_scalar} with a 65263% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 668.9x the fastest

Fastest abi_cross_scalar_real_null_entry (3.44 us) to slowest abi_cross_scalar_real_ffi_batched_scalar (2.30 ms): 668.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 3441.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 668.86x (fastest 3441.6 ns, slowest 2301974.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2316597ns | 2306032ns | 2204171ns | 2285604ns | 2419299ns | +2.16% |
| abi_cross_scalar_real_inproc_direct | 2267609ns | 2253235ns | 2173338ns | 2233555ns | 2365825ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2338400ns | 2270760ns | 2193092ns | 2246298ns | 2549206ns | +3.12% |
| abi_cross_scalar_real_null_entry | 6014ns | 5739ns | 5556ns | 5692ns | 6727ns | -99.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2312485ns | 2200413ns | 2415021ns | +2.15% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2263871ns | 2170427ns | 2361579ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2334375ns | 2189380ns | 2544289ns | +3.11% | 0.000 |
| abi_cross_scalar_real_null_entry | 3638ns | 3348ns | 4119ns | -99.84% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 85802.3 | 2314113.5 | 2312484.6 | n/a |
| abi_cross_scalar_real_inproc_direct | 11940.4 | 2259488.5 | 2263870.6 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 12107.4 | 2326893.6 | 2334374.9 | n/a |
| abi_cross_scalar_real_null_entry | 31580.6 | 3716.8 | 3638.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_real_null_entry | 0.001 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2316597ns | 2316597ns | +2.16% |
| abi_cross_scalar_real_inproc_direct | 2267609ns | 2267609ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2338400ns | 2338400ns | +3.12% |
| abi_cross_scalar_real_null_entry | 6014ns | 6014ns | -99.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2249570ns | base | --- | [2180464, 2361579] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2301975ns | no significant difference | [-31918, +149619]ns | [2220458, 2415021] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2267175ns | no significant difference | [-65517, +277686]ns | [2191661, 2544289] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_real_null_entry | 3442ns | -2246215.6ns (-99.9%) | [-2357529, -2176952]ns | [3354, 4119] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2360377ns | +2.1% | -2.9% | -99.9% |
| 2 | 2362780ns | -1.1% | +5.3% | -99.8% |
| 3 | 2305006ns | -1.6% | -2.7% | -99.9% |
| 4 | 2194133ns | +2.1% | -0.0% | -99.8% |
| 5 | 2190500ns | +0.5% | -0.1% | -99.8% |
| 6 | 2170427ns | +11.5% | +19.9% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.012 | ok |
| abi_cross_scalar_real_inproc_direct | 0.571 | HIGH+ (drift/warm-up) |
| abi_cross_scalar_real_inproc_fnptr | -0.179 | ok |
| abi_cross_scalar_real_null_entry | -0.281 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 2/6, lost 4/6
- **abi_cross_scalar_real_inproc_fnptr**: won 2/6, lost 2/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 7027977.0ns | 2312484.6ns | 303.9% | HIGH |
| abi_cross_scalar_real_inproc_direct | 6774648.7ns | 2263870.6ns | 299.3% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 7000154.2ns | 2334374.9ns | 299.9% | HIGH |
| abi_cross_scalar_real_null_entry | 125462.5ns | 3638.2ns | 3448.4% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2200413.3-2415020.9 ns)
  2200413.3 |########################################
  2211143.7 |
  2221874.1 |
  2232604.4 |########################################
  2243334.8 |
  2254065.2 |
  2264795.6 |########################################
  2275525.9 |
  2286256.3 |
  2296986.7 |
  2307717.1 |
  2318447.5 |
  2329177.8 |########################################
  2339908.2 |
  2350638.6 |
  2361369.0 |
  2372099.3 |
  2382829.7 |
  2393560.1 |
  2404290.5 |########################################
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2170427.1-2361578.5 ns)
  2170427.1 |####################
  2179984.7 |
  2189542.2 |########################################
  2199099.8 |
  2208657.4 |
  2218215.0 |
  2227772.5 |
  2237330.1 |
  2246887.7 |
  2256445.3 |
  2266002.8 |
  2275560.4 |
  2285118.0 |
  2294675.5 |
  2304233.1 |####################
  2313790.7 |
  2323348.3 |
  2332905.8 |
  2342463.4 |
  2352021.0 |####################
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2189380.0-2544289.1 ns)
  2189380.0 |########################################
  2207125.5 |
  2224870.9 |
  2242616.4 |####################
  2260361.8 |
  2278107.3 |####################
  2295852.7 |
  2313598.2 |
  2331343.7 |
  2349089.1 |
  2366834.6 |
  2384580.0 |
  2402325.5 |
  2420070.9 |
  2437816.4 |
  2455561.9 |
  2473307.3 |####################
  2491052.8 |
  2508798.2 |
  2526543.7 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 3347.5-4119.1 ns)
   3347.5 |########################################
   3386.1 |####################
   3424.7 |
   3463.2 |####################
   3501.8 |
   3540.4 |####################
   3579.0 |
   3617.6 |
   3656.2 |
   3694.7 |
   3733.3 |
   3771.9 |
   3810.5 |
   3849.1 |
   3887.7 |
   3926.2 |
   3964.8 |
   4003.4 |
   4042.0 |
   4080.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=304.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **abi_cross_scalar_real_inproc_direct**: bridge=299.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=299.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=3553.2% of algo (FFI overhead may distort results)
