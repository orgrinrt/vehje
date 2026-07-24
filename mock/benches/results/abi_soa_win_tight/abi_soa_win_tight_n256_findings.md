# abi_soa_win (tight)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_tight_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_tight_scalar_payload has the worst median (2.00 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_tight_null_entry at 3.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_tight_null_entry dominates: 28888% faster than the next best (abi_soa_win_tight_soa_payload)

abi_soa_win_tight_null_entry (3.29 us) leads abi_soa_win_tight_soa_payload (953.16 us) by 28888%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_tight_null_entry beats baseline by 100% (significant)

abi_soa_win_tight_null_entry is -2.00 ms (100%) faster than baseline abi_soa_win_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_tight_scalar_payload is an outlier: 608.9x slower than the field

abi_soa_win_tight_scalar_payload (2.00 ms) is 608.9x the fastest (3.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_tight_null_entry shows alternating (throttle bounce) (autocorr -0.57)

abi_soa_win_tight_null_entry's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 608.9x the fastest

Fastest abi_soa_win_tight_null_entry (3.29 us) to slowest abi_soa_win_tight_scalar_payload (2.00 ms): 608.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_tight_null_entry** at 3288.1 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 608.87x (fastest 3288.1 ns, slowest 2002018.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 5607ns | 5614ns | 5439ns | 5585ns | 5724ns | -99.72% |
| abi_soa_win_tight_scalar_payload | 2004905ns | 2004614ns | 2000848ns | 2003986ns | 2008313ns | base |
| abi_soa_win_tight_soa_payload | 954411ns | 955529ns | 942366ns | 954793ns | 959861ns | -52.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 3265ns | 3130ns | 3334ns | -99.84% | 0.078 |
| abi_soa_win_tight_scalar_payload | 2002349ns | 1998363ns | 2005718ns | base | 0.000 |
| abi_soa_win_tight_soa_payload | 951992ns | 940046ns | 957304ns | -52.46% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 27390.7 | 3281.2 | 3265.2 | n/a |
| abi_soa_win_tight_scalar_payload | 36909.2 | 2003214.2 | 2002349.2 | n/a |
| abi_soa_win_tight_soa_payload | 32284.8 | 952537.8 | 951991.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_soa_win_tight_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_tight_null_entry | 0.078 | 95.2% |
| abi_soa_win_tight_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_tight_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_tight_null_entry | 5607ns | 5607ns | -99.72% |
| abi_soa_win_tight_scalar_payload | 2004905ns | 2004905ns | base |
| abi_soa_win_tight_soa_payload | 954411ns | 954411ns | -52.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_tight_scalar_payload | 2002018ns | base | --- | [1999311, 2005718] | --- | --- | --- | --- |
| abi_soa_win_tight_null_entry | 3288ns | -1998724.4ns (-99.8%) | [-2002429, -1996099]ns | [3174, 3334] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_tight_soa_payload | 953164ns | -1048328.3ns (-52.4%) | [-1058790, -1043954]ns | [945507, 957304] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_tight_scalar_payload | abi_soa_win_tight_null_entry | abi_soa_win_tight_soa_payload |
|---|---|---|---|
| 1 | 2006867ns | -99.8% | -52.5% |
| 2 | 2000260ns | -99.8% | -52.0% |
| 3 | 2000421ns | -99.8% | -52.3% |
| 4 | 2004569ns | -99.8% | -52.3% |
| 5 | 1998363ns | -99.8% | -52.4% |
| 6 | 2003615ns | -99.8% | -53.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_tight_null_entry | -0.573 | HIGH- (thermal bounce) |
| abi_soa_win_tight_scalar_payload | -0.463 | moderate- |
| abi_soa_win_tight_soa_payload | 0.146 | ok |

**Consistency summary:**

- **abi_soa_win_tight_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_tight_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_tight_null_entry | 121458.5ns | 3265.2ns | 3719.8% | HIGH |
| abi_soa_win_tight_scalar_payload | 6052670.9ns | 2002349.2ns | 302.3% | HIGH |
| abi_soa_win_tight_soa_payload | 2891418.8ns | 951991.6ns | 303.7% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_tight_null_entry (n=6, range 3130.0-3333.8 ns)
   3130.0 |####################
   3140.2 |
   3150.4 |
   3160.6 |
   3170.8 |
   3180.9 |
   3191.1 |
   3201.3 |
   3211.5 |####################
   3221.7 |
   3231.9 |
   3242.1 |
   3252.2 |
   3262.4 |
   3272.6 |####################
   3282.8 |
   3293.0 |########################################
   3303.2 |
   3313.4 |
   3323.6 |
  (0 below, 1 above range)

abi_soa_win_tight_scalar_payload (n=6, range 1998363.3-2005718.1 ns)
  1998363.3 |####################
  1998731.0 |
  1999098.8 |
  1999466.5 |
  1999834.3 |
  2000202.0 |########################################
  2000569.8 |
  2000937.5 |
  2001305.2 |
  2001673.0 |
  2002040.7 |
  2002408.5 |
  2002776.2 |
  2003144.0 |
  2003511.7 |####################
  2003879.4 |
  2004247.2 |####################
  2004614.9 |
  2004982.7 |
  2005350.4 |
  (0 below, 1 above range)

abi_soa_win_tight_soa_payload (n=6, range 940046.2-957303.9 ns)
  940046.2 |########################################
  940909.1 |
  941772.0 |
  942634.9 |
  943497.8 |
  944360.6 |
  945223.5 |
  946086.4 |
  946949.3 |
  947812.2 |
  948675.1 |
  949538.0 |
  950400.8 |########################################
  951263.7 |
  952126.6 |########################################
  952989.5 |########################################
  953852.4 |
  954715.3 |########################################
  955578.2 |
  956441.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_tight_null_entry**: bridge=3694.5% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_scalar_payload**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_soa_win_tight_soa_payload**: bridge=303.8% of algo (FFI overhead may distort results)
