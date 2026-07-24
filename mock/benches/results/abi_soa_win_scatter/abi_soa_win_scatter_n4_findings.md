# abi_soa_win (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_scatter_scalar_payload has the worst median (2.16 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_scatter_null_entry at 3.92 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_scatter_null_entry dominates: 54755% faster than the next best (abi_soa_win_scatter_soa_payload)

abi_soa_win_scatter_null_entry (3.92 us) leads abi_soa_win_scatter_soa_payload (2.15 ms) by 54755%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_scatter_null_entry beats baseline by 100% (significant)

abi_soa_win_scatter_null_entry is -2.15 ms (100%) faster than baseline abi_soa_win_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_scatter_scalar_payload is an outlier: 549.9x slower than the field

abi_soa_win_scatter_scalar_payload (2.16 ms) is 549.9x the fastest (3.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 549.9x the fastest

Fastest abi_soa_win_scatter_null_entry (3.92 us) to slowest abi_soa_win_scatter_scalar_payload (2.16 ms): 549.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_scatter_null_entry** at 3921.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 549.86x (fastest 3921.2 ns, slowest 2156097.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 6232ns | 6177ns | 6078ns | 6152ns | 6428ns | -99.71% |
| abi_soa_win_scatter_scalar_payload | 2165753ns | 2159249ns | 2154821ns | 2157792ns | 2183161ns | base |
| abi_soa_win_scatter_soa_payload | 2182735ns | 2153868ns | 2148324ns | 2152686ns | 2245015ns | +0.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 3962ns | 3881ns | 4077ns | -99.82% | 0.001 |
| abi_soa_win_scatter_scalar_payload | 2162610ns | 2151889ns | 2179809ns | base | 0.000 |
| abi_soa_win_scatter_soa_payload | 2179712ns | 2145662ns | 2241662ns | +0.79% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 26251.0 | 4091.7 | 3962.2 | n/a |
| abi_soa_win_scatter_scalar_payload | 55997.0 | 2163207.7 | 2162609.6 | n/a |
| abi_soa_win_scatter_soa_payload | 57462.1 | 2169941.2 | 2179712.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_soa_win_scatter_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_scatter_null_entry | 0.001 | 99.0% |
| abi_soa_win_scatter_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_scatter_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_scatter_null_entry | 6232ns | 6232ns | -99.71% |
| abi_soa_win_scatter_scalar_payload | 2165753ns | 2165753ns | base |
| abi_soa_win_scatter_soa_payload | 2182735ns | 2182735ns | +0.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_scatter_scalar_payload | 2156098ns | base | --- | [2151922, 2179809] | --- | --- | --- | --- |
| abi_soa_win_scatter_null_entry | 3921ns | -2152098.8ns (-99.8%) | [-2175893, -2147951]ns | [3889, 4077] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_scatter_soa_payload | 2150962ns | no significant difference | [-15221, +67490]ns | [2146513, 2241662] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_scatter_scalar_payload | abi_soa_win_scatter_null_entry | abi_soa_win_scatter_soa_payload |
|---|---|---|---|
| 1 | 2151956ns | -99.8% | +0.1% |
| 2 | 2156672ns | -99.8% | -0.4% |
| 3 | 2155523ns | -99.8% | +0.3% |
| 4 | 2151889ns | -99.8% | -0.2% |
| 5 | 2166797ns | -99.8% | -1.0% |
| 6 | 2192821ns | -99.8% | +5.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_scatter_null_entry | 0.417 | moderate+ |
| abi_soa_win_scatter_scalar_payload | 0.211 | moderate+ |
| abi_soa_win_scatter_soa_payload | -0.071 | ok |

**Consistency summary:**

- **abi_soa_win_scatter_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_scatter_soa_payload**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 120591.7ns | 3962.2ns | 3043.6% | HIGH |
| abi_soa_win_scatter_scalar_payload | 6545524.2ns | 2162609.6ns | 302.7% | HIGH |
| abi_soa_win_scatter_soa_payload | 6565309.7ns | 2179712.5ns | 301.2% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_scatter_null_entry (n=6, range 3881.2-4076.6 ns)
   3881.2 |########################################
   3891.0 |########################################
   3900.7 |########################################
   3910.5 |
   3920.3 |
   3930.1 |########################################
   3939.8 |
   3949.6 |
   3959.4 |
   3969.2 |
   3978.9 |
   3988.7 |
   3998.5 |
   4008.2 |
   4018.0 |
   4027.8 |
   4037.6 |
   4047.3 |
   4057.1 |########################################
   4066.9 |
  (0 below, 1 above range)

abi_soa_win_scatter_scalar_payload (n=6, range 2151889.2-2179808.8 ns)
  2151889.2 |########################################
  2153285.2 |
  2154681.2 |####################
  2156077.1 |####################
  2157473.1 |
  2158869.1 |
  2160265.1 |
  2161661.0 |
  2163057.0 |
  2164453.0 |
  2165849.0 |####################
  2167245.0 |
  2168640.9 |
  2170036.9 |
  2171432.9 |
  2172828.9 |
  2174224.8 |
  2175620.8 |
  2177016.8 |
  2178412.8 |
  (0 below, 1 above range)

abi_soa_win_scatter_soa_payload (n=6, range 2145662.5-2241661.9 ns)
  2145662.5 |########################################
  2150462.5 |#############
  2155262.4 |
  2160062.4 |#############
  2164862.4 |
  2169662.3 |
  2174462.3 |
  2179262.3 |
  2184062.2 |
  2188862.2 |
  2193662.2 |
  2198462.1 |
  2203262.1 |
  2208062.1 |
  2212862.0 |
  2217662.0 |
  2222462.0 |
  2227261.9 |
  2232061.9 |
  2236861.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_scatter_null_entry**: bridge=3086.4% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_scalar_payload**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_soa_payload**: bridge=302.4% of algo (FFI overhead may distort results)
