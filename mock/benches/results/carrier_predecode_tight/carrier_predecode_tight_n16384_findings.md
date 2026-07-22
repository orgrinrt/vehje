# Predecoded dispatch shape, tight profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_tight_null dominates: 14% faster than the next best (carrier_pre_tight_direct)

carrier_pre_tight_null (519.12 us) leads carrier_pre_tight_direct (590.85 us) by 14%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_tight_threaded shows alternating (throttle bounce) (autocorr -0.68)

carrier_pre_tight_threaded's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_pre_tight_null** at 519117.1 ns median (-19.1% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.59x (fastest 519117.1 ns, slowest 824074.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_tight_direct | 594631ns | 593132ns | 591341ns | 592711ns | 599156ns | -7.78% |
| carrier_pre_tight_fntable | 797122ns | 797433ns | 795317ns | 796900ns | 798358ns | +23.62% |
| carrier_pre_tight_null | 522131ns | 521371ns | 518365ns | 521171ns | 525455ns | -19.03% |
| carrier_pre_tight_regcache | 843373ns | 826655ns | 802714ns | 822575ns | 894898ns | +30.79% |
| carrier_pre_tight_switch | 644821ns | 644330ns | 643355ns | 644205ns | 646478ns | base |
| carrier_pre_tight_threaded | 600014ns | 599485ns | 592141ns | 597606ns | 607562ns | -6.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_tight_direct | 592290ns | 589015ns | 596735ns | -7.82% | 0.028 |
| carrier_pre_tight_fntable | 794832ns | 793126ns | 796061ns | +23.70% | 0.021 |
| carrier_pre_tight_null | 519855ns | 516120ns | 523134ns | -19.09% | 0.032 |
| carrier_pre_tight_regcache | 841000ns | 800526ns | 892650ns | +30.89% | 0.019 |
| carrier_pre_tight_switch | 642545ns | 641155ns | 644178ns | base | 0.025 |
| carrier_pre_tight_threaded | 597754ns | 589922ns | 605280ns | -6.97% | 0.027 |

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_pre_tight_null; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_tight_direct | 0.028 | 87.4% |
| carrier_pre_tight_fntable | 0.021 | 64.9% |
| carrier_pre_tight_null | 0.032 | 99.4% |
| carrier_pre_tight_regcache | 0.020 | 62.6% |
| carrier_pre_tight_switch | 0.026 | 80.4% |
| carrier_pre_tight_threaded | 0.027 | 86.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_tight_direct | 594631ns | 594631ns | -7.78% |
| carrier_pre_tight_fntable | 797122ns | 797122ns | +23.62% |
| carrier_pre_tight_null | 522131ns | 522131ns | -19.03% |
| carrier_pre_tight_regcache | 843373ns | 843373ns | +30.79% |
| carrier_pre_tight_switch | 644821ns | 644821ns | base |
| carrier_pre_tight_threaded | 600014ns | 600014ns | -6.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_tight_switch | 642054ns | base | --- | [641403, 644178] | --- | --- | --- | --- |
| carrier_pre_tight_direct | 590851ns | -51726.4ns (-8.1%) | [-52679, -46360]ns | [589283, 596735] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_tight_fntable | 795102ns | +152948.1ns (+23.8%) | [+149682, +154230]ns | [793332, 796061] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_tight_null | 519117ns | -122512.3ns (-19.1%) | [-126109, -119448]ns | [517315, 523134] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_tight_regcache | 824075ns | +182579.6ns (+28.4%) | [+163695, +249091]ns | [806276, 892650] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_tight_threaded | 597258ns | -44787.1ns (-7.0%) | [-52834, -36752]ns | [590725, 605280] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_tight_switch | carrier_pre_tight_direct | carrier_pre_tight_fntable | carrier_pre_tight_null | carrier_pre_tight_regcache | carrier_pre_tight_threaded |
|---|---|---|---|---|---|---|
| 1 | 641652ns | -8.1% | +24.0% | -19.6% | +32.7% | -8.1% |
| 2 | 641836ns | -8.0% | +23.6% | -19.1% | +27.3% | -4.8% |
| 3 | 645465ns | -8.1% | +22.9% | -19.6% | +44.7% | -8.4% |
| 4 | 641155ns | -7.8% | +24.0% | -19.1% | +29.6% | -6.6% |
| 5 | 642272ns | -8.3% | +23.8% | -18.5% | +24.6% | -6.6% |
| 6 | 642890ns | -6.7% | +23.8% | -18.7% | +26.3% | -7.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_tight_direct | -0.239 | moderate- |
| carrier_pre_tight_fntable | 0.006 | ok |
| carrier_pre_tight_null | 0.215 | moderate+ |
| carrier_pre_tight_regcache | -0.153 | ok |
| carrier_pre_tight_switch | -0.436 | moderate- |
| carrier_pre_tight_threaded | -0.685 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_pre_tight_direct**: won 6/6, lost 0/6
- **carrier_pre_tight_fntable**: won 0/6, lost 6/6
- **carrier_pre_tight_null**: won 6/6, lost 0/6
- **carrier_pre_tight_regcache**: won 0/6, lost 6/6
- **carrier_pre_tight_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_tight_direct | 641867.0ns | 592289.6ns | 108.4% | HIGH |
| carrier_pre_tight_fntable | 832940.6ns | 794831.7ns | 104.8% | HIGH |
| carrier_pre_tight_null | 556334.9ns | 519855.3ns | 107.0% | HIGH |
| carrier_pre_tight_regcache | 878295.8ns | 841000.3ns | 104.4% | HIGH |
| carrier_pre_tight_switch | 694979.0ns | 642545.1ns | 108.2% | HIGH |
| carrier_pre_tight_threaded | 635531.6ns | 597754.3ns | 106.3% | HIGH |

