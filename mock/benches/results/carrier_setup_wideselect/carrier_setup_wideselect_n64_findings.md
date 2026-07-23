# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), wideselect profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_wideselect_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_wideselect_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_wideselect_optall shows alternating (throttle bounce) (autocorr -0.61)

carrier_setup_wideselect_optall's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_wideselect_parse)

The baseline carrier_setup_wideselect_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_wideselect_parse, carrier_setup_wideselect_predecode, carrier_setup_wideselect_emitdirect, carrier_setup_wideselect_stackcompile, carrier_setup_wideselect_emitcopypatch} vs {carrier_setup_wideselect_optall} (712% apart)

The field splits into a fast tier {carrier_setup_wideselect_parse, carrier_setup_wideselect_predecode, carrier_setup_wideselect_emitdirect, carrier_setup_wideselect_stackcompile, carrier_setup_wideselect_emitcopypatch} and a slow tier {carrier_setup_wideselect_optall} with a 712% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 156.48 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_wideselect_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 21153ns | 21601ns | 19886ns | 21165ns | 21767ns | +825.53% |
| carrier_setup_wideselect_emitdirect | 6077ns | 6108ns | 5938ns | 6070ns | 6155ns | +165.88% |
| carrier_setup_wideselect_optall | 158912ns | 158669ns | 156794ns | 158374ns | 160778ns | +6853.11% |
| carrier_setup_wideselect_parse | 2285ns | 2307ns | 2098ns | 2251ns | 2431ns | base |
| carrier_setup_wideselect_predecode | 5704ns | 5679ns | 5407ns | 5625ns | 5972ns | +149.58% |
| carrier_setup_wideselect_stackcompile | 11390ns | 11352ns | 10780ns | 11208ns | 11968ns | +398.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 18867ns | 17728ns | 19424ns | +0.00% | 0.003 |
| carrier_setup_wideselect_emitdirect | 3888ns | 3801ns | 3938ns | +0.00% | 0.016 |
| carrier_setup_wideselect_optall | 156724ns | 154639ns | 158566ns | +0.00% | 0.000 |
| carrier_setup_wideselect_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_wideselect_predecode | 3491ns | 3295ns | 3658ns | +0.00% | 0.018 |
| carrier_setup_wideselect_stackcompile | 9138ns | 8627ns | 9608ns | +0.00% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 341350 | 1298700 | 0.263 | 23.78× |
| carrier_setup_wideselect_emitdirect | 284135 | 1934607 | 0.147 | 19.80× |
| carrier_setup_wideselect_optall | 975207 | 4797428 | 0.203 | 67.94× |
| carrier_setup_wideselect_parse | 14353 | 52941 | 0.271 | 1.00× |
| carrier_setup_wideselect_predecode | 279038 | 1925213 | 0.145 | 19.44× |
| carrier_setup_wideselect_stackcompile | 300950 | 1818641 | 0.165 | 20.97× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.019 Gops/s** (carrier_setup_wideselect_predecode; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 0.003 | 17.1% |
| carrier_setup_wideselect_emitdirect | 0.016 | 84.3% |
| carrier_setup_wideselect_optall | 0.000 | 2.1% |
| carrier_setup_wideselect_predecode | 0.018 | 94.6% |
| carrier_setup_wideselect_stackcompile | 0.007 | 36.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 21153ns | 21153ns | +825.53% |
| carrier_setup_wideselect_emitdirect | 6077ns | 6077ns | +165.88% |
| carrier_setup_wideselect_optall | 158912ns | 158912ns | +6853.11% |
| carrier_setup_wideselect_parse | 2285ns | 2285ns | base |
| carrier_setup_wideselect_predecode | 5704ns | 5704ns | +149.58% |
| carrier_setup_wideselect_stackcompile | 11390ns | 11390ns | +398.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_wideselect_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_wideselect_emitcopypatch | 19266ns | +19265.8ns (+0.0%) | [+17912, +19424]ns | [17912, 19424] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_emitdirect | 3908ns | +3908.3ns (+0.0%) | [+3816, +3938]ns | [3816, 3938] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_optall | 156480ns | +156480.2ns (+0.0%) | [+155126, +158566]ns | [155126, 158566] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_predecode | 3482ns | +3482.5ns (+0.0%) | [+3332, +3658]ns | [3332, 3658] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_stackcompile | 9106ns | +9106.2ns (+0.0%) | [+8699, +9608]ns | [8699, 9608] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_wideselect_parse | carrier_setup_wideselect_emitcopypatch | carrier_setup_wideselect_emitdirect | carrier_setup_wideselect_optall | carrier_setup_wideselect_predecode | carrier_setup_wideselect_stackcompile |
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
| carrier_setup_wideselect_emitcopypatch | -0.563 | HIGH- (thermal bounce) |
| carrier_setup_wideselect_emitdirect | -0.476 | moderate- |
| carrier_setup_wideselect_optall | -0.608 | HIGH- (thermal bounce) |
| carrier_setup_wideselect_parse | 0.000 | ok |
| carrier_setup_wideselect_predecode | 0.245 | moderate+ |
| carrier_setup_wideselect_stackcompile | -0.226 | moderate- |

**Consistency summary:**

- **carrier_setup_wideselect_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_wideselect_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_wideselect_optall**: won 0/6, lost 0/6
- **carrier_setup_wideselect_predecode**: won 0/6, lost 0/6
- **carrier_setup_wideselect_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 94672.1ns | 18867.1ns | 501.8% | HIGH |
| carrier_setup_wideselect_emitdirect | 86655.0ns | 3887.6ns | 2229.0% | HIGH |
| carrier_setup_wideselect_optall | 156950.8ns | 156724.0ns | 100.1% | HIGH |
| carrier_setup_wideselect_parse | 1535.1ns | 0.0ns | 0.0% |  |
| carrier_setup_wideselect_predecode | 86272.0ns | 3490.5ns | 2471.6% | HIGH |
| carrier_setup_wideselect_stackcompile | 89102.4ns | 9137.5ns | 975.1% | HIGH |

## Distribution (algo ns)

```
carrier_setup_wideselect_emitcopypatch (n=6, range 17727.9-19423.5 ns)
  17727.9 |####################
  17812.7 |
  17897.5 |
  17982.2 |
  18067.0 |####################
  18151.8 |
  18236.6 |
  18321.4 |
  18406.2 |
  18490.9 |
  18575.7 |
  18660.5 |
  18745.3 |
  18830.1 |
  18914.9 |
  18999.6 |
  19084.4 |####################
  19169.2 |
  19254.0 |
  19338.8 |########################################
  (0 below, 1 above range)

carrier_setup_wideselect_emitdirect (n=6, range 3800.8-3938.3 ns)
   3800.8 |####################
   3807.7 |
   3814.6 |
   3821.4 |
   3828.3 |####################
   3835.2 |
   3842.1 |
   3848.9 |
   3855.8 |
   3862.7 |
   3869.6 |
   3876.5 |
   3883.3 |
   3890.2 |
   3897.1 |####################
   3904.0 |
   3910.8 |
   3917.7 |########################################
   3924.6 |
   3931.5 |
  (0 below, 1 above range)

carrier_setup_wideselect_optall (n=6, range 154638.8-158566.0 ns)
  154638.8 |########################################
  154835.2 |
  155031.5 |
  155227.9 |
  155424.2 |########################################
  155620.6 |
  155817.0 |
  156013.3 |
  156209.7 |########################################
  156406.1 |
  156602.4 |########################################
  156798.8 |
  156995.1 |
  157191.5 |########################################
  157387.9 |
  157584.2 |
  157780.6 |
  157977.0 |
  158173.3 |
  158369.7 |
  (0 below, 1 above range)

carrier_setup_wideselect_predecode (n=6, range 3294.6-3657.5 ns)
   3294.6 |####################
   3312.7 |
   3330.9 |
   3349.0 |
   3367.2 |####################
   3385.3 |
   3403.5 |
   3421.6 |
   3439.8 |
   3457.9 |
   3476.1 |########################################
   3494.2 |
   3512.3 |
   3530.5 |
   3548.6 |
   3566.8 |
   3584.9 |
   3603.1 |
   3621.2 |
   3639.4 |####################
  (0 below, 1 above range)

carrier_setup_wideselect_stackcompile (n=6, range 8626.7-9607.5 ns)
   8626.7 |########################################
   8675.7 |
   8724.8 |########################################
   8773.8 |
   8822.9 |
   8871.9 |########################################
   8920.9 |
   8970.0 |
   9019.0 |
   9068.1 |
   9117.1 |
   9166.1 |
   9215.2 |
   9264.2 |########################################
   9313.3 |
   9362.3 |
   9411.3 |
   9460.4 |
   9509.4 |########################################
   9558.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_wideselect_emitcopypatch**: bridge=501.5% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_emitdirect**: bridge=2217.8% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_optall**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_predecode**: bridge=2472.8% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_stackcompile**: bridge=978.5% of algo (FFI overhead may distort results)
