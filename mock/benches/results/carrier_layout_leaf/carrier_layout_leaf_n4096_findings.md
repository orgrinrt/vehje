# Record layout (REC12..REC32) with fixed switch dispatch, leaf profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_leaf_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_leaf_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 1.2% of the fastest

All 5 variants sit between 282.07 us and 285.41 us - a 1.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_leaf_rec20** at 282075.0 ns median (-0.5% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 282075.0 ns, slowest 285407.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 285993ns | 285422ns | 284059ns | 285238ns | 288093ns | +0.02% |
| carrier_lay_leaf_rec16 | 285058ns | 284495ns | 282677ns | 284378ns | 287269ns | -0.31% |
| carrier_lay_leaf_rec20 | 284693ns | 284287ns | 280516ns | 283990ns | 287835ns | -0.44% |
| carrier_lay_leaf_rec24 | 285942ns | 285584ns | 284680ns | 285422ns | 287353ns | base |
| carrier_lay_leaf_rec32 | 288464ns | 287626ns | 286413ns | 287547ns | 290866ns | +0.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 283765ns | 281852ns | 285836ns | +0.02% | 0.014 |
| carrier_lay_leaf_rec16 | 282849ns | 280505ns | 285044ns | -0.30% | 0.014 |
| carrier_lay_leaf_rec20 | 282443ns | 278241ns | 285568ns | -0.44% | 0.015 |
| carrier_lay_leaf_rec24 | 283701ns | 282399ns | 285112ns | base | 0.014 |
| carrier_lay_leaf_rec32 | 286256ns | 284229ns | 288642ns | +0.90% | 0.014 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 1762678 | 4495802 | 0.392 | 0.99× |
| carrier_lay_leaf_rec16 | 1765700 | 4495807 | 0.393 | 1.00× |
| carrier_lay_leaf_rec20 | 1762946 | 4495910 | 0.392 | 0.99× |
| carrier_lay_leaf_rec24 | 1772066 | 4496052 | 0.394 | 1.00× |
| carrier_lay_leaf_rec32 | 1790803 | 4495692 | 0.398 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.015 Gops/s** (carrier_lay_leaf_rec20; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.014 | 98.3% |
| carrier_lay_leaf_rec16 | 0.015 | 98.6% |
| carrier_lay_leaf_rec20 | 0.015 | 98.6% |
| carrier_lay_leaf_rec24 | 0.014 | 98.2% |
| carrier_lay_leaf_rec32 | 0.014 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_leaf_rec12 | 285993ns | 285993ns | +0.02% |
| carrier_lay_leaf_rec16 | 285058ns | 285058ns | -0.31% |
| carrier_lay_leaf_rec20 | 284693ns | 284693ns | -0.44% |
| carrier_lay_leaf_rec24 | 285942ns | 285942ns | base |
| carrier_lay_leaf_rec32 | 288464ns | 288464ns | +0.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec24 | 283357ns | base | --- | [282632, 285112] | --- | --- | --- | --- |
| carrier_lay_leaf_rec12 | 283196ns | no significant difference | [-1459, +1441]ns | [282263, 285836] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_leaf_rec16 | 282277ns | no significant difference | [-3392, +1929]ns | [281227, 285044] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_leaf_rec20 | 282075ns | no significant difference | [-5129, +2453]ns | [279687, 285568] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_leaf_rec32 | 285407ns | +1940.7ns (+0.7%) | [+1022, +4702]ns | [284717, 288642] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_leaf_rec24 | carrier_lay_leaf_rec12 | carrier_lay_leaf_rec16 | carrier_lay_leaf_rec20 | carrier_lay_leaf_rec32 |
|---|---|---|---|---|---|
| 1 | 286392ns | +0.2% | -1.5% | -1.7% | +0.5% |
| 2 | 282399ns | +0.8% | +0.5% | +1.1% | +2.5% |
| 3 | 283832ns | -0.7% | +0.8% | +0.7% | +0.6% |
| 4 | 283613ns | -0.3% | -0.6% | -1.9% | +0.2% |
| 5 | 283101ns | +0.1% | -0.9% | -0.2% | +0.8% |
| 6 | 282866ns | +0.0% | -0.2% | -0.6% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.232 | moderate+ |
| carrier_lay_leaf_rec16 | 0.160 | ok |
| carrier_lay_leaf_rec20 | -0.202 | moderate- |
| carrier_lay_leaf_rec24 | -0.313 | moderate- |
| carrier_lay_leaf_rec32 | 0.351 | moderate+ |

**Consistency summary:**

- **carrier_lay_leaf_rec12**: won 2/6, lost 3/6
- **carrier_lay_leaf_rec16**: won 4/6, lost 2/6
- **carrier_lay_leaf_rec20**: won 4/6, lost 2/6
- **carrier_lay_leaf_rec32**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 284089.0ns | 283765.1ns | 100.1% | HIGH |
| carrier_lay_leaf_rec16 | 283223.5ns | 282849.2ns | 100.1% | HIGH |
| carrier_lay_leaf_rec20 | 282662.8ns | 282443.3ns | 100.1% | HIGH |
| carrier_lay_leaf_rec24 | 284137.6ns | 283700.6ns | 100.2% | HIGH |
| carrier_lay_leaf_rec32 | 286722.7ns | 286255.6ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_lay_leaf_rec12 (n=6, range 281851.7-285836.0 ns)
  281851.7 |########################################
  282050.9 |
  282250.1 |
  282449.3 |
  282648.6 |########################################
  282847.8 |########################################
  283047.0 |
  283246.2 |########################################
  283445.4 |
  283644.6 |
  283843.8 |
  284043.1 |
  284242.3 |
  284441.5 |########################################
  284640.7 |
  284839.9 |
  285039.1 |
  285238.4 |
  285437.6 |
  285636.8 |
  (0 below, 1 above range)

carrier_lay_leaf_rec16 (n=6, range 280505.4-285044.0 ns)
  280505.4 |########################################
  280732.3 |
  280959.3 |
  281186.2 |
  281413.1 |
  281640.0 |
  281867.0 |########################################
  282093.9 |########################################
  282320.8 |########################################
  282547.7 |
  282774.7 |
  283001.6 |
  283228.5 |
  283455.5 |
  283682.4 |########################################
  283909.3 |
  284136.2 |
  284363.2 |
  284590.1 |
  284817.0 |
  (0 below, 1 above range)

carrier_lay_leaf_rec20 (n=6, range 278241.2-285568.3 ns)
  278241.2 |########################################
  278607.6 |
  278973.9 |
  279340.3 |
  279706.6 |
  280073.0 |
  280439.3 |
  280805.7 |########################################
  281172.1 |########################################
  281538.4 |
  281904.8 |
  282271.1 |
  282637.5 |########################################
  283003.8 |
  283370.2 |
  283736.6 |
  284102.9 |
  284469.3 |
  284835.6 |
  285202.0 |########################################
  (0 below, 1 above range)

carrier_lay_leaf_rec24 (n=6, range 282398.8-285112.1 ns)
  282398.8 |########################################
  282534.5 |
  282670.1 |
  282805.8 |########################################
  282941.5 |
  283077.1 |########################################
  283212.8 |
  283348.5 |
  283484.1 |########################################
  283619.8 |
  283755.4 |########################################
  283891.1 |
  284026.8 |
  284162.4 |
  284298.1 |
  284433.8 |
  284569.4 |
  284705.1 |
  284840.8 |
  284976.4 |
  (0 below, 1 above range)

carrier_lay_leaf_rec32 (n=6, range 284228.8-288642.5 ns)
  284228.8 |####################
  284449.5 |
  284670.2 |
  284890.9 |
  285111.5 |####################
  285332.2 |########################################
  285552.9 |
  285773.6 |
  285994.3 |
  286215.0 |
  286435.7 |
  286656.3 |
  286877.0 |
  287097.7 |
  287318.4 |
  287539.1 |
  287759.8 |####################
  287980.4 |
  288201.1 |
  288421.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_leaf_rec12**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec16**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec20**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec24**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec32**: bridge=100.2% of algo (FFI overhead may distort results)
