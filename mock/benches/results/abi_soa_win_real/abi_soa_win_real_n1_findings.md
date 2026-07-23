# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.43 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 5.39 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 43962% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (5.39 us) leads abi_soa_win_real_soa_payload (2.37 ms) by 43962%, a clear separation rather than a photo finish. CV 7.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.42 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 450.9x slower than the field

abi_soa_win_real_scalar_payload (2.43 ms) is 450.9x the fastest (5.39 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_real_null_entry is fastest but the noisiest (CV 7.6%)

abi_soa_win_real_null_entry wins on median (5.39 us) yet has the highest variance (CV 7.6%), while abi_soa_win_real_scalar_payload is the steadiest (CV 2.2%, 2.43 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 450.9x the fastest

Fastest abi_soa_win_real_null_entry (5.39 us) to slowest abi_soa_win_real_scalar_payload (2.43 ms): 450.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 5389.1 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 450.89x (fastest 5389.1 ns, slowest 2429895.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 8093ns | 7875ns | 7427ns | 7780ns | 8895ns | -99.67% |
| abi_soa_win_real_scalar_payload | 2439117ns | 2434694ns | 2376178ns | 2420359ns | 2498725ns | base |
| abi_soa_win_real_soa_payload | 2431793ns | 2379094ns | 2346111ns | 2378887ns | 2553993ns | -0.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 5491ns | 5050ns | 6008ns | -99.77% | 0.000 |
| abi_soa_win_real_scalar_payload | 2434221ns | 2371809ns | 2493458ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 2427033ns | 2341669ns | 2548927ns | -0.30% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 35533.4 | 5490.4 | 5491.5 | n/a |
| abi_soa_win_real_scalar_payload | 106032.6 | 2438264.2 | 2434221.1 | n/a |
| abi_soa_win_real_soa_payload | 107530.7 | 2432987.9 | 2427033.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.000 | 93.7% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_real_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 8093ns | 8093ns | -99.67% |
| abi_soa_win_real_scalar_payload | 2439117ns | 2439117ns | base |
| abi_soa_win_real_soa_payload | 2431793ns | 2431793ns | -0.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2429896ns | base | --- | [2379310, 2493458] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 5389ns | -2424262.7ns (-99.8%) | [-2488381, -2373545]ns | [5077, 6008] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 2374589ns | no significant difference | [-119548, +163593]ns | [2357584, 2548927] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2467181ns | -99.8% | -3.8% |
| 2 | 2371809ns | -99.7% | +5.0% |
| 3 | 2386810ns | -99.8% | -1.9% |
| 4 | 2460933ns | -99.8% | -3.5% |
| 5 | 2398859ns | -99.8% | +8.7% |
| 6 | 2519735ns | -99.8% | -5.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.479 | moderate- |
| abi_soa_win_real_scalar_payload | -0.263 | moderate- |
| abi_soa_win_real_soa_payload | -0.446 | moderate- |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 135336.1ns | 5491.5ns | 2464.5% | HIGH |
| abi_soa_win_real_scalar_payload | 7434327.6ns | 2434221.1ns | 305.4% | HIGH |
| abi_soa_win_real_soa_payload | 7390455.2ns | 2427033.2ns | 304.5% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 5049.6-6008.4 ns)
   5049.6 |########################################
   5097.5 |########################################
   5145.5 |
   5193.4 |
   5241.4 |
   5289.3 |########################################
   5337.2 |
   5385.2 |
   5433.1 |
   5481.0 |########################################
   5529.0 |
   5576.9 |
   5624.9 |
   5672.8 |
   5720.7 |
   5768.7 |########################################
   5816.6 |
   5864.5 |
   5912.5 |
   5960.4 |
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2371808.8-2493457.9 ns)
  2371808.8 |########################################
  2377891.3 |
  2383973.7 |########################################
  2390056.2 |
  2396138.6 |########################################
  2402221.1 |
  2408303.5 |
  2414386.0 |
  2420468.4 |
  2426550.9 |
  2432633.4 |
  2438715.8 |
  2444798.3 |
  2450880.7 |
  2456963.2 |########################################
  2463045.6 |########################################
  2469128.1 |
  2475210.5 |
  2481293.0 |
  2487375.4 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 2341669.2-2548926.7 ns)
  2341669.2 |#############
  2352032.1 |
  2362395.0 |
  2372757.8 |########################################
  2383120.7 |
  2393483.6 |
  2403846.5 |
  2414209.3 |
  2424572.2 |
  2434935.1 |
  2445298.0 |
  2455660.8 |
  2466023.7 |
  2476386.6 |
  2486749.5 |#############
  2497112.3 |
  2507475.2 |
  2517838.1 |
  2528201.0 |
  2538563.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=2455.9% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=307.5% of algo (FFI overhead may distort results)
