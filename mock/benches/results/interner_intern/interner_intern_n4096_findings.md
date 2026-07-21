# Interner intern hot path: FNV vs FxHash x load factor 25% vs 75%

4 variants, 6 samples per variant.
Baseline: **intern_fnv_lf25**

## Highlights

Baseline for all deltas below: **intern_fnv_lf25**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### intern_fnv_lf75 shows alternating (throttle bounce) (autocorr -0.56)

intern_fnv_lf75's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: intern_fx_lf75** at 57911.6 ns median (-13.7% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.23x (fastest 57911.6 ns, slowest 71393.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| intern_fnv_lf25 | 70602ns | 69344ns | 68132ns | 68960ns | 74299ns | base |
| intern_fnv_lf75 | 63089ns | 61509ns | 59820ns | 61002ns | 67854ns | -10.64% |
| intern_fx_lf25 | 74450ns | 73681ns | 71136ns | 73278ns | 77865ns | +5.45% |
| intern_fx_lf75 | 61254ns | 60191ns | 58971ns | 59956ns | 64344ns | -13.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| intern_fnv_lf25 | 68340ns | 65949ns | 71918ns | base | 0.060 |
| intern_fnv_lf75 | 60791ns | 57618ns | 65360ns | -11.05% | 0.067 |
| intern_fx_lf25 | 72158ns | 68988ns | 75500ns | +5.59% | 0.057 |
| intern_fx_lf75 | 58915ns | 56693ns | 61953ns | -13.79% | 0.070 |

## Performance model

- Peak throughput: **0.072 Gops/s** (intern_fx_lf75; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| intern_fnv_lf25 | 0.061 | 84.4% |
| intern_fnv_lf75 | 0.069 | 95.6% |
| intern_fx_lf25 | 0.057 | 79.4% |
| intern_fx_lf75 | 0.071 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| intern_fnv_lf25 | 70602ns | 70602ns | base |
| intern_fnv_lf75 | 63089ns | 63089ns | -10.64% |
| intern_fx_lf25 | 74450ns | 74450ns | +5.45% |
| intern_fx_lf75 | 61254ns | 61254ns | -13.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| intern_fnv_lf25 | 67137ns | base | --- | [65965, 71918] | --- | --- | --- | --- |
| intern_fnv_lf75 | 59290ns | -8260.6ns (-12.3%) | [-9773, -4613]ns | [57723, 65360] | YES | 0.0313 | 0.0313 | 0 |
| intern_fx_lf25 | 71393ns | +3978.8ns (+5.9%) | [+2695, +4782]ns | [69582, 75500] | YES | 0.0313 | 0.0313 | 0 |
| intern_fx_lf75 | 57912ns | -8674.0ns (-12.9%) | [-13628, -5972]ns | [56880, 61953] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | intern_fnv_lf25 | intern_fnv_lf75 | intern_fx_lf25 | intern_fx_lf75 |
|---|---|---|---|---|
| 1 | 65949ns | -11.5% | +6.4% | -13.5% |
| 2 | 67696ns | -14.6% | +5.5% | -16.3% |
| 3 | 73965ns | -8.5% | +6.5% | -22.0% |
| 4 | 66578ns | -13.5% | +7.2% | -12.7% |
| 5 | 65980ns | -4.4% | +4.6% | -7.1% |
| 6 | 69871ns | -13.9% | +3.4% | -10.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| intern_fnv_lf25 | -0.235 | moderate- |
| intern_fnv_lf75 | -0.564 | HIGH- (thermal bounce) |
| intern_fx_lf25 | -0.109 | ok |
| intern_fx_lf75 | 0.490 | moderate+ |

**Consistency summary:**

- **intern_fnv_lf75**: won 6/6, lost 0/6
- **intern_fx_lf25**: won 0/6, lost 6/6
- **intern_fx_lf75**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| intern_fnv_lf25 | 2505.1ns | 68339.9ns | 3.7% |  |
| intern_fnv_lf75 | 662.6ns | 60790.9ns | 1.1% |  |
| intern_fx_lf25 | 2619.2ns | 72158.4ns | 3.6% |  |
| intern_fx_lf75 | 656.3ns | 58915.1ns | 1.1% |  |

## Distribution (algo ns)

```
intern_fnv_lf25 (n=6, range 65949.2-71918.1 ns)
  65949.2 |########################################
  66247.6 |
  66546.1 |####################
  66844.5 |
  67143.0 |
  67441.4 |####################
  67739.9 |
  68038.3 |
  68336.8 |
  68635.2 |
  68933.6 |
  69232.1 |
  69530.5 |
  69829.0 |####################
  70127.4 |
  70425.9 |
  70724.3 |
  71022.8 |
  71321.2 |
  71619.7 |
  (0 below, 1 above range)

intern_fnv_lf75 (n=6, range 57617.9-65359.8 ns)
  57617.9 |########################################
  58005.0 |####################
  58392.1 |
  58779.2 |
  59166.3 |
  59553.4 |
  59940.5 |####################
  60327.6 |
  60714.7 |
  61101.8 |
  61488.9 |
  61875.9 |
  62263.0 |
  62650.1 |
  63037.2 |####################
  63424.3 |
  63811.4 |
  64198.5 |
  64585.6 |
  64972.7 |
  (0 below, 1 above range)

intern_fx_lf25 (n=6, range 68987.9-75499.9 ns)
  68987.9 |####################
  69313.5 |
  69639.1 |
  69964.7 |####################
  70290.3 |
  70615.9 |
  70941.5 |
  71267.1 |########################################
  71592.7 |
  71918.3 |
  72243.9 |####################
  72569.5 |
  72895.1 |
  73220.7 |
  73546.3 |
  73871.9 |
  74197.5 |
  74523.1 |
  74848.7 |
  75174.3 |
  (0 below, 1 above range)

intern_fx_lf75 (n=6, range 56692.9-61953.3 ns)
  56692.9 |########################################
  56955.9 |########################################
  57218.9 |
  57482.0 |########################################
  57745.0 |
  58008.0 |########################################
  58271.0 |
  58534.1 |
  58797.1 |
  59060.1 |
  59323.1 |
  59586.1 |
  59849.2 |
  60112.2 |
  60375.2 |
  60638.2 |
  60901.3 |
  61164.3 |########################################
  61427.3 |
  61690.3 |
  (0 below, 1 above range)

```
