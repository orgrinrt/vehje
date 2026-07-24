# abi_cross_scalar (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_tight_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_tight_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_tight_null_entry dominates: 62370% faster than the next best (abi_cross_scalar_tight_ffi_batched_scalar)

abi_cross_scalar_tight_null_entry (3.44 us) leads abi_cross_scalar_tight_ffi_batched_scalar (2.15 ms) by 62370%, a clear separation rather than a photo finish. CV 25.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_tight_null_entry beats baseline by 100% (significant)

abi_cross_scalar_tight_null_entry is -2.32 ms (100%) faster than baseline abi_cross_scalar_tight_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_tight_inproc_fnptr is an outlier: 725.2x slower than the field

abi_cross_scalar_tight_inproc_fnptr (2.50 ms) is 725.2x the fastest (3.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_tight_null_entry} vs {abi_cross_scalar_tight_ffi_batched_scalar, abi_cross_scalar_tight_inproc_direct, abi_cross_scalar_tight_inproc_fnptr} (62370% apart)

The field splits into a fast tier {abi_cross_scalar_tight_null_entry} and a slow tier {abi_cross_scalar_tight_ffi_batched_scalar, abi_cross_scalar_tight_inproc_direct, abi_cross_scalar_tight_inproc_fnptr} with a 62370% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 725.2x the fastest

Fastest abi_cross_scalar_tight_null_entry (3.44 us) to slowest abi_cross_scalar_tight_inproc_fnptr (2.50 ms): 725.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_scalar_tight_inproc_direct is inconsistent: worst-20% is 1.6x its best-20%

