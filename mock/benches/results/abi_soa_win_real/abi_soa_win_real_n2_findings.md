# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_soa_win_real_null_entry dominates: 62864% faster than the next best (abi_soa_win_real_scalar_payload)

abi_soa_win_real_null_entry (3.81 us) leads abi_soa_win_real_scalar_payload (2.40 ms) by 62864%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.40 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_soa_payload is an outlier: 631.7x slower than the field

abi_soa_win_real_soa_payload (2.41 ms) is 631.7x the fastest (3.81 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_real_null_entry is fastest but the noisiest (CV 5.7%)

abi_soa_win_real_null_entry wins on median (3.81 us) yet has the highest variance (CV 5.7%), while abi_soa_win_real_soa_payload is the steadiest (CV 3.1%, 2.41 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### abi_soa_win_real_null_entry shows alternating (throttle bounce) (autocorr -0.71)

abi_soa_win_real_null_entry's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 631.7x the fastest

Fastest abi_soa_win_real_null_entry (3.81 us) to slowest abi_soa_win_real_soa_payload (2.41 ms): 631.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 3811.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 631.71x (fastest 3811.2 ns, slowest 2407565.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 6345ns | 6343ns | 5781ns | 6234ns | 6794ns | -99.73% |
| abi_soa_win_real_scalar_payload | 2374516ns | 2404261ns | 2240659ns | 2353741ns | 2472606ns | base |
| abi_soa_win_real_soa_payload | 2384785ns | 2412385ns | 2249268ns | 2384151ns | 2453495ns | +0.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 3786ns | 3472ns | 4016ns | -99.84% | 0.001 |
| abi_soa_win_real_scalar_payload | 2369981ns | 2236630ns | 2467520ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 2380144ns | 2245744ns | 2448371ns | +0.43% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 34099.5 | 3973.4 | 3786.1 | n/a |
| abi_soa_win_real_scalar_payload | 97128.6 | 2376572.2 | 2369981.5 | n/a |
| abi_soa_win_real_soa_payload | 99073.5 | 2389455.8 | 2380143.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.001 | 91.1% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_real_soa_payload | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 6345ns | 6345ns | -99.73% |
| abi_soa_win_real_scalar_payload | 2374516ns | 2374516ns | base |
| abi_soa_win_real_soa_payload | 2384785ns | 2384785ns | +0.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2399694ns | base | --- | [2242731, 2467520] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 3811ns | -2396055.2ns (-99.8%) | [-2463511, -2239019]ns | [3531, 4016] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 2407566ns | no significant difference | [-107249, +132398]ns | [2284495, 2448371] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2383112ns | -99.8% | +3.1% |
| 2 | 2248832ns | -99.8% | +8.5% |
| 3 | 2236630ns | -99.8% | +0.4% |
| 4 | 2498454ns | -99.8% | -4.9% |
| 5 | 2416275ns | -99.9% | -3.9% |
| 6 | 2436586ns | -99.8% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.713 | HIGH- (thermal bounce) |
| abi_soa_win_real_scalar_payload | 0.116 | ok |
| abi_soa_win_real_soa_payload | -0.177 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 127879.1ns | 3786.1ns | 3377.5% | HIGH |
| abi_soa_win_real_scalar_payload | 7226048.3ns | 2369981.5ns | 304.9% | HIGH |
| abi_soa_win_real_soa_payload | 7268631.7ns | 2380143.8ns | 305.4% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 3471.7-4015.8 ns)
   3471.7 |####################
   3498.9 |
   3526.1 |
   3553.3 |
   3580.5 |####################
   3607.7 |
   3634.9 |
   3662.1 |####################
   3689.3 |
   3716.5 |
   3743.8 |
   3771.0 |
   3798.2 |
   3825.4 |
   3852.6 |
   3879.8 |
   3907.0 |
   3934.2 |########################################
   3961.4 |
   3988.6 |
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2236629.6-2467519.8 ns)
  2236629.6 |########################################
  2248174.1 |########################################
  2259718.6 |
  2271263.1 |
  2282807.6 |
  2294352.1 |
  2305896.7 |
  2317441.2 |
  2328985.7 |
  2340530.2 |
  2352074.7 |
  2363619.2 |
  2375163.7 |########################################
  2386708.2 |
  2398252.7 |
  2409797.2 |########################################
  2421341.8 |
  2432886.3 |########################################
  2444430.8 |
  2455975.3 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 2245744.2-2448370.6 ns)
  2245744.2 |########################################
  2255875.5 |
  2266006.8 |
  2276138.2 |
  2286269.5 |
  2296400.8 |
  2306532.1 |
  2316663.4 |########################################
  2326794.8 |
  2336926.1 |
  2347057.4 |
  2357188.7 |
  2367320.0 |########################################
  2377451.4 |
  2387582.7 |
  2397714.0 |
  2407845.3 |
  2417976.6 |
  2428108.0 |########################################
  2438239.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=3341.0% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=305.8% of algo (FFI overhead may distort results)
