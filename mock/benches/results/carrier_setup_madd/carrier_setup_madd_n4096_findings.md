# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), madd profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_madd_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_madd_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_setup_madd_parse)

The baseline carrier_setup_madd_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_madd_parse, carrier_setup_madd_predecode, carrier_setup_madd_emitdirect, carrier_setup_madd_stackcompile, carrier_setup_madd_emitcopypatch} vs {carrier_setup_madd_optall} (348% apart)

The field splits into a fast tier {carrier_setup_madd_parse, carrier_setup_madd_predecode, carrier_setup_madd_emitdirect, carrier_setup_madd_stackcompile, carrier_setup_madd_emitcopypatch} and a slow tier {carrier_setup_madd_optall} with a 348% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 3.83 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_madd_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 858609ns | 857524ns | 854410ns | 856905ns | 863264ns | +38498.98% |
| carrier_setup_madd_emitdirect | 248895ns | 248969ns | 247148ns | 248766ns | 249962ns | +11089.14% |
| carrier_setup_madd_optall | 3833366ns | 3833601ns | 3806631ns | 3826393ns | 3857192ns | +172230.00% |
| carrier_setup_madd_parse | 2224ns | 2215ns | 2093ns | 2187ns | 2347ns | base |
| carrier_setup_madd_predecode | 145374ns | 145056ns | 142851ns | 144445ns | 148028ns | +6435.32% |
| carrier_setup_madd_stackcompile | 344696ns | 345660ns | 338205ns | 343565ns | 349637ns | +15395.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 856217ns | 852130ns | 860804ns | +0.00% | 0.005 |
| carrier_setup_madd_emitdirect | 246695ns | 244977ns | 247760ns | +0.00% | 0.017 |
| carrier_setup_madd_optall | 3830688ns | 3804133ns | 3854449ns | +0.00% | 0.001 |
| carrier_setup_madd_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_madd_predecode | 143134ns | 140686ns | 145670ns | +0.00% | 0.029 |
| carrier_setup_madd_stackcompile | 342511ns | 336032ns | 347445ns | +0.00% | 0.012 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 5362446 | 22993662 | 0.233 | 375.46× |
| carrier_setup_madd_emitdirect | 1548694 | 9576878 | 0.162 | 108.43× |
| carrier_setup_madd_optall | 23959380 | 111587454 | 0.215 | 1677.53× |
| carrier_setup_madd_parse | 14282 | 52930 | 0.270 | 1.00× |
| carrier_setup_madd_predecode | 892090 | 6507170 | 0.137 | 62.46× |
| carrier_setup_madd_stackcompile | 2159098 | 11325500 | 0.191 | 151.17× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_setup_madd_predecode; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_madd_emitcopypatch | 0.005 | 16.5% |
| carrier_setup_madd_emitdirect | 0.017 | 57.0% |
| carrier_setup_madd_optall | 0.001 | 3.7% |
| carrier_setup_madd_predecode | 0.029 | 98.5% |
| carrier_setup_madd_stackcompile | 0.012 | 41.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 858609ns | 858609ns | +38498.98% |
| carrier_setup_madd_emitdirect | 248895ns | 248895ns | +11089.14% |
| carrier_setup_madd_optall | 3833366ns | 3833366ns | +172230.00% |
| carrier_setup_madd_parse | 2224ns | 2224ns | base |
| carrier_setup_madd_predecode | 145374ns | 145374ns | +6435.32% |
| carrier_setup_madd_stackcompile | 344696ns | 344696ns | +15395.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_madd_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_madd_emitcopypatch | 855087ns | +855087.3ns (+0.0%) | [+852758, +860804]ns | [852758, 860804] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_emitdirect | 246789ns | +246788.8ns (+0.0%) | [+245537, +247760]ns | [245537, 247760] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_optall | 3830990ns | +3830989.5ns (+0.0%) | [+3806626, +3854449]ns | [3806626, 3854449] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_predecode | 142868ns | +142868.5ns (+0.0%) | [+140864, +145670]ns | [140864, 145670] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_stackcompile | 343503ns | +343503.2ns (+0.0%) | [+336585, +347445]ns | [336585, 347445] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_madd_emitcopypatch | -0.381 | moderate- |
| carrier_setup_madd_emitdirect | -0.080 | ok |
| carrier_setup_madd_optall | 0.113 | ok |
| carrier_setup_madd_parse | 0.000 | ok |
| carrier_setup_madd_predecode | -0.273 | moderate- |
| carrier_setup_madd_stackcompile | -0.039 | ok |

