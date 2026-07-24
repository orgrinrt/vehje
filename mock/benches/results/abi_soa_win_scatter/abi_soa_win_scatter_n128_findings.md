# abi_soa_win (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_scatter_scalar_payload has the worst median (2.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_scatter_null_entry at 2.68 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_scatter_null_entry dominates: 33073% faster than the next best (abi_soa_win_scatter_soa_payload)

abi_soa_win_scatter_null_entry (2.68 us) leads abi_soa_win_scatter_soa_payload (890.02 us) by 33073%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_scatter_null_entry beats baseline by 100% (significant)

abi_soa_win_scatter_null_entry is -2.15 ms (100%) faster than baseline abi_soa_win_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_scatter_scalar_payload is an outlier: 800.8x slower than the field

abi_soa_win_scatter_scalar_payload (2.15 ms) is 800.8x the fastest (2.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 800.8x the fastest

Fastest abi_soa_win_scatter_null_entry (2.68 us) to slowest abi_soa_win_scatter_scalar_payload (2.15 ms): 800.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_scatter_null_entry** at 2682.9 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 800.82x (fastest 2682.9 ns, slowest 2148570.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 5072ns | 4984ns | 4877ns | 4961ns | 5335ns | -99.77% |
| abi_soa_win_scatter_scalar_payload | 2186844ns | 2151712ns | 2147595ns | 2150757ns | 2260598ns | base |
| abi_soa_win_scatter_soa_payload | 949907ns | 892726ns | 890607ns | 892340ns | 1065907ns | -56.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 2724ns | 2652ns | 2828ns | -99.88% | 0.047 |
| abi_soa_win_scatter_scalar_payload | 2183538ns | 2144547ns | 2256747ns | base | 0.000 |
| abi_soa_win_scatter_soa_payload | 946663ns | 888126ns | 1061416ns | -56.65% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 27198.1 | 2759.0 | 2723.8 | n/a |
| abi_soa_win_scatter_scalar_payload | 61792.2 | 2187981.2 | 2183538.1 | n/a |
| abi_soa_win_scatter_soa_payload | 48400.5 | 934942.4 | 946663.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_soa_win_scatter_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_scatter_null_entry | 0.048 | 98.8% |
| abi_soa_win_scatter_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_scatter_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_scatter_null_entry | 5072ns | 5072ns | -99.77% |
| abi_soa_win_scatter_scalar_payload | 2186844ns | 2186844ns | base |
| abi_soa_win_scatter_soa_payload | 949907ns | 949907ns | -56.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_scatter_scalar_payload | 2148570ns | base | --- | [2145297, 2256747] | --- | --- | --- | --- |
| abi_soa_win_scatter_null_entry | 2683ns | -2145887.0ns (-99.9%) | [-2253920, -2142636]ns | [2661, 2828] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_scatter_soa_payload | 890022ns | -1256631.7ns (-58.5%) | [-1259376, -1194617]ns | [888551, 1061416] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_scatter_scalar_payload | abi_soa_win_scatter_null_entry | abi_soa_win_scatter_soa_payload |
|---|---|---|---|
| 1 | 2362410ns | -99.9% | -48.1% |
| 2 | 2146048ns | -99.9% | -58.6% |
| 3 | 2144547ns | -99.9% | -58.5% |
| 4 | 2149505ns | -99.9% | -58.6% |
| 5 | 2151084ns | -99.9% | -58.4% |
| 6 | 2147635ns | -99.9% | -58.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_scatter_null_entry | -0.268 | moderate- |
| abi_soa_win_scatter_scalar_payload | -0.043 | ok |
| abi_soa_win_scatter_soa_payload | -0.043 | ok |

**Consistency summary:**

- **abi_soa_win_scatter_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_scatter_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 119584.0ns | 2723.8ns | 4390.4% | HIGH |
| abi_soa_win_scatter_scalar_payload | 6630918.0ns | 2183538.1ns | 303.7% | HIGH |
| abi_soa_win_scatter_soa_payload | 2862580.3ns | 946663.2ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_scatter_null_entry (n=6, range 2651.7-2827.5 ns)
   2651.7 |####################
   2660.5 |
   2669.3 |########################################
   2678.1 |
   2686.9 |####################
   2695.6 |
   2704.4 |
   2713.2 |
   2722.0 |
   2730.8 |
   2739.6 |
   2748.4 |
   2757.2 |
   2766.0 |
   2774.8 |####################
   2783.6 |
   2792.3 |
   2801.1 |
   2809.9 |
   2818.7 |
  (0 below, 1 above range)

abi_soa_win_scatter_scalar_payload (n=6, range 2144546.7-2256747.1 ns)
  2144546.7 |########################################
  2150156.7 |##########
  2155766.7 |
  2161376.8 |
  2166986.8 |
  2172596.8 |
  2178206.8 |
  2183816.8 |
  2189426.9 |
  2195036.9 |
  2200646.9 |
  2206256.9 |
  2211866.9 |
  2217477.0 |
  2223087.0 |
  2228697.0 |
  2234307.0 |
  2239917.0 |
  2245527.1 |
  2251137.1 |
  (0 below, 1 above range)

abi_soa_win_scatter_soa_payload (n=6, range 888126.2-1061416.2 ns)
  888126.2 |########################################
  896790.7 |
  905455.2 |
  914119.7 |
  922784.2 |
  931448.7 |
  940113.2 |
  948777.7 |
  957442.2 |
  966106.7 |
  974771.2 |
  983435.7 |
  992100.2 |
  1000764.7 |
  1009429.2 |
  1018093.7 |
  1026758.2 |
  1035422.7 |
  1044087.2 |
  1052751.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_scatter_null_entry**: bridge=4412.8% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_scalar_payload**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_soa_payload**: bridge=305.3% of algo (FFI overhead may distort results)
