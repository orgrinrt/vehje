# abi_soa_win (tight)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_tight_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_tight_scalar_payload has the worst median (2.02 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_tight_null_entry at 3.57 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_tight_null_entry dominates: 56348% faster than the next best (abi_soa_win_tight_soa_payload)

abi_soa_win_tight_null_entry (3.57 us) leads abi_soa_win_tight_soa_payload (2.02 ms) by 56348%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_tight_null_entry beats baseline by 100% (significant)

abi_soa_win_tight_null_entry is -2.01 ms (100%) faster than baseline abi_soa_win_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_tight_scalar_payload is an outlier: 565.2x slower than the field

abi_soa_win_tight_scalar_payload (2.02 ms) is 565.2x the fastest (3.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 565.2x the fastest

Fastest abi_soa_win_tight_null_entry (3.57 us) to slowest abi_soa_win_tight_scalar_payload (2.02 ms): 565.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_tight_null_entry** at 3570.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 565.23x (fastest 3570.2 ns, slowest 2017968.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 5898ns | 5970ns | 5552ns | 5904ns | 6062ns | -99.71% |
| abi_soa_win_tight_scalar_payload | 2020303ns | 2020718ns | 2010716ns | 2019633ns | 2026102ns | base |
| abi_soa_win_tight_soa_payload | 2084303ns | 2018020ns | 2013639ns | 2017046ns | 2220522ns | +3.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 3548ns | 3358ns | 3650ns | -99.82% | 0.001 |
| abi_soa_win_tight_scalar_payload | 2017619ns | 2008294ns | 2023312ns | base | 0.000 |
| abi_soa_win_tight_soa_payload | 2081422ns | 2010858ns | 2217216ns | +3.16% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 28880.3 | 3627.1 | 3548.5 | n/a |
| abi_soa_win_tight_scalar_payload | 43906.9 | 2017780.4 | 2017618.6 | n/a |
| abi_soa_win_tight_soa_payload | 52317.3 | 2070383.8 | 2081421.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_soa_win_tight_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_tight_null_entry | 0.001 | 94.0% |
| abi_soa_win_tight_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_tight_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_tight_null_entry | 5898ns | 5898ns | -99.71% |
| abi_soa_win_tight_scalar_payload | 2020303ns | 2020303ns | base |
| abi_soa_win_tight_soa_payload | 2084303ns | 2084303ns | +3.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_tight_scalar_payload | 2017968ns | base | --- | [2011575, 2023312] | --- | --- | --- | --- |
| abi_soa_win_tight_null_entry | 3570ns | -2014446.1ns (-99.8%) | [-2019762, -2008002]ns | [3425, 3650] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_tight_soa_payload | 2015322ns | no significant difference | [-6652, +196603]ns | [2011726, 2217216] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_tight_scalar_payload | abi_soa_win_tight_null_entry | abi_soa_win_tight_soa_payload |
|---|---|---|---|
| 1 | 2020252ns | -99.8% | -0.5% |
| 2 | 2015882ns | -99.8% | -0.1% |
| 3 | 2026371ns | -99.8% | +18.1% |
| 4 | 2014857ns | -99.8% | +1.3% |
| 5 | 2008294ns | -99.8% | +0.2% |
| 6 | 2020054ns | -99.8% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_tight_null_entry | 0.140 | ok |
| abi_soa_win_tight_scalar_payload | -0.219 | moderate- |
| abi_soa_win_tight_soa_payload | -0.184 | ok |

**Consistency summary:**

- **abi_soa_win_tight_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_tight_soa_payload**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 122755.9ns | 3548.5ns | 3459.4% | HIGH |
| abi_soa_win_tight_scalar_payload | 6099662.9ns | 2017618.6ns | 302.3% | HIGH |
| abi_soa_win_tight_soa_payload | 6407131.4ns | 2081421.6ns | 307.8% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_tight_null_entry (n=6, range 3357.5-3649.9 ns)
   3357.5 |########################################
   3372.1 |
   3386.7 |
   3401.4 |
   3416.0 |
   3430.6 |
   3445.2 |
   3459.9 |
   3474.5 |
   3489.1 |########################################
   3503.7 |
   3518.3 |
   3533.0 |########################################
   3547.6 |
   3562.2 |
   3576.8 |
   3591.5 |########################################
   3606.1 |
   3620.7 |
   3635.3 |########################################
  (0 below, 1 above range)

abi_soa_win_tight_scalar_payload (n=6, range 2008294.2-2023311.9 ns)
  2008294.2 |####################
  2009045.1 |
  2009796.0 |
  2010546.8 |
  2011297.7 |
  2012048.6 |
  2012799.5 |
  2013550.4 |
  2014301.3 |####################
  2015052.1 |
  2015803.0 |####################
  2016553.9 |
  2017304.8 |
  2018055.7 |
  2018806.6 |
  2019557.4 |########################################
  2020308.3 |
  2021059.2 |
  2021810.1 |
  2022561.0 |
  (0 below, 1 above range)

abi_soa_win_tight_soa_payload (n=6, range 2010858.3-2217216.5 ns)
  2010858.3 |########################################
  2021176.2 |
  2031494.1 |##########
  2041812.0 |
  2052129.9 |
  2062447.9 |
  2072765.8 |
  2083083.7 |
  2093401.6 |
  2103719.5 |
  2114037.4 |
  2124355.3 |
  2134673.2 |
  2144991.1 |
  2155309.0 |
  2165627.0 |
  2175944.9 |
  2186262.8 |
  2196580.7 |
  2206898.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_tight_null_entry**: bridge=3459.9% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_scalar_payload**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_soa_payload**: bridge=302.8% of algo (FFI overhead may distort results)
