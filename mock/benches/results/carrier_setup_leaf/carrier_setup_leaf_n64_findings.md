# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), leaf profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_leaf_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_leaf_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_setup_leaf_parse)

The baseline carrier_setup_leaf_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_leaf_parse, carrier_setup_leaf_emitdirect, carrier_setup_leaf_predecode, carrier_setup_leaf_stackcompile, carrier_setup_leaf_emitcopypatch} vs {carrier_setup_leaf_optall} (979% apart)

The field splits into a fast tier {carrier_setup_leaf_parse, carrier_setup_leaf_emitdirect, carrier_setup_leaf_predecode, carrier_setup_leaf_stackcompile, carrier_setup_leaf_emitcopypatch} and a slow tier {carrier_setup_leaf_optall} with a 979% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 108.66 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_leaf_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 12329ns | 12383ns | 11812ns | 12216ns | 12757ns | +414.83% |
| carrier_setup_leaf_emitdirect | 5102ns | 5100ns | 4898ns | 5036ns | 5303ns | +113.04% |
| carrier_setup_leaf_optall | 110842ns | 110949ns | 109379ns | 110549ns | 112013ns | +4528.45% |
| carrier_setup_leaf_parse | 2395ns | 2385ns | 2384ns | 2385ns | 2415ns | base |
| carrier_setup_leaf_predecode | 5509ns | 5569ns | 5202ns | 5522ns | 5642ns | +130.04% |
| carrier_setup_leaf_stackcompile | 10186ns | 10276ns | 9723ns | 10206ns | 10388ns | +325.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 10030ns | 9586ns | 10398ns | +0.00% | 0.006 |
| carrier_setup_leaf_emitdirect | 2805ns | 2662ns | 2938ns | +0.00% | 0.023 |
| carrier_setup_leaf_optall | 108580ns | 107198ns | 109754ns | +0.00% | 0.001 |
| carrier_setup_leaf_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_leaf_predecode | 3220ns | 3088ns | 3288ns | +0.00% | 0.020 |
| carrier_setup_leaf_stackcompile | 7878ns | 7500ns | 8045ns | +0.00% | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 295539 | 1240353 | 0.238 | 20.72× |
| carrier_setup_leaf_emitdirect | 267397 | 1701994 | 0.157 | 18.75× |
| carrier_setup_leaf_optall | 662746 | 2891082 | 0.229 | 46.46× |
| carrier_setup_leaf_parse | 14265 | 52932 | 0.269 | 1.00× |
| carrier_setup_leaf_predecode | 267897 | 1755196 | 0.153 | 18.78× |
| carrier_setup_leaf_stackcompile | 286336 | 1739505 | 0.165 | 20.07× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.024 Gops/s** (carrier_setup_leaf_emitdirect; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_leaf_emitcopypatch | 0.006 | 26.4% |
| carrier_setup_leaf_emitdirect | 0.023 | 94.6% |
| carrier_setup_leaf_optall | 0.001 | 2.4% |
| carrier_setup_leaf_predecode | 0.020 | 82.0% |
| carrier_setup_leaf_stackcompile | 0.008 | 33.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 12329ns | 12329ns | +414.83% |
| carrier_setup_leaf_emitdirect | 5102ns | 5102ns | +113.04% |
| carrier_setup_leaf_optall | 110842ns | 110842ns | +4528.45% |
| carrier_setup_leaf_parse | 2395ns | 2395ns | base |
| carrier_setup_leaf_predecode | 5509ns | 5509ns | +130.04% |
| carrier_setup_leaf_stackcompile | 10186ns | 10186ns | +325.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_leaf_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_leaf_emitcopypatch | 10068ns | +10067.9ns (+0.0%) | [+9623, +10398]ns | [9623, 10398] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_emitdirect | 2813ns | +2812.7ns (+0.0%) | [+2666, +2938]ns | [2666, 2938] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_optall | 108662ns | +108662.2ns (+0.0%) | [+107324, +109754]ns | [107324, 109754] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_predecode | 3247ns | +3246.7ns (+0.0%) | [+3125, +3288]ns | [3125, 3288] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_leaf_stackcompile | 7957ns | +7957.1ns (+0.0%) | [+7632, +8045]ns | [7632, 8045] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_leaf_emitcopypatch | -0.308 | moderate- |
| carrier_setup_leaf_emitdirect | -0.269 | moderate- |
| carrier_setup_leaf_optall | 0.029 | ok |
| carrier_setup_leaf_parse | 0.000 | ok |
| carrier_setup_leaf_predecode | 0.322 | moderate+ |
| carrier_setup_leaf_stackcompile | -0.230 | moderate- |

**Consistency summary:**

- **carrier_setup_leaf_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_leaf_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_leaf_optall**: won 0/6, lost 0/6
- **carrier_setup_leaf_predecode**: won 0/6, lost 0/6
- **carrier_setup_leaf_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_leaf_emitcopypatch | 88910.3ns | 10029.5ns | 886.5% | HIGH |
| carrier_setup_leaf_emitdirect | 85851.8ns | 2805.3ns | 3060.3% | HIGH |
| carrier_setup_leaf_optall | 108738.6ns | 108580.0ns | 100.1% | HIGH |
| carrier_setup_leaf_parse | 1623.0ns | 0.0ns | 0.0% |  |
| carrier_setup_leaf_predecode | 85646.1ns | 3219.7ns | 2660.0% | HIGH |
| carrier_setup_leaf_stackcompile | 88155.9ns | 7877.9ns | 1119.0% | HIGH |

## Distribution (algo ns)

```
carrier_setup_leaf_emitcopypatch (n=6, range 9586.2-10397.8 ns)
   9586.2 |########################################
   9626.8 |########################################
   9667.4 |
   9707.9 |
   9748.5 |
   9789.1 |
   9829.7 |
   9870.2 |
   9910.8 |########################################
   9951.4 |
   9992.0 |
  10032.6 |
  10073.1 |
  10113.7 |
  10154.3 |
  10194.9 |########################################
  10235.4 |
  10276.0 |########################################
  10316.6 |
  10357.2 |
  (0 below, 1 above range)

carrier_setup_leaf_emitdirect (n=6, range 2662.1-2937.7 ns)
   2662.1 |########################################
   2675.9 |
   2689.7 |
   2703.4 |
   2717.2 |
   2731.0 |
   2744.8 |####################
   2758.6 |
   2772.3 |
   2786.1 |
   2799.9 |
   2813.7 |
   2827.5 |
   2841.2 |
   2855.0 |
   2868.8 |####################
   2882.6 |
   2896.4 |
   2910.1 |
   2923.9 |####################
  (0 below, 1 above range)

carrier_setup_leaf_optall (n=6, range 107197.5-109753.6 ns)
  107197.5 |########################################
  107325.3 |########################################
  107453.1 |
  107580.9 |
  107708.7 |
  107836.5 |
  107964.3 |
  108092.1 |
  108219.9 |
  108347.7 |########################################
  108475.5 |
  108603.3 |
  108731.1 |
  108858.9 |########################################
  108986.7 |
  109114.5 |
  109242.3 |
  109370.1 |########################################
  109497.9 |
  109625.7 |
  (0 below, 1 above range)

carrier_setup_leaf_predecode (n=6, range 3087.5-3287.7 ns)
   3087.5 |####################
   3097.5 |
   3107.5 |
   3117.5 |
   3127.5 |
   3137.6 |
   3147.6 |
   3157.6 |####################
   3167.6 |
   3177.6 |
   3187.6 |
   3197.6 |
   3207.6 |
   3217.6 |
   3227.6 |
   3237.6 |########################################
   3247.7 |
   3257.7 |
   3267.7 |
   3277.7 |####################
  (0 below, 1 above range)

carrier_setup_leaf_stackcompile (n=6, range 7499.6-8044.8 ns)
   7499.6 |########################################
   7526.9 |
   7554.1 |
   7581.4 |
   7608.6 |
   7635.9 |
   7663.2 |
   7690.4 |
   7717.7 |
   7744.9 |########################################
   7772.2 |
   7799.5 |
   7826.7 |
   7854.0 |
   7881.2 |
   7908.5 |
   7935.8 |########################################
   7963.0 |########################################
   7990.3 |########################################
   8017.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_leaf_emitcopypatch**: bridge=880.2% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_emitdirect**: bridge=3050.5% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_optall**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_predecode**: bridge=2640.1% of algo (FFI overhead may distort results)
- **carrier_setup_leaf_stackcompile**: bridge=1105.7% of algo (FFI overhead may distort results)
