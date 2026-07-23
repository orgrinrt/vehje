# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), madd profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_madd_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_madd_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_madd_predecode shows alternating (throttle bounce) (autocorr -0.68)

carrier_setup_madd_predecode's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_madd_parse)

The baseline carrier_setup_madd_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_madd_parse, carrier_setup_madd_predecode, carrier_setup_madd_emitdirect, carrier_setup_madd_stackcompile, carrier_setup_madd_emitcopypatch} vs {carrier_setup_madd_optall} (354% apart)

The field splits into a fast tier {carrier_setup_madd_parse, carrier_setup_madd_predecode, carrier_setup_madd_emitdirect, carrier_setup_madd_stackcompile, carrier_setup_madd_emitcopypatch} and a slow tier {carrier_setup_madd_optall} with a 354% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 983.82 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_madd_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 220906ns | 218706ns | 216996ns | 218638ns | 226264ns | +9496.76% |
| carrier_setup_madd_emitdirect | 65984ns | 66972ns | 62874ns | 66060ns | 67425ns | +2766.52% |
| carrier_setup_madd_optall | 983238ns | 986224ns | 973087ns | 982457ns | 989484ns | +42614.49% |
| carrier_setup_madd_parse | 2302ns | 2234ns | 2109ns | 2211ns | 2535ns | base |
| carrier_setup_madd_predecode | 38691ns | 38416ns | 37217ns | 38051ns | 40388ns | +1580.84% |
| carrier_setup_madd_stackcompile | 78241ns | 78191ns | 75155ns | 77583ns | 80770ns | +3298.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 218638ns | 214853ns | 223845ns | +0.00% | 0.005 |
| carrier_setup_madd_emitdirect | 63752ns | 60759ns | 65159ns | +0.00% | 0.016 |
| carrier_setup_madd_optall | 980947ns | 970897ns | 987246ns | +0.00% | 0.001 |
| carrier_setup_madd_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_madd_predecode | 36503ns | 35118ns | 38105ns | +0.00% | 0.028 |
| carrier_setup_madd_stackcompile | 76045ns | 73034ns | 78513ns | +0.00% | 0.013 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 1349110 | 5767954 | 0.234 | 94.66× |
| carrier_setup_madd_emitdirect | 583306 | 3610170 | 0.162 | 40.93× |
| carrier_setup_madd_optall | 6152984 | 28523851 | 0.216 | 431.72× |
| carrier_setup_madd_parse | 14252 | 52932 | 0.269 | 1.00× |
| carrier_setup_madd_predecode | 452418 | 3324486 | 0.136 | 31.74× |
| carrier_setup_madd_stackcompile | 706551 | 4511778 | 0.157 | 49.58× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_setup_madd_predecode; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_madd_emitcopypatch | 0.005 | 16.2% |
| carrier_setup_madd_emitdirect | 0.016 | 54.3% |
| carrier_setup_madd_optall | 0.001 | 3.6% |
| carrier_setup_madd_predecode | 0.028 | 96.9% |
| carrier_setup_madd_stackcompile | 0.013 | 46.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 220906ns | 220906ns | +9496.76% |
| carrier_setup_madd_emitdirect | 65984ns | 65984ns | +2766.52% |
| carrier_setup_madd_optall | 983238ns | 983238ns | +42614.49% |
| carrier_setup_madd_parse | 2302ns | 2302ns | base |
| carrier_setup_madd_predecode | 38691ns | 38691ns | +1580.84% |
| carrier_setup_madd_stackcompile | 78241ns | 78241ns | +3298.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_madd_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_madd_emitcopypatch | 216484ns | +216483.5ns (+0.0%) | [+215587, +223845]ns | [215587, 223845] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_emitdirect | 64698ns | +64697.5ns (+0.0%) | [+61400, +65159]ns | [61400, 65159] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_optall | 983825ns | +983824.8ns (+0.0%) | [+971769, +987246]ns | [971769, 987246] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_predecode | 36235ns | +36235.0ns (+0.0%) | [+35168, +38105]ns | [35168, 38105] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_madd_stackcompile | 75999ns | +75999.1ns (+0.0%) | [+73622, +78513]ns | [73622, 78513] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_madd_emitcopypatch | -0.315 | moderate- |
| carrier_setup_madd_emitdirect | -0.347 | moderate- |
| carrier_setup_madd_optall | -0.494 | moderate- |
| carrier_setup_madd_parse | 0.000 | ok |
| carrier_setup_madd_predecode | -0.675 | HIGH- (thermal bounce) |
| carrier_setup_madd_stackcompile | -0.034 | ok |

