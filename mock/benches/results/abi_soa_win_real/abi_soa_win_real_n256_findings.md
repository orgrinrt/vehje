# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 3.14 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 28064% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (3.14 us) leads abi_soa_win_real_soa_payload (884.52 us) by 28064%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.15 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 685.3x slower than the field

abi_soa_win_real_scalar_payload (2.15 ms) is 685.3x the fastest (3.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_real_null_entry shows alternating (throttle bounce) (autocorr -0.65)

abi_soa_win_real_null_entry's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 685.3x the fastest

Fastest abi_soa_win_real_null_entry (3.14 us) to slowest abi_soa_win_real_scalar_payload (2.15 ms): 685.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 3140.6 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 685.30x (fastest 3140.6 ns, slowest 2152283.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 5555ns | 5429ns | 5385ns | 5417ns | 5848ns | -99.74% |
| abi_soa_win_real_scalar_payload | 2154813ns | 2155268ns | 2151331ns | 2154222ns | 2157441ns | base |
| abi_soa_win_real_soa_payload | 887228ns | 887074ns | 884382ns | 886426ns | 889854ns | -58.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 3196ns | 3107ns | 3335ns | -99.85% | 0.080 |
| abi_soa_win_real_scalar_payload | 2151844ns | 2148572ns | 2154390ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 884671ns | 882025ns | 887132ns | -58.89% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 28532.6 | 3236.1 | 3196.0 | n/a |
| abi_soa_win_real_scalar_payload | 55299.5 | 2153903.7 | 2151843.6 | n/a |
| abi_soa_win_real_soa_payload | 39477.2 | 884684.6 | 884671.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.082 | 98.9% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_real_soa_payload | 0.000 | 0.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 5555ns | 5555ns | -99.74% |
| abi_soa_win_real_scalar_payload | 2154813ns | 2154813ns | base |
| abi_soa_win_real_soa_payload | 887228ns | 887228ns | -58.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2152283ns | base | --- | [2148858, 2154390] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 3141ns | -2149155.2ns (-99.9%) | [-2151161, -2145626]ns | [3112, 3335] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 884520ns | -1268684.6ns (-58.9%) | [-1270631, -1262201]ns | [882362, 887132] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2153088ns | -99.9% | -59.0% |
| 2 | 2149143ns | -99.8% | -58.7% |
| 3 | 2148572ns | -99.9% | -58.8% |
| 4 | 2155691ns | -99.8% | -58.9% |
| 5 | 2151668ns | -99.9% | -58.9% |
| 6 | 2152899ns | -99.9% | -59.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.652 | HIGH- (thermal bounce) |
| abi_soa_win_real_scalar_payload | -0.225 | moderate- |
| abi_soa_win_real_soa_payload | -0.155 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 121808.6ns | 3196.0ns | 3811.3% | HIGH |
| abi_soa_win_real_scalar_payload | 6516619.7ns | 2151843.6ns | 302.8% | HIGH |
| abi_soa_win_real_soa_payload | 2695202.6ns | 884671.3ns | 304.7% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 3106.7-3334.8 ns)
   3106.7 |########################################
   3118.1 |########################################
   3129.5 |########################################
   3140.9 |########################################
   3152.3 |
   3163.7 |
   3175.1 |
   3186.5 |
   3197.9 |
   3209.3 |
   3220.8 |
   3232.2 |
   3243.6 |
   3255.0 |
   3266.4 |
   3277.8 |
   3289.2 |
   3300.6 |
   3312.0 |
   3323.4 |########################################
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2148572.5-2154389.5 ns)
  2148572.5 |########################################
  2148863.4 |########################################
  2149154.2 |
  2149445.1 |
  2149735.9 |
  2150026.8 |
  2150317.6 |
  2150608.5 |
  2150899.3 |
  2151190.2 |
  2151481.0 |########################################
  2151771.9 |
  2152062.7 |
  2152353.6 |
  2152644.4 |########################################
  2152935.3 |########################################
  2153226.1 |
  2153517.0 |
  2153807.8 |
  2154098.7 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 882025.4-887131.5 ns)
  882025.4 |########################################
  882280.7 |
  882536.0 |########################################
  882791.3 |
  883046.6 |
  883301.9 |
  883557.2 |
  883812.5 |########################################
  884067.8 |
  884323.1 |
  884578.4 |
  884833.8 |########################################
  885089.1 |
  885344.4 |
  885599.7 |
  885855.0 |########################################
  886110.3 |
  886365.6 |
  886620.9 |
  886876.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=3871.6% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=304.9% of algo (FFI overhead may distort results)
