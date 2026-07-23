# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), madd profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_madd_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_madd_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_setup_madd_parse)

The baseline carrier_setup_madd_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_madd_parse, carrier_setup_madd_predecode, carrier_setup_madd_emitdirect, carrier_setup_madd_stackcompile, carrier_setup_madd_emitcopypatch} vs {carrier_setup_madd_optall} (767% apart)

The field splits into a fast tier {carrier_setup_madd_parse, carrier_setup_madd_predecode, carrier_setup_madd_emitdirect, carrier_setup_madd_stackcompile, carrier_setup_madd_emitcopypatch} and a slow tier {carrier_setup_madd_optall} with a 767% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 126.62 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_madd_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 16859ns | 16990ns | 16108ns | 16797ns | 17328ns | +567.45% |
| carrier_setup_madd_emitdirect | 6884ns | 7016ns | 6477ns | 6854ns | 7132ns | +172.54% |
| carrier_setup_madd_optall | 129243ns | 128950ns | 125120ns | 128290ns | 132733ns | +5016.71% |
| carrier_setup_madd_parse | 2526ns | 2551ns | 2402ns | 2502ns | 2623ns | base |
| carrier_setup_madd_predecode | 5962ns | 6051ns | 5384ns | 6012ns | 6176ns | +136.03% |
| carrier_setup_madd_stackcompile | 10955ns | 11006ns | 10642ns | 10946ns | 11126ns | +333.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 14499ns | 13859ns | 14896ns | +0.00% | 0.004 |
| carrier_setup_madd_emitdirect | 4509ns | 4236ns | 4687ns | +0.00% | 0.014 |
| carrier_setup_madd_optall | 126962ns | 122874ns | 130471ns | +0.00% | 0.001 |
| carrier_setup_madd_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_madd_predecode | 3611ns | 3277ns | 3749ns | +0.00% | 0.018 |
| carrier_setup_madd_stackcompile | 8554ns | 8251ns | 8732ns | +0.00% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 307312 | 1311725 | 0.234 | 21.27× |
| carrier_setup_madd_emitdirect | 268866 | 1595206 | 0.169 | 18.61× |
| carrier_setup_madd_optall | 763129 | 3419570 | 0.223 | 52.81× |
| carrier_setup_madd_parse | 14449 | 52953 | 0.273 | 1.00× |
| carrier_setup_madd_predecode | 267602 | 1839036 | 0.146 | 18.52× |
| carrier_setup_madd_stackcompile | 281787 | 1732102 | 0.163 | 19.50× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.020 Gops/s** (carrier_setup_madd_predecode; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_madd_emitcopypatch | 0.004 | 22.4% |
| carrier_setup_madd_emitdirect | 0.014 | 71.5% |
| carrier_setup_madd_optall | 0.001 | 2.6% |
| carrier_setup_madd_predecode | 0.018 | 89.7% |
| carrier_setup_madd_stackcompile | 0.007 | 38.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 16859ns | 16859ns | +567.45% |
| carrier_setup_madd_emitdirect | 6884ns | 6884ns | +172.54% |
| carrier_setup_madd_optall | 129243ns | 129243ns | +5016.71% |
| carrier_setup_madd_parse | 2526ns | 2526ns | base |
| carrier_setup_madd_predecode | 5962ns | 5962ns | +136.03% |
| carrier_setup_madd_stackcompile | 10955ns | 10955ns | +333.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_madd_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_madd_emitcopypatch | 14613ns | +14613.1ns (+0.0%) | [+13989, +14896]ns | [13989, 14896] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_emitdirect | 4583ns | +4582.9ns (+0.0%) | [+4256, +4687]ns | [4256, 4687] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_optall | 126625ns | +126624.6ns (+0.0%) | [+123792, +130471]ns | [123792, 130471] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_predecode | 3652ns | +3651.9ns (+0.0%) | [+3432, +3749]ns | [3432, 3749] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_stackcompile | 8586ns | +8586.5ns (+0.0%) | [+8345, +8732]ns | [8345, 8732] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_madd_emitcopypatch | -0.443 | moderate- |
| carrier_setup_madd_emitdirect | -0.007 | ok |
| carrier_setup_madd_optall | -0.348 | moderate- |
| carrier_setup_madd_parse | 0.000 | ok |
| carrier_setup_madd_predecode | 0.101 | ok |
| carrier_setup_madd_stackcompile | 0.143 | ok |

**Consistency summary:**

- **carrier_setup_madd_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_madd_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_madd_optall**: won 0/6, lost 0/6
- **carrier_setup_madd_predecode**: won 0/6, lost 0/6
- **carrier_setup_madd_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 90923.2ns | 14499.5ns | 627.1% | HIGH |
| carrier_setup_madd_emitdirect | 87206.2ns | 4508.7ns | 1934.2% | HIGH |
| carrier_setup_madd_optall | 126930.4ns | 126962.5ns | 100.0% | HIGH |
| carrier_setup_madd_parse | 1713.4ns | 0.0ns | 0.0% |  |
| carrier_setup_madd_predecode | 86794.6ns | 3611.1ns | 2403.5% | HIGH |
| carrier_setup_madd_stackcompile | 88786.9ns | 8554.3ns | 1037.9% | HIGH |

## Distribution (algo ns)

```
carrier_setup_madd_emitcopypatch (n=6, range 13858.8-14896.5 ns)
  13858.8 |####################
  13910.7 |
  13962.6 |
  14014.5 |
  14066.3 |
  14118.2 |####################
  14170.1 |
  14222.0 |
  14273.9 |
  14325.8 |
  14377.6 |
  14429.5 |
  14481.4 |
  14533.3 |
  14585.2 |########################################
  14637.1 |
  14689.0 |
  14740.8 |
  14792.7 |
  14844.6 |####################
  (0 below, 1 above range)

carrier_setup_madd_emitdirect (n=6, range 4235.8-4687.2 ns)
   4235.8 |########################################
   4258.4 |########################################
   4280.9 |
   4303.5 |
   4326.1 |
   4348.7 |
   4371.2 |
   4393.8 |
   4416.4 |
   4439.0 |
   4461.5 |
   4484.1 |
   4506.7 |
   4529.2 |########################################
   4551.8 |
   4574.4 |
   4597.0 |
   4619.5 |########################################
   4642.1 |########################################
   4664.7 |
  (0 below, 1 above range)

carrier_setup_madd_optall (n=6, range 122873.7-130471.2 ns)
  122873.7 |########################################
  123253.6 |
  123633.5 |
  124013.3 |
  124393.2 |########################################
  124773.1 |
  125153.0 |########################################
  125532.8 |
  125912.7 |
  126292.6 |
  126672.5 |
  127052.4 |
  127432.2 |########################################
  127812.1 |
  128192.0 |
  128571.9 |
  128951.7 |
  129331.6 |
  129711.5 |
  130091.4 |########################################
  (0 below, 1 above range)

carrier_setup_madd_predecode (n=6, range 3276.7-3749.4 ns)
   3276.7 |########################################
   3300.3 |
   3324.0 |
   3347.6 |
   3371.2 |
   3394.9 |
   3418.5 |
   3442.1 |
   3465.8 |
   3489.4 |
   3513.0 |
   3536.7 |
   3560.3 |
   3584.0 |########################################
   3607.6 |########################################
   3631.2 |
   3654.9 |
   3678.5 |########################################
   3702.1 |
   3725.8 |########################################
  (0 below, 1 above range)

carrier_setup_madd_stackcompile (n=6, range 8250.8-8731.9 ns)
   8250.8 |########################################
   8274.9 |
   8298.9 |
   8323.0 |
   8347.0 |
   8371.1 |
   8395.1 |
   8419.2 |########################################
   8443.2 |
   8467.3 |
   8491.3 |
   8515.4 |
   8539.5 |
   8563.5 |########################################
   8587.6 |########################################
   8611.6 |
   8635.7 |
   8659.7 |########################################
   8683.8 |
   8707.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_madd_emitcopypatch**: bridge=613.6% of algo (FFI overhead may distort results)
- **carrier_setup_madd_emitdirect**: bridge=1907.1% of algo (FFI overhead may distort results)
- **carrier_setup_madd_optall**: bridge=99.7% of algo (FFI overhead may distort results)
- **carrier_setup_madd_predecode**: bridge=2375.9% of algo (FFI overhead may distort results)
- **carrier_setup_madd_stackcompile**: bridge=1030.6% of algo (FFI overhead may distort results)
