# abi_soa_win (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_scatter_scalar_payload has the worst median (2.36 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_scatter_null_entry at 3.23 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_scatter_null_entry dominates: 28186% faster than the next best (abi_soa_win_scatter_soa_payload)

abi_soa_win_scatter_null_entry (3.23 us) leads abi_soa_win_scatter_soa_payload (913.05 us) by 28186%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_scatter_null_entry beats baseline by 100% (significant)

abi_soa_win_scatter_null_entry is -2.36 ms (100%) faster than baseline abi_soa_win_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_scatter_scalar_payload is an outlier: 730.7x slower than the field

abi_soa_win_scatter_scalar_payload (2.36 ms) is 730.7x the fastest (3.23 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 730.7x the fastest

Fastest abi_soa_win_scatter_null_entry (3.23 us) to slowest abi_soa_win_scatter_scalar_payload (2.36 ms): 730.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_scatter_null_entry** at 3227.9 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 730.66x (fastest 3227.9 ns, slowest 2358487.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 5542ns | 5560ns | 5342ns | 5542ns | 5644ns | -99.78% |
| abi_soa_win_scatter_scalar_payload | 2513067ns | 2362508ns | 2190123ns | 2323633ns | 2958688ns | base |
| abi_soa_win_scatter_soa_payload | 914715ns | 916114ns | 896524ns | 910694ns | 929843ns | -63.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 3219ns | 3126ns | 3266ns | -99.87% | 0.080 |
| abi_soa_win_scatter_scalar_payload | 2508960ns | 2186672ns | 2954025ns | base | 0.000 |
| abi_soa_win_scatter_soa_payload | 911646ns | 893705ns | 926468ns | -63.66% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 28905.1 | 3221.9 | 3218.9 | n/a |
| abi_soa_win_scatter_scalar_payload | 80056.2 | 2487205.5 | 2508959.6 | n/a |
| abi_soa_win_scatter_soa_payload | 51362.8 | 913641.3 | 911645.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_soa_win_scatter_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_scatter_null_entry | 0.079 | 96.8% |
| abi_soa_win_scatter_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_scatter_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_scatter_null_entry | 5542ns | 5542ns | -99.78% |
| abi_soa_win_scatter_scalar_payload | 2513067ns | 2513067ns | base |
| abi_soa_win_scatter_soa_payload | 914715ns | 914715ns | -63.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_scatter_scalar_payload | 2358487ns | base | --- | [2214367, 2954025] | --- | --- | --- | --- |
| abi_soa_win_scatter_null_entry | 3228ns | -2355259.2ns (-99.9%) | [-2950759, -2211204]ns | [3163, 3266] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_scatter_soa_payload | 913049ns | -1451689.4ns (-61.6%) | [-2043882, -1296370]ns | [895420, 926468] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_scatter_scalar_payload | abi_soa_win_scatter_null_entry | abi_soa_win_scatter_soa_payload |
|---|---|---|---|
| 1 | 2798127ns | -99.9% | -68.1% |
| 2 | 3109923ns | -99.9% | -70.2% |
| 3 | 2186672ns | -99.9% | -58.4% |
| 4 | 2437971ns | -99.9% | -63.2% |
| 5 | 2279003ns | -99.9% | -59.8% |
| 6 | 2242061ns | -99.9% | -58.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_scatter_null_entry | -0.310 | moderate- |
| abi_soa_win_scatter_scalar_payload | 0.119 | ok |
| abi_soa_win_scatter_soa_payload | -0.268 | moderate- |

**Consistency summary:**

- **abi_soa_win_scatter_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_scatter_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 122198.2ns | 3218.9ns | 3796.2% | HIGH |
| abi_soa_win_scatter_scalar_payload | 7484060.6ns | 2508959.6ns | 298.3% | HIGH |
| abi_soa_win_scatter_soa_payload | 2792428.5ns | 911645.9ns | 306.3% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_scatter_null_entry (n=6, range 3126.2-3265.6 ns)
   3126.2 |########################################
   3133.2 |
   3140.1 |
   3147.1 |
   3154.1 |
   3161.1 |
   3168.0 |
   3175.0 |
   3182.0 |
   3188.9 |
   3195.9 |########################################
   3202.9 |
   3209.8 |
   3216.8 |########################################
   3223.8 |
   3230.8 |########################################
   3237.7 |
   3244.7 |
   3251.7 |
   3258.6 |########################################
  (0 below, 1 above range)

abi_soa_win_scatter_scalar_payload (n=6, range 2186672.5-2954025.0 ns)
  2186672.5 |########################################
  2225040.1 |########################################
  2263407.8 |########################################
  2301775.4 |
  2340143.0 |
  2378510.6 |
  2416878.2 |########################################
  2455245.9 |
  2493613.5 |
  2531981.1 |
  2570348.8 |
  2608716.4 |
  2647084.0 |
  2685451.6 |
  2723819.2 |
  2762186.9 |########################################
  2800554.5 |
  2838922.1 |
  2877289.8 |
  2915657.4 |
  (0 below, 1 above range)

abi_soa_win_scatter_soa_payload (n=6, range 893705.4-926468.3 ns)
  893705.4 |########################################
  895343.5 |
  896981.7 |########################################
  898619.8 |
  900258.0 |
  901896.1 |
  903534.3 |
  905172.4 |
  906810.6 |
  908448.7 |########################################
  910086.9 |
  911725.0 |
  913363.2 |
  915001.3 |########################################
  916639.5 |
  918277.6 |
  919915.8 |
  921553.9 |
  923192.1 |
  924830.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_scatter_null_entry**: bridge=3803.9% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_scalar_payload**: bridge=300.0% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_soa_payload**: bridge=306.0% of algo (FFI overhead may distort results)
