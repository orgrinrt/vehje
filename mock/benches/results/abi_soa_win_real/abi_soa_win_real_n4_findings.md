# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.41 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 4.43 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 52570% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (4.43 us) leads abi_soa_win_real_soa_payload (2.33 ms) by 52570%, a clear separation rather than a photo finish. CV 4.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.41 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 544.8x slower than the field

abi_soa_win_real_scalar_payload (2.41 ms) is 544.8x the fastest (4.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 544.8x the fastest

Fastest abi_soa_win_real_null_entry (4.43 us) to slowest abi_soa_win_real_scalar_payload (2.41 ms): 544.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 4427.5 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 544.77x (fastest 4427.5 ns, slowest 2411989.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 6887ns | 7005ns | 6384ns | 6855ns | 7187ns | -99.71% |
| abi_soa_win_real_scalar_payload | 2387481ns | 2416632ns | 2274495ns | 2380606ns | 2454287ns | base |
| abi_soa_win_real_soa_payload | 2325978ns | 2336536ns | 2222737ns | 2313483ns | 2396340ns | -2.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 4360ns | 4063ns | 4556ns | -99.82% | 0.001 |
| abi_soa_win_real_scalar_payload | 2382492ns | 2270091ns | 2448249ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 2321469ns | 2218979ns | 2391431ns | -2.56% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 34431.6 | 4568.9 | 4360.2 | n/a |
| abi_soa_win_real_scalar_payload | 105157.2 | 2385598.5 | 2382492.2 | n/a |
| abi_soa_win_real_soa_payload | 98176.0 | 2315470.5 | 2321468.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.001 | 91.8% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_real_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 6887ns | 6887ns | -99.71% |
| abi_soa_win_real_scalar_payload | 2387481ns | 2387481ns | base |
| abi_soa_win_real_soa_payload | 2325978ns | 2325978ns | -2.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2411989ns | base | --- | [2287238, 2448249] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 4428ns | -2407892.3ns (-99.8%) | [-2443781, -2282723]ns | [4097, 4556] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 2331975ns | no significant difference | [-142459, +32243]ns | [2241000, 2391431] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2304385ns | -99.8% | +3.5% |
| 2 | 2457348ns | -99.8% | -3.8% |
| 3 | 2413991ns | -99.8% | -0.7% |
| 4 | 2270091ns | -99.8% | -2.3% |
| 5 | 2409988ns | -99.8% | -6.1% |
| 6 | 2439150ns | -99.8% | -5.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.474 | moderate- |
| abi_soa_win_real_scalar_payload | -0.292 | moderate- |
| abi_soa_win_real_soa_payload | 0.197 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 129698.8ns | 4360.2ns | 2974.6% | HIGH |
| abi_soa_win_real_scalar_payload | 7254508.3ns | 2382492.2ns | 304.5% | HIGH |
| abi_soa_win_real_soa_payload | 7048793.1ns | 2321468.7ns | 303.6% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 4062.9-4556.2 ns)
   4062.9 |########################################
   4087.6 |
   4112.2 |########################################
   4136.9 |
   4161.6 |
   4186.2 |
   4210.9 |
   4235.6 |
   4260.2 |
   4284.9 |
   4309.6 |
   4334.2 |
   4358.9 |########################################
   4383.6 |
   4408.2 |
   4432.9 |
   4457.6 |
   4482.2 |########################################
   4506.9 |
   4531.6 |########################################
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2270091.2-2448249.1 ns)
  2270091.2 |########################################
  2278999.1 |
  2287907.0 |
  2296814.9 |########################################
  2305722.8 |
  2314630.7 |
  2323538.6 |
  2332446.5 |
  2341354.4 |
  2350262.3 |
  2359170.2 |
  2368078.1 |
  2376986.0 |
  2385893.9 |
  2394801.8 |
  2403709.7 |########################################
  2412617.6 |########################################
  2421525.5 |
  2430433.4 |########################################
  2439341.3 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 2218978.8-2391431.5 ns)
  2218978.8 |########################################
  2227601.4 |
  2236224.1 |
  2244846.7 |
  2253469.3 |
  2262092.0 |########################################
  2270714.6 |
  2279337.2 |
  2287959.9 |
  2296582.5 |########################################
  2305205.1 |
  2313827.8 |
  2322450.4 |
  2331073.0 |
  2339695.7 |
  2348318.3 |
  2356940.9 |########################################
  2365563.6 |
  2374186.2 |
  2382808.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=2894.8% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=304.8% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=303.5% of algo (FFI overhead may distort results)
