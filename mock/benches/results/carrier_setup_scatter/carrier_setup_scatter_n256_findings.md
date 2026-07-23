# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), scatter profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_scatter_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_scatter_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_scatter_emitdirect shows alternating (throttle bounce) (autocorr -0.60)

carrier_setup_scatter_emitdirect's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_scatter_parse)

The baseline carrier_setup_scatter_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_scatter_parse, carrier_setup_scatter_predecode, carrier_setup_scatter_emitdirect, carrier_setup_scatter_stackcompile, carrier_setup_scatter_emitcopypatch} vs {carrier_setup_scatter_optall} (856% apart)

The field splits into a fast tier {carrier_setup_scatter_parse, carrier_setup_scatter_predecode, carrier_setup_scatter_emitdirect, carrier_setup_scatter_stackcompile, carrier_setup_scatter_emitcopypatch} and a slow tier {carrier_setup_scatter_optall} with a 856% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 642.35 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_scatter_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 71565ns | 69538ns | 67873ns | 69277ns | 76844ns | +3099.95% |
| carrier_setup_scatter_emitdirect | 18078ns | 17918ns | 17520ns | 17805ns | 18765ns | +708.32% |
| carrier_setup_scatter_optall | 640559ns | 644771ns | 617182ns | 638702ns | 655032ns | +28541.77% |
| carrier_setup_scatter_parse | 2236ns | 2220ns | 2141ns | 2218ns | 2312ns | base |
| carrier_setup_scatter_predecode | 12375ns | 11874ns | 11572ns | 11816ns | 13616ns | +453.34% |
| carrier_setup_scatter_stackcompile | 28656ns | 26488ns | 25288ns | 26280ns | 33904ns | +1181.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 69161ns | 65614ns | 74233ns | +0.00% | 0.004 |
| carrier_setup_scatter_emitdirect | 15857ns | 15366ns | 16457ns | +0.00% | 0.016 |
| carrier_setup_scatter_optall | 638150ns | 614923ns | 652543ns | +0.00% | 0.000 |
| carrier_setup_scatter_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_scatter_predecode | 10102ns | 9405ns | 11205ns | +0.00% | 0.025 |
| carrier_setup_scatter_stackcompile | 26246ns | 23138ns | 31074ns | +0.00% | 0.010 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 602348 | 2202489 | 0.273 | 42.21× |
| carrier_setup_scatter_emitdirect | 338350 | 2071068 | 0.163 | 23.71× |
| carrier_setup_scatter_optall | 3875520 | 18449597 | 0.210 | 271.60× |
| carrier_setup_scatter_parse | 14269 | 52927 | 0.270 | 1.00× |
| carrier_setup_scatter_predecode | 306323 | 2207371 | 0.139 | 21.47× |
| carrier_setup_scatter_stackcompile | 376144 | 2274773 | 0.165 | 26.36× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.027 Gops/s** (carrier_setup_scatter_predecode; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_scatter_emitcopypatch | 0.004 | 14.0% |
| carrier_setup_scatter_emitdirect | 0.016 | 59.8% |
| carrier_setup_scatter_optall | 0.000 | 1.5% |
| carrier_setup_scatter_predecode | 0.027 | 97.5% |
| carrier_setup_scatter_stackcompile | 0.011 | 38.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 71565ns | 71565ns | +3099.95% |
| carrier_setup_scatter_emitdirect | 18078ns | 18078ns | +708.32% |
| carrier_setup_scatter_optall | 640559ns | 640559ns | +28541.77% |
| carrier_setup_scatter_parse | 2236ns | 2236ns | base |
| carrier_setup_scatter_predecode | 12375ns | 12375ns | +453.34% |
| carrier_setup_scatter_stackcompile | 28656ns | 28656ns | +1181.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_scatter_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_scatter_emitcopypatch | 67192ns | +67192.1ns (+0.0%) | [+66059, +74233]ns | [66059, 74233] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_emitdirect | 15717ns | +15716.9ns (+0.0%) | [+15396, +16457]ns | [15396, 16457] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_optall | 642354ns | +642354.3ns (+0.0%) | [+619553, +652543]ns | [619553, 652543] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_predecode | 9650ns | +9649.8ns (+0.0%) | [+9452, +11205]ns | [9452, 11205] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_stackcompile | 24257ns | +24257.1ns (+0.0%) | [+23407, +31074]ns | [23407, 31074] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_scatter_emitcopypatch | 0.033 | ok |
| carrier_setup_scatter_emitdirect | -0.597 | HIGH- (thermal bounce) |
| carrier_setup_scatter_optall | -0.364 | moderate- |
| carrier_setup_scatter_parse | 0.000 | ok |
| carrier_setup_scatter_predecode | -0.257 | moderate- |
| carrier_setup_scatter_stackcompile | -0.135 | ok |

**Consistency summary:**

- **carrier_setup_scatter_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_scatter_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_scatter_optall**: won 0/6, lost 0/6
- **carrier_setup_scatter_predecode**: won 0/6, lost 0/6
- **carrier_setup_scatter_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 133557.2ns | 69161.3ns | 193.1% | HIGH |
| carrier_setup_scatter_emitdirect | 93245.4ns | 15856.9ns | 588.0% | HIGH |
| carrier_setup_scatter_optall | 637507.8ns | 638150.0ns | 99.9% | HIGH |
| carrier_setup_scatter_parse | 1513.7ns | 0.0ns | 0.0% |  |
| carrier_setup_scatter_predecode | 90018.0ns | 10102.2ns | 891.1% | HIGH |
| carrier_setup_scatter_stackcompile | 98465.4ns | 26246.0ns | 375.2% | HIGH |

## Distribution (algo ns)

```
carrier_setup_scatter_emitcopypatch (n=6, range 65614.2-74232.7 ns)
  65614.2 |####################
  66045.1 |
  66476.1 |########################################
  66907.0 |
  67337.9 |####################
  67768.8 |
  68199.8 |
  68630.7 |
  69061.6 |
  69492.5 |
  69923.4 |
  70354.4 |
  70785.3 |
  71216.2 |
  71647.1 |####################
  72078.1 |
  72509.0 |
  72939.9 |
  73370.8 |
  73801.8 |
  (0 below, 1 above range)

carrier_setup_scatter_emitdirect (n=6, range 15366.2-16457.3 ns)
  15366.2 |####################
  15420.8 |########################################
  15475.3 |
  15529.9 |
  15584.4 |
  15639.0 |
  15693.5 |
  15748.1 |
  15802.6 |
  15857.2 |
  15911.8 |
  15966.3 |####################
  16020.9 |
  16075.4 |
  16130.0 |####################
  16184.5 |
  16239.1 |
  16293.6 |
  16348.2 |
  16402.7 |
  (0 below, 1 above range)

carrier_setup_scatter_optall (n=6, range 614922.9-652542.9 ns)
  614922.9 |########################################
  616803.9 |
  618684.9 |
  620565.9 |
  622446.9 |########################################
  624327.9 |
  626208.9 |
  628089.9 |
  629970.9 |
  631851.9 |
  633732.9 |
  635613.9 |
  637494.9 |
  639375.9 |########################################
  641256.9 |
  643137.9 |########################################
  645018.9 |
  646899.9 |
  648780.9 |
  650661.9 |########################################
  (0 below, 1 above range)

carrier_setup_scatter_predecode (n=6, range 9405.4-11204.6 ns)
   9405.4 |####################
   9495.4 |####################
   9585.3 |########################################
   9675.3 |####################
   9765.2 |
   9855.2 |
   9945.2 |
  10035.1 |
  10125.1 |
  10215.0 |
  10305.0 |
  10395.0 |
  10484.9 |
  10574.9 |
  10664.8 |
  10754.8 |
  10844.8 |
  10934.7 |
  11024.7 |
  11114.6 |
  (0 below, 1 above range)

carrier_setup_scatter_stackcompile (n=6, range 23137.9-31074.0 ns)
  23137.9 |########################################
  23534.7 |########################################
  23931.5 |########################################
  24328.3 |########################################
  24725.1 |
  25121.9 |########################################
  25518.7 |
  25915.5 |
  26312.3 |
  26709.1 |
  27105.9 |
  27502.7 |
  27899.5 |
  28296.3 |
  28693.1 |
  29089.9 |
  29486.7 |
  29883.5 |
  30280.3 |
  30677.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_scatter_emitcopypatch**: bridge=198.0% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_emitdirect**: bridge=589.3% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_optall**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_predecode**: bridge=922.9% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_stackcompile**: bridge=399.4% of algo (FFI overhead may distort results)