abi_cross_scalar_tight_inproc_direct's best 20% of batches run at 2.01 ms but its worst 20% at 3.29 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: abi_cross_scalar_tight_null_entry** at 3440.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 725.17x (fastest 3440.8 ns, slowest 2495207.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2243085ns | 2153779ns | 2041564ns | 2135760ns | 2504834ns | -12.52% |
| abi_cross_scalar_tight_inproc_direct | 2564196ns | 2328439ns | 2012772ns | 2258359ns | 3298665ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2447729ns | 2501544ns | 2040320ns | 2375622ns | 2759593ns | -4.54% |
| abi_cross_scalar_tight_null_entry | 6360ns | 5991ns | 5402ns | 5894ns | 7538ns | -99.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2238029ns | 2037485ns | 2498094ns | -12.56% | 0.000 |
| abi_cross_scalar_tight_inproc_direct | 2559507ns | 2009624ns | 3293672ns | base | 0.000 |
| abi_cross_scalar_tight_inproc_fnptr | 2442396ns | 2036506ns | 2754076ns | -4.58% | 0.000 |
| abi_cross_scalar_tight_null_entry | 3803ns | 3168ns | 4725ns | -99.85% | 0.067 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 96156.7 | 2289095.2 | 2238029.1 | 0 |
| abi_cross_scalar_tight_inproc_direct | 10351.9 | 2358839.8 | 2559507.2 | n/a |
| abi_cross_scalar_tight_inproc_fnptr | 16241.5 | 2440836.6 | 2442396.5 | 0 |
| abi_cross_scalar_tight_null_entry | 40365.1 | 3607.9 | 3803.0 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.081 Gops/s** (abi_cross_scalar_tight_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_tight_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_tight_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_tight_null_entry | 0.074 | 92.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2243085ns | 2243085ns | -12.52% |
| abi_cross_scalar_tight_inproc_direct | 2564196ns | 2564196ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2447729ns | 2447729ns | -4.54% |
| abi_cross_scalar_tight_null_entry | 6360ns | 6360ns | -99.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_inproc_direct | 2323003ns | base | --- | [2061846, 3293672] | --- | --- | --- | --- |
| abi_cross_scalar_tight_ffi_batched_scalar | 2149505ns | no significant difference | [-998267, +69772]ns | [2066488, 2498094] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_tight_inproc_fnptr | 2495207ns | no significant difference | [-859660, +426921]ns | [2077906, 2754076] | no | 1.0000 | 0.6875 | 0 |
| abi_cross_scalar_tight_null_entry | 3441ns | -2319383.4ns (-99.8%) | [-3289126, -2058603]ns | [3243, 4725] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_tight_inproc_direct | abi_cross_scalar_tight_ffi_batched_scalar | abi_cross_scalar_tight_inproc_fnptr | abi_cross_scalar_tight_null_entry |
|---|---|---|---|---|
| 1 | 2425077ns | +3.1% | +13.0% | -99.9% |
| 2 | 2220930ns | -4.5% | +24.2% | -99.8% |
| 3 | 2726105ns | -23.1% | -22.3% | -99.9% |
| 4 | 2009624ns | +1.4% | +1.3% | -99.8% |
| 5 | 2114068ns | +3.0% | +6.4% | -99.8% |
| 6 | 3861240ns | -35.4% | -28.8% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 0.052 | ok |
| abi_cross_scalar_tight_inproc_direct | -0.186 | ok |
| abi_cross_scalar_tight_inproc_fnptr | 0.242 | moderate+ |
| abi_cross_scalar_tight_null_entry | -0.068 | ok |

**Consistency summary:**

- **abi_cross_scalar_tight_ffi_batched_scalar**: won 3/6, lost 3/6
- **abi_cross_scalar_tight_inproc_fnptr**: won 2/6, lost 4/6
- **abi_cross_scalar_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 6938358.0ns | 2238029.1ns | 310.0% | HIGH |
| abi_cross_scalar_tight_inproc_direct | 7145165.8ns | 2559507.2ns | 279.2% | HIGH |
| abi_cross_scalar_tight_inproc_fnptr | 7298205.1ns | 2442396.5ns | 298.8% | HIGH |
| abi_cross_scalar_tight_null_entry | 144415.3ns | 3803.0ns | 3797.4% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_tight_ffi_batched_scalar (n=6, range 2037485.0-2498094.3 ns)
  2037485.0 |########################################
  2060515.5 |
  2083545.9 |########################################
  2106576.4 |########################################
  2129606.9 |
  2152637.3 |
  2175667.8 |########################################
  2198698.3 |
  2221728.7 |
  2244759.2 |
  2267789.7 |
  2290820.1 |
  2313850.6 |
  2336881.1 |
  2359911.5 |
  2382942.0 |
  2405972.5 |
  2429002.9 |
  2452033.4 |
  2475063.9 |########################################
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_direct (n=6, range 2009624.2-3293672.5 ns)
  2009624.2 |########################################
  2073826.6 |########################################
  2138029.0 |
  2202231.4 |########################################
  2266433.9 |
  2330636.3 |
  2394838.7 |########################################
  2459041.1 |
  2523243.5 |
  2587445.9 |
  2651648.4 |
  2715850.8 |########################################
  2780053.2 |
  2844255.6 |
  2908458.0 |
  2972660.4 |
  3036862.8 |
  3101065.3 |
  3165267.7 |
  3229470.1 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_fnptr (n=6, range 2036506.2-2754076.5 ns)
  2036506.2 |####################
  2072384.7 |
  2108263.2 |####################
  2144141.7 |
  2180020.2 |
  2215898.8 |####################
  2251777.3 |
  2287655.8 |
  2323534.3 |
  2359412.8 |
  2395291.3 |
  2431169.8 |
  2467048.4 |
  2502926.9 |
  2538805.4 |
  2574683.9 |
  2610562.4 |
  2646440.9 |
  2682319.4 |
  2718197.9 |########################################
  (0 below, 1 above range)

abi_cross_scalar_tight_null_entry (n=6, range 3168.3-4725.0 ns)
   3168.3 |########################################
   3246.1 |########################################
   3324.0 |########################################
   3401.8 |
   3479.6 |########################################
   3557.5 |
   3635.3 |
   3713.1 |########################################
   3791.0 |
   3868.8 |
   3946.7 |
   4024.5 |
   4102.3 |
   4180.2 |
   4258.0 |
   4335.8 |
   4413.7 |
   4491.5 |
   4569.3 |
   4647.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_tight_ffi_batched_scalar**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_direct**: CV=24.5% (high variance, measurements may be unstable)
- **abi_cross_scalar_tight_inproc_direct**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_fnptr**: bridge=292.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_null_entry**: CV=22.8% (high variance, measurements may be unstable)
- **abi_cross_scalar_tight_null_entry**: bridge=3818.5% of algo (FFI overhead may distort results)
