# Interner intern hot path: FNV vs FxHash x load factor 25% vs 75%

4 variants, 6 samples per variant.
Baseline: **intern_fnv_lf25**

## Highlights

Baseline for all deltas below: **intern_fnv_lf25**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Speed leader intern_fnv_lf75 vs stability leader intern_fx_lf75 (+9% speed for 2.1x steadier)

intern_fnv_lf75 is fastest (8.40 us, CV 8.1%); intern_fx_lf75 gives up 9.2% median for 2.1x lower variance (CV 4.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: intern_fnv_lf75** at 8402.1 ns median (-21.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.34x (fastest 8402.1 ns, slowest 11230.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| intern_fnv_lf25 | 12882ns | 13080ns | 11386ns | 12664ns | 13955ns | base |
| intern_fnv_lf75 | 11127ns | 10628ns | 10328ns | 10592ns | 12329ns | -13.62% |
| intern_fx_lf25 | 13537ns | 13593ns | 10902ns | 13263ns | 15264ns | +5.09% |
| intern_fx_lf75 | 11454ns | 11511ns | 10712ns | 11380ns | 11935ns | -11.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| intern_fnv_lf25 | 10480ns | 9185ns | 11364ns | base | 0.098 |
| intern_fnv_lf75 | 8794ns | 8167ns | 9749ns | -16.09% | 0.116 |
| intern_fx_lf25 | 11148ns | 8762ns | 12660ns | +6.38% | 0.092 |
| intern_fx_lf75 | 9139ns | 8543ns | 9509ns | -12.79% | 0.112 |

## Performance model

- Peak throughput: **0.125 Gops/s** (intern_fnv_lf75; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| intern_fnv_lf25 | 0.096 | 76.6% |
| intern_fnv_lf75 | 0.122 | 97.2% |
| intern_fx_lf25 | 0.091 | 72.7% |
| intern_fx_lf75 | 0.112 | 89.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| intern_fnv_lf25 | 12882ns | 12882ns | base |
| intern_fnv_lf75 | 11127ns | 11127ns | -13.62% |
| intern_fx_lf25 | 13537ns | 13537ns | +5.09% |
| intern_fx_lf75 | 11454ns | 11454ns | -11.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| intern_fnv_lf25 | 10656ns | base | --- | [9420, 11364] | --- | --- | --- | --- |
| intern_fnv_lf75 | 8402ns | -1265.7ns (-11.9%) | [-2874, -918]ns | [8231, 9749] | YES | 0.0469 | 0.0313 | 0 |
| intern_fx_lf25 | 11230ns | no significant difference | [-696, +1656]ns | [9554, 12660] | no | 0.6875 | 0.6875 | 0 |
| intern_fx_lf75 | 9173ns | -1359.1ns (-12.8%) | [-1981, -682]ns | [8736, 9509] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | intern_fnv_lf25 | intern_fnv_lf75 | intern_fx_lf25 | intern_fx_lf75 |
|---|---|---|---|---|
| 1 | 11524ns | -28.0% | +7.3% | -17.8% |
| 2 | 10465ns | -7.8% | +12.0% | -10.1% |
| 3 | 11205ns | -12.1% | +15.7% | -14.9% |
| 4 | 10846ns | -23.2% | -4.6% | -17.7% |
| 5 | 9185ns | -11.1% | +16.9% | -7.0% |
| 6 | 9655ns | -12.2% | -9.2% | -7.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| intern_fnv_lf25 | 0.203 | moderate+ |
| intern_fnv_lf75 | 0.169 | ok |
| intern_fx_lf25 | 0.135 | ok |
| intern_fx_lf75 | 0.462 | moderate+ |

**Consistency summary:**

- **intern_fnv_lf75**: won 6/6, lost 0/6
- **intern_fx_lf25**: won 2/6, lost 4/6
- **intern_fx_lf75**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| intern_fnv_lf25 | 2322.6ns | 10480.0ns | 22.2% | HIGH |
| intern_fnv_lf75 | 751.6ns | 8794.0ns | 8.5% | HIGH |
| intern_fx_lf25 | 2302.3ns | 11148.4ns | 20.7% | HIGH |
| intern_fx_lf75 | 735.8ns | 9139.2ns | 8.1% | HIGH |

## Distribution (algo ns)

```
intern_fnv_lf25 (n=6, range 9185.4-11364.4 ns)
   9185.4 |########################################
   9294.3 |
   9403.3 |
   9512.2 |
   9621.2 |########################################
   9730.1 |
   9839.1 |
   9948.0 |
  10057.0 |
  10165.9 |
  10274.9 |
  10383.8 |########################################
  10492.8 |
  10601.7 |
  10710.7 |
  10819.6 |########################################
  10928.6 |
  11037.5 |
  11146.5 |########################################
  11255.4 |
  (0 below, 1 above range)

intern_fnv_lf75 (n=6, range 8167.1-9749.0 ns)
   8167.1 |########################################
   8246.2 |########################################
   8325.3 |########################################
   8404.4 |########################################
   8483.5 |
   8562.6 |
   8641.7 |
   8720.7 |
   8799.8 |
   8878.9 |
   8958.0 |
   9037.1 |
   9116.2 |
   9195.3 |
   9274.4 |
   9353.5 |
   9432.6 |
   9511.7 |
   9590.8 |########################################
   9669.9 |
  (0 below, 1 above range)

intern_fx_lf25 (n=6, range 8762.5-12660.4 ns)
   8762.5 |########################################
   8957.4 |
   9152.3 |
   9347.2 |
   9542.1 |
   9737.0 |
   9931.9 |
  10126.8 |
  10321.7 |########################################
  10516.6 |
  10711.5 |########################################
  10906.3 |
  11101.2 |
  11296.1 |
  11491.0 |
  11685.9 |########################################
  11880.8 |
  12075.7 |
  12270.6 |########################################
  12465.5 |
  (0 below, 1 above range)

intern_fx_lf75 (n=6, range 8542.9-9508.8 ns)
   8542.9 |####################
   8591.2 |
   8639.5 |
   8687.8 |
   8736.1 |
   8784.4 |
   8832.7 |
   8880.9 |
   8929.2 |########################################
   8977.5 |
   9025.8 |
   9074.1 |
   9122.4 |
   9170.7 |
   9219.0 |
   9267.3 |
   9315.6 |
   9363.9 |####################
   9412.2 |
   9460.5 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **intern_fnv_lf25**: bridge=22.2% of algo (FFI overhead may distort results)
- **intern_fx_lf25**: bridge=20.7% of algo (FFI overhead may distort results)
