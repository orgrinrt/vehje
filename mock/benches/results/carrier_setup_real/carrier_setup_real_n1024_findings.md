# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), real profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_real_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_real_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_real_stackcompile shows alternating (throttle bounce) (autocorr -0.69)

carrier_setup_real_stackcompile's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_real_parse)

The baseline carrier_setup_real_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_real_parse, carrier_setup_real_predecode, carrier_setup_real_emitdirect, carrier_setup_real_stackcompile, carrier_setup_real_emitcopypatch} vs {carrier_setup_real_optall} (757% apart)

The field splits into a fast tier {carrier_setup_real_parse, carrier_setup_real_predecode, carrier_setup_real_emitdirect, carrier_setup_real_stackcompile, carrier_setup_real_emitcopypatch} and a slow tier {carrier_setup_real_optall} with a 757% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 2.42 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_real_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 285278ns | 285188ns | 283675ns | 284922ns | 286614ns | +12106.51% |
| carrier_setup_real_emitdirect | 71556ns | 71730ns | 62974ns | 71072ns | 76573ns | +2961.76% |
| carrier_setup_real_optall | 2410258ns | 2426801ns | 2329660ns | 2400716ns | 2464870ns | +103030.30% |
| carrier_setup_real_parse | 2337ns | 2301ns | 2109ns | 2274ns | 2546ns | base |
| carrier_setup_real_predecode | 36887ns | 37114ns | 35178ns | 36547ns | 38251ns | +1478.31% |
| carrier_setup_real_stackcompile | 82462ns | 82511ns | 79876ns | 82217ns | 84124ns | +3428.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 283092ns | 281539ns | 284441ns | +0.00% | 0.004 |
| carrier_setup_real_emitdirect | 69300ns | 60765ns | 74376ns | +0.00% | 0.015 |
| carrier_setup_real_optall | 2407885ns | 2327402ns | 2462492ns | +0.00% | 0.000 |
| carrier_setup_real_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_real_predecode | 34678ns | 33084ns | 35950ns | +0.00% | 0.030 |
| carrier_setup_real_stackcompile | 80166ns | 77646ns | 81795ns | +0.00% | 0.013 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 1770303 | 5699363 | 0.311 | 123.75× |
| carrier_setup_real_emitdirect | 632186 | 3400419 | 0.186 | 44.19× |
| carrier_setup_real_optall | 15105726 | 69318349 | 0.218 | 1055.93× |
| carrier_setup_real_parse | 14306 | 52920 | 0.270 | 1.00× |
| carrier_setup_real_predecode | 425579 | 3133772 | 0.136 | 29.75× |
| carrier_setup_real_stackcompile | 693606 | 4226934 | 0.164 | 48.48× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_setup_real_predecode; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_real_emitcopypatch | 0.004 | 11.7% |
| carrier_setup_real_emitdirect | 0.015 | 47.7% |
| carrier_setup_real_optall | 0.000 | 1.4% |
| carrier_setup_real_predecode | 0.029 | 94.8% |
| carrier_setup_real_stackcompile | 0.013 | 41.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_real_emitcopypatch | 285278ns | 285278ns | +12106.51% |
| carrier_setup_real_emitdirect | 71556ns | 71556ns | +2961.76% |
| carrier_setup_real_optall | 2410258ns | 2410258ns | +103030.30% |
| carrier_setup_real_parse | 2337ns | 2337ns | base |
| carrier_setup_real_predecode | 36887ns | 36887ns | +1478.31% |
| carrier_setup_real_stackcompile | 82462ns | 82462ns | +3428.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_real_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_real_emitcopypatch | 283018ns | +283018.1ns (+0.0%) | [+281818, +284441]ns | [281818, 284441] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_emitdirect | 69373ns | +69373.3ns (+0.0%) | [+64152, +74376]ns | [64152, 74376] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_optall | 2424422ns | +2424422.5ns (+0.0%) | [+2336739, +2462492]ns | [2336739, 2462492] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_predecode | 34890ns | +34890.0ns (+0.0%) | [+33194, +35950]ns | [33194, 35950] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_stackcompile | 80213ns | +80213.1ns (+0.0%) | [+78489, +81795]ns | [78489, 81795] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_real_parse | carrier_setup_real_emitcopypatch | carrier_setup_real_emitdirect | carrier_setup_real_optall | carrier_setup_real_predecode | carrier_setup_real_stackcompile |
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
| carrier_setup_real_emitcopypatch | -0.578 | HIGH- (thermal bounce) |
| carrier_setup_real_emitdirect | -0.542 | HIGH- (thermal bounce) |
| carrier_setup_real_optall | -0.614 | HIGH- (thermal bounce) |
| carrier_setup_real_parse | 0.000 | ok |
| carrier_setup_real_predecode | 0.008 | ok |
| carrier_setup_real_stackcompile | -0.690 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_setup_real_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_real_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_real_optall**: won 0/6, lost 0/6
- **carrier_setup_real_predecode**: won 0/6, lost 0/6
- **carrier_setup_real_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 283398.1ns | 283092.1ns | 100.1% | HIGH |
| carrier_setup_real_emitdirect | 135939.6ns | 69300.5ns | 196.2% | HIGH |
| carrier_setup_real_optall | 2407535.4ns | 2407884.7ns | 100.0% | HIGH |
| carrier_setup_real_parse | 1593.4ns | 0.0ns | 0.0% |  |
| carrier_setup_real_predecode | 103864.0ns | 34677.8ns | 299.5% | HIGH |
| carrier_setup_real_stackcompile | 149779.3ns | 80165.8ns | 186.8% | HIGH |

