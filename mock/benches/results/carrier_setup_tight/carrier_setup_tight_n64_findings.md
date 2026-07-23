# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), tight profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_tight_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_tight_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_tight_stackcompile shows alternating (throttle bounce) (autocorr -0.62)

carrier_setup_tight_stackcompile's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_tight_parse)

The baseline carrier_setup_tight_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_tight_parse, carrier_setup_tight_predecode, carrier_setup_tight_emitdirect, carrier_setup_tight_stackcompile, carrier_setup_tight_emitcopypatch} vs {carrier_setup_tight_optall} (675% apart)

The field splits into a fast tier {carrier_setup_tight_parse, carrier_setup_tight_predecode, carrier_setup_tight_emitdirect, carrier_setup_tight_stackcompile, carrier_setup_tight_emitcopypatch} and a slow tier {carrier_setup_tight_optall} with a 675% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 107.63 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_tight_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 16322ns | 16103ns | 15489ns | 16014ns | 17200ns | +609.69% |
| carrier_setup_tight_emitdirect | 6586ns | 6525ns | 6490ns | 6515ns | 6740ns | +186.36% |
| carrier_setup_tight_optall | 110278ns | 109863ns | 107798ns | 109766ns | 112287ns | +4695.12% |
| carrier_setup_tight_parse | 2300ns | 2302ns | 2144ns | 2273ns | 2417ns | base |
| carrier_setup_tight_predecode | 5718ns | 5702ns | 5403ns | 5641ns | 5993ns | +148.65% |
| carrier_setup_tight_stackcompile | 10127ns | 10111ns | 9871ns | 10050ns | 10371ns | +340.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 14073ns | 13346ns | 14815ns | +0.00% | 0.005 |
| carrier_setup_tight_emitdirect | 4363ns | 4265ns | 4456ns | +0.00% | 0.015 |
| carrier_setup_tight_optall | 108035ns | 105617ns | 110002ns | +0.00% | 0.001 |
| carrier_setup_tight_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_tight_predecode | 3452ns | 3267ns | 3625ns | +0.00% | 0.019 |
| carrier_setup_tight_stackcompile | 7916ns | 7708ns | 8101ns | +0.00% | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 329794 | 1380802 | 0.239 | 22.85× |
| carrier_setup_tight_emitdirect | 282716 | 1631319 | 0.173 | 19.59× |
| carrier_setup_tight_optall | 660332 | 3088324 | 0.214 | 45.75× |
| carrier_setup_tight_parse | 14433 | 52911 | 0.273 | 1.00× |
| carrier_setup_tight_predecode | 275703 | 1899546 | 0.145 | 19.10× |
| carrier_setup_tight_stackcompile | 301419 | 1838961 | 0.164 | 20.88× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.020 Gops/s** (carrier_setup_tight_predecode; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_tight_emitcopypatch | 0.005 | 23.5% |
| carrier_setup_tight_emitdirect | 0.015 | 75.2% |
| carrier_setup_tight_optall | 0.001 | 3.0% |
| carrier_setup_tight_predecode | 0.019 | 94.9% |
| carrier_setup_tight_stackcompile | 0.008 | 41.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 16322ns | 16322ns | +609.69% |
| carrier_setup_tight_emitdirect | 6586ns | 6586ns | +186.36% |
| carrier_setup_tight_optall | 110278ns | 110278ns | +4695.12% |
| carrier_setup_tight_parse | 2300ns | 2300ns | base |
| carrier_setup_tight_predecode | 5718ns | 5718ns | +148.65% |
| carrier_setup_tight_stackcompile | 10127ns | 10127ns | +340.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_tight_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_tight_emitcopypatch | 13891ns | +13891.0ns (+0.0%) | [+13512, +14815]ns | [13512, 14815] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_emitdirect | 4344ns | +4343.9ns (+0.0%) | [+4288, +4456]ns | [4288, 4456] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_optall | 107626ns | +107626.0ns (+0.0%) | [+106477, +110002]ns | [106477, 110002] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_predecode | 3443ns | +3443.3ns (+0.0%) | [+3286, +3625]ns | [3286, 3625] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_stackcompile | 7908ns | +7907.7ns (+0.0%) | [+7740, +8101]ns | [7740, 8101] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_tight_emitcopypatch | 0.015 | ok |
| carrier_setup_tight_emitdirect | -0.239 | moderate- |
| carrier_setup_tight_optall | -0.119 | ok |
| carrier_setup_tight_parse | 0.000 | ok |
| carrier_setup_tight_predecode | -0.090 | ok |
| carrier_setup_tight_stackcompile | -0.618 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_setup_tight_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_tight_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_tight_optall**: won 0/6, lost 0/6
- **carrier_setup_tight_predecode**: won 0/6, lost 0/6
- **carrier_setup_tight_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 93781.2ns | 14072.6ns | 666.4% | HIGH |
| carrier_setup_tight_emitdirect | 86660.4ns | 4362.5ns | 1986.5% | HIGH |
| carrier_setup_tight_optall | 107879.7ns | 108035.2ns | 99.9% | HIGH |
| carrier_setup_tight_parse | 1558.6ns | 0.0ns | 0.0% |  |
| carrier_setup_tight_predecode | 86263.6ns | 3451.5ns | 2499.3% | HIGH |
| carrier_setup_tight_stackcompile | 88818.8ns | 7916.3ns | 1122.0% | HIGH |

## Distribution (algo ns)

```
carrier_setup_tight_emitcopypatch (n=6, range 13345.8-14815.0 ns)
  13345.8 |####################
  13419.3 |
  13492.7 |
  13566.2 |
  13639.6 |########################################
  13713.1 |
  13786.6 |
  13860.0 |
  13933.5 |
  14006.9 |
  14080.4 |####################
  14153.9 |
  14227.3 |
  14300.8 |
  14374.2 |
  14447.7 |
  14521.2 |
  14594.6 |
  14668.1 |
  14741.5 |####################
  (0 below, 1 above range)

carrier_setup_tight_emitdirect (n=6, range 4265.4-4456.0 ns)
   4265.4 |########################################
   4274.9 |
   4284.5 |
   4294.0 |
   4303.5 |########################################
   4313.1 |
   4322.6 |
   4332.1 |########################################
   4341.7 |########################################
   4351.2 |
   4360.7 |
   4370.3 |
   4379.8 |
   4389.3 |
   4398.9 |########################################
   4408.4 |
   4417.9 |
   4427.5 |
   4437.0 |
   4446.5 |
  (0 below, 1 above range)

carrier_setup_tight_optall (n=6, range 105617.1-110002.5 ns)
  105617.1 |####################
  105836.4 |
  106055.6 |
  106274.9 |
  106494.2 |
  106713.5 |
  106932.7 |
  107152.0 |####################
  107371.3 |
  107590.5 |########################################
  107809.8 |
  108029.1 |
  108248.3 |
  108467.6 |
  108686.9 |
  108906.1 |
  109125.4 |####################
  109344.7 |
  109564.0 |
  109783.2 |
  (0 below, 1 above range)

carrier_setup_tight_predecode (n=6, range 3267.1-3625.0 ns)
   3267.1 |########################################
   3285.0 |
   3302.9 |########################################
   3320.8 |########################################
   3338.7 |
   3356.6 |
   3374.5 |
   3392.4 |
   3410.3 |
   3428.2 |
   3446.1 |
   3463.9 |
   3481.8 |
   3499.7 |
   3517.6 |
   3535.5 |########################################
   3553.4 |
   3571.3 |
   3589.2 |
   3607.1 |########################################
  (0 below, 1 above range)

carrier_setup_tight_stackcompile (n=6, range 7708.3-8101.4 ns)
   7708.3 |########################################
   7728.0 |
   7747.6 |
   7767.3 |########################################
   7786.9 |
   7806.6 |########################################
   7826.2 |
   7845.9 |
   7865.6 |
   7885.2 |
   7904.9 |
   7924.5 |
   7944.2 |
   7963.8 |
   7983.5 |
   8003.2 |########################################
   8022.8 |########################################
   8042.5 |
   8062.1 |
   8081.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_tight_emitcopypatch**: bridge=678.4% of algo (FFI overhead may distort results)
- **carrier_setup_tight_emitdirect**: bridge=1995.0% of algo (FFI overhead may distort results)
- **carrier_setup_tight_optall**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_tight_predecode**: bridge=2505.0% of algo (FFI overhead may distort results)
- **carrier_setup_tight_stackcompile**: bridge=1125.7% of algo (FFI overhead may distort results)
