# Predecoded dispatch shape, scatter profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_scatter_null dominates: 19% faster than the next best (carrier_pre_scatter_direct)

carrier_pre_scatter_null (25.69 us) leads carrier_pre_scatter_direct (30.54 us) by 19%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_scatter_null beats baseline by 32% (significant)

carrier_pre_scatter_null is -12.29 us (32%) faster than baseline carrier_pre_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_scatter_fntable is an outlier: 2.1x slower than the field

carrier_pre_scatter_fntable (52.86 us) is 2.1x the fastest (25.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_scatter_null, carrier_pre_scatter_direct, carrier_pre_scatter_threaded, carrier_pre_scatter_switch, carrier_pre_scatter_regcache} vs {carrier_pre_scatter_fntable} (31% apart)

The field splits into a fast tier {carrier_pre_scatter_null, carrier_pre_scatter_direct, carrier_pre_scatter_threaded, carrier_pre_scatter_switch, carrier_pre_scatter_regcache} and a slow tier {carrier_pre_scatter_fntable} with a 31% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_scatter_null** at 25691.5 ns median (-32.1% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.06x (fastest 25691.5 ns, slowest 52855.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 32909ns | 32731ns | 32111ns | 32579ns | 33805ns | -19.48% |
| carrier_pre_scatter_fntable | 56720ns | 55035ns | 53442ns | 54886ns | 61111ns | +38.77% |
| carrier_pre_scatter_null | 27777ns | 27961ns | 26360ns | 27741ns | 28539ns | -32.04% |
| carrier_pre_scatter_regcache | 42502ns | 42474ns | 41360ns | 42171ns | 43569ns | +3.99% |
| carrier_pre_scatter_switch | 40873ns | 40096ns | 38448ns | 39727ns | 43804ns | base |
| carrier_pre_scatter_threaded | 37532ns | 36695ns | 36315ns | 36576ns | 39574ns | -8.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 30690ns | 29878ns | 31541ns | -20.58% | 0.033 |
| carrier_pre_scatter_fntable | 54518ns | 51266ns | 58860ns | +41.08% | 0.019 |
| carrier_pre_scatter_null | 25495ns | 24207ns | 26154ns | -34.03% | 0.040 |
| carrier_pre_scatter_regcache | 40324ns | 39252ns | 41333ns | +4.35% | 0.025 |
| carrier_pre_scatter_switch | 38644ns | 36222ns | 41591ns | base | 0.026 |
| carrier_pre_scatter_threaded | 35338ns | 34168ns | 37263ns | -8.55% | 0.029 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 392388 | 1086614 | 0.361 | 0.82× |
| carrier_pre_scatter_fntable | 558155 | 1239321 | 0.450 | 1.17× |
| carrier_pre_scatter_null | 396666 | 1749417 | 0.227 | 0.83× |
| carrier_pre_scatter_regcache | 508493 | 1899913 | 0.268 | 1.07× |
| carrier_pre_scatter_switch | 477033 | 1299972 | 0.367 | 1.00× |
| carrier_pre_scatter_threaded | 446757 | 1460322 | 0.306 | 0.94× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.042 Gops/s** (carrier_pre_scatter_null; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_scatter_direct | 0.034 | 79.3% |
| carrier_pre_scatter_fntable | 0.019 | 45.8% |
| carrier_pre_scatter_null | 0.040 | 94.2% |
| carrier_pre_scatter_regcache | 0.025 | 60.1% |
| carrier_pre_scatter_switch | 0.027 | 64.0% |
| carrier_pre_scatter_threaded | 0.030 | 70.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_scatter_direct | 32909ns | 32909ns | -19.48% |
| carrier_pre_scatter_fntable | 56720ns | 56720ns | +38.77% |
| carrier_pre_scatter_null | 27777ns | 27777ns | -32.04% |
| carrier_pre_scatter_regcache | 42502ns | 42502ns | +3.99% |
| carrier_pre_scatter_switch | 40873ns | 40873ns | base |
| carrier_pre_scatter_threaded | 37532ns | 37532ns | -8.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_scatter_switch | 37839ns | base | --- | [36502, 41591] | --- | --- | --- | --- |
| carrier_pre_scatter_direct | 30536ns | -7241.1ns (-19.1%) | [-11063, -5558]ns | [29993, 31541] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_scatter_fntable | 52856ns | +14592.2ns (+38.6%) | [+11738, +21292]ns | [51839, 58860] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_scatter_null | 25691ns | -12285.2ns (-32.5%) | [-16345, -10818]ns | [24638, 26154] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_scatter_regcache | 40300ns | no significant difference | [-1782, +4091]ns | [39340, 41333] | no | 0.2188 | 0.2188 | 0 |
| carrier_pre_scatter_threaded | 34558ns | -2309.0ns (-6.1%) | [-6844, -764]ns | [34193, 37263] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_scatter_switch | carrier_pre_scatter_direct | carrier_pre_scatter_fntable | carrier_pre_scatter_null | carrier_pre_scatter_regcache | carrier_pre_scatter_threaded |
|---|---|---|---|---|---|---|
| 1 | 37977ns | -16.3% | +38.0% | -31.6% | +3.4% | +0.1% |
| 2 | 38078ns | -20.9% | +38.6% | -34.2% | +5.9% | -4.1% |
| 3 | 36222ns | -13.7% | +46.1% | -28.3% | +8.9% | -5.7% |
| 4 | 37702ns | -20.1% | +68.6% | -30.2% | +11.9% | -8.7% |
| 5 | 36782ns | -18.8% | +39.4% | -34.2% | +10.1% | -7.0% |
| 6 | 45103ns | -31.4% | +20.0% | -43.6% | -10.7% | -23.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_scatter_direct | -0.361 | moderate- |
| carrier_pre_scatter_fntable | -0.352 | moderate- |
| carrier_pre_scatter_null | -0.333 | moderate- |
| carrier_pre_scatter_regcache | -0.254 | moderate- |
| carrier_pre_scatter_switch | -0.118 | ok |
| carrier_pre_scatter_threaded | 0.371 | moderate+ |

**Consistency summary:**

- **carrier_pre_scatter_direct**: won 6/6, lost 0/6
- **carrier_pre_scatter_fntable**: won 0/6, lost 6/6
- **carrier_pre_scatter_null**: won 6/6, lost 0/6
- **carrier_pre_scatter_regcache**: won 1/6, lost 5/6
- **carrier_pre_scatter_threaded**: won 5/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 95447.0ns | 30689.9ns | 311.0% | HIGH |
| carrier_pre_scatter_fntable | 123542.1ns | 54518.0ns | 226.6% | HIGH |
| carrier_pre_scatter_null | 104579.8ns | 25494.6ns | 410.2% | HIGH |
| carrier_pre_scatter_regcache | 121830.3ns | 40324.3ns | 302.1% | HIGH |
| carrier_pre_scatter_switch | 116068.9ns | 38643.9ns | 300.4% | HIGH |
| carrier_pre_scatter_threaded | 108513.0ns | 35338.1ns | 307.1% | HIGH |

## Distribution (algo ns)

```
carrier_pre_scatter_direct (n=6, range 29877.9-31541.0 ns)
  29877.9 |####################
  29961.1 |
  30044.2 |########################################
  30127.4 |
  30210.5 |
  30293.7 |
  30376.8 |
  30460.0 |
  30543.2 |
  30626.3 |
  30709.5 |
  30792.6 |
  30875.8 |####################
  30958.9 |
  31042.1 |
  31125.3 |
  31208.4 |####################
  31291.6 |
  31374.7 |
  31457.9 |
  (0 below, 1 above range)

carrier_pre_scatter_fntable (n=6, range 51266.2-58859.6 ns)
  51266.2 |####################
  51645.9 |
  52025.5 |
  52405.2 |########################################
  52784.9 |####################
  53164.5 |
  53544.2 |
  53923.9 |####################
  54303.6 |
  54683.2 |
  55062.9 |
  55442.6 |
  55822.2 |
  56201.9 |
  56581.6 |
  56961.2 |
  57340.9 |
  57720.6 |
  58100.3 |
  58479.9 |
  (0 below, 1 above range)

carrier_pre_scatter_null (n=6, range 24206.7-26154.4 ns)
  24206.7 |####################
  24304.1 |
  24401.5 |
  24498.9 |
  24596.2 |
  24693.6 |
  24791.0 |
  24888.4 |
  24985.8 |####################
  25083.2 |
  25180.6 |
  25277.9 |
  25375.3 |####################
  25472.7 |
  25570.1 |
  25667.5 |
  25764.9 |
  25862.2 |
  25959.6 |########################################
  26057.0 |
  (0 below, 1 above range)

carrier_pre_scatter_regcache (n=6, range 39251.7-41333.1 ns)
  39251.7 |########################################
  39355.8 |########################################
  39459.8 |
  39563.9 |
  39668.0 |
  39772.1 |
  39876.1 |
  39980.2 |
  40084.3 |
  40188.4 |########################################
  40292.4 |########################################
  40396.5 |########################################
  40500.6 |
  40604.6 |
  40708.7 |
  40812.8 |
  40916.9 |
  41020.9 |
  41125.0 |
  41229.1 |
  (0 below, 1 above range)

carrier_pre_scatter_switch (n=6, range 36221.7-41590.6 ns)
  36221.7 |####################
  36490.1 |
  36758.6 |####################
  37027.0 |
  37295.5 |
  37563.9 |####################
  37832.4 |########################################
  38100.8 |
  38369.3 |
  38637.7 |
  38906.2 |
  39174.6 |
  39443.0 |
  39711.5 |
  39979.9 |
  40248.4 |
  40516.8 |
  40785.3 |
  41053.7 |
  41322.2 |
  (0 below, 1 above range)

carrier_pre_scatter_threaded (n=6, range 34167.5-37263.3 ns)
  34167.5 |########################################
  34322.3 |####################
  34477.1 |
  34631.9 |####################
  34786.7 |
  34941.4 |
  35096.2 |
  35251.0 |
  35405.8 |
  35560.6 |
  35715.4 |
  35870.2 |
  36025.0 |
  36179.8 |
  36334.6 |
  36489.4 |####################
  36644.1 |
  36798.9 |
  36953.7 |
  37108.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_scatter_direct**: bridge=310.9% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_fntable**: bridge=240.4% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_null**: bridge=409.9% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_regcache**: bridge=302.6% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_switch**: bridge=302.4% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_threaded**: bridge=307.7% of algo (FFI overhead may distort results)
