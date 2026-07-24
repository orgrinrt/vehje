# abi_soa_win (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_soa_win_wideselect_null_entry dominates: 59130% faster than the next best (abi_soa_win_wideselect_scalar_payload)

abi_soa_win_wideselect_null_entry (3.49 us) leads abi_soa_win_wideselect_scalar_payload (2.07 ms) by 59130%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_wideselect_null_entry beats baseline by 100% (significant)

abi_soa_win_wideselect_null_entry is -2.06 ms (100%) faster than baseline abi_soa_win_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_wideselect_soa_payload is an outlier: 593.0x slower than the field

abi_soa_win_wideselect_soa_payload (2.07 ms) is 593.0x the fastest (3.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 593.0x the fastest

Fastest abi_soa_win_wideselect_null_entry (3.49 us) to slowest abi_soa_win_wideselect_soa_payload (2.07 ms): 593.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_wideselect_null_entry** at 3487.7 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 593.01x (fastest 3487.7 ns, slowest 2068226.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 5787ns | 5805ns | 5518ns | 5716ns | 6027ns | -99.72% |
| abi_soa_win_wideselect_scalar_payload | 2068612ns | 2068433ns | 2066479ns | 2068175ns | 2070334ns | base |
| abi_soa_win_wideselect_soa_payload | 2073285ns | 2070757ns | 2068195ns | 2070371ns | 2080202ns | +0.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 3481ns | 3338ns | 3610ns | -99.83% | 0.001 |
| abi_soa_win_wideselect_scalar_payload | 2065992ns | 2063695ns | 2067768ns | base | 0.000 |
| abi_soa_win_wideselect_soa_payload | 2070628ns | 2065620ns | 2077424ns | +0.22% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 28104.2 | 3569.9 | 3480.6 | n/a |
| abi_soa_win_wideselect_scalar_payload | 40991.2 | 2067748.6 | 2065992.0 | n/a |
| abi_soa_win_wideselect_soa_payload | 44326.9 | 2071863.3 | 2070627.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_soa_win_wideselect_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_wideselect_null_entry | 0.001 | 95.7% |
| abi_soa_win_wideselect_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_wideselect_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 5787ns | 5787ns | -99.72% |
| abi_soa_win_wideselect_scalar_payload | 2068612ns | 2068612ns | base |
| abi_soa_win_wideselect_soa_payload | 2073285ns | 2073285ns | +0.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_scalar_payload | 2065766ns | base | --- | [2064442, 2067768] | --- | --- | --- | --- |
| abi_soa_win_wideselect_null_entry | 3488ns | -2062342.5ns (-99.8%) | [-2064294, -2060898]ns | [3344, 3610] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_wideselect_soa_payload | 2068226ns | no significant difference | [-835, +12952]ns | [2066234, 2077424] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_wideselect_scalar_payload | abi_soa_win_wideselect_null_entry | abi_soa_win_wideselect_soa_payload |
|---|---|---|---|
| 1 | 2065190ns | -99.8% | +0.1% |
| 2 | 2068068ns | -99.8% | +0.0% |
| 3 | 2066285ns | -99.8% | +0.0% |
| 4 | 2063695ns | -99.8% | +0.8% |
| 5 | 2065248ns | -99.8% | +0.5% |
| 6 | 2067468ns | -99.8% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_wideselect_null_entry | -0.029 | ok |
| abi_soa_win_wideselect_scalar_payload | -0.086 | ok |
| abi_soa_win_wideselect_soa_payload | 0.001 | ok |

**Consistency summary:**

- **abi_soa_win_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_wideselect_soa_payload**: won 0/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 121748.8ns | 3480.6ns | 3498.0% | HIGH |
| abi_soa_win_wideselect_scalar_payload | 6245673.0ns | 2065992.0ns | 302.3% | HIGH |
| abi_soa_win_wideselect_soa_payload | 6263556.2ns | 2070627.8ns | 302.5% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_wideselect_null_entry (n=6, range 3338.3-3610.2 ns)
   3338.3 |########################################
   3351.9 |
   3365.5 |
   3379.1 |
   3392.7 |
   3406.3 |
   3419.9 |
   3433.5 |
   3447.1 |
   3460.7 |
   3474.2 |####################
   3487.8 |####################
   3501.4 |
   3515.0 |
   3528.6 |
   3542.2 |
   3555.8 |
   3569.4 |
   3583.0 |
   3596.6 |####################
  (0 below, 1 above range)

abi_soa_win_wideselect_scalar_payload (n=6, range 2063694.6-2067767.5 ns)
  2063694.6 |####################
  2063898.2 |
  2064101.9 |
  2064305.5 |
  2064509.2 |
  2064712.8 |
  2064916.5 |
  2065120.1 |########################################
  2065323.8 |
  2065527.4 |
  2065731.1 |
  2065934.7 |
  2066138.3 |####################
  2066342.0 |
  2066545.6 |
  2066749.3 |
  2066952.9 |
  2067156.6 |
  2067360.2 |####################
  2067563.9 |
  (0 below, 1 above range)

abi_soa_win_wideselect_soa_payload (n=6, range 2065620.0-2077423.5 ns)
  2065620.0 |####################
  2066210.2 |
  2066800.4 |####################
  2067390.5 |
  2067980.7 |########################################
  2068570.9 |
  2069161.1 |
  2069751.2 |
  2070341.4 |
  2070931.6 |
  2071521.8 |
  2072112.0 |
  2072702.1 |
  2073292.3 |
  2073882.5 |
  2074472.7 |
  2075062.8 |####################
  2075653.0 |
  2076243.2 |
  2076833.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_wideselect_null_entry**: bridge=3517.6% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_scalar_payload**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_soa_payload**: bridge=302.4% of algo (FFI overhead may distort results)
