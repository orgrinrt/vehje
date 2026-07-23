# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), wideselect profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_wideselect_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_wideselect_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_wideselect_emitdirect shows alternating (throttle bounce) (autocorr -0.61)

carrier_setup_wideselect_emitdirect's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_wideselect_parse)

The baseline carrier_setup_wideselect_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_wideselect_parse, carrier_setup_wideselect_predecode, carrier_setup_wideselect_emitdirect, carrier_setup_wideselect_stackcompile, carrier_setup_wideselect_emitcopypatch} vs {carrier_setup_wideselect_optall} (717% apart)

The field splits into a fast tier {carrier_setup_wideselect_parse, carrier_setup_wideselect_predecode, carrier_setup_wideselect_emitdirect, carrier_setup_wideselect_stackcompile, carrier_setup_wideselect_emitcopypatch} and a slow tier {carrier_setup_wideselect_optall} with a 717% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 653.25 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_wideselect_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 82309ns | 82132ns | 80480ns | 81811ns | 83970ns | +3594.33% |
| carrier_setup_wideselect_emitdirect | 17988ns | 17906ns | 16848ns | 17620ns | 19109ns | +707.35% |
| carrier_setup_wideselect_optall | 658264ns | 655730ns | 648285ns | 654765ns | 668501ns | +29445.27% |
| carrier_setup_wideselect_parse | 2228ns | 2219ns | 2136ns | 2194ns | 2325ns | base |
| carrier_setup_wideselect_predecode | 12406ns | 12390ns | 11999ns | 12359ns | 12680ns | +456.84% |
| carrier_setup_wideselect_stackcompile | 26154ns | 26437ns | 25232ns | 26106ns | 26686ns | +1073.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 80073ns | 78186ns | 81727ns | +0.00% | 0.003 |
| carrier_setup_wideselect_emitdirect | 15765ns | 14691ns | 16821ns | +0.00% | 0.016 |
| carrier_setup_wideselect_optall | 655741ns | 645800ns | 665870ns | +0.00% | 0.000 |
| carrier_setup_wideselect_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_wideselect_predecode | 10197ns | 9888ns | 10413ns | +0.00% | 0.025 |
| carrier_setup_wideselect_stackcompile | 23927ns | 23067ns | 24411ns | +0.00% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 716970 | 2596067 | 0.276 | 49.96× |
| carrier_setup_wideselect_emitdirect | 330527 | 2213913 | 0.149 | 23.03× |
| carrier_setup_wideselect_optall | 4016014 | 18417608 | 0.218 | 279.83× |
| carrier_setup_wideselect_parse | 14352 | 52920 | 0.271 | 1.00× |
| carrier_setup_wideselect_predecode | 319475 | 2301888 | 0.139 | 22.26× |
| carrier_setup_wideselect_stackcompile | 371101 | 2292426 | 0.162 | 25.86× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_setup_wideselect_predecode; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 0.003 | 12.4% |
| carrier_setup_wideselect_emitdirect | 0.016 | 63.1% |
| carrier_setup_wideselect_optall | 0.000 | 1.5% |
| carrier_setup_wideselect_predecode | 0.025 | 97.1% |
| carrier_setup_wideselect_stackcompile | 0.011 | 40.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 82309ns | 82309ns | +3594.33% |
| carrier_setup_wideselect_emitdirect | 17988ns | 17988ns | +707.35% |
| carrier_setup_wideselect_optall | 658264ns | 658264ns | +29445.27% |
| carrier_setup_wideselect_parse | 2228ns | 2228ns | base |
| carrier_setup_wideselect_predecode | 12406ns | 12406ns | +456.84% |
| carrier_setup_wideselect_stackcompile | 26154ns | 26154ns | +1073.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_wideselect_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_wideselect_emitcopypatch | 79911ns | +79911.0ns (+0.0%) | [+78581, +81727]ns | [78581, 81727] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_emitdirect | 15681ns | +15680.6ns (+0.0%) | [+14794, +16821]ns | [14794, 16821] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_optall | 653252ns | +653252.1ns (+0.0%) | [+648102, +665870]ns | [648102, 665870] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_predecode | 10178ns | +10177.9ns (+0.0%) | [+9999, +10413]ns | [9999, 10413] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_stackcompile | 24208ns | +24207.7ns (+0.0%) | [+23164, +24411]ns | [23164, 24411] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_wideselect_emitcopypatch | 0.397 | moderate+ |
| carrier_setup_wideselect_emitdirect | -0.611 | HIGH- (thermal bounce) |
| carrier_setup_wideselect_optall | -0.308 | moderate- |
| carrier_setup_wideselect_parse | 0.000 | ok |
| carrier_setup_wideselect_predecode | -0.164 | ok |
| carrier_setup_wideselect_stackcompile | 0.303 | moderate+ |

