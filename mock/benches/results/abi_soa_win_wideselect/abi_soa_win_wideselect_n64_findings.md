# abi_soa_win (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_wideselect_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_wideselect_scalar_payload has the worst median (2.05 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_wideselect_null_entry at 2.57 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_wideselect_null_entry dominates: 35632% faster than the next best (abi_soa_win_wideselect_soa_payload)

abi_soa_win_wideselect_null_entry (2.57 us) leads abi_soa_win_wideselect_soa_payload (918.77 us) by 35632%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_wideselect_null_entry beats baseline by 100% (significant)

abi_soa_win_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_soa_win_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_wideselect_scalar_payload is an outlier: 798.1x slower than the field

abi_soa_win_wideselect_scalar_payload (2.05 ms) is 798.1x the fastest (2.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 798.1x the fastest

Fastest abi_soa_win_wideselect_null_entry (2.57 us) to slowest abi_soa_win_wideselect_scalar_payload (2.05 ms): 798.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_wideselect_null_entry** at 2571.2 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 798.14x (fastest 2571.2 ns, slowest 2052216.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 4888ns | 4905ns | 4739ns | 4897ns | 4949ns | -99.76% |
| abi_soa_win_wideselect_scalar_payload | 2056003ns | 2054858ns | 2052920ns | 2054682ns | 2059528ns | base |
| abi_soa_win_wideselect_soa_payload | 919324ns | 921176ns | 908702ns | 920772ns | 922463ns | -55.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 2565ns | 2505ns | 2595ns | -99.88% | 0.025 |
| abi_soa_win_wideselect_scalar_payload | 2053374ns | 2050495ns | 2056850ns | base | 0.000 |
| abi_soa_win_wideselect_soa_payload | 916921ns | 906323ns | 920035ns | -55.35% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 28936.1 | 2811.3 | 2564.6 | n/a |
| abi_soa_win_wideselect_scalar_payload | 41437.1 | 2054379.6 | 2053374.5 | n/a |
| abi_soa_win_wideselect_soa_payload | 33479.4 | 916863.5 | 916921.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_soa_win_wideselect_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_wideselect_null_entry | 0.025 | 97.4% |
| abi_soa_win_wideselect_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_wideselect_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 4888ns | 4888ns | -99.76% |
| abi_soa_win_wideselect_scalar_payload | 2056003ns | 2056003ns | base |
| abi_soa_win_wideselect_soa_payload | 919324ns | 919324ns | -55.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_scalar_payload | 2052217ns | base | --- | [2051057, 2056850] | --- | --- | --- | --- |
| abi_soa_win_wideselect_null_entry | 2571ns | -2049645.6ns (-99.9%) | [-2054294, -2048490]ns | [2527, 2595] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_wideselect_soa_payload | 918765ns | -1133276.0ns (-55.2%) | [-1143901, -1132182]ns | [911964, 920035] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_wideselect_scalar_payload | abi_soa_win_wideselect_null_entry | abi_soa_win_wideselect_soa_payload |
|---|---|---|---|
| 1 | 2052342ns | -99.9% | -55.2% |
| 2 | 2050495ns | -99.9% | -55.8% |
| 3 | 2051618ns | -99.9% | -55.3% |
| 4 | 2052092ns | -99.9% | -55.2% |
| 5 | 2052465ns | -99.9% | -55.2% |
| 6 | 2061235ns | -99.9% | -55.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_wideselect_null_entry | -0.394 | moderate- |
| abi_soa_win_wideselect_scalar_payload | 0.056 | ok |
| abi_soa_win_wideselect_soa_payload | -0.200 | ok |

**Consistency summary:**

- **abi_soa_win_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_wideselect_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 115083.8ns | 2564.6ns | 4487.4% | HIGH |
| abi_soa_win_wideselect_scalar_payload | 6207402.5ns | 2053374.5ns | 302.3% | HIGH |
| abi_soa_win_wideselect_soa_payload | 2786032.3ns | 916921.3ns | 303.8% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_wideselect_null_entry (n=6, range 2505.0-2595.4 ns)
   2505.0 |########################################
   2509.5 |
   2514.0 |
   2518.6 |
   2523.1 |
   2527.6 |
   2532.1 |
   2536.6 |
   2541.2 |
   2545.7 |########################################
   2550.2 |
   2554.7 |
   2559.2 |########################################
   2563.8 |
   2568.3 |
   2572.8 |
   2577.3 |########################################
   2581.8 |########################################
   2586.4 |
   2590.9 |
  (0 below, 1 above range)

abi_soa_win_wideselect_scalar_payload (n=6, range 2050495.0-2056849.8 ns)
  2050495.0 |####################
  2050812.7 |
  2051130.5 |
  2051448.2 |####################
  2051766.0 |
  2052083.7 |########################################
  2052401.4 |####################
  2052719.2 |
  2053036.9 |
  2053354.7 |
  2053672.4 |
  2053990.1 |
  2054307.9 |
  2054625.6 |
  2054943.4 |
  2055261.1 |
  2055578.8 |
  2055896.6 |
  2056214.3 |
  2056532.1 |
  (0 below, 1 above range)

abi_soa_win_wideselect_soa_payload (n=6, range 906322.9-920034.8 ns)
  906322.9 |####################
  907008.5 |
  907694.1 |
  908379.7 |
  909065.3 |
  909750.9 |
  910436.5 |
  911122.1 |
  911807.7 |
  912493.3 |
  913178.9 |
  913864.4 |
  914550.0 |
  915235.6 |
  915921.2 |
  916606.8 |
  917292.4 |####################
  917978.0 |####################
  918663.6 |
  919349.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_wideselect_null_entry**: bridge=4464.4% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_scalar_payload**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_soa_payload**: bridge=303.7% of algo (FFI overhead may distort results)