**Consistency summary:**

- **carrier_setup_madd_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_madd_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_madd_optall**: won 0/6, lost 0/6
- **carrier_setup_madd_predecode**: won 0/6, lost 0/6
- **carrier_setup_madd_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 856871.1ns | 856216.5ns | 100.1% | HIGH |
| carrier_setup_madd_emitdirect | 246749.3ns | 246695.2ns | 100.0% | HIGH |
| carrier_setup_madd_optall | 3825572.8ns | 3830688.1ns | 99.9% | HIGH |
| carrier_setup_madd_parse | 1510.9ns | 0.0ns | 0.0% |  |
| carrier_setup_madd_predecode | 143277.6ns | 143134.1ns | 100.1% | HIGH |
| carrier_setup_madd_stackcompile | 343529.5ns | 342511.0ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_setup_madd_emitcopypatch (n=6, range 852130.4-860804.2 ns)
  852130.4 |########################################
  852564.1 |
  852997.8 |########################################
  853431.5 |
  853865.2 |########################################
  854298.8 |
  854732.5 |
  855166.2 |
  855599.9 |
  856033.6 |########################################
  856467.3 |
  856901.0 |
  857334.7 |
  857768.3 |
  858202.0 |
  858635.7 |
  859069.4 |
  859503.1 |
  859936.8 |
  860370.5 |########################################
  (0 below, 1 above range)

carrier_setup_madd_emitdirect (n=6, range 244976.7-247759.6 ns)
  244976.7 |########################################
  245115.8 |
  245255.0 |
  245394.1 |
  245533.3 |
  245672.4 |
  245811.6 |
  245950.7 |
  246089.9 |########################################
  246229.0 |########################################
  246368.2 |
  246507.3 |
  246646.4 |
  246785.6 |
  246924.7 |
  247063.9 |
  247203.0 |########################################
  247342.2 |
  247481.3 |########################################
  247620.5 |
  (0 below, 1 above range)

carrier_setup_madd_optall (n=6, range 3804132.9-3854448.8 ns)
  3804132.9 |########################################
  3806648.7 |########################################
  3809164.5 |
  3811680.3 |
  3814196.1 |
  3816711.9 |
  3819227.7 |
  3821743.4 |########################################
  3824259.2 |
  3826775.0 |
  3829290.8 |
  3831806.6 |
  3834322.4 |
  3836838.2 |
  3839354.0 |########################################
  3841869.8 |
  3844385.6 |########################################
  3846901.4 |
  3849417.2 |
  3851933.0 |
  (0 below, 1 above range)

carrier_setup_madd_predecode (n=6, range 140685.8-145669.6 ns)
  140685.8 |####################
  140935.0 |####################
  141184.2 |
  141433.4 |
  141682.6 |
  141931.8 |####################
  142180.9 |
  142430.1 |
  142679.3 |
  142928.5 |
  143177.7 |
  143426.9 |
  143676.1 |########################################
  143925.3 |
  144174.5 |
  144423.6 |
  144672.8 |
  144922.0 |
  145171.2 |
  145420.4 |
  (0 below, 1 above range)

carrier_setup_madd_stackcompile (n=6, range 336031.7-347444.6 ns)
  336031.7 |####################
  336602.3 |####################
  337173.0 |
  337743.6 |
  338314.3 |
  338884.9 |
  339455.6 |
  340026.2 |
  340596.9 |
  341167.5 |
  341738.2 |####################
  342308.8 |
  342879.4 |
  343450.1 |
  344020.7 |
  344591.4 |
  345162.0 |########################################
  345732.7 |
  346303.3 |
  346874.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_madd_emitcopypatch**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_madd_emitdirect**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_setup_madd_optall**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_setup_madd_predecode**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_madd_stackcompile**: bridge=99.6% of algo (FFI overhead may distort results)