**Consistency summary:**

- **carrier_setup_wideselect_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_wideselect_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_wideselect_optall**: won 0/6, lost 0/6
- **carrier_setup_wideselect_predecode**: won 0/6, lost 0/6
- **carrier_setup_wideselect_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 152083.5ns | 80073.0ns | 189.9% | HIGH |
| carrier_setup_wideselect_emitdirect | 90862.7ns | 15765.0ns | 576.4% | HIGH |
| carrier_setup_wideselect_optall | 654345.0ns | 655741.4ns | 99.8% | HIGH |
| carrier_setup_wideselect_parse | 1502.4ns | 0.0ns | 0.0% |  |
| carrier_setup_wideselect_predecode | 91374.2ns | 10196.7ns | 896.1% | HIGH |
| carrier_setup_wideselect_stackcompile | 95393.6ns | 23927.5ns | 398.7% | HIGH |

## Distribution (algo ns)

```
carrier_setup_wideselect_emitcopypatch (n=6, range 78185.8-81726.7 ns)
  78185.8 |########################################
  78362.8 |
  78539.9 |
  78716.9 |
  78894.0 |########################################
  79071.0 |
  79248.1 |
  79425.1 |########################################
  79602.2 |
  79779.2 |
  79956.2 |
  80133.3 |########################################
  80310.3 |
  80487.4 |
  80664.4 |########################################
  80841.5 |
  81018.5 |
  81195.6 |
  81372.6 |
  81549.7 |
  (0 below, 1 above range)

carrier_setup_wideselect_emitdirect (n=6, range 14691.2-16820.8 ns)
  14691.2 |########################################
  14797.7 |########################################
  14904.2 |
  15010.6 |########################################
  15117.1 |
  15223.6 |
  15330.1 |
  15436.6 |
  15543.0 |
  15649.5 |
  15756.0 |
  15862.5 |
  15969.0 |
  16075.4 |
  16181.9 |########################################
  16288.4 |
  16394.9 |
  16501.4 |
  16607.8 |########################################
  16714.3 |
  (0 below, 1 above range)

carrier_setup_wideselect_optall (n=6, range 645799.6-665870.4 ns)
  645799.6 |########################################
  646803.1 |
  647806.7 |
  648810.2 |
  649813.8 |########################################
  650817.3 |########################################
  651820.9 |
  652824.4 |
  653827.9 |
  654831.5 |########################################
  655835.0 |
  656838.6 |
  657842.1 |
  658845.7 |
  659849.2 |
  660852.7 |
  661856.3 |
  662859.8 |
  663863.4 |########################################
  664866.9 |
  (0 below, 1 above range)

carrier_setup_wideselect_predecode (n=6, range 9887.5-10412.9 ns)
   9887.5 |########################################
   9913.8 |
   9940.0 |
   9966.3 |
   9992.6 |
  10018.9 |
  10045.1 |
  10071.4 |
  10097.7 |########################################
  10123.9 |########################################
  10150.2 |
  10176.5 |
  10202.7 |########################################
  10229.0 |
  10255.3 |
  10281.5 |
  10307.8 |########################################
  10334.1 |
  10360.4 |
  10386.6 |
  (0 below, 1 above range)

carrier_setup_wideselect_stackcompile (n=6, range 23067.1-24410.8 ns)
  23067.1 |####################
  23134.3 |
  23201.5 |####################
  23268.7 |
  23335.8 |
  23403.0 |
  23470.2 |
  23537.4 |
  23604.6 |
  23671.8 |
  23738.9 |
  23806.1 |
  23873.3 |
  23940.5 |
  24007.7 |
  24074.9 |####################
  24142.1 |
  24209.2 |
  24276.4 |########################################
  24343.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_wideselect_emitcopypatch**: bridge=191.8% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_emitdirect**: bridge=574.2% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_optall**: bridge=99.7% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_predecode**: bridge=896.1% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_stackcompile**: bridge=397.9% of algo (FFI overhead may distort results)
