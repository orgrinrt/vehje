# abi_soa_win (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_scatter_scalar_payload has the worst median (2.16 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_scatter_null_entry at 2.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_scatter_null_entry dominates: 38964% faster than the next best (abi_soa_win_scatter_soa_payload)

abi_soa_win_scatter_null_entry (2.29 us) leads abi_soa_win_scatter_soa_payload (893.67 us) by 38964%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_scatter_null_entry beats baseline by 100% (significant)

abi_soa_win_scatter_null_entry is -2.16 ms (100%) faster than baseline abi_soa_win_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_scatter_scalar_payload is an outlier: 944.4x slower than the field

abi_soa_win_scatter_scalar_payload (2.16 ms) is 944.4x the fastest (2.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 944.4x the fastest

Fastest abi_soa_win_scatter_null_entry (2.29 us) to slowest abi_soa_win_scatter_scalar_payload (2.16 ms): 944.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_scatter_null_entry** at 2287.7 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 944.42x (fastest 2287.7 ns, slowest 2160556.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 4604ns | 4606ns | 4474ns | 4599ns | 4677ns | -99.79% |
| abi_soa_win_scatter_scalar_payload | 2185542ns | 2163899ns | 2148073ns | 2163190ns | 2237805ns | base |
| abi_soa_win_scatter_soa_payload | 913591ns | 896365ns | 891995ns | 895811ns | 951058ns | -58.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 2279ns | 2241ns | 2304ns | -99.90% | 0.014 |
| abi_soa_win_scatter_scalar_payload | 2182062ns | 2145060ns | 2233881ns | base | 0.000 |
| abi_soa_win_scatter_soa_payload | 910730ns | 889508ns | 947732ns | -58.26% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 27594.2 | 2421.1 | 2279.3 | n/a |
| abi_soa_win_scatter_scalar_payload | 68079.9 | 2180102.6 | 2182062.3 | n/a |
| abi_soa_win_scatter_soa_payload | 48974.7 | 963473.7 | 910730.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_soa_win_scatter_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_scatter_null_entry | 0.014 | 97.9% |
| abi_soa_win_scatter_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_scatter_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_scatter_null_entry | 4604ns | 4604ns | -99.79% |
| abi_soa_win_scatter_scalar_payload | 2185542ns | 2185542ns | base |
| abi_soa_win_scatter_soa_payload | 913591ns | 913591ns | -58.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_scatter_scalar_payload | 2160557ns | base | --- | [2151749, 2233881] | --- | --- | --- | --- |
| abi_soa_win_scatter_null_entry | 2288ns | -2158269.0ns (-99.9%) | [-2231608, -2149472]ns | [2246, 2304] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_scatter_soa_payload | 893673ns | -1266897.3ns (-58.6%) | [-1310760, -1236339]ns | [890785, 947732] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_scatter_scalar_payload | abi_soa_win_scatter_null_entry | abi_soa_win_scatter_soa_payload |
|---|---|---|---|
| 1 | 2145060ns | -99.9% | -58.5% |
| 2 | 2159696ns | -99.9% | -58.7% |
| 3 | 2158439ns | -99.9% | -58.7% |
| 4 | 2299023ns | -99.9% | -58.6% |
| 5 | 2161418ns | -99.9% | -56.3% |
| 6 | 2168738ns | -99.9% | -58.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_scatter_null_entry | 0.098 | ok |
| abi_soa_win_scatter_scalar_payload | -0.212 | moderate- |
| abi_soa_win_scatter_soa_payload | 0.199 | ok |

**Consistency summary:**

- **abi_soa_win_scatter_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_scatter_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 116495.5ns | 2279.3ns | 5111.0% | HIGH |
| abi_soa_win_scatter_scalar_payload | 6640333.7ns | 2182062.3ns | 304.3% | HIGH |
| abi_soa_win_scatter_soa_payload | 2827661.5ns | 910730.1ns | 310.5% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_scatter_null_entry (n=6, range 2240.8-2303.8 ns)
   2240.8 |########################################
   2243.9 |
   2247.1 |
   2250.2 |########################################
   2253.4 |
   2256.5 |
   2259.7 |
   2262.8 |
   2266.0 |
   2269.1 |
   2272.3 |
   2275.4 |########################################
   2278.6 |
   2281.7 |
   2284.9 |
   2288.0 |
   2291.2 |
   2294.3 |########################################
   2297.5 |
   2300.6 |########################################
  (0 below, 1 above range)

abi_soa_win_scatter_scalar_payload (n=6, range 2145059.6-2233880.8 ns)
  2145059.6 |#############
  2149500.7 |
  2153941.7 |
  2158382.8 |########################################
  2162823.8 |
  2167264.9 |#############
  2171706.0 |
  2176147.0 |
  2180588.1 |
  2185029.1 |
  2189470.2 |
  2193911.3 |
  2198352.3 |
  2202793.4 |
  2207234.4 |
  2211675.5 |
  2216116.6 |
  2220557.6 |
  2224998.7 |
  2229439.7 |
  (0 below, 1 above range)

abi_soa_win_scatter_soa_payload (n=6, range 889508.3-947732.5 ns)
  889508.3 |########################################
  892419.5 |#############
  895330.7 |
  898241.9 |
  901153.1 |
  904064.4 |
  906975.6 |
  909886.8 |
  912798.0 |
  915709.2 |
  918620.4 |
  921531.6 |
  924442.8 |
  927354.0 |
  930265.2 |
  933176.4 |
  936087.7 |
  938998.9 |
  941910.1 |#############
  944821.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_scatter_null_entry**: bridge=5089.6% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_scalar_payload**: bridge=304.8% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_soa_payload**: bridge=304.9% of algo (FFI overhead may distort results)
