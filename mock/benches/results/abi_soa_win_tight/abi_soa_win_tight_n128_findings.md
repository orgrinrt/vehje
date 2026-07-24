# abi_soa_win (tight)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_tight_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_tight_scalar_payload has the worst median (2.00 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_tight_null_entry at 2.76 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_tight_null_entry dominates: 34440% faster than the next best (abi_soa_win_tight_soa_payload)

abi_soa_win_tight_null_entry (2.76 us) leads abi_soa_win_tight_soa_payload (954.32 us) by 34440%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_tight_null_entry beats baseline by 100% (significant)

abi_soa_win_tight_null_entry is -2.00 ms (100%) faster than baseline abi_soa_win_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_tight_scalar_payload is an outlier: 725.2x slower than the field

abi_soa_win_tight_scalar_payload (2.00 ms) is 725.2x the fastest (2.76 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 725.2x the fastest

Fastest abi_soa_win_tight_null_entry (2.76 us) to slowest abi_soa_win_tight_scalar_payload (2.00 ms): 725.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_tight_null_entry** at 2762.9 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 725.22x (fastest 2762.9 ns, slowest 2003759.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 5113ns | 5101ns | 4890ns | 5076ns | 5280ns | -99.75% |
| abi_soa_win_tight_scalar_payload | 2007517ns | 2006275ns | 2002612ns | 2005731ns | 2012648ns | base |
| abi_soa_win_tight_soa_payload | 956251ns | 956670ns | 952624ns | 956528ns | 957650ns | -52.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 2757ns | 2684ns | 2812ns | -99.86% | 0.046 |
| abi_soa_win_tight_scalar_payload | 2004959ns | 1999992ns | 2010002ns | base | 0.000 |
| abi_soa_win_tight_soa_payload | 953860ns | 950297ns | 955201ns | -52.42% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 27152.8 | 2789.8 | 2757.0 | n/a |
| abi_soa_win_tight_scalar_payload | 37247.0 | 2005797.8 | 2004958.6 | n/a |
| abi_soa_win_tight_soa_payload | 32068.1 | 954137.3 | 953859.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_soa_win_tight_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_tight_null_entry | 0.046 | 97.1% |
| abi_soa_win_tight_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_tight_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_tight_null_entry | 5113ns | 5113ns | -99.75% |
| abi_soa_win_tight_scalar_payload | 2007517ns | 2007517ns | base |
| abi_soa_win_tight_soa_payload | 956251ns | 956251ns | -52.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_tight_scalar_payload | 2003760ns | base | --- | [2001114, 2010002] | --- | --- | --- | --- |
| abi_soa_win_tight_null_entry | 2763ns | -2000959.4ns (-99.9%) | [-2007248, -1998398]ns | [2696, 2812] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_tight_soa_payload | 954316ns | -1051696.9ns (-52.5%) | [-1055323, -1046276]ns | [952063, 955201] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_tight_scalar_payload | abi_soa_win_tight_null_entry | abi_soa_win_tight_soa_payload |
|---|---|---|---|
| 1 | 2011075ns | -99.9% | -52.6% |
| 2 | 2008928ns | -99.9% | -52.4% |
| 3 | 2002734ns | -99.9% | -52.6% |
| 4 | 2004786ns | -99.9% | -52.4% |
| 5 | 2002236ns | -99.9% | -52.3% |
| 6 | 1999992ns | -99.9% | -52.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_tight_null_entry | 0.088 | ok |
| abi_soa_win_tight_scalar_payload | 0.331 | moderate+ |
| abi_soa_win_tight_soa_payload | -0.261 | moderate- |

**Consistency summary:**

- **abi_soa_win_tight_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_tight_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 119734.5ns | 2757.0ns | 4343.0% | HIGH |
| abi_soa_win_tight_scalar_payload | 6058537.4ns | 2004958.6ns | 302.2% | HIGH |
| abi_soa_win_tight_soa_payload | 2895322.9ns | 953859.9ns | 303.5% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_tight_null_entry (n=6, range 2683.7-2811.5 ns)
   2683.7 |########################################
   2690.1 |
   2696.5 |
   2702.9 |########################################
   2709.3 |
   2715.6 |
   2722.0 |
   2728.4 |
   2734.8 |
   2741.2 |
   2747.6 |########################################
   2754.0 |
   2760.4 |
   2766.8 |
   2773.2 |########################################
   2779.6 |
   2785.9 |
   2792.3 |
   2798.7 |########################################
   2805.1 |
  (0 below, 1 above range)

abi_soa_win_tight_scalar_payload (n=6, range 1999992.5-2010001.6 ns)
  1999992.5 |########################################
  2000493.0 |
  2000993.4 |
  2001493.9 |
  2001994.3 |########################################
  2002494.8 |########################################
  2002995.2 |
  2003495.7 |
  2003996.2 |
  2004496.6 |########################################
  2004997.1 |
  2005497.5 |
  2005998.0 |
  2006498.4 |
  2006998.9 |
  2007499.4 |
  2007999.8 |
  2008500.3 |########################################
  2009000.7 |
  2009501.2 |
  (0 below, 1 above range)

abi_soa_win_tight_soa_payload (n=6, range 950296.7-955201.1 ns)
  950296.7 |####################
  950541.9 |
  950787.1 |
  951032.4 |
  951277.6 |
  951522.8 |
  951768.0 |
  952013.2 |
  952258.4 |
  952503.7 |
  952748.9 |
  952994.1 |
  953239.3 |
  953484.5 |
  953729.7 |########################################
  953975.0 |
  954220.2 |
  954465.4 |####################
  954710.6 |
  954955.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_tight_null_entry**: bridge=4340.4% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_scalar_payload**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_soa_payload**: bridge=303.5% of algo (FFI overhead may distort results)
