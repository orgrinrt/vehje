# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 2.72 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 33031% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (2.72 us) leads abi_soa_win_real_soa_payload (900.83 us) by 33031%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.15 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 790.3x slower than the field

abi_soa_win_real_scalar_payload (2.15 ms) is 790.3x the fastest (2.72 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 790.3x the fastest

Fastest abi_soa_win_real_null_entry (2.72 us) to slowest abi_soa_win_real_scalar_payload (2.15 ms): 790.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 2718.9 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 790.33x (fastest 2718.9 ns, slowest 2148864.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 4992ns | 4991ns | 4889ns | 4973ns | 5072ns | -99.78% |
| abi_soa_win_real_scalar_payload | 2254199ns | 2151504ns | 2144255ns | 2149665ns | 2465972ns | base |
| abi_soa_win_real_soa_payload | 920373ns | 903522ns | 885008ns | 897711ns | 972049ns | -59.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 2716ns | 2655ns | 2765ns | -99.88% | 0.047 |
| abi_soa_win_real_scalar_payload | 2251366ns | 2141715ns | 2462785ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 917001ns | 882436ns | 967166ns | -59.27% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 27531.2 | 2752.3 | 2715.5 | n/a |
| abi_soa_win_real_scalar_payload | 52317.9 | 2251975.9 | 2251366.2 | n/a |
| abi_soa_win_real_soa_payload | 43626.4 | 914647.8 | 917001.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.047 | 97.7% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_real_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 4992ns | 4992ns | -99.78% |
| abi_soa_win_real_scalar_payload | 2254199ns | 2254199ns | base |
| abi_soa_win_real_soa_payload | 920373ns | 920373ns | -59.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2148864ns | base | --- | [2142450, 2462785] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 2719ns | -2146145.5ns (-99.9%) | [-2460058, -2139749]ns | [2663, 2765] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 900828ns | -1262413.6ns (-58.7%) | [-1561957, -1178724]ns | [883011, 967166] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2149134ns | -99.9% | -58.9% |
| 2 | 2141715ns | -99.9% | -58.8% |
| 3 | 2762236ns | -99.9% | -66.9% |
| 4 | 2143184ns | -99.9% | -54.2% |
| 5 | 2163333ns | -99.9% | -59.0% |
| 6 | 2148595ns | -99.9% | -55.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.170 | ok |
| abi_soa_win_real_scalar_payload | -0.260 | moderate- |
| abi_soa_win_real_soa_payload | -0.222 | moderate- |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 119658.7ns | 2715.5ns | 4406.4% | HIGH |
| abi_soa_win_real_scalar_payload | 6744995.8ns | 2251366.2ns | 299.6% | HIGH |
| abi_soa_win_real_soa_payload | 2835947.7ns | 917001.4ns | 309.3% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 2655.4-2764.6 ns)
   2655.4 |########################################
   2660.9 |
   2666.3 |########################################
   2671.8 |
   2677.2 |
   2682.7 |
   2688.2 |
   2693.6 |
   2699.1 |
   2704.5 |########################################
   2710.0 |
   2715.5 |
   2720.9 |
   2726.4 |########################################
   2731.8 |
   2737.3 |
   2742.8 |########################################
   2748.2 |
   2753.7 |
   2759.1 |
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2141715.0-2462784.5 ns)
  2141715.0 |########################################
  2157768.5 |##########
  2173822.0 |
  2189875.4 |
  2205928.9 |
  2221982.4 |
  2238035.9 |
  2254089.3 |
  2270142.8 |
  2286196.3 |
  2302249.8 |
  2318303.3 |
  2334356.7 |
  2350410.2 |
  2366463.7 |
  2382517.2 |
  2398570.6 |
  2414624.1 |
  2430677.6 |
  2446731.1 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 882435.8-967165.8 ns)
  882435.8 |########################################
  886672.3 |####################
  890908.8 |
  895145.3 |
  899381.8 |
  903618.3 |
  907854.8 |
  912091.3 |####################
  916327.8 |
  920564.3 |
  924800.8 |
  929037.3 |
  933273.8 |
  937510.3 |
  941746.8 |
  945983.3 |
  950219.8 |####################
  954456.3 |
  958692.8 |
  962929.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=4400.9% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=305.1% of algo (FFI overhead may distort results)
