# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), madd profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_madd_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_madd_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_madd_emitcopypatch shows alternating (throttle bounce) (autocorr -0.55)

carrier_setup_madd_emitcopypatch's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_madd_parse)

The baseline carrier_setup_madd_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_madd_parse, carrier_setup_madd_predecode, carrier_setup_madd_emitdirect, carrier_setup_madd_stackcompile, carrier_setup_madd_emitcopypatch} vs {carrier_setup_madd_optall} (544% apart)

The field splits into a fast tier {carrier_setup_madd_parse, carrier_setup_madd_predecode, carrier_setup_madd_emitdirect, carrier_setup_madd_stackcompile, carrier_setup_madd_emitcopypatch} and a slow tier {carrier_setup_madd_optall} with a 544% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 368.49 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_madd_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 59424ns | 59518ns | 56739ns | 59010ns | 61388ns | +2446.46% |
| carrier_setup_madd_emitdirect | 19005ns | 19141ns | 18005ns | 19033ns | 19463ns | +714.42% |
| carrier_setup_madd_optall | 371393ns | 370627ns | 363016ns | 369067ns | 379069ns | +15815.01% |
| carrier_setup_madd_parse | 2334ns | 2314ns | 2207ns | 2283ns | 2472ns | base |
| carrier_setup_madd_predecode | 12748ns | 12822ns | 11816ns | 12673ns | 13325ns | +446.27% |
| carrier_setup_madd_stackcompile | 25318ns | 25049ns | 23931ns | 24774ns | 26826ns | +984.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 57151ns | 54573ns | 59021ns | +0.00% | 0.004 |
| carrier_setup_madd_emitdirect | 16688ns | 15809ns | 17106ns | +0.00% | 0.015 |
| carrier_setup_madd_optall | 369221ns | 360884ns | 376902ns | +0.00% | 0.001 |
| carrier_setup_madd_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_madd_predecode | 10471ns | 9703ns | 10961ns | +0.00% | 0.024 |
| carrier_setup_madd_stackcompile | 23027ns | 21793ns | 24411ns | +0.00% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 516589 | 2174362 | 0.238 | 35.88× |
| carrier_setup_madd_emitdirect | 319380 | 1973062 | 0.162 | 22.18× |
| carrier_setup_madd_optall | 2318798 | 10048026 | 0.231 | 161.06× |
| carrier_setup_madd_parse | 14398 | 52901 | 0.272 | 1.00× |
| carrier_setup_madd_predecode | 304035 | 2157696 | 0.141 | 21.12× |
| carrier_setup_madd_stackcompile | 347304 | 2173060 | 0.160 | 24.12× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_setup_madd_predecode; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_madd_emitcopypatch | 0.004 | 17.0% |
| carrier_setup_madd_emitdirect | 0.015 | 57.8% |
| carrier_setup_madd_optall | 0.001 | 2.6% |
| carrier_setup_madd_predecode | 0.024 | 92.1% |
| carrier_setup_madd_stackcompile | 0.011 | 42.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 59424ns | 59424ns | +2446.46% |
| carrier_setup_madd_emitdirect | 19005ns | 19005ns | +714.42% |
| carrier_setup_madd_optall | 371393ns | 371393ns | +15815.01% |
| carrier_setup_madd_parse | 2334ns | 2334ns | base |
| carrier_setup_madd_predecode | 12748ns | 12748ns | +446.27% |
| carrier_setup_madd_stackcompile | 25318ns | 25318ns | +984.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_madd_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_madd_emitcopypatch | 57240ns | +57239.8ns (+0.0%) | [+55193, +59021]ns | [55193, 59021] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_emitdirect | 16799ns | +16798.9ns (+0.0%) | [+16159, +17106]ns | [16159, 17106] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_optall | 368490ns | +368489.8ns (+0.0%) | [+362271, +376902]ns | [362271, 376902] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_predecode | 10536ns | +10535.8ns (+0.0%) | [+9915, +10961]ns | [9915, 10961] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_stackcompile | 22772ns | +22771.7ns (+0.0%) | [+21899, +24411]ns | [21899, 24411] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_madd_emitcopypatch | -0.545 | HIGH- (thermal bounce) |
| carrier_setup_madd_emitdirect | -0.199 | ok |
| carrier_setup_madd_optall | 0.397 | moderate+ |
| carrier_setup_madd_parse | 0.000 | ok |
| carrier_setup_madd_predecode | 0.035 | ok |
| carrier_setup_madd_stackcompile | 0.159 | ok |

