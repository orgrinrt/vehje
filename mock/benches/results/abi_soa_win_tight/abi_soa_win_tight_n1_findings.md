# abi_soa_win (tight)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_soa_win_tight_null_entry dominates: 40541% faster than the next best (abi_soa_win_tight_scalar_payload)

abi_soa_win_tight_null_entry (5.01 us) leads abi_soa_win_tight_scalar_payload (2.03 ms) by 40541%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_tight_null_entry beats baseline by 100% (significant)

abi_soa_win_tight_null_entry is -2.03 ms (100%) faster than baseline abi_soa_win_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_tight_soa_payload is an outlier: 407.0x slower than the field

abi_soa_win_tight_soa_payload (2.04 ms) is 407.0x the fastest (5.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 407.0x the fastest

Fastest abi_soa_win_tight_null_entry (5.01 us) to slowest abi_soa_win_tight_soa_payload (2.04 ms): 407.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_tight_null_entry** at 5005.4 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 406.97x (fastest 5005.4 ns, slowest 2037056.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 7281ns | 7378ns | 6929ns | 7255ns | 7497ns | -99.66% |
| abi_soa_win_tight_scalar_payload | 2113412ns | 2037277ns | 2031391ns | 2036217ns | 2270216ns | base |
| abi_soa_win_tight_soa_payload | 2051220ns | 2040183ns | 2028289ns | 2037346ns | 2083498ns | -2.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 4961ns | 4764ns | 5105ns | -99.76% | 0.000 |
| abi_soa_win_tight_scalar_payload | 2110144ns | 2028486ns | 2266295ns | base | 0.000 |
| abi_soa_win_tight_soa_payload | 2048089ns | 2025509ns | 2080012ns | -2.94% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 27801.7 | 5026.0 | 4960.8 | n/a |
| abi_soa_win_tight_scalar_payload | 60677.9 | 2109901.7 | 2110143.8 | n/a |
| abi_soa_win_tight_soa_payload | 57892.0 | 2049481.5 | 2048089.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_soa_win_tight_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_tight_null_entry | 0.000 | 95.2% |
| abi_soa_win_tight_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_tight_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_tight_null_entry | 7281ns | 7281ns | -99.66% |
| abi_soa_win_tight_scalar_payload | 2113412ns | 2113412ns | base |
| abi_soa_win_tight_soa_payload | 2051220ns | 2051220ns | -2.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_tight_scalar_payload | 2034245ns | base | --- | [2029891, 2266295] | --- | --- | --- | --- |
| abi_soa_win_tight_null_entry | 5005ns | -2029473.5ns (-99.8%) | [-2261286, -2024790]ns | [4772, 5105] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_tight_soa_payload | 2037057ns | no significant difference | [-191373, +7168]ns | [2027199, 2080012] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_tight_scalar_payload | abi_soa_win_tight_null_entry | abi_soa_win_tight_soa_payload |
|---|---|---|---|
| 1 | 2031296ns | -99.7% | +0.1% |
| 2 | 2035915ns | -99.8% | -0.5% |
| 3 | 2028486ns | -99.8% | +0.0% |
| 4 | 2487170ns | -99.8% | -15.0% |
| 5 | 2045420ns | -99.8% | -0.2% |
| 6 | 2032576ns | -99.8% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_tight_null_entry | -0.390 | moderate- |
| abi_soa_win_tight_scalar_payload | -0.224 | moderate- |
| abi_soa_win_tight_soa_payload | -0.170 | ok |

**Consistency summary:**

- **abi_soa_win_tight_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_tight_soa_payload**: won 3/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 124367.7ns | 4960.8ns | 2507.0% | HIGH |
| abi_soa_win_tight_scalar_payload | 6432279.6ns | 2110143.8ns | 304.8% | HIGH |
| abi_soa_win_tight_soa_payload | 6203027.3ns | 2048089.4ns | 302.9% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_tight_null_entry (n=6, range 4764.2-5105.4 ns)
   4764.2 |########################################
   4781.3 |
   4798.3 |
   4815.4 |
   4832.4 |
   4849.5 |
   4866.6 |
   4883.6 |
   4900.7 |
   4917.7 |
   4934.8 |
   4951.9 |
   4968.9 |####################
   4986.0 |
   5003.0 |
   5020.1 |
   5037.2 |########################################
   5054.2 |
   5071.3 |
   5088.3 |
  (0 below, 1 above range)

abi_soa_win_tight_scalar_payload (n=6, range 2028486.2-2266295.2 ns)
  2028486.2 |########################################
  2040376.6 |##########
  2052267.1 |
  2064157.6 |
  2076048.0 |
  2087938.4 |
  2099828.9 |
  2111719.4 |
  2123609.8 |
  2135500.2 |
  2147390.7 |
  2159281.1 |
  2171171.6 |
  2183062.1 |
  2194952.5 |
  2206843.0 |
  2218733.4 |
  2230623.9 |
  2242514.3 |
  2254404.8 |
  (0 below, 1 above range)

abi_soa_win_tight_soa_payload (n=6, range 2025508.7-2080012.3 ns)
  2025508.7 |########################################
  2028233.9 |########################################
  2030959.1 |########################################
  2033684.2 |
  2036409.4 |
  2039134.6 |########################################
  2041859.8 |
  2044585.0 |########################################
  2047310.1 |
  2050035.3 |
  2052760.5 |
  2055485.7 |
  2058210.9 |
  2060936.0 |
  2063661.2 |
  2066386.4 |
  2069111.6 |
  2071836.8 |
  2074561.9 |
  2077287.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_tight_null_entry**: bridge=2506.6% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_scalar_payload**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_soa_payload**: bridge=302.7% of algo (FFI overhead may distort results)
