# abi_soa_win (tight)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_tight_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_tight_scalar_payload has the worst median (2.01 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_tight_null_entry at 3.03 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_tight_null_entry dominates: 31793% faster than the next best (abi_soa_win_tight_soa_payload)

abi_soa_win_tight_null_entry (3.03 us) leads abi_soa_win_tight_soa_payload (965.95 us) by 31793%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_tight_null_entry beats baseline by 100% (significant)

abi_soa_win_tight_null_entry is -2.01 ms (100%) faster than baseline abi_soa_win_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_tight_scalar_payload is an outlier: 663.9x slower than the field

abi_soa_win_tight_scalar_payload (2.01 ms) is 663.9x the fastest (3.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 663.9x the fastest

Fastest abi_soa_win_tight_null_entry (3.03 us) to slowest abi_soa_win_tight_scalar_payload (2.01 ms): 663.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_tight_null_entry** at 3028.8 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 663.95x (fastest 3028.8 ns, slowest 2010932.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 5399ns | 5405ns | 5171ns | 5344ns | 5596ns | -99.73% |
| abi_soa_win_tight_scalar_payload | 2014148ns | 2013575ns | 2007882ns | 2011930ns | 2020608ns | base |
| abi_soa_win_tight_soa_payload | 968269ns | 968515ns | 965416ns | 967857ns | 970315ns | -51.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 3067ns | 2965ns | 3196ns | -99.85% | 0.003 |
| abi_soa_win_tight_scalar_payload | 2011490ns | 2005331ns | 2017839ns | base | 0.000 |
| abi_soa_win_tight_soa_payload | 965736ns | 962931ns | 967794ns | -51.99% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 26903.6 | 3155.2 | 3066.9 | n/a |
| abi_soa_win_tight_scalar_payload | 40420.1 | 2010675.1 | 2011490.3 | n/a |
| abi_soa_win_tight_soa_payload | 33612.6 | 963255.7 | 965735.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_soa_win_tight_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_tight_null_entry | 0.003 | 97.9% |
| abi_soa_win_tight_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_tight_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_tight_null_entry | 5399ns | 5399ns | -99.73% |
| abi_soa_win_tight_scalar_payload | 2014148ns | 2014148ns | base |
| abi_soa_win_tight_soa_payload | 968269ns | 968269ns | -51.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_tight_scalar_payload | 2010932ns | base | --- | [2005700, 2017839] | --- | --- | --- | --- |
| abi_soa_win_tight_null_entry | 3029ns | -2007956.9ns (-99.9%) | [-2014695, -2002618]ns | [2975, 3196] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_tight_soa_payload | 965951ns | -1043682.7ns (-51.9%) | [-1053552, -1040028]ns | [963463, 967794] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_tight_scalar_payload | abi_soa_win_tight_null_entry | abi_soa_win_tight_soa_payload |
|---|---|---|---|
| 1 | 2005331ns | -99.8% | -51.9% |
| 2 | 2017130ns | -99.8% | -52.1% |
| 3 | 2018548ns | -99.8% | -52.3% |
| 4 | 2014386ns | -99.9% | -51.9% |
| 5 | 2006068ns | -99.9% | -51.8% |
| 6 | 2007479ns | -99.9% | -51.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_tight_null_entry | 0.471 | moderate+ |
| abi_soa_win_tight_scalar_payload | 0.182 | ok |
| abi_soa_win_tight_soa_payload | -0.110 | ok |

**Consistency summary:**

- **abi_soa_win_tight_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_tight_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 120005.3ns | 3066.9ns | 3913.0% | HIGH |
| abi_soa_win_tight_scalar_payload | 6073978.1ns | 2011490.3ns | 302.0% | HIGH |
| abi_soa_win_tight_soa_payload | 2922724.8ns | 965735.8ns | 302.6% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_tight_null_entry (n=6, range 2965.0-3196.4 ns)
   2965.0 |########################################
   2976.6 |########################################
   2988.1 |
   2999.7 |########################################
   3011.3 |
   3022.9 |
   3034.4 |
   3046.0 |########################################
   3057.6 |
   3069.2 |
   3080.7 |
   3092.3 |
   3103.9 |
   3115.4 |
   3127.0 |
   3138.6 |
   3150.2 |
   3161.7 |########################################
   3173.3 |
   3184.9 |
  (0 below, 1 above range)

abi_soa_win_tight_scalar_payload (n=6, range 2005331.2-2017838.9 ns)
  2005331.2 |########################################
  2005956.6 |########################################
  2006582.0 |
  2007207.4 |########################################
  2007832.8 |
  2008458.1 |
  2009083.5 |
  2009708.9 |
  2010334.3 |
  2010959.7 |
  2011585.1 |
  2012210.5 |
  2012835.8 |
  2013461.2 |
  2014086.6 |########################################
  2014712.0 |
  2015337.4 |
  2015962.8 |
  2016588.2 |########################################
  2017213.6 |
  (0 below, 1 above range)

abi_soa_win_tight_soa_payload (n=6, range 962930.8-967793.9 ns)
  962930.8 |########################################
  963174.0 |
  963417.1 |
  963660.3 |
  963903.4 |########################################
  964146.6 |
  964389.7 |
  964632.9 |
  964876.1 |
  965119.2 |
  965362.4 |
  965605.5 |########################################
  965848.7 |
  966091.8 |########################################
  966335.0 |
  966578.2 |
  966821.3 |
  967064.5 |########################################
  967307.6 |
  967550.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_tight_null_entry**: bridge=3957.8% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_scalar_payload**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_soa_payload**: bridge=302.4% of algo (FFI overhead may distort results)
