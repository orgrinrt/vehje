# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), scatter profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_scatter_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_scatter_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_setup_scatter_parse)

The baseline carrier_setup_scatter_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_scatter_parse, carrier_setup_scatter_predecode, carrier_setup_scatter_emitdirect, carrier_setup_scatter_stackcompile, carrier_setup_scatter_emitcopypatch} vs {carrier_setup_scatter_optall} (1119% apart)

The field splits into a fast tier {carrier_setup_scatter_parse, carrier_setup_scatter_predecode, carrier_setup_scatter_emitdirect, carrier_setup_scatter_stackcompile, carrier_setup_scatter_emitcopypatch} and a slow tier {carrier_setup_scatter_optall} with a 1119% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 167.03 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_scatter_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 15958ns | 15978ns | 15312ns | 15790ns | 16534ns | +611.86% |
| carrier_setup_scatter_emitdirect | 6298ns | 6340ns | 5991ns | 6226ns | 6560ns | +180.95% |
| carrier_setup_scatter_optall | 169334ns | 169304ns | 168386ns | 169099ns | 170161ns | +7453.55% |
| carrier_setup_scatter_parse | 2242ns | 2185ns | 2133ns | 2168ns | 2406ns | base |
| carrier_setup_scatter_predecode | 5519ns | 5518ns | 5310ns | 5488ns | 5669ns | +146.18% |
| carrier_setup_scatter_stackcompile | 10984ns | 11013ns | 10396ns | 10980ns | 11283ns | +389.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 13724ns | 13171ns | 14230ns | +0.00% | 0.005 |
| carrier_setup_scatter_emitdirect | 4034ns | 3842ns | 4182ns | +0.00% | 0.016 |
| carrier_setup_scatter_optall | 167069ns | 166192ns | 167910ns | +0.00% | 0.000 |
| carrier_setup_scatter_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_scatter_predecode | 3293ns | 3177ns | 3401ns | +0.00% | 0.019 |
| carrier_setup_scatter_stackcompile | 8753ns | 8273ns | 9007ns | +0.00% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 333999 | 1454537 | 0.230 | 23.31× |
| carrier_setup_scatter_emitdirect | 282016 | 1732544 | 0.163 | 19.68× |
| carrier_setup_scatter_optall | 1035804 | 5075806 | 0.204 | 72.28× |
| carrier_setup_scatter_parse | 14330 | 52920 | 0.271 | 1.00× |
| carrier_setup_scatter_predecode | 281598 | 1942912 | 0.145 | 19.65× |
| carrier_setup_scatter_stackcompile | 302678 | 1846115 | 0.164 | 21.12× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.020 Gops/s** (carrier_setup_scatter_predecode; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_scatter_emitcopypatch | 0.005 | 23.2% |
| carrier_setup_scatter_emitdirect | 0.016 | 78.1% |
| carrier_setup_scatter_optall | 0.000 | 1.9% |
| carrier_setup_scatter_predecode | 0.019 | 96.7% |
| carrier_setup_scatter_stackcompile | 0.007 | 36.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 15958ns | 15958ns | +611.86% |
| carrier_setup_scatter_emitdirect | 6298ns | 6298ns | +180.95% |
| carrier_setup_scatter_optall | 169334ns | 169334ns | +7453.55% |
| carrier_setup_scatter_parse | 2242ns | 2242ns | base |
| carrier_setup_scatter_predecode | 5519ns | 5519ns | +146.18% |
| carrier_setup_scatter_stackcompile | 10984ns | 10984ns | +389.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_scatter_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_scatter_emitcopypatch | 13706ns | +13706.5ns (+0.0%) | [+13235, +14230]ns | [13235, 14230] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_emitdirect | 4068ns | +4067.7ns (+0.0%) | [+3851, +4182]ns | [3851, 4182] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_optall | 167029ns | +167029.0ns (+0.0%) | [+166266, +167910]ns | [166266, 167910] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_predecode | 3284ns | +3283.9ns (+0.0%) | [+3193, +3401]ns | [3193, 3401] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_stackcompile | 8772ns | +8772.5ns (+0.0%) | [+8480, +9007]ns | [8480, 9007] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_scatter_parse | carrier_setup_scatter_emitcopypatch | carrier_setup_scatter_emitdirect | carrier_setup_scatter_optall | carrier_setup_scatter_predecode | carrier_setup_scatter_stackcompile |
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
| carrier_setup_scatter_emitcopypatch | -0.351 | moderate- |
| carrier_setup_scatter_emitdirect | -0.059 | ok |
| carrier_setup_scatter_optall | -0.302 | moderate- |
| carrier_setup_scatter_parse | 0.000 | ok |
| carrier_setup_scatter_predecode | -0.065 | ok |
| carrier_setup_scatter_stackcompile | 0.296 | moderate+ |

**Consistency summary:**

- **carrier_setup_scatter_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_scatter_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_scatter_optall**: won 0/6, lost 0/6
- **carrier_setup_scatter_predecode**: won 0/6, lost 0/6
- **carrier_setup_scatter_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 93851.0ns | 13723.9ns | 683.8% | HIGH |
| carrier_setup_scatter_emitdirect | 87097.5ns | 4033.5ns | 2159.3% | HIGH |
| carrier_setup_scatter_optall | 166832.7ns | 167068.6ns | 99.9% | HIGH |
| carrier_setup_scatter_parse | 1511.3ns | 0.0ns | 0.0% |  |
| carrier_setup_scatter_predecode | 86117.8ns | 3292.8ns | 2615.3% | HIGH |
| carrier_setup_scatter_stackcompile | 89190.0ns | 8753.3ns | 1018.9% | HIGH |

## Distribution (algo ns)

```
carrier_setup_scatter_emitcopypatch (n=6, range 13170.8-14230.4 ns)
  13170.8 |########################################
  13223.8 |
  13276.8 |########################################
  13329.7 |
  13382.7 |
  13435.7 |
  13488.7 |
  13541.7 |########################################
  13594.6 |
  13647.6 |
  13700.6 |
  13753.6 |
  13806.6 |
  13859.5 |########################################
  13912.5 |########################################
  13965.5 |
  14018.5 |
  14071.5 |
  14124.4 |
  14177.4 |
  (0 below, 1 above range)

carrier_setup_scatter_emitdirect (n=6, range 3842.1-4181.9 ns)
   3842.1 |####################
   3859.1 |####################
   3876.1 |
   3893.1 |
   3910.1 |
   3927.0 |
   3944.0 |
   3961.0 |####################
   3978.0 |
   3995.0 |
   4012.0 |
   4029.0 |
   4046.0 |
   4062.9 |
   4079.9 |
   4096.9 |
   4113.9 |
   4130.9 |
   4147.9 |
   4164.9 |########################################
  (0 below, 1 above range)

carrier_setup_scatter_optall (n=6, range 166191.7-167910.4 ns)
  166191.7 |########################################
  166277.6 |########################################
  166363.6 |
  166449.5 |
  166535.4 |
  166621.4 |
  166707.3 |
  166793.2 |########################################
  166879.2 |
  166965.1 |
  167051.0 |
  167137.0 |########################################
  167222.9 |########################################
  167308.9 |
  167394.8 |
  167480.7 |
  167566.7 |
  167652.6 |
  167738.5 |
  167824.5 |
  (0 below, 1 above range)

carrier_setup_scatter_predecode (n=6, range 3176.7-3401.4 ns)
   3176.7 |########################################
   3187.9 |
   3199.2 |########################################
   3210.4 |
   3221.6 |
   3232.9 |
   3244.1 |
   3255.4 |########################################
   3266.6 |
   3277.8 |
   3289.1 |
   3300.3 |########################################
   3311.5 |
   3322.8 |
   3334.0 |########################################
   3345.3 |
   3356.5 |
   3367.7 |
   3379.0 |
   3390.2 |
  (0 below, 1 above range)

carrier_setup_scatter_stackcompile (n=6, range 8272.9-9007.3 ns)
   8272.9 |####################
   8309.6 |
   8346.3 |
   8383.1 |
   8419.8 |
   8456.5 |
   8493.2 |
   8529.9 |
   8566.7 |
   8603.4 |
   8640.1 |
   8676.8 |########################################
   8713.5 |
   8750.3 |
   8787.0 |
   8823.7 |####################
   8860.4 |
   8897.1 |
   8933.9 |####################
   8970.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_scatter_emitcopypatch**: bridge=682.1% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_emitdirect**: bridge=2141.2% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_optall**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_predecode**: bridge=2615.9% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_stackcompile**: bridge=1015.9% of algo (FFI overhead may distort results)
