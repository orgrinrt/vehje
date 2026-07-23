# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), real profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_real_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_real_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_setup_real_parse)

The baseline carrier_setup_real_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_real_parse, carrier_setup_real_predecode, carrier_setup_real_emitdirect, carrier_setup_real_stackcompile, carrier_setup_real_emitcopypatch} vs {carrier_setup_real_optall} (927% apart)

The field splits into a fast tier {carrier_setup_real_parse, carrier_setup_real_predecode, carrier_setup_real_emitdirect, carrier_setup_real_stackcompile, carrier_setup_real_emitcopypatch} and a slow tier {carrier_setup_real_optall} with a 927% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 155.74 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_real_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 17237ns | 17559ns | 15369ns | 17511ns | 17760ns | +590.08% |
| carrier_setup_real_emitdirect | 6436ns | 6516ns | 5858ns | 6407ns | 6769ns | +157.67% |
| carrier_setup_real_optall | 158057ns | 158005ns | 155682ns | 157387ns | 160250ns | +6227.80% |
| carrier_setup_real_parse | 2498ns | 2551ns | 2385ns | 2497ns | 2556ns | base |
| carrier_setup_real_predecode | 5710ns | 5803ns | 5177ns | 5769ns | 5888ns | +128.61% |
| carrier_setup_real_stackcompile | 11868ns | 11935ns | 11164ns | 11912ns | 12153ns | +375.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 14882ns | 13248ns | 15343ns | +0.00% | 0.004 |
| carrier_setup_real_emitdirect | 4114ns | 3762ns | 4326ns | +0.00% | 0.016 |
| carrier_setup_real_optall | 155769ns | 153442ns | 157921ns | +0.00% | 0.000 |
| carrier_setup_real_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_real_predecode | 3392ns | 3070ns | 3500ns | +0.00% | 0.019 |
| carrier_setup_real_stackcompile | 9501ns | 8911ns | 9753ns | +0.00% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 310537 | 1314538 | 0.236 | 21.82× |
| carrier_setup_real_emitdirect | 273472 | 1690412 | 0.162 | 19.21× |
| carrier_setup_real_optall | 947343 | 4664122 | 0.203 | 66.56× |
| carrier_setup_real_parse | 14234 | 52913 | 0.269 | 1.00× |
| carrier_setup_real_predecode | 265750 | 1876973 | 0.142 | 18.67× |
| carrier_setup_real_stackcompile | 285754 | 1722781 | 0.166 | 20.08× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.021 Gops/s** (carrier_setup_real_predecode; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_real_emitcopypatch | 0.004 | 20.3% |
| carrier_setup_real_emitdirect | 0.015 | 73.3% |
| carrier_setup_real_optall | 0.000 | 2.0% |
| carrier_setup_real_predecode | 0.019 | 89.1% |
| carrier_setup_real_stackcompile | 0.007 | 32.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_real_emitcopypatch | 17237ns | 17237ns | +590.08% |
| carrier_setup_real_emitdirect | 6436ns | 6436ns | +157.67% |
| carrier_setup_real_optall | 158057ns | 158057ns | +6227.80% |
| carrier_setup_real_parse | 2498ns | 2498ns | base |
| carrier_setup_real_predecode | 5710ns | 5710ns | +128.61% |
| carrier_setup_real_stackcompile | 11868ns | 11868ns | +375.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_real_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_real_emitcopypatch | 15158ns | +15158.1ns (+0.0%) | [+14145, +15343]ns | [14145, 15343] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_emitdirect | 4187ns | +4187.4ns (+0.0%) | [+3828, +4326]ns | [3828, 4326] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_optall | 155739ns | +155739.1ns (+0.0%) | [+153646, +157921]ns | [153646, 157921] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_predecode | 3447ns | +3446.9ns (+0.0%) | [+3230, +3500]ns | [3230, 3500] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_stackcompile | 9555ns | +9555.0ns (+0.0%) | [+9195, +9753]ns | [9195, 9753] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_real_emitcopypatch | -0.326 | moderate- |
| carrier_setup_real_emitdirect | 0.003 | ok |
| carrier_setup_real_optall | 0.105 | ok |
| carrier_setup_real_parse | 0.000 | ok |
| carrier_setup_real_predecode | -0.198 | ok |
| carrier_setup_real_stackcompile | -0.140 | ok |

**Consistency summary:**

- **carrier_setup_real_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_real_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_real_optall**: won 0/6, lost 0/6
- **carrier_setup_real_predecode**: won 0/6, lost 0/6
- **carrier_setup_real_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 91830.8ns | 14881.9ns | 617.1% | HIGH |
| carrier_setup_real_emitdirect | 86819.1ns | 4114.0ns | 2110.3% | HIGH |
| carrier_setup_real_optall | 155768.9ns | 155768.7ns | 100.0% | HIGH |
| carrier_setup_real_parse | 1691.8ns | 0.0ns | 0.0% |  |
| carrier_setup_real_predecode | 86179.7ns | 3392.4ns | 2540.4% | HIGH |
| carrier_setup_real_stackcompile | 88773.8ns | 9501.0ns | 934.4% | HIGH |

## Distribution (algo ns)

```
carrier_setup_real_emitcopypatch (n=6, range 13247.9-15343.0 ns)
  13247.9 |####################
  13352.7 |
  13457.4 |
  13562.2 |
  13666.9 |
  13771.7 |
  13876.4 |
  13981.2 |
  14085.9 |
  14190.7 |
  14295.4 |
  14400.2 |
  14504.9 |
  14609.7 |
  14714.4 |
  14819.2 |
  14923.9 |
  15028.7 |####################
  15133.4 |########################################
  15238.2 |####################
  (0 below, 1 above range)

carrier_setup_real_emitdirect (n=6, range 3762.1-4326.4 ns)
   3762.1 |########################################
   3790.3 |
   3818.5 |
   3846.8 |
   3875.0 |########################################
   3903.2 |
   3931.4 |
   3959.6 |
   3987.8 |
   4016.1 |
   4044.3 |
   4072.5 |
   4100.7 |########################################
   4128.9 |
   4157.1 |
   4185.4 |
   4213.6 |
   4241.8 |########################################
   4270.0 |
   4298.2 |########################################
  (0 below, 1 above range)

carrier_setup_real_optall (n=6, range 153442.1-157920.6 ns)
  153442.1 |########################################
  153666.0 |########################################
  153890.0 |
  154113.9 |
  154337.8 |
  154561.7 |
  154785.6 |
  155009.6 |
  155233.5 |########################################
  155457.4 |
  155681.3 |
  155905.3 |
  156129.2 |########################################
  156353.1 |
  156577.0 |
  156801.0 |
  157024.9 |########################################
  157248.8 |
  157472.7 |
  157696.7 |
  (0 below, 1 above range)

carrier_setup_real_predecode (n=6, range 3070.4-3499.8 ns)
   3070.4 |########################################
   3091.9 |
   3113.3 |
   3134.8 |
   3156.3 |
   3177.8 |
   3199.2 |
   3220.7 |
   3242.2 |
   3263.6 |
   3285.1 |
   3306.6 |
   3328.0 |
   3349.5 |
   3371.0 |########################################
   3392.5 |
   3413.9 |########################################
   3435.4 |
   3456.9 |########################################
   3478.3 |########################################
  (0 below, 1 above range)

carrier_setup_real_stackcompile (n=6, range 8910.8-9753.3 ns)
   8910.8 |########################################
   8952.9 |
   8995.1 |
   9037.2 |
   9079.3 |
   9121.4 |
   9163.6 |
   9205.7 |
   9247.8 |
   9289.9 |
   9332.1 |
   9374.2 |
   9416.3 |
   9458.5 |########################################
   9500.6 |########################################
   9542.7 |
   9584.8 |########################################
   9627.0 |
   9669.1 |########################################
   9711.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_real_emitcopypatch**: bridge=606.9% of algo (FFI overhead may distort results)
- **carrier_setup_real_emitdirect**: bridge=2072.0% of algo (FFI overhead may distort results)
- **carrier_setup_real_optall**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_setup_real_predecode**: bridge=2500.7% of algo (FFI overhead may distort results)
- **carrier_setup_real_stackcompile**: bridge=930.1% of algo (FFI overhead may distort results)
