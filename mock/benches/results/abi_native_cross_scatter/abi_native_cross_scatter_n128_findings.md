# abi_native_cross (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_scatter_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_scatter_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_native_cross_scatter_null_entry dominates: 332% faster than the next best (abi_native_cross_scatter_native_ffi_w)

abi_native_cross_scatter_null_entry (2.71 us) leads abi_native_cross_scatter_native_ffi_w (11.72 us) by 332%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_scatter_null_entry beats baseline by 76% (significant)

abi_native_cross_scatter_null_entry is -8.93 us (76%) faster than baseline abi_native_cross_scatter_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_scatter_inproc_native is an outlier: 4.4x slower than the field

abi_native_cross_scatter_inproc_native (11.95 us) is 4.4x the fastest (2.71 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_scatter_native_ffi_w shows warm-up / thermal drift (autocorr +0.53)

abi_native_cross_scatter_native_ffi_w's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.4x the fastest

Fastest abi_native_cross_scatter_null_entry (2.71 us) to slowest abi_native_cross_scatter_inproc_native (11.95 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_scatter_null_entry** at 2715.0 ns median (-76.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.40x (fastest 2715.0 ns, slowest 11946.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 14422ns | 14267ns | 13876ns | 14157ns | 15093ns | +2.48% |
| abi_native_cross_scatter_native_ffi_w | 14073ns | 14053ns | 13738ns | 13972ns | 14392ns | base |
| abi_native_cross_scatter_null_entry | 5001ns | 4951ns | 4839ns | 4914ns | 5213ns | -64.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 12074ns | 11606ns | 12644ns | +2.79% | 0.011 |
| abi_native_cross_scatter_native_ffi_w | 11746ns | 11464ns | 12021ns | base | 0.011 |
| abi_native_cross_scatter_null_entry | 2732ns | 2631ns | 2843ns | -76.74% | 0.047 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 5387.8 | 11764.4 | 12074.5 | 60 |
| abi_native_cross_scatter_native_ffi_w | 25074.2 | 11891.1 | 11746.4 | n/a |
| abi_native_cross_scatter_null_entry | 26098.6 | 2795.2 | 2732.0 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_native_cross_scatter_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_scatter_inproc_native | 0.011 | 22.0% |
| abi_native_cross_scatter_native_ffi_w | 0.011 | 22.4% |
| abi_native_cross_scatter_null_entry | 0.047 | 96.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 14422ns | 14422ns | +2.48% |
| abi_native_cross_scatter_native_ffi_w | 14073ns | 14073ns | base |
| abi_native_cross_scatter_null_entry | 5001ns | 5001ns | -64.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_scatter_native_ffi_w | 11725ns | base | --- | [11493, 12021] | --- | --- | --- | --- |
| abi_native_cross_scatter_inproc_native | 11946ns | no significant difference | [-231, +1151]ns | [11633, 12644] | no | 1.0000 | 1.0000 | 0 |
| abi_native_cross_scatter_null_entry | 2715ns | -8934.4ns (-76.2%) | [-9371, -8738]ns | [2638, 2843] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_scatter_native_ffi_w | abi_native_cross_scatter_inproc_native | abi_native_cross_scatter_null_entry |
|---|---|---|---|
| 1 | 11523ns | +9.2% | -77.2% |
| 2 | 11464ns | +10.9% | -74.9% |
| 3 | 11676ns | -0.6% | -76.2% |
| 4 | 11774ns | +1.7% | -76.2% |
| 5 | 11952ns | -2.5% | -77.9% |
| 6 | 12089ns | -1.4% | -78.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_scatter_inproc_native | 0.164 | ok |
| abi_native_cross_scatter_native_ffi_w | 0.533 | HIGH+ (drift/warm-up) |
| abi_native_cross_scatter_null_entry | -0.094 | ok |

**Consistency summary:**

- **abi_native_cross_scatter_inproc_native**: won 3/6, lost 3/6
- **abi_native_cross_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 123991.9ns | 12074.5ns | 1026.9% | HIGH |
| abi_native_cross_scatter_native_ffi_w | 143126.4ns | 11746.4ns | 1218.5% | HIGH |
| abi_native_cross_scatter_null_entry | 118457.8ns | 2732.0ns | 4335.9% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_scatter_inproc_native (n=6, range 11606.2-12644.4 ns)
  11606.2 |########################################
  11658.1 |########################################
  11710.0 |
  11761.9 |
  11813.8 |
  11865.8 |
  11917.7 |########################################
  11969.6 |########################################
  12021.5 |
  12073.4 |
  12125.3 |
  12177.2 |
  12229.1 |
  12281.0 |
  12332.9 |
  12384.9 |
  12436.8 |
  12488.7 |
  12540.6 |########################################
  12592.5 |
  (0 below, 1 above range)

abi_native_cross_scatter_native_ffi_w (n=6, range 11463.8-12020.9 ns)
  11463.8 |########################################
  11491.7 |
  11519.5 |########################################
  11547.4 |
  11575.2 |
  11603.1 |
  11630.9 |
  11658.8 |########################################
  11686.6 |
  11714.5 |
  11742.3 |
  11770.2 |########################################
  11798.0 |
  11825.9 |
  11853.7 |
  11881.6 |
  11909.4 |
  11937.3 |########################################
  11965.1 |
  11993.0 |
  (0 below, 1 above range)

abi_native_cross_scatter_null_entry (n=6, range 2631.2-2842.9 ns)
   2631.2 |########################################
   2641.8 |########################################
   2652.4 |########################################
   2663.0 |
   2673.5 |
   2684.1 |
   2694.7 |
   2705.3 |
   2715.9 |
   2726.5 |
   2737.1 |
   2747.6 |
   2758.2 |
   2768.8 |########################################
   2779.4 |
   2790.0 |
   2800.6 |########################################
   2811.1 |
   2821.7 |
   2832.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_scatter_inproc_native**: bridge=1043.2% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_native_ffi_w**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **abi_native_cross_scatter_native_ffi_w**: bridge=1221.0% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_null_entry**: bridge=4349.4% of algo (FFI overhead may distort results)
