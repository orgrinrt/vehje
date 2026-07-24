# abi_soa_win (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_scatter_scalar_payload has the worst median (2.16 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_scatter_null_entry at 3.45 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_scatter_null_entry dominates: 62442% faster than the next best (abi_soa_win_scatter_soa_payload)

abi_soa_win_scatter_null_entry (3.45 us) leads abi_soa_win_scatter_soa_payload (2.16 ms) by 62442%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_scatter_null_entry beats baseline by 100% (significant)

abi_soa_win_scatter_null_entry is -2.16 ms (100%) faster than baseline abi_soa_win_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_scatter_scalar_payload is an outlier: 626.5x slower than the field

abi_soa_win_scatter_scalar_payload (2.16 ms) is 626.5x the fastest (3.45 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_scatter_soa_payload shows alternating (throttle bounce) (autocorr -0.71)

abi_soa_win_scatter_soa_payload's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 626.5x the fastest

Fastest abi_soa_win_scatter_null_entry (3.45 us) to slowest abi_soa_win_scatter_scalar_payload (2.16 ms): 626.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_scatter_null_entry** at 3447.5 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 626.52x (fastest 3447.5 ns, slowest 2159918.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 5765ns | 5771ns | 5615ns | 5724ns | 5901ns | -99.73% |
| abi_soa_win_scatter_scalar_payload | 2167343ns | 2163112ns | 2156421ns | 2161245ns | 2181951ns | base |
| abi_soa_win_scatter_soa_payload | 2159711ns | 2159103ns | 2156180ns | 2158397ns | 2163447ns | -0.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 3452ns | 3373ns | 3535ns | -99.84% | 0.001 |
| abi_soa_win_scatter_scalar_payload | 2164200ns | 2153512ns | 2178606ns | base | 0.000 |
| abi_soa_win_scatter_soa_payload | 2156763ns | 2153052ns | 2160523ns | -0.34% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 27188.5 | 3510.2 | 3452.5 | n/a |
| abi_soa_win_scatter_scalar_payload | 57880.4 | 2164142.2 | 2164200.2 | n/a |
| abi_soa_win_scatter_soa_payload | 54035.4 | 2154332.9 | 2156763.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_soa_win_scatter_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_scatter_null_entry | 0.001 | 97.8% |
| abi_soa_win_scatter_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_scatter_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_scatter_null_entry | 5765ns | 5765ns | -99.73% |
| abi_soa_win_scatter_scalar_payload | 2167343ns | 2167343ns | base |
| abi_soa_win_scatter_soa_payload | 2159711ns | 2159711ns | -0.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_scatter_scalar_payload | 2159918ns | base | --- | [2154077, 2178606] | --- | --- | --- | --- |
| abi_soa_win_scatter_null_entry | 3448ns | -2156464.0ns (-99.8%) | [-2175158, -2150621]ns | [3375, 3535] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_scatter_soa_payload | 2156145ns | no significant difference | [-24984, +2615]ns | [2153621, 2160523] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_scatter_scalar_payload | abi_soa_win_scatter_null_entry | abi_soa_win_scatter_soa_payload |
|---|---|---|---|
| 1 | 2193687ns | -99.8% | -1.9% |
| 2 | 2159876ns | -99.8% | +0.1% |
| 3 | 2163525ns | -99.8% | -0.4% |
| 4 | 2159960ns | -99.8% | -0.1% |
| 5 | 2153512ns | -99.8% | +0.1% |
| 6 | 2154641ns | -99.8% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_scatter_null_entry | 0.051 | ok |
| abi_soa_win_scatter_scalar_payload | 0.023 | ok |
| abi_soa_win_scatter_soa_payload | -0.714 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_soa_win_scatter_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_scatter_soa_payload**: won 2/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 119769.6ns | 3452.5ns | 3469.1% | HIGH |
| abi_soa_win_scatter_scalar_payload | 6553215.1ns | 2164200.2ns | 302.8% | HIGH |
| abi_soa_win_scatter_soa_payload | 6521207.0ns | 2156763.1ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_scatter_null_entry (n=6, range 3373.3-3534.6 ns)
   3373.3 |########################################
   3381.4 |
   3389.4 |
   3397.5 |
   3405.6 |
   3413.6 |
   3421.7 |####################
   3429.8 |
   3437.8 |
   3445.9 |
   3453.9 |
   3462.0 |
   3470.1 |####################
   3478.1 |
   3486.2 |
   3494.3 |
   3502.3 |
   3510.4 |
   3518.5 |
   3526.5 |####################
  (0 below, 1 above range)

abi_soa_win_scatter_scalar_payload (n=6, range 2153512.5-2178605.7 ns)
  2153512.5 |########################################
  2154767.2 |
  2156021.8 |
  2157276.5 |
  2158531.1 |
  2159785.8 |########################################
  2161040.4 |
  2162295.1 |####################
  2163549.8 |
  2164804.4 |
  2166059.1 |
  2167313.7 |
  2168568.4 |
  2169823.0 |
  2171077.7 |
  2172332.4 |
  2173587.0 |
  2174841.7 |
  2176096.3 |
  2177351.0 |
  (0 below, 1 above range)

abi_soa_win_scatter_soa_payload (n=6, range 2153052.5-2160522.7 ns)
  2153052.5 |########################################
  2153426.0 |
  2153799.5 |
  2154173.0 |########################################
  2154546.5 |
  2154920.0 |
  2155293.6 |
  2155667.1 |########################################
  2156040.6 |
  2156414.1 |########################################
  2156787.6 |
  2157161.1 |
  2157534.6 |
  2157908.1 |########################################
  2158281.6 |
  2158655.2 |
  2159028.7 |
  2159402.2 |
  2159775.7 |
  2160149.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_scatter_null_entry**: bridge=3465.5% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_scalar_payload**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_soa_payload**: bridge=302.7% of algo (FFI overhead may distort results)
