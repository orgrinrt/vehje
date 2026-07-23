# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), scatter profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_scatter_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_scatter_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_scatter_stackcompile shows alternating (throttle bounce) (autocorr -0.54)

carrier_setup_scatter_stackcompile's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_scatter_parse)

The baseline carrier_setup_scatter_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_scatter_parse, carrier_setup_scatter_predecode, carrier_setup_scatter_stackcompile, carrier_setup_scatter_emitdirect, carrier_setup_scatter_emitcopypatch} vs {carrier_setup_scatter_optall} (740% apart)

The field splits into a fast tier {carrier_setup_scatter_parse, carrier_setup_scatter_predecode, carrier_setup_scatter_stackcompile, carrier_setup_scatter_emitdirect, carrier_setup_scatter_emitcopypatch} and a slow tier {carrier_setup_scatter_optall} with a 740% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 50.22 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_scatter_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 5980808ns | 5986043ns | 5943217ns | 5975274ns | 6007902ns | +281515.42% |
| carrier_setup_scatter_emitdirect | 3532161ns | 3530591ns | 3508154ns | 3525029ns | 3554862ns | +166217.17% |
| carrier_setup_scatter_optall | 50518985ns | 50223055ns | 49826737ns | 50120126ns | 51463398ns | +2378663.27% |
| carrier_setup_scatter_parse | 2124ns | 2124ns | 2113ns | 2121ns | 2134ns | base |
| carrier_setup_scatter_predecode | 726725ns | 724567ns | 703471ns | 722085ns | 745313ns | +34118.96% |
| carrier_setup_scatter_stackcompile | 1949852ns | 1945010ns | 1907050ns | 1942216ns | 1982707ns | +91711.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 5976603ns | 5939282ns | 6003474ns | +0.00% | 0.003 |
| carrier_setup_scatter_emitdirect | 3528688ns | 3505138ns | 3551093ns | +0.00% | 0.005 |
| carrier_setup_scatter_optall | 50514654ns | 49822706ns | 51458883ns | +0.00% | 0.000 |
| carrier_setup_scatter_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_scatter_predecode | 724267ns | 701176ns | 743000ns | +0.00% | 0.023 |
| carrier_setup_scatter_stackcompile | 1947161ns | 1904503ns | 1979848ns | +0.00% | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 37296482 | 103098024 | 0.362 | 2045.51× |
| carrier_setup_scatter_emitdirect | 22014241 | 47139174 | 0.467 | 1207.36× |
| carrier_setup_scatter_optall | 312971679 | 1291962785 | 0.242 | 17164.81× |
| carrier_setup_scatter_parse | 18233 | 79538 | 0.229 | 1.00× |
| carrier_setup_scatter_predecode | 4526927 | 24750477 | 0.183 | 248.28× |
| carrier_setup_scatter_stackcompile | 12159712 | 42054991 | 0.289 | 666.89× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.023 Gops/s** (carrier_setup_scatter_predecode; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_scatter_emitcopypatch | 0.003 | 11.7% |
| carrier_setup_scatter_emitdirect | 0.005 | 19.9% |
| carrier_setup_scatter_optall | 0.000 | 1.4% |
| carrier_setup_scatter_predecode | 0.023 | 97.1% |
| carrier_setup_scatter_stackcompile | 0.008 | 36.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 5980808ns | 5980808ns | +281515.42% |
| carrier_setup_scatter_emitdirect | 3532161ns | 3532161ns | +166217.17% |
| carrier_setup_scatter_optall | 50518985ns | 50518985ns | +2378663.27% |
| carrier_setup_scatter_parse | 2124ns | 2124ns | base |
| carrier_setup_scatter_predecode | 726725ns | 726725ns | +34118.96% |
| carrier_setup_scatter_stackcompile | 1949852ns | 1949852ns | +91711.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_scatter_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_scatter_emitcopypatch | 5981794ns | +5981794.2ns (+0.0%) | [+5944542, +6003474]ns | [5944542, 6003474] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_emitdirect | 3527198ns | +3527198.4ns (+0.0%) | [+3507773, +3551093]ns | [3507773, 3551093] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_optall | 50218743ns | +50218742.9ns (+0.0%) | [+49866335, +51458883]ns | [49866335, 51458883] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_predecode | 722177ns | +722177.3ns (+0.0%) | [+707623, +743000]ns | [707623, 743000] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_stackcompile | 1942315ns | +1942315.2ns (+0.0%) | [+1919321, +1979848]ns | [1919321, 1979848] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_scatter_parse | carrier_setup_scatter_emitcopypatch | carrier_setup_scatter_emitdirect | carrier_setup_scatter_optall | carrier_setup_scatter_predecode | carrier_setup_scatter_stackcompile |
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
| carrier_setup_scatter_emitcopypatch | -0.253 | moderate- |
| carrier_setup_scatter_emitdirect | -0.004 | ok |
| carrier_setup_scatter_optall | 0.285 | moderate+ |
| carrier_setup_scatter_parse | 0.000 | ok |
| carrier_setup_scatter_predecode | -0.351 | moderate- |
| carrier_setup_scatter_stackcompile | -0.542 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_setup_scatter_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_scatter_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_scatter_optall**: won 0/6, lost 0/6
- **carrier_setup_scatter_predecode**: won 0/6, lost 0/6
- **carrier_setup_scatter_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 5976092.4ns | 5976603.2ns | 100.0% | HIGH |
| carrier_setup_scatter_emitdirect | 3532047.9ns | 3528688.1ns | 100.1% | HIGH |
| carrier_setup_scatter_optall | 50464317.5ns | 50514653.7ns | 99.9% | HIGH |
| carrier_setup_scatter_parse | 2728.8ns | 0.0ns | 0.0% |  |
| carrier_setup_scatter_predecode | 724978.6ns | 724266.8ns | 100.1% | HIGH |
| carrier_setup_scatter_stackcompile | 1949918.6ns | 1947161.4ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_setup_scatter_emitcopypatch (n=6, range 5939282.5-6003473.8 ns)
  5939282.5 |####################
  5942492.1 |
  5945701.6 |
  5948911.2 |####################
  5952120.8 |
  5955330.3 |
  5958539.9 |
  5961749.4 |####################
  5964959.0 |
  5968168.6 |
  5971378.1 |
  5974587.7 |
  5977797.2 |
  5981006.8 |
  5984216.4 |
  5987425.9 |
  5990635.5 |
  5993845.1 |
  5997054.6 |########################################
  6000264.2 |
  (0 below, 1 above range)

carrier_setup_scatter_emitdirect (n=6, range 3505137.9-3551093.0 ns)
  3505137.9 |########################################
  3507435.7 |
  3509733.4 |########################################
  3512031.2 |
  3514328.9 |
  3516626.7 |
  3518924.4 |########################################
  3521222.2 |
  3523519.9 |
  3525817.7 |
  3528115.4 |
  3530413.2 |
  3532710.9 |########################################
  3535008.7 |
  3537306.4 |
  3539604.2 |
  3541901.9 |
  3544199.7 |
  3546497.4 |
  3548795.2 |########################################
  (0 below, 1 above range)

carrier_setup_scatter_optall (n=6, range 49822706.2-51458883.1 ns)
  49822706.2 |########################################
  49904515.0 |########################################
  49986323.9 |
  50068132.7 |########################################
  50149941.6 |
  50231750.4 |
  50313559.3 |########################################
  50395368.1 |
  50477177.0 |
  50558985.8 |
  50640794.7 |
  50722603.5 |
  50804412.3 |
  50886221.2 |########################################
  50968030.0 |
  51049838.9 |
  51131647.7 |
  51213456.6 |
  51295265.4 |
  51377074.3 |
  (0 below, 1 above range)

carrier_setup_scatter_predecode (n=6, range 701176.2-743000.0 ns)
  701176.2 |########################################
  703267.4 |
  705358.6 |
  707449.8 |
  709541.0 |
  711632.1 |
  713723.3 |########################################
  715814.5 |
  717905.7 |########################################
  719996.9 |
  722088.1 |
  724179.3 |########################################
  726270.5 |
  728361.7 |
  730452.9 |
  732544.1 |########################################
  734635.2 |
  736726.4 |
  738817.6 |
  740908.8 |
  (0 below, 1 above range)

carrier_setup_scatter_stackcompile (n=6, range 1904503.3-1979848.1 ns)
  1904503.3 |########################################
  1908270.5 |
  1912037.8 |
  1915805.0 |
  1919572.3 |
  1923339.5 |
  1927106.8 |
  1930874.0 |########################################
  1934641.2 |
  1938408.5 |########################################
  1942175.7 |########################################
  1945943.0 |
  1949710.2 |
  1953477.5 |
  1957244.7 |########################################
  1961011.9 |
  1964779.2 |
  1968546.4 |
  1972313.7 |
  1976080.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_scatter_emitcopypatch**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_emitdirect**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_optall**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_predecode**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_stackcompile**: bridge=100.0% of algo (FFI overhead may distort results)