## Distribution (algo ns)

```
carrier_pre_tight_direct (n=6, range 589015.0-596734.8 ns)
  589015.0 |########################################
  589401.0 |########################################
  589787.0 |
  590173.0 |########################################
  590559.0 |
  590944.9 |########################################
  591330.9 |
  591716.9 |
  592102.9 |
  592488.9 |
  592874.9 |
  593260.9 |########################################
  593646.9 |
  594032.9 |
  594418.9 |
  594804.9 |
  595190.8 |
  595576.8 |
  595962.8 |
  596348.8 |
  (0 below, 1 above range)

carrier_pre_tight_fntable (n=6, range 793126.2-796060.6 ns)
  793126.2 |########################################
  793272.9 |
  793419.6 |########################################
  793566.4 |
  793713.1 |
  793859.8 |
  794006.5 |
  794153.2 |
  794300.0 |
  794446.7 |
  794593.4 |
  794740.1 |
  794886.8 |########################################
  795033.6 |
  795180.3 |########################################
  795327.0 |
  795473.7 |
  795620.4 |
  795767.2 |
  795913.9 |########################################
  (0 below, 1 above range)

carrier_pre_tight_null (n=6, range 516120.0-523133.5 ns)
  516120.0 |########################################
  516470.7 |
  516821.3 |
  517172.0 |
  517522.7 |
  517873.4 |
  518224.0 |########################################
  518574.7 |########################################
  518925.4 |
  519276.1 |########################################
  519626.8 |
  519977.4 |
  520328.1 |
  520678.8 |
  521029.5 |
  521380.1 |
  521730.8 |
  522081.5 |
  522432.2 |########################################
  522782.8 |
  (0 below, 1 above range)

carrier_pre_tight_regcache (n=6, range 800526.2-892650.0 ns)
  800526.2 |########################################
  805132.4 |
  809738.6 |########################################
  814344.8 |########################################
  818951.0 |
  823557.1 |
  828163.3 |########################################
  832769.5 |
  837375.7 |
  841981.9 |
  846588.1 |
  851194.3 |########################################
  855800.5 |
  860406.7 |
  865012.9 |
  869619.1 |
  874225.2 |
  878831.4 |
  883437.6 |
  888043.8 |
  (0 below, 1 above range)

carrier_pre_tight_switch (n=6, range 641154.6-644177.9 ns)
  641154.6 |########################################
  641305.8 |
  641456.9 |
  641608.1 |########################################
  641759.3 |########################################
  641910.4 |
  642061.6 |
  642212.8 |########################################
  642363.9 |
  642515.1 |
  642666.2 |
  642817.4 |########################################
  642968.6 |
  643119.7 |
  643270.9 |
  643422.1 |
  643573.2 |
  643724.4 |
  643875.6 |
  644026.7 |
  (0 below, 1 above range)

carrier_pre_tight_threaded (n=6, range 589921.7-605279.6 ns)
  589921.7 |########################################
  590689.6 |
  591457.5 |########################################
  592225.4 |
  592993.3 |
  593761.2 |
  594529.1 |
  595296.9 |########################################
  596064.8 |
  596832.7 |
  597600.6 |
  598368.5 |########################################
  599136.4 |########################################
  599904.3 |
  600672.2 |
  601440.1 |
  602208.0 |
  602975.9 |
  603743.8 |
  604511.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_tight_direct**: bridge=108.5% of algo (FFI overhead may distort results)
- **carrier_pre_tight_fntable**: bridge=104.8% of algo (FFI overhead may distort results)
- **carrier_pre_tight_null**: bridge=107.1% of algo (FFI overhead may distort results)
- **carrier_pre_tight_regcache**: bridge=104.5% of algo (FFI overhead may distort results)
- **carrier_pre_tight_switch**: bridge=108.2% of algo (FFI overhead may distort results)
- **carrier_pre_tight_threaded**: bridge=106.2% of algo (FFI overhead may distort results)
