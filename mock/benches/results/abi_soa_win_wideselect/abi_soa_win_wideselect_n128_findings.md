# abi_soa_win (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_wideselect_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_wideselect_scalar_payload has the worst median (2.05 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_wideselect_null_entry at 2.78 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_wideselect_null_entry dominates: 33072% faster than the next best (abi_soa_win_wideselect_soa_payload)

abi_soa_win_wideselect_null_entry (2.78 us) leads abi_soa_win_wideselect_soa_payload (922.19 us) by 33072%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_wideselect_null_entry beats baseline by 100% (significant)

abi_soa_win_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_soa_win_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_wideselect_scalar_payload is an outlier: 739.0x slower than the field

abi_soa_win_wideselect_scalar_payload (2.05 ms) is 739.0x the fastest (2.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 739.0x the fastest

Fastest abi_soa_win_wideselect_null_entry (2.78 us) to slowest abi_soa_win_wideselect_scalar_payload (2.05 ms): 739.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_wideselect_null_entry** at 2780.0 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 739.02x (fastest 2780.0 ns, slowest 2054476.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 5135ns | 5139ns | 4935ns | 5129ns | 5243ns | -99.75% |
| abi_soa_win_wideselect_scalar_payload | 2072142ns | 2056987ns | 2045229ns | 2055725ns | 2110226ns | base |
| abi_soa_win_wideselect_soa_payload | 932358ns | 924593ns | 917100ns | 922433ns | 954873ns | -55.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 2787ns | 2689ns | 2862ns | -99.87% | 0.046 |
| abi_soa_win_wideselect_scalar_payload | 2069476ns | 2042825ns | 2107262ns | base | 0.000 |
| abi_soa_win_wideselect_soa_payload | 929864ns | 914820ns | 952161ns | -55.07% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 28642.4 | 2868.8 | 2787.4 | n/a |
| abi_soa_win_wideselect_scalar_payload | 46297.5 | 2083774.7 | 2069475.6 | n/a |
| abi_soa_win_wideselect_soa_payload | 37859.0 | 930031.2 | 929863.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_soa_win_wideselect_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_wideselect_null_entry | 0.046 | 96.7% |
| abi_soa_win_wideselect_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_wideselect_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 5135ns | 5135ns | -99.75% |
| abi_soa_win_wideselect_scalar_payload | 2072142ns | 2072142ns | base |
| abi_soa_win_wideselect_soa_payload | 932358ns | 932358ns | -55.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_scalar_payload | 2054476ns | base | --- | [2046689, 2107262] | --- | --- | --- | --- |
| abi_soa_win_wideselect_null_entry | 2780ns | -2051680.4ns (-99.9%) | [-2104434, -2043950]ns | [2720, 2862] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_wideselect_soa_payload | 922186ns | -1138136.2ns (-55.4%) | [-1155100, -1125599]ns | [915243, 952161] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_wideselect_scalar_payload | abi_soa_win_wideselect_null_entry | abi_soa_win_wideselect_soa_payload |
|---|---|---|---|
| 1 | 2056206ns | -99.9% | -55.5% |
| 2 | 2042825ns | -99.9% | -54.9% |
| 3 | 2052746ns | -99.9% | -55.1% |
| 4 | 2050553ns | -99.9% | -55.4% |
| 5 | 2100318ns | -99.9% | -54.7% |
| 6 | 2114205ns | -99.9% | -54.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_wideselect_null_entry | -0.230 | moderate- |
| abi_soa_win_wideselect_scalar_payload | 0.427 | moderate+ |
| abi_soa_win_wideselect_soa_payload | 0.295 | moderate+ |

**Consistency summary:**

- **abi_soa_win_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_wideselect_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 121251.2ns | 2787.4ns | 4349.9% | HIGH |
| abi_soa_win_wideselect_scalar_payload | 6353231.4ns | 2069475.6ns | 307.0% | HIGH |
| abi_soa_win_wideselect_soa_payload | 2837903.2ns | 929863.7ns | 305.2% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_wideselect_null_entry (n=6, range 2689.2-2862.3 ns)
   2689.2 |########################################
   2697.9 |
   2706.5 |
   2715.2 |
   2723.8 |
   2732.5 |
   2741.1 |
   2749.8 |########################################
   2758.4 |
   2767.1 |########################################
   2775.8 |
   2784.4 |########################################
   2793.1 |
   2801.7 |
   2810.4 |
   2819.0 |########################################
   2827.7 |
   2836.3 |
   2845.0 |
   2853.6 |
  (0 below, 1 above range)

abi_soa_win_wideselect_scalar_payload (n=6, range 2042825.0-2107261.6 ns)
  2042825.0 |########################################
  2046046.8 |
  2049268.7 |########################################
  2052490.5 |########################################
  2055712.3 |########################################
  2058934.2 |
  2062156.0 |
  2065377.8 |
  2068599.7 |
  2071821.5 |
  2075043.3 |
  2078265.2 |
  2081487.0 |
  2084708.8 |
  2087930.7 |
  2091152.5 |
  2094374.3 |
  2097596.2 |########################################
  2100818.0 |
  2104039.8 |
  (0 below, 1 above range)

abi_soa_win_wideselect_soa_payload (n=6, range 914820.0-952161.2 ns)
  914820.0 |########################################
  916687.1 |
  918554.1 |
  920421.2 |####################
  922288.2 |####################
  924155.3 |
  926022.4 |
  927889.4 |
  929756.5 |
  931623.6 |
  933490.6 |
  935357.7 |
  937224.8 |
  939091.8 |
  940958.9 |
  942825.9 |
  944693.0 |
  946560.1 |
  948427.1 |
  950294.2 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_wideselect_null_entry**: bridge=4368.2% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_scalar_payload**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_soa_payload**: bridge=303.9% of algo (FFI overhead may distort results)
