# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), wideselect profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_wideselect_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_wideselect_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_wideselect_emitdirect shows warm-up / thermal drift (autocorr +0.53)

carrier_setup_wideselect_emitdirect's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_wideselect_parse)

The baseline carrier_setup_wideselect_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_wideselect_parse, carrier_setup_wideselect_predecode, carrier_setup_wideselect_stackcompile, carrier_setup_wideselect_emitdirect, carrier_setup_wideselect_emitcopypatch} vs {carrier_setup_wideselect_optall} (698% apart)

The field splits into a fast tier {carrier_setup_wideselect_parse, carrier_setup_wideselect_predecode, carrier_setup_wideselect_stackcompile, carrier_setup_wideselect_emitdirect, carrier_setup_wideselect_emitcopypatch} and a slow tier {carrier_setup_wideselect_optall} with a 698% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 44.09 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_wideselect_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 5524845ns | 5528568ns | 5477670ns | 5526327ns | 5546210ns | +251523.02% |
| carrier_setup_wideselect_emitdirect | 2643991ns | 2642792ns | 2619314ns | 2639462ns | 2663124ns | +120317.68% |
| carrier_setup_wideselect_optall | 43986966ns | 44094582ns | 43176085ns | 44032405ns | 44324247ns | +2003238.32% |
| carrier_setup_wideselect_parse | 2196ns | 2165ns | 2091ns | 2156ns | 2308ns | base |
| carrier_setup_wideselect_predecode | 1261652ns | 1262531ns | 1247012ns | 1257946ns | 1274530ns | +57360.55% |
| carrier_setup_wideselect_stackcompile | 2615215ns | 2609736ns | 2567497ns | 2605744ns | 2653281ns | +119007.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 5520699ns | 5473910ns | 5541699ns | +0.00% | 0.003 |
| carrier_setup_wideselect_emitdirect | 2640629ns | 2616615ns | 2659503ns | +0.00% | 0.006 |
| carrier_setup_wideselect_optall | 43982571ns | 43172383ns | 44319671ns | +0.00% | 0.000 |
| carrier_setup_wideselect_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_wideselect_predecode | 1258964ns | 1244305ns | 1271763ns | +0.00% | 0.013 |
| carrier_setup_wideselect_stackcompile | 2612248ns | 2564724ns | 2649989ns | +0.00% | 0.006 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 34188463 | 128136806 | 0.267 | 1867.56× |
| carrier_setup_wideselect_emitdirect | 16337417 | 54596985 | 0.299 | 892.44× |
| carrier_setup_wideselect_optall | 272229041 | 1105790985 | 0.246 | 14870.62× |
| carrier_setup_wideselect_parse | 18306 | 79566 | 0.230 | 1.00× |
| carrier_setup_wideselect_predecode | 7846181 | 26362694 | 0.298 | 428.60× |
| carrier_setup_wideselect_stackcompile | 16162769 | 42823753 | 0.377 | 882.90× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.013 Gops/s** (carrier_setup_wideselect_predecode; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 0.003 | 22.5% |
| carrier_setup_wideselect_emitdirect | 0.006 | 47.1% |
| carrier_setup_wideselect_optall | 0.000 | 2.8% |
| carrier_setup_wideselect_predecode | 0.013 | 98.8% |
| carrier_setup_wideselect_stackcompile | 0.006 | 47.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 5524845ns | 5524845ns | +251523.02% |
| carrier_setup_wideselect_emitdirect | 2643991ns | 2643991ns | +120317.68% |
| carrier_setup_wideselect_optall | 43986966ns | 43986966ns | +2003238.32% |
| carrier_setup_wideselect_parse | 2196ns | 2196ns | base |
| carrier_setup_wideselect_predecode | 1261652ns | 1261652ns | +57360.55% |
| carrier_setup_wideselect_stackcompile | 2615215ns | 2615215ns | +119007.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_wideselect_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_wideselect_emitcopypatch | 5524513ns | +5524513.3ns (+0.0%) | [+5495883, +5541699]ns | [5495883, 5541699] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_emitdirect | 2639386ns | +2639386.2ns (+0.0%) | [+2622997, +2659503]ns | [2622997, 2659503] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_optall | 44089809ns | +44089809.4ns (+0.0%) | [+43538234, +44319671]ns | [43538234, 44319671] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_predecode | 1259866ns | +1259865.9ns (+0.0%) | [+1245263, +1271763]ns | [1245263, 1271763] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_stackcompile | 2606792ns | +2606791.7ns (+0.0%) | [+2579961, +2649989]ns | [2579961, 2649989] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_wideselect_parse | carrier_setup_wideselect_emitcopypatch | carrier_setup_wideselect_emitdirect | carrier_setup_wideselect_optall | carrier_setup_wideselect_predecode | carrier_setup_wideselect_stackcompile |
|---|---|---|---|---|---|---|
| 1 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 2 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 3 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 4 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 5 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 6 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_setup_wideselect_emitcopypatch | -0.010 | ok |
| carrier_setup_wideselect_emitdirect | 0.532 | HIGH+ (drift/warm-up) |
| carrier_setup_wideselect_optall | -0.046 | ok |
| carrier_setup_wideselect_parse | 0.000 | ok |
| carrier_setup_wideselect_predecode | -0.008 | ok |
| carrier_setup_wideselect_stackcompile | 0.189 | ok |

**Consistency summary:**

- **carrier_setup_wideselect_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_wideselect_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_wideselect_optall**: won 0/6, lost 0/6
- **carrier_setup_wideselect_predecode**: won 0/6, lost 0/6
- **carrier_setup_wideselect_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 5515639.0ns | 5520698.6ns | 99.9% | HIGH |
| carrier_setup_wideselect_emitdirect | 2644094.4ns | 2640628.8ns | 100.1% | HIGH |
| carrier_setup_wideselect_optall | 43991756.5ns | 43982571.3ns | 100.0% | HIGH |
| carrier_setup_wideselect_parse | 2810.8ns | 0.0ns | 0.0% |  |
| carrier_setup_wideselect_predecode | 1258936.3ns | 1258964.0ns | 100.0% | HIGH |
| carrier_setup_wideselect_stackcompile | 2610779.5ns | 2612247.5ns | 99.9% | HIGH |

## Distribution (algo ns)

```
carrier_setup_wideselect_emitcopypatch (n=6, range 5473910.4-5541699.2 ns)
  5473910.4 |########################################
  5477299.8 |
  5480689.3 |
  5484078.7 |
  5487468.2 |
  5490857.6 |
  5494247.0 |
  5497636.5 |
  5501025.9 |
  5504415.3 |
  5507804.8 |
  5511194.2 |
  5514583.7 |########################################
  5517973.1 |
  5521362.5 |########################################
  5524752.0 |########################################
  5528141.4 |
  5531530.8 |
  5534920.3 |########################################
  5538309.7 |
  (0 below, 1 above range)

carrier_setup_wideselect_emitdirect (n=6, range 2616615.4-2659502.7 ns)
  2616615.4 |########################################
  2618759.8 |
  2620904.1 |
  2623048.5 |
  2625192.9 |
  2627337.2 |########################################
  2629481.6 |########################################
  2631626.0 |
  2633770.3 |
  2635914.7 |
  2638059.0 |
  2640203.4 |
  2642347.8 |
  2644492.1 |
  2646636.5 |########################################
  2648780.9 |
  2650925.2 |
  2653069.6 |########################################
  2655214.0 |
  2657358.3 |
  (0 below, 1 above range)

carrier_setup_wideselect_optall (n=6, range 43172382.9-44319670.7 ns)
  43172382.9 |########################################
  43229747.3 |
  43287111.7 |
  43344476.1 |
  43401840.5 |
  43459204.8 |
  43516569.2 |
  43573933.6 |
  43631298.0 |
  43688662.4 |
  43746026.8 |
  43803391.2 |
  43860755.6 |########################################
  43918119.9 |########################################
  43975484.3 |
  44032848.7 |
  44090213.1 |
  44147577.5 |
  44204941.9 |########################################
  44262306.3 |########################################
  (0 below, 1 above range)

carrier_setup_wideselect_predecode (n=6, range 1244304.6-1271763.1 ns)
  1244304.6 |########################################
  1245677.5 |########################################
  1247050.5 |
  1248423.4 |
  1249796.3 |
  1251169.2 |########################################
  1252542.2 |
  1253915.1 |
  1255288.0 |
  1256660.9 |
  1258033.9 |
  1259406.8 |
  1260779.7 |
  1262152.7 |
  1263525.6 |
  1264898.5 |
  1266271.4 |########################################
  1267644.4 |########################################
  1269017.3 |
  1270390.2 |
  (0 below, 1 above range)

carrier_setup_wideselect_stackcompile (n=6, range 2564724.2-2649989.4 ns)
  2564724.2 |########################################
  2568987.5 |
  2573250.7 |
  2577514.0 |
  2581777.2 |
  2586040.5 |
  2590303.8 |
  2594567.0 |########################################
  2598830.3 |########################################
  2603093.5 |
  2607356.8 |
  2611620.1 |########################################
  2615883.3 |
  2620146.6 |
  2624409.8 |
  2628673.1 |
  2632936.4 |
  2637199.6 |
  2641462.9 |########################################
  2645726.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_wideselect_emitcopypatch**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_emitdirect**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **carrier_setup_wideselect_emitdirect**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_optall**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_predecode**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_stackcompile**: bridge=100.1% of algo (FFI overhead may distort results)
