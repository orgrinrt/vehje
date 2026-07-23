# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, wideselect profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_wideselect_none**

## Highlights

Baseline for all deltas below: **carrier_opt_wideselect_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_wideselect_canon beats baseline by 33% (significant)

carrier_opt_wideselect_canon is -132.09 us (33%) faster than baseline carrier_opt_wideselect_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_wideselect_cse shows alternating (throttle bounce) (autocorr -0.63)

carrier_opt_wideselect_cse's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_wideselect_canon, carrier_opt_wideselect_cse, carrier_opt_wideselect_all} vs {carrier_opt_wideselect_dce, carrier_opt_wideselect_none, carrier_opt_wideselect_fold} (37% apart)

The field splits into a fast tier {carrier_opt_wideselect_canon, carrier_opt_wideselect_cse, carrier_opt_wideselect_all} and a slow tier {carrier_opt_wideselect_dce, carrier_opt_wideselect_none, carrier_opt_wideselect_fold} with a 37% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_opt_wideselect_canon** at 272268.5 ns median (-32.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.52x (fastest 272268.5 ns, slowest 414240.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 291822ns | 287925ns | 264228ns | 281873ns | 320544ns | -27.56% |
| carrier_opt_wideselect_canon | 282416ns | 274752ns | 261374ns | 272926ns | 307172ns | -29.90% |
| carrier_opt_wideselect_cse | 293238ns | 286969ns | 270366ns | 284160ns | 318292ns | -27.21% |
| carrier_opt_wideselect_dce | 398933ns | 393590ns | 372105ns | 388321ns | 428266ns | -0.98% |
| carrier_opt_wideselect_fold | 416804ns | 417231ns | 393537ns | 412644ns | 434676ns | +3.46% |
| carrier_opt_wideselect_none | 402872ns | 404390ns | 392370ns | 401867ns | 409631ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 289236ns | 261936ns | 317787ns | -27.76% | 0.014 |
| carrier_opt_wideselect_canon | 279929ns | 259070ns | 304504ns | -30.09% | 0.015 |
| carrier_opt_wideselect_cse | 290778ns | 267826ns | 315671ns | -27.38% | 0.014 |
| carrier_opt_wideselect_dce | 396387ns | 369872ns | 425406ns | -1.00% | 0.010 |
| carrier_opt_wideselect_fold | 414077ns | 391195ns | 431809ns | +3.42% | 0.010 |
| carrier_opt_wideselect_none | 400398ns | 389990ns | 406975ns | base | 0.010 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 1806270 | 3779006 | 0.478 | 0.72× |
| carrier_opt_wideselect_canon | 1743571 | 3828010 | 0.455 | 0.70× |
| carrier_opt_wideselect_cse | 1814374 | 3829020 | 0.474 | 0.73× |
| carrier_opt_wideselect_dce | 2484880 | 4346910 | 0.572 | 0.99× |
| carrier_opt_wideselect_fold | 2577258 | 4338388 | 0.594 | 1.03× |
| carrier_opt_wideselect_none | 2497506 | 4345474 | 0.575 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.016 Gops/s** (carrier_opt_wideselect_canon; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_wideselect_all | 0.014 | 90.8% |
| carrier_opt_wideselect_canon | 0.015 | 95.2% |
| carrier_opt_wideselect_cse | 0.014 | 91.0% |
| carrier_opt_wideselect_dce | 0.010 | 66.2% |
| carrier_opt_wideselect_fold | 0.010 | 62.5% |
| carrier_opt_wideselect_none | 0.010 | 64.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_wideselect_all | 291822ns | 291822ns | -27.56% |
| carrier_opt_wideselect_canon | 282416ns | 282416ns | -29.90% |
| carrier_opt_wideselect_cse | 293238ns | 293238ns | -27.21% |
| carrier_opt_wideselect_dce | 398933ns | 398933ns | -0.98% |
| carrier_opt_wideselect_fold | 416804ns | 416804ns | +3.46% |
| carrier_opt_wideselect_none | 402872ns | 402872ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_wideselect_none | 402038ns | base | --- | [392182, 406975] | --- | --- | --- | --- |
| carrier_opt_wideselect_all | 285273ns | -121534.1ns (-30.2%) | [-137557, -74395]ns | [264648, 317787] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_wideselect_canon | 272269ns | -132091.0ns (-32.9%) | [-135635, -93681]ns | [263016, 304504] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_wideselect_cse | 284702ns | -111333.1ns (-27.7%) | [-126225, -91303]ns | [271960, 315671] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_wideselect_dce | 391050ns | no significant difference | [-29542, +24644]ns | [372706, 425406] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_wideselect_fold | 414241ns | no significant difference | [-9805, +35606]ns | [396180, 431809] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_wideselect_none | carrier_opt_wideselect_all | carrier_opt_wideselect_canon | carrier_opt_wideselect_cse | carrier_opt_wideselect_dce | carrier_opt_wideselect_fold |
|---|---|---|---|---|---|---|
| 1 | 402080ns | -30.8% | -33.1% | -27.6% | -6.6% | -2.7% |
| 2 | 389990ns | -17.7% | -33.6% | -28.6% | +5.2% | +9.8% |
| 3 | 411535ns | -29.0% | -33.0% | -20.6% | +7.0% | -2.1% |
| 4 | 401995ns | -34.8% | -26.1% | -31.3% | +0.6% | -0.2% |
| 5 | 402415ns | -33.6% | -33.7% | -24.3% | -8.1% | +8.2% |
| 6 | 394374ns | -20.2% | -20.9% | -32.1% | -4.2% | +7.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_wideselect_all | -0.097 | ok |
| carrier_opt_wideselect_canon | -0.189 | ok |
| carrier_opt_wideselect_cse | -0.629 | HIGH- (thermal bounce) |
| carrier_opt_wideselect_dce | 0.260 | moderate+ |
| carrier_opt_wideselect_fold | -0.224 | moderate- |
| carrier_opt_wideselect_none | -0.448 | moderate- |

**Consistency summary:**

- **carrier_opt_wideselect_all**: won 6/6, lost 0/6
- **carrier_opt_wideselect_canon**: won 6/6, lost 0/6
- **carrier_opt_wideselect_cse**: won 6/6, lost 0/6
- **carrier_opt_wideselect_dce**: won 3/6, lost 3/6
- **carrier_opt_wideselect_fold**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 290008.7ns | 289236.0ns | 100.3% | HIGH |
| carrier_opt_wideselect_canon | 280256.2ns | 279929.2ns | 100.1% | HIGH |
| carrier_opt_wideselect_cse | 291994.1ns | 290777.6ns | 100.4% | HIGH |
| carrier_opt_wideselect_dce | 397909.7ns | 396387.1ns | 100.4% | HIGH |
| carrier_opt_wideselect_fold | 415399.9ns | 414076.8ns | 100.3% | HIGH |
| carrier_opt_wideselect_none | 402194.2ns | 400398.1ns | 100.4% | HIGH |

## Distribution (algo ns)

```
carrier_opt_wideselect_all (n=6, range 261936.2-317786.7 ns)
  261936.2 |########################################
  264728.7 |########################################
  267521.2 |
  270313.8 |
  273106.3 |
  275898.8 |########################################
  278691.3 |
  281483.9 |
  284276.4 |
  287068.9 |
  289861.4 |########################################
  292653.9 |
  295446.5 |
  298239.0 |
  301031.5 |
  303824.0 |
  306616.6 |
  309409.1 |
  312201.6 |########################################
  314994.1 |
  (0 below, 1 above range)

carrier_opt_wideselect_canon (n=6, range 259070.4-304503.5 ns)
  259070.4 |########################################
  261342.1 |
  263613.7 |
  265885.4 |########################################
  268157.0 |########################################
  270428.7 |
  272700.3 |
  274972.0 |########################################
  277243.7 |
  279515.3 |
  281787.0 |
  284058.6 |
  286330.3 |
  288601.9 |
  290873.6 |
  293145.3 |
  295416.9 |########################################
  297688.6 |
  299960.2 |
  302231.9 |
  (0 below, 1 above range)

carrier_opt_wideselect_cse (n=6, range 267826.2-315671.5 ns)
  267826.2 |########################################
  270218.5 |
  272610.7 |
  275003.0 |########################################
  277395.2 |########################################
  279787.5 |
  282179.8 |
  284572.0 |
  286964.3 |
  289356.6 |########################################
  291748.8 |
  294141.1 |
  296533.4 |
  298925.6 |
  301317.9 |
  303710.1 |########################################
  306102.4 |
  308494.7 |
  310886.9 |
  313279.2 |
  (0 below, 1 above range)

carrier_opt_wideselect_dce (n=6, range 369872.1-425406.1 ns)
  369872.1 |####################
  372648.8 |
  375425.5 |########################################
  378202.2 |
  380978.9 |
  383755.6 |
  386532.3 |
  389309.0 |
  392085.7 |
  394862.4 |
  397639.1 |
  400415.8 |
  403192.5 |####################
  405969.2 |
  408745.9 |####################
  411522.6 |
  414299.3 |
  417076.0 |
  419852.7 |
  422629.4 |
  (0 below, 1 above range)

carrier_opt_wideselect_fold (n=6, range 391195.0-431809.2 ns)
  391195.0 |########################################
  393225.7 |
  395256.4 |
  397287.1 |
  399317.8 |########################################
  401348.5 |########################################
  403379.3 |
  405410.0 |
  407440.7 |
  409471.4 |
  411502.1 |
  413532.8 |
  415563.5 |
  417594.2 |
  419624.9 |
  421655.7 |
  423686.4 |########################################
  425717.1 |
  427747.8 |########################################
  429778.5 |
  (0 below, 1 above range)

carrier_opt_wideselect_none (n=6, range 389990.4-406974.8 ns)
  389990.4 |#############
  390839.6 |
  391688.8 |
  392538.1 |
  393387.3 |
  394236.5 |#############
  395085.7 |
  395934.9 |
  396784.2 |
  397633.4 |
  398482.6 |
  399331.8 |
  400181.0 |
  401030.3 |
  401879.5 |########################################
  402728.7 |
  403577.9 |
  404427.1 |
  405276.4 |
  406125.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_wideselect_all**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_canon**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cse**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_dce**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_fold**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_none**: bridge=100.3% of algo (FFI overhead may distort results)
