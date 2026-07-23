# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), real profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_real_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_real_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_setup_real_parse)

The baseline carrier_setup_real_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_real_parse, carrier_setup_real_predecode, carrier_setup_real_emitdirect, carrier_setup_real_stackcompile, carrier_setup_real_emitcopypatch} vs {carrier_setup_real_optall} (795% apart)

The field splits into a fast tier {carrier_setup_real_parse, carrier_setup_real_predecode, carrier_setup_real_emitdirect, carrier_setup_real_stackcompile, carrier_setup_real_emitcopypatch} and a slow tier {carrier_setup_real_optall} with a 795% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 599.23 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_real_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 68906ns | 69192ns | 66298ns | 68719ns | 70491ns | +2719.37% |
| carrier_setup_real_emitdirect | 19097ns | 19236ns | 18060ns | 19103ns | 19606ns | +681.37% |
| carrier_setup_real_optall | 601289ns | 601501ns | 597668ns | 600721ns | 603951ns | +24502.31% |
| carrier_setup_real_parse | 2444ns | 2445ns | 2267ns | 2428ns | 2558ns | base |
| carrier_setup_real_predecode | 12273ns | 12466ns | 11132ns | 12348ns | 12731ns | +402.15% |
| carrier_setup_real_stackcompile | 26479ns | 26620ns | 25628ns | 26328ns | 27131ns | +983.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 66653ns | 64150ns | 68194ns | +0.00% | 0.004 |
| carrier_setup_real_emitdirect | 16718ns | 15842ns | 17161ns | +0.00% | 0.015 |
| carrier_setup_real_optall | 599022ns | 595491ns | 601622ns | +0.00% | 0.000 |
| carrier_setup_real_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_real_predecode | 9908ns | 9013ns | 10249ns | +0.00% | 0.026 |
| carrier_setup_real_stackcompile | 24127ns | 23339ns | 24724ns | +0.00% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 600672 | 2195884 | 0.274 | 42.11× |
| carrier_setup_real_emitdirect | 322481 | 1962262 | 0.164 | 22.61× |
| carrier_setup_real_optall | 3754119 | 18205964 | 0.206 | 263.18× |
| carrier_setup_real_parse | 14264 | 52914 | 0.270 | 1.00× |
| carrier_setup_real_predecode | 300562 | 2213153 | 0.136 | 21.07× |
| carrier_setup_real_stackcompile | 351630 | 2197092 | 0.160 | 24.65× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.028 Gops/s** (carrier_setup_real_predecode; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_real_emitcopypatch | 0.004 | 13.5% |
| carrier_setup_real_emitdirect | 0.015 | 53.6% |
| carrier_setup_real_optall | 0.000 | 1.5% |
| carrier_setup_real_predecode | 0.025 | 89.3% |
| carrier_setup_real_stackcompile | 0.011 | 37.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_real_emitcopypatch | 68906ns | 68906ns | +2719.37% |
| carrier_setup_real_emitdirect | 19097ns | 19097ns | +681.37% |
| carrier_setup_real_optall | 601289ns | 601289ns | +24502.31% |
| carrier_setup_real_parse | 2444ns | 2444ns | base |
| carrier_setup_real_predecode | 12273ns | 12273ns | +402.15% |
| carrier_setup_real_stackcompile | 26479ns | 26479ns | +983.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_real_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_real_emitcopypatch | 66935ns | +66935.0ns (+0.0%) | [+64829, +68194]ns | [64829, 68194] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_emitdirect | 16817ns | +16817.3ns (+0.0%) | [+16176, +17161]ns | [16176, 17161] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_optall | 599227ns | +599227.2ns (+0.0%) | [+596216, +601622]ns | [596216, 601622] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_predecode | 10096ns | +10096.2ns (+0.0%) | [+9378, +10249]ns | [9378, 10249] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_stackcompile | 24260ns | +24259.6ns (+0.0%) | [+23396, +24724]ns | [23396, 24724] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_real_emitcopypatch | -0.178 | ok |
| carrier_setup_real_emitdirect | -0.402 | moderate- |
| carrier_setup_real_optall | -0.358 | moderate- |
| carrier_setup_real_parse | 0.000 | ok |
| carrier_setup_real_predecode | -0.085 | ok |
| carrier_setup_real_stackcompile | 0.176 | ok |

**Consistency summary:**

- **carrier_setup_real_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_real_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_real_optall**: won 0/6, lost 0/6
- **carrier_setup_real_predecode**: won 0/6, lost 0/6
- **carrier_setup_real_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 133392.6ns | 66652.6ns | 200.1% | HIGH |
| carrier_setup_real_emitdirect | 93287.5ns | 16718.2ns | 558.0% | HIGH |
| carrier_setup_real_optall | 598939.1ns | 599021.9ns | 100.0% | HIGH |
| carrier_setup_real_parse | 1649.1ns | 0.0ns | 0.0% |  |
| carrier_setup_real_predecode | 91497.1ns | 9907.8ns | 923.5% | HIGH |
| carrier_setup_real_stackcompile | 96461.9ns | 24126.6ns | 399.8% | HIGH |

## Distribution (algo ns)

```
carrier_setup_real_emitcopypatch (n=6, range 64150.0-68193.9 ns)
  64150.0 |########################################
  64352.2 |
  64554.4 |
  64756.6 |
  64958.8 |
  65161.0 |
  65363.2 |########################################
  65565.4 |
  65767.6 |
  65969.8 |
  66172.0 |
  66374.2 |########################################
  66576.4 |
  66778.6 |
  66980.8 |
  67183.0 |
  67385.2 |########################################
  67587.4 |
  67789.6 |
  67991.8 |########################################
  (0 below, 1 above range)

carrier_setup_real_emitdirect (n=6, range 15842.1-17160.8 ns)
  15842.1 |####################
  15908.0 |
  15974.0 |
  16039.9 |
  16105.9 |
  16171.8 |
  16237.7 |
  16303.7 |
  16369.6 |
  16435.5 |
  16501.5 |####################
  16567.4 |
  16633.3 |
  16699.3 |
  16765.2 |########################################
  16831.2 |
  16897.1 |####################
  16963.0 |
  17029.0 |
  17094.9 |
  (0 below, 1 above range)

carrier_setup_real_optall (n=6, range 595491.2-601622.1 ns)
  595491.2 |########################################
  595797.7 |
  596104.3 |
  596410.8 |
  596717.4 |########################################
  597023.9 |
  597330.5 |
  597637.0 |
  597943.6 |########################################
  598250.1 |
  598556.6 |
  598863.2 |
  599169.7 |
  599476.3 |
  599782.8 |
  600089.4 |
  600395.9 |########################################
  600702.5 |########################################
  601009.0 |
  601315.6 |
  (0 below, 1 above range)

carrier_setup_real_predecode (n=6, range 9012.9-10249.4 ns)
   9012.9 |####################
   9074.7 |
   9136.5 |
   9198.4 |
   9260.2 |
   9322.0 |
   9383.8 |
   9445.7 |
   9507.5 |
   9569.3 |
   9631.1 |
   9692.9 |####################
   9754.8 |
   9816.6 |
   9878.4 |
   9940.2 |
  10002.1 |####################
  10063.9 |
  10125.7 |
  10187.5 |########################################
  (0 below, 1 above range)

carrier_setup_real_stackcompile (n=6, range 23338.8-24723.6 ns)
  23338.8 |########################################
  23408.0 |########################################
  23477.3 |
  23546.5 |
  23615.8 |
  23685.0 |
  23754.2 |
  23823.5 |
  23892.7 |
  23961.9 |
  24031.2 |
  24100.4 |########################################
  24169.7 |
  24238.9 |
  24308.1 |
  24377.4 |########################################
  24446.6 |
  24515.8 |########################################
  24585.1 |
  24654.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_real_emitcopypatch**: bridge=199.9% of algo (FFI overhead may distort results)
- **carrier_setup_real_emitdirect**: bridge=555.6% of algo (FFI overhead may distort results)
- **carrier_setup_real_optall**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_real_predecode**: bridge=912.0% of algo (FFI overhead may distort results)
- **carrier_setup_real_stackcompile**: bridge=400.0% of algo (FFI overhead may distort results)
