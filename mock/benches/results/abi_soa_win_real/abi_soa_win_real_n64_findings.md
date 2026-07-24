# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 2.60 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 33852% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (2.60 us) leads abi_soa_win_real_soa_payload (882.18 us) by 33852%, a clear separation rather than a photo finish. CV 8.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.15 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 829.1x slower than the field

abi_soa_win_real_scalar_payload (2.15 ms) is 829.1x the fastest (2.60 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_real_null_entry is fastest but the noisiest (CV 8.7%)

abi_soa_win_real_null_entry wins on median (2.60 us) yet has the highest variance (CV 8.7%), while abi_soa_win_real_soa_payload is the steadiest (CV 0.1%, 882.18 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 829.1x the fastest

Fastest abi_soa_win_real_null_entry (2.60 us) to slowest abi_soa_win_real_scalar_payload (2.15 ms): 829.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 2598.3 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 829.06x (fastest 2598.3 ns, slowest 2154195.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 5213ns | 4889ns | 4848ns | 4876ns | 5900ns | -99.76% |
| abi_soa_win_real_scalar_payload | 2155666ns | 2156989ns | 2144396ns | 2154306ns | 2163341ns | base |
| abi_soa_win_real_soa_payload | 885031ns | 884652ns | 884523ns | 884614ns | 885911ns | -58.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 2693ns | 2543ns | 2919ns | -99.87% | 0.024 |
| abi_soa_win_real_scalar_payload | 2152773ns | 2141753ns | 2160161ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 882557ns | 882049ns | 883437ns | -59.00% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 29531.4 | 2964.7 | 2692.8 | n/a |
| abi_soa_win_real_scalar_payload | 48583.3 | 2149775.7 | 2152773.3 | n/a |
| abi_soa_win_real_soa_payload | 35618.4 | 882397.8 | 882556.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.025 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.025 | 97.9% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_real_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 5213ns | 5213ns | -99.76% |
| abi_soa_win_real_scalar_payload | 2155666ns | 2155666ns | base |
| abi_soa_win_real_soa_payload | 885031ns | 885031ns | -58.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2154195ns | base | --- | [2143964, 2160161] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 2598ns | -2151630.0ns (-99.9%) | [-2157242, -2141370]ns | [2561, 2919] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 882182ns | -1272111.9ns (-59.1%) | [-1278012, -1260526]ns | [882051, 883437] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2141753ns | -99.9% | -58.7% |
| 2 | 2155377ns | -99.9% | -59.1% |
| 3 | 2146174ns | -99.9% | -58.9% |
| 4 | 2160127ns | -99.9% | -59.2% |
| 5 | 2153013ns | -99.9% | -59.0% |
| 6 | 2160195ns | -99.9% | -59.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.134 | ok |
| abi_soa_win_real_scalar_payload | -0.323 | moderate- |
| abi_soa_win_real_soa_payload | -0.123 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 119537.3ns | 2692.8ns | 4439.1% | HIGH |
| abi_soa_win_real_scalar_payload | 6499622.1ns | 2152773.3ns | 301.9% | HIGH |
| abi_soa_win_real_soa_payload | 2683976.4ns | 882556.6ns | 304.1% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 2543.3-2919.4 ns)
   2543.3 |########################################
   2562.1 |########################################
   2580.9 |########################################
   2599.7 |########################################
   2618.5 |
   2637.3 |########################################
   2656.1 |
   2674.9 |
   2693.7 |
   2712.5 |
   2731.4 |
   2750.2 |
   2769.0 |
   2787.8 |
   2806.6 |
   2825.4 |
   2844.2 |
   2863.0 |
   2881.8 |
   2900.6 |
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2141753.3-2160161.0 ns)
  2141753.3 |########################################
  2142673.7 |
  2143594.1 |
  2144514.5 |
  2145434.8 |########################################
  2146355.2 |
  2147275.6 |
  2148196.0 |
  2149116.4 |
  2150036.8 |
  2150957.2 |
  2151877.6 |
  2152797.9 |########################################
  2153718.3 |
  2154638.7 |########################################
  2155559.1 |
  2156479.5 |
  2157399.9 |
  2158320.3 |
  2159240.7 |########################################
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 882048.7-883437.3 ns)
  882048.7 |########################################
  882118.1 |
  882187.6 |#############
  882257.0 |
  882326.4 |
  882395.8 |
  882465.3 |
  882534.7 |
  882604.1 |
  882673.6 |#############
  882743.0 |
  882812.4 |
  882881.9 |
  882951.3 |
  883020.7 |
  883090.2 |
  883159.6 |
  883229.0 |
  883298.4 |
  883367.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=4461.3% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=304.1% of algo (FFI overhead may distort results)
