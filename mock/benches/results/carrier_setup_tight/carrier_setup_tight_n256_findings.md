# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), tight profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_tight_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_tight_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_setup_tight_parse)

The baseline carrier_setup_tight_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_tight_parse, carrier_setup_tight_predecode, carrier_setup_tight_emitdirect, carrier_setup_tight_stackcompile, carrier_setup_tight_emitcopypatch} vs {carrier_setup_tight_optall} (301% apart)

The field splits into a fast tier {carrier_setup_tight_parse, carrier_setup_tight_predecode, carrier_setup_tight_emitdirect, carrier_setup_tight_stackcompile, carrier_setup_tight_emitcopypatch} and a slow tier {carrier_setup_tight_optall} with a 301% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 226.72 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_tight_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 58554ns | 58834ns | 56427ns | 58370ns | 59894ns | +2447.14% |
| carrier_setup_tight_emitdirect | 18142ns | 18152ns | 17405ns | 17915ns | 18851ns | +689.19% |
| carrier_setup_tight_optall | 228853ns | 228939ns | 225269ns | 228148ns | 231701ns | +9855.23% |
| carrier_setup_tight_parse | 2299ns | 2307ns | 2112ns | 2262ns | 2447ns | base |
| carrier_setup_tight_predecode | 12303ns | 12226ns | 11845ns | 12170ns | 12731ns | +435.19% |
| carrier_setup_tight_stackcompile | 25579ns | 25632ns | 24155ns | 25272ns | 26752ns | +1012.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 56307ns | 54278ns | 57616ns | +0.00% | 0.005 |
| carrier_setup_tight_emitdirect | 15910ns | 15260ns | 16539ns | +0.00% | 0.016 |
| carrier_setup_tight_optall | 226627ns | 223090ns | 229436ns | +0.00% | 0.001 |
| carrier_setup_tight_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_tight_predecode | 10090ns | 9711ns | 10456ns | +0.00% | 0.025 |
| carrier_setup_tight_stackcompile | 23285ns | 22040ns | 24248ns | +0.00% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 516334 | 2169175 | 0.238 | 35.80× |
| carrier_setup_tight_emitdirect | 341216 | 2103589 | 0.162 | 23.66× |
| carrier_setup_tight_optall | 1402262 | 6877633 | 0.204 | 97.22× |
| carrier_setup_tight_parse | 14424 | 52927 | 0.273 | 1.00× |
| carrier_setup_tight_predecode | 309725 | 2215604 | 0.140 | 21.47× |
| carrier_setup_tight_stackcompile | 357766 | 2232882 | 0.160 | 24.80× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_setup_tight_predecode; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_tight_emitcopypatch | 0.005 | 17.2% |
| carrier_setup_tight_emitdirect | 0.016 | 61.0% |
| carrier_setup_tight_optall | 0.001 | 4.3% |
| carrier_setup_tight_predecode | 0.026 | 97.0% |
| carrier_setup_tight_stackcompile | 0.011 | 41.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 58554ns | 58554ns | +2447.14% |
| carrier_setup_tight_emitdirect | 18142ns | 18142ns | +689.19% |
| carrier_setup_tight_optall | 228853ns | 228853ns | +9855.23% |
| carrier_setup_tight_parse | 2299ns | 2299ns | base |
| carrier_setup_tight_predecode | 12303ns | 12303ns | +435.19% |
| carrier_setup_tight_stackcompile | 25579ns | 25579ns | +1012.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_tight_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_tight_emitcopypatch | 56555ns | +56555.2ns (+0.0%) | [+54749, +57616]ns | [54749, 57616] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_emitdirect | 15931ns | +15931.5ns (+0.0%) | [+15261, +16539]ns | [15261, 16539] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_optall | 226723ns | +226723.4ns (+0.0%) | [+223720, +229436]ns | [223720, 229436] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_predecode | 10012ns | +10011.6ns (+0.0%) | [+9803, +10456]ns | [9803, 10456] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_stackcompile | 23392ns | +23392.5ns (+0.0%) | [+22216, +24248]ns | [22216, 24248] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_tight_parse | carrier_setup_tight_emitcopypatch | carrier_setup_tight_emitdirect | carrier_setup_tight_optall | carrier_setup_tight_predecode | carrier_setup_tight_stackcompile |
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
| carrier_setup_tight_emitcopypatch | -0.407 | moderate- |
| carrier_setup_tight_emitdirect | -0.181 | ok |
| carrier_setup_tight_optall | -0.330 | moderate- |
| carrier_setup_tight_parse | 0.000 | ok |
| carrier_setup_tight_predecode | -0.284 | moderate- |
| carrier_setup_tight_stackcompile | 0.125 | ok |