## Distribution (algo ns)

```
carrier_setup_real_emitcopypatch (n=6, range 281538.8-284440.8 ns)
  281538.8 |########################################
  281683.9 |
  281829.0 |
  281974.1 |########################################
  282119.2 |
  282264.3 |########################################
  282409.4 |
  282554.5 |
  282699.6 |
  282844.7 |
  282989.8 |
  283134.9 |
  283280.0 |
  283425.1 |
  283570.2 |########################################
  283715.3 |
  283860.4 |########################################
  284005.5 |
  284150.6 |
  284295.7 |
  (0 below, 1 above range)

carrier_setup_real_emitdirect (n=6, range 60765.4-74376.2 ns)
  60765.4 |########################################
  61445.9 |
  62126.5 |
  62807.0 |
  63487.6 |
  64168.1 |
  64848.7 |
  65529.2 |
  66209.7 |
  66890.3 |########################################
  67570.8 |
  68251.4 |########################################
  68931.9 |
  69612.5 |########################################
  70293.0 |
  70973.5 |########################################
  71654.1 |
  72334.6 |
  73015.2 |
  73695.7 |
  (0 below, 1 above range)

carrier_setup_real_optall (n=6, range 2327401.7-2462492.3 ns)
  2327401.7 |####################
  2334156.2 |
  2340910.8 |####################
  2347665.3 |
  2354419.8 |
  2361174.4 |
  2367928.9 |
  2374683.4 |
  2381437.9 |
  2388192.5 |####################
  2394947.0 |
  2401701.5 |
  2408456.1 |
  2415210.6 |
  2421965.1 |
  2428719.6 |
  2435474.2 |
  2442228.7 |
  2448983.2 |
  2455737.8 |########################################
  (0 below, 1 above range)

carrier_setup_real_predecode (n=6, range 33084.2-35949.6 ns)
  33084.2 |####################
  33227.5 |####################
  33370.7 |
  33514.0 |
  33657.3 |
  33800.5 |
  33943.8 |####################
  34087.1 |
  34230.4 |
  34373.6 |
  34516.9 |
  34660.2 |
  34803.4 |
  34946.7 |
  35090.0 |
  35233.2 |
  35376.5 |
  35519.8 |
  35663.1 |########################################
  35806.3 |
  (0 below, 1 above range)

carrier_setup_real_stackcompile (n=6, range 77645.8-81795.2 ns)
  77645.8 |########################################
  77853.3 |
  78060.7 |
  78268.2 |
  78475.7 |
  78683.2 |
  78890.6 |
  79098.1 |
  79305.6 |########################################
  79513.1 |########################################
  79720.5 |
  79928.0 |
  80135.5 |
  80342.9 |
  80550.4 |
  80757.9 |########################################
  80965.4 |
  81172.8 |########################################
  81380.3 |
  81587.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_real_emitcopypatch**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_setup_real_emitdirect**: bridge=195.8% of algo (FFI overhead may distort results)
- **carrier_setup_real_optall**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_real_predecode**: bridge=298.1% of algo (FFI overhead may distort results)
- **carrier_setup_real_stackcompile**: bridge=193.6% of algo (FFI overhead may distort results)
