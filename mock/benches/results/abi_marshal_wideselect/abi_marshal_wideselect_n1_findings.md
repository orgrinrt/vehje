# abi_marshal (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_wideselect_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_wideselect_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_wideselect_marshal_null dominates: 20143% faster than the next best (abi_marshal_wideselect_soa_native)

abi_marshal_wideselect_marshal_null (10.07 us) leads abi_marshal_wideselect_soa_native (2.04 ms) by 20143%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_wideselect_marshal_null beats baseline by 99% (significant)

abi_marshal_wideselect_marshal_null is -2.04 ms (99%) faster than baseline abi_marshal_wideselect_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_wideselect_soa_transposed is an outlier: 203.9x slower than the field

abi_marshal_wideselect_soa_transposed (2.05 ms) is 203.9x the fastest (10.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_wideselect_marshal_null shows alternating (throttle bounce) (autocorr -0.64)

abi_marshal_wideselect_marshal_null's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_wideselect_marshal_null} vs {abi_marshal_wideselect_soa_native, abi_marshal_wideselect_aos, abi_marshal_wideselect_soa_transposed} (20143% apart)

The field splits into a fast tier {abi_marshal_wideselect_marshal_null} and a slow tier {abi_marshal_wideselect_soa_native, abi_marshal_wideselect_aos, abi_marshal_wideselect_soa_transposed} with a 20143% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 203.9x the fastest

Fastest abi_marshal_wideselect_marshal_null (10.07 us) to slowest abi_marshal_wideselect_soa_transposed (2.05 ms): 203.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_wideselect_marshal_null** at 10074.2 ns median (-99.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 203.95x (fastest 10074.2 ns, slowest 2054601.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2050602ns | 2050420ns | 2038876ns | 2049761ns | 2057726ns | base |
| abi_marshal_wideselect_marshal_null | 12537ns | 12342ns | 12165ns | 12301ns | 13077ns | -99.39% |
| abi_marshal_wideselect_soa_native | 2042498ns | 2041887ns | 2038307ns | 2040766ns | 2047190ns | -0.40% |
| abi_marshal_wideselect_soa_transposed | 2056956ns | 2057215ns | 2045562ns | 2053781ns | 2067415ns | +0.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2048103ns | 2036544ns | 2055059ns | base | 0.000 |
| abi_marshal_wideselect_marshal_null | 10257ns | 9962ns | 10696ns | -99.50% | 0.000 |
| abi_marshal_wideselect_soa_native | 2039991ns | 2035924ns | 2044663ns | -0.40% | 0.000 |
| abi_marshal_wideselect_soa_transposed | 2054380ns | 2043063ns | 2064798ns | +0.31% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_wideselect_aos | 40203.6 | 2047667.5 | 2048102.8 | n/a |
| abi_marshal_wideselect_marshal_null | 28517.4 | 10371.3 | 10256.9 | n/a |
| abi_marshal_wideselect_soa_native | 39732.4 | 2038734.9 | 2039991.0 | n/a |
| abi_marshal_wideselect_soa_transposed | 41517.2 | 2053256.0 | 2054380.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_wideselect_marshal_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_wideselect_aos | 0.000 | 0.5% |
| abi_marshal_wideselect_marshal_null | 0.000 | 98.9% |
| abi_marshal_wideselect_soa_native | 0.000 | 0.5% |
| abi_marshal_wideselect_soa_transposed | 0.000 | 0.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_wideselect_aos | 2050602ns | 2050602ns | base |
| abi_marshal_wideselect_marshal_null | 12537ns | 12537ns | -99.39% |
| abi_marshal_wideselect_soa_native | 2042498ns | 2042498ns | -0.40% |
| abi_marshal_wideselect_soa_transposed | 2056956ns | 2056956ns | +0.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2048004ns | base | --- | [2041245, 2055059] | --- | --- | --- | --- |
| abi_marshal_wideselect_marshal_null | 10074ns | -2037641.9ns (-99.5%) | [-2045059, -2030837]ns | [10000, 10696] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_wideselect_soa_native | 2039275ns | no significant difference | [-19025, +3418]ns | [2036035, 2044663] | no | 0.3281 | 0.2188 | 0 |
| abi_marshal_wideselect_soa_transposed | 2054602ns | no significant difference | [-5069, +19843]ns | [2043742, 2064798] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_wideselect_aos | abi_marshal_wideselect_marshal_null | abi_marshal_wideselect_soa_native | abi_marshal_wideselect_soa_transposed |
|---|---|---|---|---|
| 1 | 2049110ns | -99.5% | -0.6% | -0.3% |
| 2 | 2048511ns | -99.5% | -0.5% | -0.2% |
| 3 | 2045945ns | -99.5% | -0.2% | +0.9% |
| 4 | 2061009ns | -99.5% | -1.2% | +0.3% |
| 5 | 2047497ns | -99.5% | -0.4% | +0.1% |
| 6 | 2036544ns | -99.5% | +0.6% | +1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_wideselect_aos | -0.095 | ok |
| abi_marshal_wideselect_marshal_null | -0.641 | HIGH- (thermal bounce) |
| abi_marshal_wideselect_soa_native | -0.050 | ok |
| abi_marshal_wideselect_soa_transposed | 0.136 | ok |

**Consistency summary:**

- **abi_marshal_wideselect_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_wideselect_soa_native**: won 5/6, lost 1/6
- **abi_marshal_wideselect_soa_transposed**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_wideselect_aos | 6189701.0ns | 2048102.8ns | 302.2% | HIGH |
| abi_marshal_wideselect_marshal_null | 140878.7ns | 10256.9ns | 1373.5% | HIGH |
| abi_marshal_wideselect_soa_native | 6164887.5ns | 2039991.0ns | 302.2% | HIGH |
| abi_marshal_wideselect_soa_transposed | 6202390.3ns | 2054380.4ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_marshal_wideselect_aos (n=6, range 2036544.2-2055059.4 ns)
  2036544.2 |########################################
  2037470.0 |
  2038395.7 |
  2039321.5 |
  2040247.2 |
  2041173.0 |
  2042098.7 |
  2043024.5 |
  2043950.3 |
  2044876.0 |
  2045801.8 |########################################
  2046727.5 |########################################
  2047653.3 |########################################
  2048579.0 |########################################
  2049504.8 |
  2050430.6 |
  2051356.3 |
  2052282.1 |
  2053207.8 |
  2054133.6 |
  (0 below, 1 above range)

abi_marshal_wideselect_marshal_null (n=6, range 9961.7-10696.2 ns)
   9961.7 |####################
   9998.4 |
  10035.2 |########################################
  10071.9 |####################
  10108.6 |
  10145.3 |
  10182.1 |
  10218.8 |
  10255.5 |
  10292.2 |
  10329.0 |
  10365.7 |
  10402.4 |
  10439.2 |
  10475.9 |
  10512.6 |
  10549.3 |
  10586.1 |
  10622.8 |####################
  10659.5 |
  (0 below, 1 above range)

abi_marshal_wideselect_soa_native (n=6, range 2035924.2-2044662.9 ns)
  2035924.2 |########################################
  2036361.1 |
  2036798.1 |
  2037235.0 |
  2037671.9 |
  2038108.9 |
  2038545.8 |
  2038982.7 |########################################
  2039419.7 |
  2039856.6 |
  2040293.5 |
  2040730.5 |
  2041167.4 |####################
  2041604.4 |
  2042041.3 |
  2042478.2 |
  2042915.2 |
  2043352.1 |
  2043789.0 |
  2044226.0 |
  (0 below, 1 above range)

abi_marshal_wideselect_soa_transposed (n=6, range 2043062.9-2064797.5 ns)
  2043062.9 |########################################
  2044149.6 |########################################
  2045236.4 |
  2046323.1 |
  2047409.8 |
  2048496.5 |
  2049583.3 |########################################
  2050670.0 |
  2051756.7 |
  2052843.5 |
  2053930.2 |
  2055016.9 |
  2056103.7 |
  2057190.4 |
  2058277.1 |########################################
  2059363.8 |
  2060450.6 |
  2061537.3 |
  2062624.0 |########################################
  2063710.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_wideselect_aos**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_marshal_null**: bridge=1385.9% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_soa_native**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_soa_transposed**: bridge=302.2% of algo (FFI overhead may distort results)
