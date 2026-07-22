# Value representation (static / runtime-tagged / NaN-boxed)

3 variants, 6 samples per variant.
Baseline: **carrier_vr_static**

## Highlights

Baseline for all deltas below: **carrier_vr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vr_static) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vr_static has the worst median (183.22 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vr_tagged at 176.51 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6.71 us) is smaller than the fastest variant's own run-to-run std-dev (34.93 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_vr_tagged vs stability leader carrier_vr_static (+4% speed for 1.4x steadier)

carrier_vr_tagged is fastest (176.51 us, CV 19.8%); carrier_vr_static gives up 3.8% median for 1.4x lower variance (CV 14.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### carrier_vr_nanbox is inconsistent: worst-20% is 1.6x its best-20%

carrier_vr_nanbox's best 20% of batches run at 140.65 us but its worst 20% at 228.83 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 3.8% of the fastest

All 3 variants sit between 176.51 us and 183.22 us - a 3.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_vr_tagged** at 176508.3 ns median (-3.7% vs baseline)
- Spread: 1.04x (fastest 176508.3 ns, slowest 183222.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vr_nanbox | 185707ns | 181461ns | 143256ns | 169181ns | 231722ns | +0.43% |
| carrier_vr_static | 184911ns | 185980ns | 153681ns | 176183ns | 213617ns | base |
| carrier_vr_tagged | 178150ns | 179144ns | 136222ns | 167629ns | 214896ns | -3.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vr_nanbox | 182979ns | 140653ns | 228826ns | +0.36% | 0.022 |
| carrier_vr_static | 182324ns | 151411ns | 210896ns | base | 0.022 |
| carrier_vr_tagged | 175522ns | 134028ns | 211967ns | -3.73% | 0.023 |

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_vr_tagged; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vr_nanbox | 0.023 | 74.9% |
| carrier_vr_static | 0.022 | 73.2% |
| carrier_vr_tagged | 0.023 | 75.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vr_nanbox | 185707ns | 185707ns | +0.43% |
| carrier_vr_static | 184911ns | 184911ns | base |
| carrier_vr_tagged | 178150ns | 178150ns | -3.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vr_static | 183222ns | base | --- | [152854, 210896] | --- | --- | --- | --- |
| carrier_vr_nanbox | 178854ns | no significant difference | [-50213, +40842]ns | [141258, 228826] | no | 1.0000 | 1.0000 | 0 |
| carrier_vr_tagged | 176508ns | no significant difference | [-49768, +28745]ns | [138092, 211967] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vr_static | carrier_vr_nanbox | carrier_vr_tagged |
|---|---|---|---|
| 1 | 183624ns | +14.7% | +17.3% |
| 2 | 154296ns | -8.1% | -7.9% |
| 3 | 228645ns | -38.5% | -35.9% |
| 4 | 193148ns | +26.5% | +6.9% |
| 5 | 182821ns | +16.7% | +14.1% |
| 6 | 151411ns | -2.9% | -11.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vr_nanbox | -0.119 | ok |
| carrier_vr_static | -0.211 | moderate- |
| carrier_vr_tagged | -0.220 | moderate- |

**Consistency summary:**

- **carrier_vr_nanbox**: won 3/6, lost 3/6
- **carrier_vr_tagged**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vr_nanbox | 183966.8ns | 182979.2ns | 100.5% | HIGH |
| carrier_vr_static | 182688.6ns | 182324.1ns | 100.2% | HIGH |
| carrier_vr_tagged | 176262.4ns | 175522.5ns | 100.4% | HIGH |

## Distribution (algo ns)

```
carrier_vr_nanbox (n=6, range 140653.3-228826.5 ns)
  140653.3 |########################################
  145062.0 |####################
  149470.6 |
  153879.3 |
  158287.9 |
  162696.6 |
  167105.2 |
  171513.9 |
  175922.6 |
  180331.2 |
  184739.9 |
  189148.5 |
  193557.2 |
  197965.8 |
  202374.5 |
  206783.2 |####################
  211191.8 |####################
  215600.5 |
  220009.1 |
  224417.8 |
  (0 below, 1 above range)

carrier_vr_static (n=6, range 151410.8-210896.2 ns)
  151410.8 |########################################
  154385.1 |
  157359.3 |
  160333.6 |
  163307.9 |
  166282.2 |
  169256.4 |
  172230.7 |
  175205.0 |
  178179.3 |
  181153.5 |########################################
  184127.8 |
  187102.1 |
  190076.3 |
  193050.6 |####################
  196024.9 |
  198999.2 |
  201973.4 |
  204947.7 |
  207922.0 |
  (0 below, 1 above range)

carrier_vr_tagged (n=6, range 134027.9-211967.3 ns)
  134027.9 |########################################
  137924.9 |
  141821.8 |########################################
  145718.8 |########################################
  149615.8 |
  153512.8 |
  157409.7 |
  161306.7 |
  165203.7 |
  169100.6 |
  172997.6 |
  176894.6 |
  180791.5 |
  184688.5 |
  188585.5 |
  192482.4 |
  196379.4 |
  200276.4 |
  204173.4 |########################################
  208070.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vr_nanbox**: CV=22.6% (high variance, measurements may be unstable)
- **carrier_vr_nanbox**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_vr_static**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_vr_tagged**: bridge=100.6% of algo (FFI overhead may distort results)