**Consistency summary:**

- **carrier_setup_madd_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_madd_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_madd_optall**: won 0/6, lost 0/6
- **carrier_setup_madd_predecode**: won 0/6, lost 0/6
- **carrier_setup_madd_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 114403.0ns | 57151.4ns | 200.2% | HIGH |
| carrier_setup_madd_emitdirect | 90684.9ns | 16687.9ns | 543.4% | HIGH |
| carrier_setup_madd_optall | 368707.4ns | 369221.0ns | 99.9% | HIGH |
| carrier_setup_madd_parse | 1647.6ns | 0.0ns | 0.0% |  |
| carrier_setup_madd_predecode | 89660.8ns | 10470.7ns | 856.3% | HIGH |
| carrier_setup_madd_stackcompile | 92671.7ns | 23027.4ns | 402.4% | HIGH |

## Distribution (algo ns)

```
carrier_setup_madd_emitcopypatch (n=6, range 54573.3-59021.1 ns)
  54573.3 |########################################
  54795.7 |
  55018.1 |
  55240.5 |
  55462.9 |
  55685.2 |########################################
  55907.6 |########################################
  56130.0 |
  56352.4 |
  56574.8 |
  56797.2 |
  57019.6 |
  57242.0 |
  57464.3 |
  57686.7 |
  57909.1 |
  58131.5 |
  58353.9 |########################################
  58576.3 |########################################
  58798.7 |
  (0 below, 1 above range)

carrier_setup_madd_emitdirect (n=6, range 15809.2-17105.6 ns)
  15809.2 |####################
  15874.0 |
  15938.8 |
  16003.7 |
  16068.5 |
  16133.3 |
  16198.1 |
  16262.9 |
  16327.8 |
  16392.6 |
  16457.4 |####################
  16522.2 |
  16587.0 |####################
  16651.9 |
  16716.7 |
  16781.5 |
  16846.3 |
  16911.1 |
  16976.0 |########################################
  17040.8 |
  (0 below, 1 above range)

carrier_setup_madd_optall (n=6, range 360884.2-376902.2 ns)
  360884.2 |########################################
  361685.1 |
  362486.0 |
  363286.9 |########################################
  364087.8 |
  364888.7 |
  365689.6 |
  366490.5 |########################################
  367291.4 |
  368092.3 |
  368893.2 |
  369694.1 |########################################
  370495.0 |
  371295.9 |
  372096.8 |
  372897.7 |
  373698.6 |
  374499.5 |
  375300.4 |
  376101.3 |########################################
  (0 below, 1 above range)

carrier_setup_madd_predecode (n=6, range 9703.3-10961.5 ns)
   9703.3 |####################
   9766.2 |
   9829.1 |
   9892.0 |
   9954.9 |
  10017.8 |
  10080.7 |####################
  10143.7 |####################
  10206.6 |
  10269.5 |
  10332.4 |
  10395.3 |
  10458.2 |
  10521.1 |
  10584.0 |
  10646.9 |
  10709.8 |
  10772.7 |
  10835.6 |
  10898.5 |########################################
  (0 below, 1 above range)

carrier_setup_madd_stackcompile (n=6, range 21793.3-24411.2 ns)
  21793.3 |########################################
  21924.2 |########################################
  22055.1 |########################################
  22186.0 |
  22316.9 |
  22447.8 |
  22578.7 |
  22709.6 |
  22840.5 |
  22971.4 |
  23102.3 |
  23233.2 |
  23364.1 |########################################
  23495.0 |
  23625.9 |
  23756.8 |
  23887.7 |########################################
  24018.6 |
  24149.5 |
  24280.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_madd_emitcopypatch**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_setup_madd_emitdirect**: bridge=538.2% of algo (FFI overhead may distort results)
- **carrier_setup_madd_optall**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_setup_madd_predecode**: bridge=848.0% of algo (FFI overhead may distort results)
- **carrier_setup_madd_stackcompile**: bridge=407.0% of algo (FFI overhead may distort results)
