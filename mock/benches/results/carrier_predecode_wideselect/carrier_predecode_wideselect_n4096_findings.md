# Predecoded dispatch shape, wideselect profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_wideselect_null dominates: 117% faster than the next best (carrier_pre_wideselect_direct)

carrier_pre_wideselect_null (104.03 us) leads carrier_pre_wideselect_direct (226.26 us) by 117%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_wideselect_null beats baseline by 72% (significant)

carrier_pre_wideselect_null is -263.48 us (72%) faster than baseline carrier_pre_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_wideselect_fntable is an outlier: 4.3x slower than the field

carrier_pre_wideselect_fntable (446.11 us) is 4.3x the fastest (104.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_wideselect_null} vs {carrier_pre_wideselect_direct, carrier_pre_wideselect_threaded, carrier_pre_wideselect_switch, carrier_pre_wideselect_regcache, carrier_pre_wideselect_fntable} (117% apart)

The field splits into a fast tier {carrier_pre_wideselect_null} and a slow tier {carrier_pre_wideselect_direct, carrier_pre_wideselect_threaded, carrier_pre_wideselect_switch, carrier_pre_wideselect_regcache, carrier_pre_wideselect_fntable} with a 117% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.3x the fastest

Fastest carrier_pre_wideselect_null (104.03 us) to slowest carrier_pre_wideselect_fntable (446.11 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_pre_wideselect_null** at 104031.2 ns median (-71.7% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.29x (fastest 104031.2 ns, slowest 446112.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 225624ns | 228451ns | 217825ns | 225377ns | 229893ns | -39.84% |
| carrier_pre_wideselect_fntable | 446878ns | 448429ns | 423287ns | 446563ns | 459146ns | +19.16% |
| carrier_pre_wideselect_null | 106010ns | 106312ns | 103306ns | 105420ns | 108248ns | -71.73% |
| carrier_pre_wideselect_regcache | 375706ns | 378496ns | 345150ns | 373014ns | 395023ns | +0.18% |
| carrier_pre_wideselect_switch | 375020ns | 369799ns | 362650ns | 368745ns | 390619ns | base |
| carrier_pre_wideselect_threaded | 334268ns | 335068ns | 311171ns | 327343ns | 356204ns | -10.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 223425ns | 215637ns | 227704ns | -40.07% | 0.018 |
| carrier_pre_wideselect_fntable | 444623ns | 421103ns | 456973ns | +19.26% | 0.009 |
| carrier_pre_wideselect_null | 103770ns | 101159ns | 105989ns | -72.17% | 0.039 |
| carrier_pre_wideselect_regcache | 373511ns | 342982ns | 392780ns | +0.18% | 0.011 |
| carrier_pre_wideselect_switch | 372821ns | 360433ns | 388428ns | base | 0.011 |
| carrier_pre_wideselect_threaded | 332020ns | 308817ns | 353987ns | -10.94% | 0.012 |

## Performance model

- Peak throughput: **0.040 Gops/s** (carrier_pre_wideselect_null; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_wideselect_direct | 0.018 | 44.7% |
| carrier_pre_wideselect_fntable | 0.009 | 22.7% |
| carrier_pre_wideselect_null | 0.039 | 97.2% |
| carrier_pre_wideselect_regcache | 0.011 | 26.9% |
| carrier_pre_wideselect_switch | 0.011 | 27.5% |
| carrier_pre_wideselect_threaded | 0.012 | 30.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_wideselect_direct | 225624ns | 225624ns | -39.84% |
| carrier_pre_wideselect_fntable | 446878ns | 446878ns | +19.16% |
| carrier_pre_wideselect_null | 106010ns | 106010ns | -71.73% |
| carrier_pre_wideselect_regcache | 375706ns | 375706ns | +0.18% |
| carrier_pre_wideselect_switch | 375020ns | 375020ns | base |
| carrier_pre_wideselect_threaded | 334268ns | 334268ns | -10.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_wideselect_switch | 367578ns | base | --- | [362457, 388428] | --- | --- | --- | --- |
| carrier_pre_wideselect_direct | 226262ns | -140267.8ns (-38.2%) | [-172118, -135802]ns | [216309, 227704] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_wideselect_fntable | 446113ns | +74172.5ns (+20.2%) | [+53445, +87787]ns | [430782, 456973] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_wideselect_null | 104031ns | -263476.0ns (-71.7%) | [-286161, -257517]ns | [101289, 105989] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_wideselect_regcache | 376330ns | no significant difference | [-14255, +16443]ns | [351422, 392780] | no | 1.0000 | 1.0000 | 0 |
| carrier_pre_wideselect_threaded | 332823ns | -38638.1ns (-10.5%) | [-71221, -12543]ns | [309251, 353987] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_wideselect_switch | carrier_pre_wideselect_direct | carrier_pre_wideselect_fntable | carrier_pre_wideselect_null | carrier_pre_wideselect_regcache | carrier_pre_wideselect_threaded |
|---|---|---|---|---|---|---|
| 1 | 364481ns | -38.1% | +25.4% | -72.2% | +7.2% | -1.6% |
| 2 | 382489ns | -43.3% | +19.5% | -73.0% | -0.8% | -10.5% |
| 3 | 394367ns | -45.3% | +11.7% | -74.3% | +0.1% | -21.7% |
| 4 | 366579ns | -37.7% | +20.2% | -70.9% | +1.8% | -15.5% |
| 5 | 360433ns | -37.0% | +16.8% | -70.7% | -0.2% | -10.3% |
| 6 | 368578ns | -38.4% | +22.5% | -71.6% | -6.9% | -5.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_wideselect_direct | 0.188 | ok |
| carrier_pre_wideselect_fntable | 0.056 | ok |
| carrier_pre_wideselect_null | 0.042 | ok |
| carrier_pre_wideselect_regcache | 0.340 | moderate+ |
| carrier_pre_wideselect_switch | 0.147 | ok |
| carrier_pre_wideselect_threaded | 0.267 | moderate+ |

**Consistency summary:**

- **carrier_pre_wideselect_direct**: won 6/6, lost 0/6
- **carrier_pre_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_pre_wideselect_null**: won 6/6, lost 0/6
- **carrier_pre_wideselect_regcache**: won 3/6, lost 2/6
- **carrier_pre_wideselect_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 240776.1ns | 223425.1ns | 107.8% | HIGH |
| carrier_pre_wideselect_fntable | 455092.6ns | 444622.6ns | 102.4% | HIGH |
| carrier_pre_wideselect_null | 112566.7ns | 103769.7ns | 108.5% | HIGH |
| carrier_pre_wideselect_regcache | 385264.5ns | 373510.6ns | 103.1% | HIGH |
| carrier_pre_wideselect_switch | 404082.2ns | 372821.2ns | 108.4% | HIGH |
| carrier_pre_wideselect_threaded | 347137.4ns | 332020.3ns | 104.6% | HIGH |

## Distribution (algo ns)

```
carrier_pre_wideselect_direct (n=6, range 215637.1-227704.0 ns)
  215637.1 |########################################
  216240.4 |
  216843.8 |########################################
  217447.1 |
  218050.5 |
  218653.8 |
  219257.2 |
  219860.5 |
  220463.8 |
  221067.2 |
  221670.5 |
  222273.9 |
  222877.2 |
  223480.6 |
  224083.9 |
  224687.2 |
  225290.6 |########################################
  225893.9 |
  226497.3 |########################################
  227100.6 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_fntable (n=6, range 421103.3-456973.2 ns)
  421103.3 |####################
  422896.8 |
  424690.3 |
  426483.8 |
  428277.3 |
  430070.8 |
  431864.3 |
  433657.7 |
  435451.2 |
  437244.7 |
  439038.2 |########################################
  440831.7 |
  442625.2 |
  444418.7 |
  446212.2 |
  448005.7 |
  449799.2 |
  451592.7 |####################
  453386.2 |
  455179.7 |####################
  (0 below, 1 above range)

carrier_pre_wideselect_null (n=6, range 101159.2-105988.8 ns)
  101159.2 |########################################
  101400.7 |########################################
  101642.2 |
  101883.6 |
  102125.1 |
  102366.6 |
  102608.1 |
  102849.5 |
  103091.0 |
  103332.5 |########################################
  103574.0 |
  103815.5 |
  104056.9 |
  104298.4 |
  104539.9 |########################################
  104781.4 |
  105022.8 |
  105264.3 |########################################
  105505.8 |
  105747.3 |
  (0 below, 1 above range)

carrier_pre_wideselect_regcache (n=6, range 342981.7-392779.6 ns)
  342981.7 |########################################
  345471.6 |
  347961.5 |
  350451.4 |
  352941.3 |
  355431.2 |
  357921.1 |########################################
  360411.0 |
  362900.9 |
  365390.8 |
  367880.7 |
  370370.5 |
  372860.4 |########################################
  375350.3 |
  377840.2 |########################################
  380330.1 |
  382820.0 |
  385309.9 |
  387799.8 |
  390289.7 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_switch (n=6, range 360433.3-388427.9 ns)
  360433.3 |########################################
  361833.0 |
  363232.8 |########################################
  364632.5 |
  366032.2 |########################################
  367432.0 |########################################
  368831.7 |
  370231.4 |
  371631.1 |
  373030.9 |
  374430.6 |
  375830.3 |
  377230.1 |
  378629.8 |
  380029.5 |
  381429.2 |########################################
  382829.0 |
  384228.7 |
  385628.4 |
  387028.2 |
  (0 below, 1 above range)

carrier_pre_wideselect_threaded (n=6, range 308816.7-353986.7 ns)
  308816.7 |########################################
  311075.2 |
  313333.7 |
  315592.2 |
  317850.7 |
  320109.2 |
  322367.7 |####################
  324626.2 |
  326884.7 |
  329143.2 |
  331401.7 |
  333660.2 |
  335918.7 |
  338177.2 |
  340435.7 |####################
  342694.2 |
  344952.7 |
  347211.2 |
  349469.7 |####################
  351728.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_wideselect_direct**: bridge=106.8% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_fntable**: bridge=102.2% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_null**: bridge=108.1% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_regcache**: bridge=102.9% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_switch**: bridge=110.0% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_threaded**: bridge=104.3% of algo (FFI overhead may distort results)
