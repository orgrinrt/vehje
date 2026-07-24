# abi_soa_win (tight)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_tight_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_tight_scalar_payload has the worst median (2.01 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_tight_null_entry at 2.60 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_tight_null_entry dominates: 36717% faster than the next best (abi_soa_win_tight_soa_payload)

abi_soa_win_tight_null_entry (2.60 us) leads abi_soa_win_tight_soa_payload (959.09 us) by 36717%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_tight_null_entry beats baseline by 100% (significant)

abi_soa_win_tight_null_entry is -2.01 ms (100%) faster than baseline abi_soa_win_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_tight_scalar_payload is an outlier: 770.8x slower than the field

abi_soa_win_tight_scalar_payload (2.01 ms) is 770.8x the fastest (2.60 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 770.8x the fastest

Fastest abi_soa_win_tight_null_entry (2.60 us) to slowest abi_soa_win_tight_scalar_payload (2.01 ms): 770.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_tight_null_entry** at 2605.0 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 770.84x (fastest 2605.0 ns, slowest 2008048.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 4951ns | 4961ns | 4710ns | 4934ns | 5097ns | -99.75% |
| abi_soa_win_tight_scalar_payload | 2010923ns | 2010736ns | 2006980ns | 2009531ns | 2014985ns | base |
| abi_soa_win_tight_soa_payload | 963265ns | 961575ns | 960196ns | 961428ns | 967555ns | -52.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 2600ns | 2490ns | 2660ns | -99.87% | 0.006 |
| abi_soa_win_tight_scalar_payload | 2008279ns | 2004296ns | 2012358ns | base | 0.000 |
| abi_soa_win_tight_soa_payload | 960783ns | 957741ns | 965022ns | -52.16% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 27755.2 | 2685.1 | 2599.8 | n/a |
| abi_soa_win_tight_scalar_payload | 39760.8 | 2009386.7 | 2008278.9 | n/a |
| abi_soa_win_tight_soa_payload | 33747.5 | 960015.3 | 960782.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_soa_win_tight_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_tight_null_entry | 0.006 | 95.6% |
| abi_soa_win_tight_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_tight_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_tight_null_entry | 4951ns | 4951ns | -99.75% |
| abi_soa_win_tight_scalar_payload | 2010923ns | 2010923ns | base |
| abi_soa_win_tight_soa_payload | 963265ns | 963265ns | -52.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_tight_scalar_payload | 2008049ns | base | --- | [2004430, 2012358] | --- | --- | --- | --- |
| abi_soa_win_tight_null_entry | 2605ns | -2005398.4ns (-99.9%) | [-2009822, -2001817]ns | [2535, 2660] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_tight_soa_payload | 959089ns | -1049115.1ns (-52.2%) | [-1052655, -1040719]ns | [958238, 965022] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_tight_scalar_payload | abi_soa_win_tight_null_entry | abi_soa_win_tight_soa_payload |
|---|---|---|---|
| 1 | 2011937ns | -99.9% | -52.3% |
| 2 | 2004564ns | -99.9% | -51.7% |
| 3 | 2006250ns | -99.9% | -52.2% |
| 4 | 2004296ns | -99.9% | -52.2% |
| 5 | 2012780ns | -99.9% | -52.2% |
| 6 | 2009848ns | -99.9% | -52.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_tight_null_entry | -0.159 | ok |
| abi_soa_win_tight_scalar_payload | -0.126 | ok |
| abi_soa_win_tight_soa_payload | -0.379 | moderate- |

**Consistency summary:**

- **abi_soa_win_tight_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_tight_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 118639.4ns | 2599.8ns | 4563.4% | HIGH |
| abi_soa_win_tight_scalar_payload | 6067067.6ns | 2008278.9ns | 302.1% | HIGH |
| abi_soa_win_tight_soa_payload | 2913111.1ns | 960782.8ns | 303.2% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_tight_null_entry (n=6, range 2490.0-2659.8 ns)
   2490.0 |####################
   2498.5 |
   2507.0 |
   2515.5 |
   2524.0 |
   2532.4 |
   2540.9 |
   2549.4 |
   2557.9 |
   2566.4 |
   2574.9 |########################################
   2583.4 |
   2591.9 |
   2600.4 |
   2608.9 |
   2617.4 |
   2625.8 |####################
   2634.3 |
   2642.8 |####################
   2651.3 |
  (0 below, 1 above range)

abi_soa_win_tight_scalar_payload (n=6, range 2004296.2-2012358.1 ns)
  2004296.2 |########################################
  2004699.3 |
  2005102.4 |
  2005505.5 |
  2005908.6 |####################
  2006311.7 |
  2006714.8 |
  2007117.9 |
  2007521.0 |
  2007924.1 |
  2008327.2 |
  2008730.3 |
  2009133.4 |
  2009536.5 |####################
  2009939.6 |
  2010342.7 |
  2010745.8 |
  2011148.9 |
  2011552.0 |####################
  2011955.1 |
  (0 below, 1 above range)

abi_soa_win_tight_soa_payload (n=6, range 957740.8-965022.1 ns)
  957740.8 |####################
  958104.9 |
  958468.9 |####################
  958833.0 |########################################
  959197.1 |
  959561.1 |
  959925.2 |
  960289.2 |
  960653.3 |
  961017.4 |
  961381.4 |####################
  961745.5 |
  962109.6 |
  962473.6 |
  962837.7 |
  963201.7 |
  963565.8 |
  963929.9 |
  964293.9 |
  964658.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_tight_null_entry**: bridge=4562.6% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_scalar_payload**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_soa_payload**: bridge=303.2% of algo (FFI overhead may distort results)
