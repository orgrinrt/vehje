# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), madd profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_madd_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_madd_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_setup_madd_parse)

The baseline carrier_setup_madd_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_madd_parse, carrier_setup_madd_predecode, carrier_setup_madd_stackcompile, carrier_setup_madd_emitdirect, carrier_setup_madd_emitcopypatch} vs {carrier_setup_madd_optall} (216% apart)

The field splits into a fast tier {carrier_setup_madd_parse, carrier_setup_madd_predecode, carrier_setup_madd_stackcompile, carrier_setup_madd_emitdirect, carrier_setup_madd_emitcopypatch} and a slow tier {carrier_setup_madd_optall} with a 216% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 12.00 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_madd_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 3808039ns | 3797042ns | 3785689ns | 3793410ns | 3841158ns | +174315.33% |
| carrier_setup_madd_emitdirect | 1564579ns | 1564443ns | 1560898ns | 1563994ns | 1567298ns | +71560.65% |
| carrier_setup_madd_optall | 11994785ns | 12000369ns | 11949392ns | 11985296ns | 12031714ns | +549283.64% |
| carrier_setup_madd_parse | 2183ns | 2151ns | 2096ns | 2139ns | 2294ns | base |
| carrier_setup_madd_predecode | 551498ns | 550260ns | 547190ns | 549858ns | 556111ns | +25159.62% |
| carrier_setup_madd_stackcompile | 1300913ns | 1306934ns | 1284749ns | 1301154ns | 1308634ns | +59484.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 3804882ns | 3782977ns | 3837700ns | +0.00% | 0.004 |
| carrier_setup_madd_emitdirect | 1562059ns | 1558569ns | 1564611ns | +0.00% | 0.010 |
| carrier_setup_madd_optall | 11991611ns | 11946388ns | 12028430ns | +0.00% | 0.001 |
| carrier_setup_madd_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_madd_predecode | 549228ns | 545011ns | 553730ns | +0.00% | 0.030 |
| carrier_setup_madd_stackcompile | 1298516ns | 1282481ns | 1306315ns | +0.00% | 0.013 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 23728025 | 106308699 | 0.223 | 1301.31× |
| carrier_setup_madd_emitdirect | 9797197 | 51826297 | 0.189 | 537.30× |
| carrier_setup_madd_optall | 75122356 | 389137554 | 0.193 | 4119.91× |
| carrier_setup_madd_parse | 18234 | 79559 | 0.229 | 1.00× |
| carrier_setup_madd_predecode | 3443609 | 25711431 | 0.134 | 188.86× |
| carrier_setup_madd_stackcompile | 8163802 | 41825199 | 0.195 | 447.72× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.030 Gops/s** (carrier_setup_madd_predecode; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_madd_emitcopypatch | 0.004 | 14.4% |
| carrier_setup_madd_emitdirect | 0.010 | 34.9% |
| carrier_setup_madd_optall | 0.001 | 4.5% |
| carrier_setup_madd_predecode | 0.030 | 99.5% |
| carrier_setup_madd_stackcompile | 0.013 | 41.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 3808039ns | 3808039ns | +174315.33% |
| carrier_setup_madd_emitdirect | 1564579ns | 1564579ns | +71560.65% |
| carrier_setup_madd_optall | 11994785ns | 11994785ns | +549283.64% |
| carrier_setup_madd_parse | 2183ns | 2183ns | base |
| carrier_setup_madd_predecode | 551498ns | 551498ns | +25159.62% |
| carrier_setup_madd_stackcompile | 1300913ns | 1300913ns | +59484.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_madd_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_madd_emitcopypatch | 3793798ns | +3793797.7ns (+0.0%) | [+3783148, +3837700]ns | [3783148, 3837700] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_emitdirect | 1561964ns | +1561963.8ns (+0.0%) | [+1559600, +1564611]ns | [1559600, 1564611] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_optall | 11997247ns | +11997246.7ns (+0.0%) | [+11949156, +12028430]ns | [11949156, 12028430] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_predecode | 548007ns | +548007.1ns (+0.0%) | [+545948, +553730]ns | [545948, 553730] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_stackcompile | 1304448ns | +1304447.5ns (+0.0%) | [+1284787, +1306315]ns | [1284787, 1306315] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_madd_parse | carrier_setup_madd_emitcopypatch | carrier_setup_madd_emitdirect | carrier_setup_madd_optall | carrier_setup_madd_predecode | carrier_setup_madd_stackcompile |
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
| carrier_setup_madd_emitcopypatch | -0.365 | moderate- |
| carrier_setup_madd_emitdirect | -0.489 | moderate- |
| carrier_setup_madd_optall | -0.435 | moderate- |
| carrier_setup_madd_parse | 0.000 | ok |
| carrier_setup_madd_predecode | -0.177 | ok |
| carrier_setup_madd_stackcompile | 0.356 | moderate+ |

**Consistency summary:**

- **carrier_setup_madd_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_madd_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_madd_optall**: won 0/6, lost 0/6
- **carrier_setup_madd_predecode**: won 0/6, lost 0/6
- **carrier_setup_madd_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 3801333.3ns | 3804881.7ns | 99.9% | HIGH |
| carrier_setup_madd_emitdirect | 1563565.8ns | 1562058.6ns | 100.1% | HIGH |
| carrier_setup_madd_optall | 12009945.9ns | 11991611.0ns | 100.2% | HIGH |
| carrier_setup_madd_parse | 2798.5ns | 0.0ns | 0.0% |  |
| carrier_setup_madd_predecode | 548939.8ns | 549228.2ns | 99.9% | HIGH |
| carrier_setup_madd_stackcompile | 1304040.5ns | 1298516.3ns | 100.4% | HIGH |

## Distribution (algo ns)

```
carrier_setup_madd_emitcopypatch (n=6, range 3782977.1-3837699.5 ns)
  3782977.1 |########################################
  3785713.2 |
  3788449.3 |
  3791185.5 |####################
  3793921.6 |####################
  3796657.7 |
  3799393.8 |
  3802130.0 |
  3804866.1 |
  3807602.2 |
  3810338.3 |
  3813074.4 |####################
  3815810.6 |
  3818546.7 |
  3821282.8 |
  3824018.9 |
  3826755.1 |
  3829491.2 |
  3832227.3 |
  3834963.4 |
  (0 below, 1 above range)

carrier_setup_madd_emitdirect (n=6, range 1558568.8-1564611.5 ns)
  1558568.8 |########################################
  1558870.9 |
  1559173.1 |
  1559475.2 |
  1559777.3 |
  1560079.5 |
  1560381.6 |########################################
  1560683.7 |
  1560985.9 |
  1561288.0 |
  1561590.1 |########################################
  1561892.3 |########################################
  1562194.4 |
  1562496.5 |
  1562798.7 |
  1563100.8 |
  1563402.9 |
  1563705.1 |
  1564007.2 |
  1564309.3 |########################################
  (0 below, 1 above range)

carrier_setup_madd_optall (n=6, range 11946387.5-12028429.8 ns)
  11946387.5 |########################################
  11950489.6 |########################################
  11954591.7 |
  11958693.8 |
  11962796.0 |
  11966898.1 |
  11971000.2 |
  11975102.3 |
  11979204.4 |
  11983306.5 |
  11987408.7 |
  11991510.8 |########################################
  11995612.9 |
  11999715.0 |########################################
  12003817.1 |
  12007919.2 |
  12012021.3 |
  12016123.5 |########################################
  12020225.6 |
  12024327.7 |
  (0 below, 1 above range)

carrier_setup_madd_predecode (n=6, range 545010.8-553730.0 ns)
  545010.8 |########################################
  545446.8 |
  545882.7 |
  546318.7 |
  546754.6 |########################################
  547190.6 |########################################
  547626.6 |
  548062.5 |
  548498.5 |########################################
  548934.4 |
  549370.4 |
  549806.4 |
  550242.3 |
  550678.3 |
  551114.2 |
  551550.2 |
  551986.2 |########################################
  552422.1 |
  552858.1 |
  553294.0 |
  (0 below, 1 above range)

carrier_setup_madd_stackcompile (n=6, range 1282480.8-1306314.6 ns)
  1282480.8 |####################
  1283672.5 |
  1284864.2 |
  1286055.9 |####################
  1287247.6 |
  1288439.2 |
  1289630.9 |
  1290822.6 |
  1292014.3 |
  1293206.0 |
  1294397.7 |
  1295589.4 |
  1296781.1 |
  1297972.7 |
  1299164.4 |
  1300356.1 |
  1301547.8 |
  1302739.5 |
  1303931.2 |########################################
  1305122.9 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_madd_emitcopypatch**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_madd_emitdirect**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_madd_optall**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_madd_predecode**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_madd_stackcompile**: bridge=99.9% of algo (FFI overhead may distort results)
