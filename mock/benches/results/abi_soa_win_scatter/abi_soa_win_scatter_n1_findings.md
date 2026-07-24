# abi_soa_win (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_scatter_scalar_payload has the worst median (2.16 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_scatter_null_entry at 4.86 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_scatter_null_entry dominates: 44255% faster than the next best (abi_soa_win_scatter_soa_payload)

abi_soa_win_scatter_null_entry (4.86 us) leads abi_soa_win_scatter_soa_payload (2.15 ms) by 44255%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_scatter_null_entry beats baseline by 100% (significant)

abi_soa_win_scatter_null_entry is -2.15 ms (100%) faster than baseline abi_soa_win_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_scatter_scalar_payload is an outlier: 443.7x slower than the field

abi_soa_win_scatter_scalar_payload (2.16 ms) is 443.7x the fastest (4.86 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 443.7x the fastest

Fastest abi_soa_win_scatter_null_entry (4.86 us) to slowest abi_soa_win_scatter_scalar_payload (2.16 ms): 443.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_scatter_null_entry** at 4857.3 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 443.71x (fastest 4857.3 ns, slowest 2155221.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 7255ns | 7137ns | 6968ns | 7121ns | 7600ns | -99.66% |
| abi_soa_win_scatter_scalar_payload | 2156265ns | 2158186ns | 2142897ns | 2156436ns | 2162691ns | base |
| abi_soa_win_scatter_soa_payload | 2212935ns | 2157410ns | 2150515ns | 2157070ns | 2327942ns | +2.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 4947ns | 4752ns | 5200ns | -99.77% | 0.000 |
| abi_soa_win_scatter_scalar_payload | 2153321ns | 2140167ns | 2159741ns | base | 0.000 |
| abi_soa_win_scatter_soa_payload | 2209784ns | 2147627ns | 2324317ns | +2.62% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 27416.2 | 5010.8 | 4947.4 | n/a |
| abi_soa_win_scatter_scalar_payload | 52043.5 | 2150526.1 | 2153320.9 | n/a |
| abi_soa_win_scatter_soa_payload | 65729.7 | 2272097.7 | 2209784.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_soa_win_scatter_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_scatter_null_entry | 0.000 | 97.8% |
| abi_soa_win_scatter_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_scatter_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_scatter_null_entry | 7255ns | 7255ns | -99.66% |
| abi_soa_win_scatter_scalar_payload | 2156265ns | 2156265ns | base |
| abi_soa_win_scatter_soa_payload | 2212935ns | 2212935ns | +2.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_scatter_scalar_payload | 2155221ns | base | --- | [2145000, 2159741] | --- | --- | --- | --- |
| abi_soa_win_scatter_null_entry | 4857ns | -2150165.4ns (-99.8%) | [-2154934, -2140021]ns | [4785, 5200] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_scatter_soa_payload | 2154451ns | no significant difference | [-6930, +176927]ns | [2150584, 2324317] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_scatter_scalar_payload | abi_soa_win_scatter_null_entry | abi_soa_win_scatter_soa_payload |
|---|---|---|---|
| 1 | 2154378ns | -99.8% | -0.3% |
| 2 | 2140167ns | -99.8% | +0.6% |
| 3 | 2156065ns | -99.8% | -0.1% |
| 4 | 2162203ns | -99.8% | -0.3% |
| 5 | 2157279ns | -99.8% | +0.0% |
| 6 | 2149834ns | -99.8% | +15.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_scatter_null_entry | 0.429 | moderate+ |
| abi_soa_win_scatter_scalar_payload | -0.015 | ok |
| abi_soa_win_scatter_soa_payload | -0.020 | ok |

**Consistency summary:**

- **abi_soa_win_scatter_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_scatter_soa_payload**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 124006.5ns | 4947.4ns | 2506.5% | HIGH |
| abi_soa_win_scatter_scalar_payload | 6512213.7ns | 2153320.9ns | 302.4% | HIGH |
| abi_soa_win_scatter_soa_payload | 6867663.8ns | 2209784.1ns | 310.8% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_scatter_null_entry (n=6, range 4751.7-5199.6 ns)
   4751.7 |####################
   4774.1 |
   4796.5 |
   4818.9 |####################
   4841.3 |########################################
   4863.7 |
   4886.1 |
   4908.5 |
   4930.9 |
   4953.3 |
   4975.6 |
   4998.0 |
   5020.4 |
   5042.8 |
   5065.2 |
   5087.6 |
   5110.0 |
   5132.4 |####################
   5154.8 |
   5177.2 |
  (0 below, 1 above range)

abi_soa_win_scatter_scalar_payload (n=6, range 2140166.7-2159741.0 ns)
  2140166.7 |########################################
  2141145.4 |
  2142124.1 |
  2143102.9 |
  2144081.6 |
  2145060.3 |
  2146039.0 |
  2147017.7 |
  2147996.4 |
  2148975.2 |########################################
  2149953.9 |
  2150932.6 |
  2151911.3 |
  2152890.0 |
  2153868.7 |########################################
  2154847.5 |
  2155826.2 |########################################
  2156804.9 |########################################
  2157783.6 |
  2158762.3 |
  (0 below, 1 above range)

abi_soa_win_scatter_soa_payload (n=6, range 2147627.1-2324317.2 ns)
  2147627.1 |########################################
  2156461.6 |##########
  2165296.1 |
  2174130.6 |
  2182965.1 |
  2191799.6 |
  2200634.1 |
  2209468.7 |
  2218303.2 |
  2227137.7 |
  2235972.2 |
  2244806.7 |
  2253641.2 |
  2262475.7 |
  2271310.2 |
  2280144.7 |
  2288979.2 |
  2297813.7 |
  2306648.2 |
  2315482.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_scatter_null_entry**: bridge=2566.2% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_scalar_payload**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_soa_payload**: bridge=302.5% of algo (FFI overhead may distort results)
