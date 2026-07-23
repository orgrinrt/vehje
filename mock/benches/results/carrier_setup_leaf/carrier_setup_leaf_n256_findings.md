# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), leaf profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_leaf_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_leaf_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_setup_leaf_parse)

The baseline carrier_setup_leaf_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_leaf_parse, carrier_setup_leaf_emitdirect, carrier_setup_leaf_predecode, carrier_setup_leaf_stackcompile, carrier_setup_leaf_emitcopypatch} vs {carrier_setup_leaf_optall} (591% apart)

The field splits into a fast tier {carrier_setup_leaf_parse, carrier_setup_leaf_emitdirect, carrier_setup_leaf_predecode, carrier_setup_leaf_stackcompile, carrier_setup_leaf_emitcopypatch} and a slow tier {carrier_setup_leaf_optall} with a 591% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 254.58 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_leaf_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 38607ns | 39138ns | 36547ns | 38581ns | 39675ns | +1598.50% |
| carrier_setup_leaf_emitdirect | 12296ns | 12361ns | 11600ns | 12217ns | 12763ns | +440.97% |
| carrier_setup_leaf_optall | 254197ns | 256758ns | 240282ns | 256157ns | 258215ns | +11083.43% |
| carrier_setup_leaf_parse | 2273ns | 2218ns | 2216ns | 2218ns | 2384ns | base |
| carrier_setup_leaf_predecode | 12677ns | 12525ns | 11879ns | 12342ns | 13577ns | +457.71% |
| carrier_setup_leaf_stackcompile | 21298ns | 21211ns | 20495ns | 21023ns | 22113ns | +837.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 36344ns | 34408ns | 37354ns | +0.00% | 0.007 |
| carrier_setup_leaf_emitdirect | 10045ns | 9469ns | 10425ns | +0.00% | 0.025 |
| carrier_setup_leaf_optall | 251980ns | 238041ns | 256007ns | +0.00% | 0.001 |
| carrier_setup_leaf_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_leaf_predecode | 10448ns | 9677ns | 11267ns | +0.00% | 0.025 |
| carrier_setup_leaf_stackcompile | 19064ns | 18345ns | 19759ns | +0.00% | 0.013 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 439205 | 1898467 | 0.231 | 30.68× |
| carrier_setup_leaf_emitdirect | 301961 | 1898354 | 0.159 | 21.09× |
| carrier_setup_leaf_optall | 1574942 | 6440632 | 0.245 | 110.01× |
| carrier_setup_leaf_parse | 14316 | 52924 | 0.271 | 1.00× |
| carrier_setup_leaf_predecode | 307368 | 1726537 | 0.178 | 21.47× |
| carrier_setup_leaf_stackcompile | 354375 | 2112133 | 0.168 | 24.75× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.027 Gops/s** (carrier_setup_leaf_emitdirect; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_leaf_emitcopypatch | 0.007 | 25.7% |
| carrier_setup_leaf_emitdirect | 0.025 | 93.8% |
| carrier_setup_leaf_optall | 0.001 | 3.7% |
| carrier_setup_leaf_predecode | 0.025 | 91.7% |
| carrier_setup_leaf_stackcompile | 0.013 | 49.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 38607ns | 38607ns | +1598.50% |
| carrier_setup_leaf_emitdirect | 12296ns | 12296ns | +440.97% |
| carrier_setup_leaf_optall | 254197ns | 254197ns | +11083.43% |
| carrier_setup_leaf_parse | 2273ns | 2273ns | base |
| carrier_setup_leaf_predecode | 12677ns | 12677ns | +457.71% |
| carrier_setup_leaf_stackcompile | 21298ns | 21298ns | +837.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_leaf_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_leaf_emitcopypatch | 36849ns | +36849.2ns (+0.0%) | [+34829, +37354]ns | [34829, 37354] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_emitdirect | 10090ns | +10090.2ns (+0.0%) | [+9619, +10425]ns | [9619, 10425] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_optall | 254582ns | +254582.0ns (+0.0%) | [+245352, +256007]ns | [245352, 256007] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_predecode | 10324ns | +10323.5ns (+0.0%) | [+9753, +11267]ns | [9753, 11267] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_stackcompile | 19005ns | +19004.8ns (+0.0%) | [+18430, +19759]ns | [18430, 19759] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_leaf_parse | carrier_setup_leaf_emitcopypatch | carrier_setup_leaf_emitdirect | carrier_setup_leaf_optall | carrier_setup_leaf_predecode | carrier_setup_leaf_stackcompile |
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
| carrier_setup_leaf_emitcopypatch | 0.452 | moderate+ |
| carrier_setup_leaf_emitdirect | -0.288 | moderate- |
| carrier_setup_leaf_optall | -0.216 | moderate- |
| carrier_setup_leaf_parse | 0.000 | ok |
| carrier_setup_leaf_predecode | -0.095 | ok |
| carrier_setup_leaf_stackcompile | 0.318 | moderate+ |

**Consistency summary:**

- **carrier_setup_leaf_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_leaf_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_leaf_optall**: won 0/6, lost 0/6
- **carrier_setup_leaf_predecode**: won 0/6, lost 0/6
- **carrier_setup_leaf_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 109373.0ns | 36344.1ns | 300.9% | HIGH |
| carrier_setup_leaf_emitdirect | 88514.5ns | 10044.6ns | 881.2% | HIGH |
| carrier_setup_leaf_optall | 251164.1ns | 251980.4ns | 99.7% | HIGH |
| carrier_setup_leaf_parse | 1572.1ns | 0.0ns | 0.0% |  |
| carrier_setup_leaf_predecode | 88914.1ns | 10448.0ns | 851.0% | HIGH |
| carrier_setup_leaf_stackcompile | 95161.5ns | 19064.4ns | 499.2% | HIGH |

## Distribution (algo ns)

```
carrier_setup_leaf_emitcopypatch (n=6, range 34408.3-37353.9 ns)
  34408.3 |####################
  34555.6 |
  34702.9 |
  34850.1 |
  34997.4 |
  35144.7 |####################
  35292.0 |
  35439.3 |
  35586.6 |
  35733.8 |
  35881.1 |
  36028.4 |
  36175.7 |
  36323.0 |
  36470.3 |
  36617.5 |####################
  36764.8 |
  36912.1 |########################################
  37059.4 |
  37206.7 |
  (0 below, 1 above range)

carrier_setup_leaf_emitdirect (n=6, range 9468.8-10424.8 ns)
   9468.8 |########################################
   9516.6 |
   9564.4 |
   9612.2 |
   9660.0 |
   9707.8 |
   9755.6 |########################################
   9803.4 |
   9851.2 |
   9899.0 |
   9946.8 |
   9994.6 |########################################
  10042.4 |
  10090.2 |
  10138.0 |########################################
  10185.8 |
  10233.6 |########################################
  10281.4 |
  10329.2 |
  10377.0 |
  (0 below, 1 above range)

carrier_setup_leaf_optall (n=6, range 238040.8-256007.2 ns)
  238040.8 |####################
  238939.1 |
  239837.4 |
  240735.8 |
  241634.1 |
  242532.4 |
  243430.7 |
  244329.1 |
  245227.4 |
  246125.7 |
  247024.0 |
  247922.3 |
  248820.7 |
  249719.0 |
  250617.3 |
  251515.6 |
  252414.0 |####################
  253312.3 |
  254210.6 |########################################
  255108.9 |####################
  (0 below, 1 above range)

carrier_setup_leaf_predecode (n=6, range 9677.1-11267.3 ns)
   9677.1 |########################################
   9756.6 |########################################
   9836.1 |
   9915.6 |
   9995.1 |
  10074.6 |########################################
  10154.2 |
  10233.7 |
  10313.2 |
  10392.7 |
  10472.2 |########################################
  10551.7 |
  10631.2 |
  10710.7 |########################################
  10790.2 |
  10869.8 |
  10949.3 |
  11028.8 |
  11108.3 |
  11187.8 |
  (0 below, 1 above range)

carrier_setup_leaf_stackcompile (n=6, range 18344.6-19758.5 ns)
  18344.6 |########################################
  18415.3 |
  18486.0 |########################################
  18556.7 |
  18627.4 |
  18698.1 |########################################
  18768.8 |
  18839.5 |
  18910.2 |
  18980.9 |
  19051.6 |
  19122.3 |
  19193.0 |
  19263.7 |########################################
  19334.4 |
  19405.1 |
  19475.8 |
  19546.5 |
  19617.2 |
  19687.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_leaf_emitcopypatch**: bridge=301.1% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_emitdirect**: bridge=875.7% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_optall**: bridge=99.5% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_predecode**: bridge=858.9% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_stackcompile**: bridge=499.4% of algo (FFI overhead may distort results)