**Consistency summary:**

- **carrier_setup_madd_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_madd_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_madd_optall**: won 0/6, lost 0/6
- **carrier_setup_madd_predecode**: won 0/6, lost 0/6
- **carrier_setup_madd_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_madd_emitcopypatch | 218803.8ns | 218638.5ns | 100.1% | HIGH |
| carrier_setup_madd_emitdirect | 127637.8ns | 63751.9ns | 200.2% | HIGH |
| carrier_setup_madd_optall | 981811.4ns | 980946.8ns | 100.1% | HIGH |
| carrier_setup_madd_parse | 1549.3ns | 0.0ns | 0.0% |  |
| carrier_setup_madd_predecode | 109423.9ns | 36502.9ns | 299.8% | HIGH |
| carrier_setup_madd_stackcompile | 151593.4ns | 76044.7ns | 199.3% | HIGH |

## Distribution (algo ns)

```
carrier_setup_madd_emitcopypatch (n=6, range 214852.9-223844.8 ns)
  214852.9 |#############
  215302.5 |
  215752.1 |
  216201.7 |########################################
  216651.3 |
  217100.9 |
  217550.5 |
  218000.1 |
  218449.7 |
  218899.3 |
  219348.8 |
  219798.4 |
  220248.0 |
  220697.6 |
  221147.2 |
  221596.8 |
  222046.4 |
  222496.0 |
  222945.6 |
  223395.2 |#############
  (0 below, 1 above range)

carrier_setup_madd_emitdirect (n=6, range 60758.8-65158.6 ns)
  60758.8 |####################
  60978.8 |
  61198.8 |
  61418.8 |
  61638.8 |
  61858.7 |####################
  62078.7 |
  62298.7 |
  62518.7 |
  62738.7 |
  62958.7 |
  63178.7 |
  63398.7 |
  63618.6 |
  63838.6 |
  64058.6 |
  64278.6 |####################
  64498.6 |
  64718.6 |
  64938.6 |########################################
  (0 below, 1 above range)

carrier_setup_madd_optall (n=6, range 970896.7-987246.4 ns)
  970896.7 |########################################
  971714.2 |
  972531.7 |########################################
  973349.2 |
  974166.6 |
  974984.1 |
  975801.6 |
  976619.1 |
  977436.6 |
  978254.1 |
  979071.6 |
  979889.1 |
  980706.5 |
  981524.0 |
  982341.5 |
  983159.0 |########################################
  983976.5 |########################################
  984794.0 |
  985611.5 |########################################
  986429.0 |
  (0 below, 1 above range)

carrier_setup_madd_predecode (n=6, range 35117.5-38105.4 ns)
  35117.5 |########################################
  35266.9 |
  35416.3 |####################
  35565.7 |
  35715.1 |
  35864.5 |
  36013.9 |
  36163.3 |
  36312.7 |
  36462.1 |
  36611.5 |
  36760.9 |
  36910.3 |####################
  37059.7 |####################
  37209.1 |
  37358.5 |
  37507.9 |
  37657.3 |
  37806.7 |
  37956.1 |
  (0 below, 1 above range)

carrier_setup_madd_stackcompile (n=6, range 73033.8-78513.1 ns)
  73033.8 |########################################
  73307.8 |
  73581.7 |
  73855.7 |
  74129.7 |########################################
  74403.6 |
  74677.6 |
  74951.6 |
  75225.5 |
  75499.5 |########################################
  75773.5 |
  76047.4 |
  76321.4 |########################################
  76595.3 |
  76869.3 |
  77143.3 |
  77417.2 |
  77691.2 |
  77965.2 |
  78239.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_madd_emitcopypatch**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_setup_madd_emitdirect**: bridge=200.4% of algo (FFI overhead may distort results)
- **carrier_setup_madd_optall**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_madd_predecode**: bridge=300.0% of algo (FFI overhead may distort results)
- **carrier_setup_madd_stackcompile**: bridge=199.6% of algo (FFI overhead may distort results)
