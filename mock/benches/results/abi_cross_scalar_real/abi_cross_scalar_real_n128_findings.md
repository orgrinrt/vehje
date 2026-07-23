# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 79483% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (2.95 us) leads abi_cross_scalar_real_inproc_direct (2.35 ms) by 79483%, a clear separation rather than a photo finish. CV 16.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.35 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_inproc_fnptr is an outlier: 831.5x slower than the field

abi_cross_scalar_real_inproc_fnptr (2.45 ms) is 831.5x the fastest (2.95 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_real_null_entry is fastest but the noisiest (CV 16.9%)

abi_cross_scalar_real_null_entry wins on median (2.95 us) yet has the highest variance (CV 16.9%), while abi_cross_scalar_real_ffi_batched_scalar is the steadiest (CV 5.8%, 2.39 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### abi_cross_scalar_real_inproc_fnptr shows warm-up / thermal drift (autocorr +0.53)

abi_cross_scalar_real_inproc_fnptr's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} (79483% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} with a 79483% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 831.5x the fastest

Fastest abi_cross_scalar_real_null_entry (2.95 us) to slowest abi_cross_scalar_real_inproc_fnptr (2.45 ms): 831.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 2952.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 831.52x (fastest 2952.1 ns, slowest 2454737.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2387888ns | 2399015ns | 2186230ns | 2349723ns | 2545964ns | +1.12% |
| abi_cross_scalar_real_inproc_direct | 2361472ns | 2354111ns | 2174198ns | 2314786ns | 2525137ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2441555ns | 2459696ns | 2192535ns | 2374704ns | 2666342ns | +3.39% |
| abi_cross_scalar_real_null_entry | 5684ns | 5497ns | 4982ns | 5421ns | 6431ns | -99.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2383222ns | 2182742ns | 2540299ns | +1.11% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2357002ns | 2170919ns | 2519936ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2436634ns | 2189109ns | 2659919ns | +3.38% | 0.000 |
| abi_cross_scalar_real_null_entry | 3120ns | 2717ns | 3611ns | -99.87% | 0.041 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 105708.7 | 2377093.7 | 2383222.2 | n/a |
| abi_cross_scalar_real_inproc_direct | 13396.5 | 2380208.8 | 2357002.3 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 14540.6 | 2451383.9 | 2436633.7 | n/a |
| abi_cross_scalar_real_null_entry | 35932.5 | 3484.3 | 3120.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.047 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_real_null_entry | 0.043 | 92.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2387888ns | 2387888ns | +1.12% |
| abi_cross_scalar_real_inproc_direct | 2361472ns | 2361472ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2441555ns | 2441555ns | +3.39% |
| abi_cross_scalar_real_null_entry | 5684ns | 5684ns | -99.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2349363ns | base | --- | [2201708, 2519936] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2394319ns | no significant difference | [-35952, +101271]ns | [2215049, 2540299] | no | 0.3281 | 0.2188 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2454737ns | no significant difference | [-32505, +202122]ns | [2195245, 2659919] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_real_null_entry | 2952ns | -2346386.5ns (-99.9%) | [-2516350, -2198910]ns | [2798, 3611] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2311555ns | +7.5% | +6.9% | -99.9% |
| 2 | 2566500ns | +1.1% | +4.7% | -99.9% |
| 3 | 2387171ns | +0.3% | +10.3% | -99.9% |
| 4 | 2473371ns | -3.2% | -1.4% | -99.8% |
| 5 | 2232497ns | +0.7% | -1.4% | -99.9% |
| 6 | 2170919ns | +0.5% | +0.8% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.435 | moderate+ |
| abi_cross_scalar_real_inproc_direct | 0.081 | ok |
| abi_cross_scalar_real_inproc_fnptr | 0.528 | HIGH+ (drift/warm-up) |
| abi_cross_scalar_real_null_entry | -0.283 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 1/6, lost 5/6
- **abi_cross_scalar_real_inproc_fnptr**: won 2/6, lost 4/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 7253436.2ns | 2383222.2ns | 304.4% | HIGH |
| abi_cross_scalar_real_inproc_direct | 7088600.0ns | 2357002.3ns | 300.7% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 7346753.5ns | 2436633.7ns | 301.5% | HIGH |
| abi_cross_scalar_real_null_entry | 134967.8ns | 3120.2ns | 4325.6% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2182742.1-2540299.1 ns)
  2182742.1 |####################
  2200620.0 |
  2218497.8 |
  2236375.7 |####################
  2254253.5 |
  2272131.4 |
  2290009.2 |
  2307887.1 |
  2325764.9 |
  2343642.8 |
  2361520.6 |
  2379398.5 |########################################
  2397276.3 |
  2415154.2 |
  2433032.0 |
  2450909.9 |
  2468787.7 |####################
  2486665.6 |
  2504543.4 |
  2522421.3 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2170919.2-2519935.6 ns)
  2170919.2 |########################################
  2188370.0 |
  2205820.8 |
  2223271.7 |########################################
  2240722.5 |
  2258173.3 |
  2275624.1 |
  2293074.9 |
  2310525.8 |########################################
  2327976.6 |
  2345427.4 |
  2362878.2 |
  2380329.0 |########################################
  2397779.9 |
  2415230.7 |
  2432681.5 |
  2450132.3 |
  2467583.1 |########################################
  2485034.0 |
  2502484.8 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2189108.8-2659918.5 ns)
  2189108.8 |########################################
  2212649.3 |
  2236189.8 |
  2259730.3 |
  2283270.8 |
  2306811.2 |
  2330351.7 |
  2353892.2 |
  2377432.7 |
  2400973.2 |
  2424513.7 |####################
  2448054.2 |####################
  2471594.6 |
  2495135.1 |
  2518675.6 |
  2542216.1 |
  2565756.6 |
  2589297.1 |
  2612837.6 |####################
  2636378.1 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 2716.7-3610.8 ns)
   2716.7 |####################
   2761.4 |
   2806.1 |
   2850.8 |####################
   2895.5 |
   2940.2 |########################################
   2984.9 |####################
   3029.6 |
   3074.3 |
   3119.0 |
   3163.8 |
   3208.5 |
   3253.2 |
   3297.9 |
   3342.6 |
   3387.3 |
   3432.0 |
   3476.7 |
   3521.4 |
   3566.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=299.0% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=298.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=4367.6% of algo (FFI overhead may distort results)
