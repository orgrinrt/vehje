# abi_soa_win (tight)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_tight_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_tight_scalar_payload has the worst median (2.01 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_tight_null_entry at 4.05 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_tight_null_entry dominates: 49553% faster than the next best (abi_soa_win_tight_soa_payload)

abi_soa_win_tight_null_entry (4.05 us) leads abi_soa_win_tight_soa_payload (2.01 ms) by 49553%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_tight_null_entry beats baseline by 100% (significant)

abi_soa_win_tight_null_entry is -2.01 ms (100%) faster than baseline abi_soa_win_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_tight_scalar_payload is an outlier: 497.3x slower than the field

abi_soa_win_tight_scalar_payload (2.01 ms) is 497.3x the fastest (4.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 497.3x the fastest

Fastest abi_soa_win_tight_null_entry (4.05 us) to slowest abi_soa_win_tight_scalar_payload (2.01 ms): 497.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_tight_null_entry** at 4047.5 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 497.33x (fastest 4047.5 ns, slowest 2012941.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 6350ns | 6359ns | 6055ns | 6267ns | 6621ns | -99.71% |
| abi_soa_win_tight_scalar_payload | 2167019ns | 2015626ns | 2009411ns | 2013593ns | 2475961ns | base |
| abi_soa_win_tight_soa_payload | 2019291ns | 2012371ns | 2007259ns | 2011298ns | 2037297ns | -6.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 4049ns | 3879ns | 4220ns | -99.81% | 0.001 |
| abi_soa_win_tight_scalar_payload | 2164236ns | 2006960ns | 2472731ns | base | 0.000 |
| abi_soa_win_tight_soa_payload | 2016612ns | 2004793ns | 2034421ns | -6.82% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 27667.8 | 4192.4 | 4049.2 | n/a |
| abi_soa_win_tight_scalar_payload | 48108.0 | 2112504.5 | 2164236.4 | n/a |
| abi_soa_win_tight_soa_payload | 42493.9 | 2016301.3 | 2016611.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_soa_win_tight_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_tight_null_entry | 0.001 | 95.8% |
| abi_soa_win_tight_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_tight_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_tight_null_entry | 6350ns | 6350ns | -99.71% |
| abi_soa_win_tight_scalar_payload | 2167019ns | 2167019ns | base |
| abi_soa_win_tight_soa_payload | 2019291ns | 2019291ns | -6.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_tight_scalar_payload | 2012941ns | base | --- | [2007037, 2472731] | --- | --- | --- | --- |
| abi_soa_win_tight_null_entry | 4048ns | -2009061.2ns (-99.8%) | [-2468663, -2002837]ns | [3880, 4220] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_tight_soa_payload | 2009719ns | no significant difference | [-443413, +7785]ns | [2005696, 2034421] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_tight_scalar_payload | abi_soa_win_tight_null_entry | abi_soa_win_tight_soa_payload |
|---|---|---|---|
| 1 | 2007114ns | -99.8% | +0.6% |
| 2 | 2006960ns | -99.8% | +0.2% |
| 3 | 2010023ns | -99.8% | -0.3% |
| 4 | 2544198ns | -99.8% | -21.1% |
| 5 | 2401264ns | -99.8% | -14.6% |
| 6 | 2015859ns | -99.8% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_tight_null_entry | -0.390 | moderate- |
| abi_soa_win_tight_scalar_payload | 0.153 | ok |
| abi_soa_win_tight_soa_payload | -0.310 | moderate- |

**Consistency summary:**

- **abi_soa_win_tight_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_tight_soa_payload**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 122132.1ns | 4049.2ns | 3016.2% | HIGH |
| abi_soa_win_tight_scalar_payload | 6357038.6ns | 2164236.4ns | 293.7% | HIGH |
| abi_soa_win_tight_soa_payload | 6099003.2ns | 2016611.9ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_tight_null_entry (n=6, range 3879.2-4220.4 ns)
   3879.2 |########################################
   3896.3 |
   3913.3 |
   3930.4 |
   3947.4 |
   3964.5 |
   3981.6 |
   3998.6 |
   4015.7 |
   4032.7 |########################################
   4049.8 |
   4066.9 |
   4083.9 |####################
   4101.0 |
   4118.0 |
   4135.1 |
   4152.2 |
   4169.2 |
   4186.3 |
   4203.3 |
  (0 below, 1 above range)

abi_soa_win_tight_scalar_payload (n=6, range 2006960.0-2472731.0 ns)
  2006960.0 |########################################
  2030248.6 |
  2053537.1 |
  2076825.7 |
  2100114.2 |
  2123402.8 |
  2146691.3 |
  2169979.9 |
  2193268.4 |
  2216557.0 |
  2239845.5 |
  2263134.1 |
  2286422.6 |
  2309711.2 |
  2332999.7 |
  2356288.3 |
  2379576.8 |##########
  2402865.4 |
  2426153.9 |
  2449442.5 |
  (0 below, 1 above range)

abi_soa_win_tight_soa_payload (n=6, range 2004793.3-2034420.8 ns)
  2004793.3 |########################################
  2006274.7 |########################################
  2007756.1 |########################################
  2009237.4 |
  2010718.8 |########################################
  2012200.2 |
  2013681.6 |
  2015162.9 |
  2016644.3 |
  2018125.7 |########################################
  2019607.1 |
  2021088.4 |
  2022569.8 |
  2024051.2 |
  2025532.6 |
  2027013.9 |
  2028495.3 |
  2029976.7 |
  2031458.1 |
  2032939.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_tight_null_entry**: bridge=3022.4% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_scalar_payload**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_soa_payload**: bridge=302.7% of algo (FFI overhead may distort results)
