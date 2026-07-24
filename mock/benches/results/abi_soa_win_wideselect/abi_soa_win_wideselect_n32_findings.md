# abi_soa_win (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_wideselect_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_wideselect_scalar_payload has the worst median (2.06 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_wideselect_null_entry at 2.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_wideselect_null_entry dominates: 40328% faster than the next best (abi_soa_win_wideselect_soa_payload)

abi_soa_win_wideselect_null_entry (2.29 us) leads abi_soa_win_wideselect_soa_payload (924.02 us) by 40328%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_wideselect_null_entry beats baseline by 100% (significant)

abi_soa_win_wideselect_null_entry is -2.06 ms (100%) faster than baseline abi_soa_win_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_wideselect_scalar_payload is an outlier: 901.4x slower than the field

abi_soa_win_wideselect_scalar_payload (2.06 ms) is 901.4x the fastest (2.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 901.4x the fastest

Fastest abi_soa_win_wideselect_null_entry (2.29 us) to slowest abi_soa_win_wideselect_scalar_payload (2.06 ms): 901.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_wideselect_null_entry** at 2285.6 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 901.38x (fastest 2285.6 ns, slowest 2060189.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 4587ns | 4641ns | 4341ns | 4569ns | 4737ns | -99.78% |
| abi_soa_win_wideselect_scalar_payload | 2063182ns | 2062701ns | 2057338ns | 2061765ns | 2068229ns | base |
| abi_soa_win_wideselect_soa_payload | 931612ns | 926486ns | 924275ns | 926129ns | 943504ns | -54.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 2287ns | 2194ns | 2365ns | -99.89% | 0.014 |
| abi_soa_win_wideselect_scalar_payload | 2060600ns | 2054874ns | 2065504ns | base | 0.000 |
| abi_soa_win_wideselect_soa_payload | 929080ns | 921941ns | 940742ns | -54.91% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 28381.1 | 2405.4 | 2287.0 | n/a |
| abi_soa_win_wideselect_scalar_payload | 41995.4 | 2060203.5 | 2060599.6 | n/a |
| abi_soa_win_wideselect_soa_payload | 38179.3 | 929689.9 | 929079.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.015 Gops/s** (abi_soa_win_wideselect_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_wideselect_null_entry | 0.014 | 96.0% |
| abi_soa_win_wideselect_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_wideselect_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 4587ns | 4587ns | -99.78% |
| abi_soa_win_wideselect_scalar_payload | 2063182ns | 2063182ns | base |
| abi_soa_win_wideselect_soa_payload | 931612ns | 931612ns | -54.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_scalar_payload | 2060189ns | base | --- | [2056106, 2065504] | --- | --- | --- | --- |
| abi_soa_win_wideselect_null_entry | 2286ns | -2057903.6ns (-99.9%) | [-2063293, -2053741]ns | [2211, 2365] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_wideselect_soa_payload | 924020ns | -1133831.1ns (-55.0%) | [-1143027, -1117702]ns | [922477, 940742] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_wideselect_scalar_payload | abi_soa_win_wideselect_null_entry | abi_soa_win_wideselect_soa_payload |
|---|---|---|---|
| 1 | 2061310ns | -99.9% | -53.6% |
| 2 | 2061792ns | -99.9% | -55.3% |
| 3 | 2069215ns | -99.9% | -55.4% |
| 4 | 2059069ns | -99.9% | -55.1% |
| 5 | 2054874ns | -99.9% | -55.0% |
| 6 | 2057337ns | -99.9% | -55.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_wideselect_null_entry | 0.406 | moderate+ |
| abi_soa_win_wideselect_scalar_payload | 0.208 | moderate+ |
| abi_soa_win_wideselect_soa_payload | -0.083 | ok |

**Consistency summary:**

- **abi_soa_win_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_wideselect_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 117726.2ns | 2287.0ns | 5147.6% | HIGH |
| abi_soa_win_wideselect_scalar_payload | 6225538.4ns | 2060599.6ns | 302.1% | HIGH |
| abi_soa_win_wideselect_soa_payload | 2827880.6ns | 929079.6ns | 304.4% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_wideselect_null_entry (n=6, range 2193.8-2364.8 ns)
   2193.8 |########################################
   2202.4 |
   2210.9 |
   2219.5 |########################################
   2228.0 |
   2236.6 |
   2245.1 |
   2253.7 |########################################
   2262.2 |
   2270.8 |
   2279.3 |
   2287.9 |
   2296.4 |
   2305.0 |
   2313.5 |########################################
   2322.1 |
   2330.6 |########################################
   2339.2 |
   2347.7 |
   2356.2 |
  (0 below, 1 above range)

abi_soa_win_wideselect_scalar_payload (n=6, range 2054874.2-2065503.9 ns)
  2054874.2 |########################################
  2055405.7 |
  2055937.2 |
  2056468.7 |
  2057000.1 |########################################
  2057531.6 |
  2058063.1 |
  2058594.6 |########################################
  2059126.1 |
  2059657.6 |
  2060189.1 |
  2060720.6 |
  2061252.1 |########################################
  2061783.5 |########################################
  2062315.0 |
  2062846.5 |
  2063378.0 |
  2063909.5 |
  2064441.0 |
  2064972.5 |
  (0 below, 1 above range)

abi_soa_win_wideselect_soa_payload (n=6, range 921940.8-940741.8 ns)
  921940.8 |####################
  922880.9 |########################################
  923820.9 |####################
  924761.0 |####################
  925701.0 |
  926641.1 |
  927581.1 |
  928521.2 |
  929461.2 |
  930401.3 |
  931341.3 |
  932281.4 |
  933221.4 |
  934161.5 |
  935101.5 |
  936041.6 |
  936981.6 |
  937921.7 |
  938861.7 |
  939801.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_wideselect_null_entry**: bridge=5162.6% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_scalar_payload**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_soa_payload**: bridge=303.9% of algo (FFI overhead may distort results)
