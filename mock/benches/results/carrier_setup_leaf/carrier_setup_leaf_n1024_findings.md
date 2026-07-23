# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), leaf profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_leaf_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_leaf_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_leaf_predecode shows alternating (throttle bounce) (autocorr -0.54)

carrier_setup_leaf_predecode's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_leaf_parse)

The baseline carrier_setup_leaf_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_leaf_parse, carrier_setup_leaf_predecode, carrier_setup_leaf_emitdirect, carrier_setup_leaf_stackcompile, carrier_setup_leaf_emitcopypatch} vs {carrier_setup_leaf_optall} (434% apart)

The field splits into a fast tier {carrier_setup_leaf_parse, carrier_setup_leaf_predecode, carrier_setup_leaf_emitdirect, carrier_setup_leaf_stackcompile, carrier_setup_leaf_emitcopypatch} and a slow tier {carrier_setup_leaf_optall} with a 434% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 740.24 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_leaf_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 140984ns | 140893ns | 138678ns | 140414ns | 142991ns | +6291.21% |
| carrier_setup_leaf_emitdirect | 47365ns | 47309ns | 44402ns | 47137ns | 49187ns | +2047.18% |
| carrier_setup_leaf_optall | 742904ns | 742468ns | 737483ns | 741552ns | 747643ns | +33578.05% |
| carrier_setup_leaf_parse | 2206ns | 2207ns | 2108ns | 2176ns | 2299ns | base |
| carrier_setup_leaf_predecode | 44781ns | 44606ns | 43173ns | 44189ns | 46474ns | +1930.07% |
| carrier_setup_leaf_stackcompile | 63036ns | 62061ns | 61670ns | 62008ns | 65259ns | +2757.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 138806ns | 136561ns | 140773ns | +0.00% | 0.007 |
| carrier_setup_leaf_emitdirect | 45053ns | 42228ns | 46785ns | +0.00% | 0.023 |
| carrier_setup_leaf_optall | 740625ns | 735194ns | 745301ns | +0.00% | 0.001 |
| carrier_setup_leaf_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_leaf_predecode | 42570ns | 41069ns | 44181ns | +0.00% | 0.024 |
| carrier_setup_leaf_stackcompile | 60847ns | 59490ns | 63014ns | +0.00% | 0.017 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 865924 | 3730015 | 0.232 | 60.52× |
| carrier_setup_leaf_emitdirect | 425176 | 2350338 | 0.181 | 29.72× |
| carrier_setup_leaf_optall | 4639104 | 18556151 | 0.250 | 324.24× |
| carrier_setup_leaf_parse | 14308 | 52920 | 0.270 | 1.00× |
| carrier_setup_leaf_predecode | 470346 | 2325626 | 0.202 | 32.87× |
| carrier_setup_leaf_stackcompile | 572188 | 3248528 | 0.176 | 39.99× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.025 Gops/s** (carrier_setup_leaf_predecode; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_leaf_emitcopypatch | 0.007 | 29.6% |
| carrier_setup_leaf_emitdirect | 0.023 | 91.2% |
| carrier_setup_leaf_optall | 0.001 | 5.5% |
| carrier_setup_leaf_predecode | 0.024 | 96.9% |
| carrier_setup_leaf_stackcompile | 0.017 | 68.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 140984ns | 140984ns | +6291.21% |
| carrier_setup_leaf_emitdirect | 47365ns | 47365ns | +2047.18% |
| carrier_setup_leaf_optall | 742904ns | 742904ns | +33578.05% |
| carrier_setup_leaf_parse | 2206ns | 2206ns | base |
| carrier_setup_leaf_predecode | 44781ns | 44781ns | +1930.07% |
| carrier_setup_leaf_stackcompile | 63036ns | 63036ns | +2757.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_leaf_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_leaf_emitcopypatch | 138719ns | +138719.3ns (+0.0%) | [+136924, +140773]ns | [136924, 140773] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_emitdirect | 45013ns | +45012.7ns (+0.0%) | [+43361, +46785]ns | [43361, 46785] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_optall | 740243ns | +740243.3ns (+0.0%) | [+736331, +745301]ns | [736331, 745301] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_predecode | 42394ns | +42393.8ns (+0.0%) | [+41135, +44181]ns | [41135, 44181] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_stackcompile | 59911ns | +59911.4ns (+0.0%) | [+59614, +63014]ns | [59614, 63014] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_leaf_emitcopypatch | -0.444 | moderate- |
| carrier_setup_leaf_emitdirect | -0.002 | ok |
| carrier_setup_leaf_optall | 0.100 | ok |
| carrier_setup_leaf_parse | 0.000 | ok |
| carrier_setup_leaf_predecode | -0.542 | HIGH- (thermal bounce) |
| carrier_setup_leaf_stackcompile | 0.006 | ok |

**Consistency summary:**

- **carrier_setup_leaf_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_leaf_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_leaf_optall**: won 0/6, lost 0/6
- **carrier_setup_leaf_predecode**: won 0/6, lost 0/6
- **carrier_setup_leaf_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 138801.2ns | 138805.6ns | 100.0% | HIGH |
| carrier_setup_leaf_emitdirect | 96469.5ns | 45052.8ns | 214.1% | HIGH |
| carrier_setup_leaf_optall | 739374.9ns | 740625.0ns | 99.8% | HIGH |
| carrier_setup_leaf_parse | 1498.7ns | 0.0ns | 0.0% |  |
| carrier_setup_leaf_predecode | 109204.4ns | 42569.7ns | 256.5% | HIGH |
| carrier_setup_leaf_stackcompile | 121635.5ns | 60846.5ns | 199.9% | HIGH |

## Distribution (algo ns)

```
carrier_setup_leaf_emitcopypatch (n=6, range 136561.2-140773.4 ns)
  136561.2 |########################################
  136771.8 |
  136982.4 |
  137193.0 |########################################
  137403.6 |########################################
  137614.2 |
  137824.8 |
  138035.5 |
  138246.1 |
  138456.7 |
  138667.3 |
  138877.9 |
  139088.5 |
  139299.1 |
  139509.7 |
  139720.3 |########################################
  139930.9 |
  140141.5 |
  140352.1 |########################################
  140562.7 |
  (0 below, 1 above range)

carrier_setup_leaf_emitdirect (n=6, range 42228.3-46784.8 ns)
  42228.3 |########################################
  42456.1 |
  42684.0 |
  42911.8 |
  43139.6 |
  43367.4 |
  43595.2 |
  43823.1 |
  44050.9 |
  44278.7 |########################################
  44506.6 |########################################
  44734.4 |
  44962.2 |
  45190.0 |
  45417.9 |########################################
  45645.7 |
  45873.5 |
  46101.3 |
  46329.2 |########################################
  46557.0 |
  (0 below, 1 above range)

carrier_setup_leaf_optall (n=6, range 735193.8-745301.0 ns)
  735193.8 |########################################
  735699.2 |
  736204.5 |
  736709.9 |
  737215.2 |########################################
  737720.6 |########################################
  738226.0 |
  738731.3 |
  739236.7 |
  739742.0 |
  740247.4 |
  740752.8 |
  741258.1 |
  741763.5 |
  742268.8 |########################################
  742774.2 |
  743279.6 |########################################
  743784.9 |
  744290.3 |
  744795.6 |
  (0 below, 1 above range)

carrier_setup_leaf_predecode (n=6, range 41068.8-44180.6 ns)
  41068.8 |########################################
  41224.4 |
  41380.0 |
  41535.6 |
  41691.2 |
  41846.8 |
  42002.3 |####################
  42157.9 |
  42313.5 |
  42469.1 |
  42624.7 |####################
  42780.3 |
  42935.9 |
  43091.5 |
  43247.1 |####################
  43402.7 |
  43558.2 |
  43713.8 |
  43869.4 |
  44025.0 |
  (0 below, 1 above range)

carrier_setup_leaf_stackcompile (n=6, range 59489.6-63014.4 ns)
  59489.6 |####################
  59665.8 |########################################
  59842.1 |
  60018.3 |####################
  60194.6 |
  60370.8 |
  60547.0 |
  60723.3 |
  60899.5 |
  61075.8 |####################
  61252.0 |
  61428.2 |
  61604.5 |
  61780.7 |
  61957.0 |
  62133.2 |
  62309.4 |
  62485.7 |
  62661.9 |
  62838.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_leaf_emitcopypatch**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_emitdirect**: bridge=207.0% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_optall**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_predecode**: bridge=258.9% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_stackcompile**: bridge=200.0% of algo (FFI overhead may distort results)