**Consistency summary:**

- **carrier_setup_tight_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_tight_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_tight_optall**: won 0/6, lost 0/6
- **carrier_setup_tight_predecode**: won 0/6, lost 0/6
- **carrier_setup_tight_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 112920.2ns | 56306.9ns | 200.5% | HIGH |
| carrier_setup_tight_emitdirect | 93798.3ns | 15910.2ns | 589.5% | HIGH |
| carrier_setup_tight_optall | 225976.5ns | 226626.5ns | 99.7% | HIGH |
| carrier_setup_tight_parse | 1541.7ns | 0.0ns | 0.0% |  |
| carrier_setup_tight_predecode | 89526.2ns | 10090.4ns | 887.2% | HIGH |
| carrier_setup_tight_stackcompile | 93097.8ns | 23285.4ns | 399.8% | HIGH |

## Distribution (algo ns)

```
carrier_setup_tight_emitcopypatch (n=6, range 54278.3-57616.4 ns)
  54278.3 |########################################
  54445.2 |
  54612.1 |
  54779.0 |
  54945.9 |
  55112.8 |########################################
  55279.7 |
  55446.7 |
  55613.6 |
  55780.5 |
  55947.4 |
  56114.3 |
  56281.2 |########################################
  56448.1 |
  56615.0 |
  56781.9 |########################################
  56948.8 |
  57115.7 |########################################
  57282.6 |
  57449.5 |
  (0 below, 1 above range)

carrier_setup_tight_emitdirect (n=6, range 15259.6-16538.5 ns)
  15259.6 |########################################
  15323.5 |
  15387.5 |
  15451.4 |
  15515.4 |####################
  15579.3 |
  15643.3 |
  15707.2 |
  15771.2 |
  15835.1 |
  15899.1 |
  15963.0 |
  16027.0 |
  16090.9 |
  16154.9 |
  16218.8 |
  16282.8 |####################
  16346.7 |####################
  16410.7 |
  16474.6 |
  (0 below, 1 above range)

carrier_setup_tight_optall (n=6, range 223090.0-229435.9 ns)
  223090.0 |####################
  223407.3 |
  223724.6 |
  224041.9 |####################
  224359.2 |
  224676.5 |
  224993.8 |
  225311.0 |
  225628.3 |
  225945.6 |
  226262.9 |
  226580.2 |########################################
  226897.5 |
  227214.8 |####################
  227532.1 |
  227849.4 |
  228166.7 |
  228484.0 |
  228801.3 |
  229118.6 |
  (0 below, 1 above range)

carrier_setup_tight_predecode (n=6, range 9710.8-10456.5 ns)
   9710.8 |########################################
   9748.1 |
   9785.4 |
   9822.6 |
   9859.9 |########################################
   9897.2 |########################################
   9934.5 |
   9971.8 |
  10009.1 |
  10046.3 |
  10083.6 |########################################
  10120.9 |########################################
  10158.2 |
  10195.5 |
  10232.8 |
  10270.0 |
  10307.3 |
  10344.6 |
  10381.9 |
  10419.2 |
  (0 below, 1 above range)

carrier_setup_tight_stackcompile (n=6, range 22039.6-24247.9 ns)
  22039.6 |########################################
  22150.0 |
  22260.4 |
  22370.8 |########################################
  22481.3 |
  22591.7 |
  22702.1 |
  22812.5 |########################################
  22922.9 |
  23033.3 |
  23143.8 |
  23254.2 |
  23364.6 |
  23475.0 |
  23585.4 |
  23695.8 |
  23806.2 |########################################
  23916.7 |
  24027.1 |
  24137.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_tight_emitcopypatch**: bridge=200.4% of algo (FFI overhead may distort results)
- **carrier_setup_tight_emitdirect**: bridge=581.0% of algo (FFI overhead may distort results)
- **carrier_setup_tight_optall**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_setup_tight_predecode**: bridge=892.6% of algo (FFI overhead may distort results)
- **carrier_setup_tight_stackcompile**: bridge=400.1% of algo (FFI overhead may distort results)
