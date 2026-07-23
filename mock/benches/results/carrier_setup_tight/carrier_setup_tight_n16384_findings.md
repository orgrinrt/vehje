# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), tight profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_tight_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_tight_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_tight_predecode shows alternating (throttle bounce) (autocorr -0.57)

carrier_setup_tight_predecode's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_tight_parse)

The baseline carrier_setup_tight_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_tight_parse, carrier_setup_tight_predecode, carrier_setup_tight_stackcompile, carrier_setup_tight_emitdirect, carrier_setup_tight_emitcopypatch} vs {carrier_setup_tight_optall} (212% apart)

The field splits into a fast tier {carrier_setup_tight_parse, carrier_setup_tight_predecode, carrier_setup_tight_stackcompile, carrier_setup_tight_emitdirect, carrier_setup_tight_emitcopypatch} and a slow tier {carrier_setup_tight_optall} with a 212% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 12.15 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_tight_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 3900565ns | 3890776ns | 3877154ns | 3889188ns | 3929336ns | +180348.04% |
| carrier_setup_tight_emitdirect | 1624242ns | 1618650ns | 1608612ns | 1615977ns | 1644453ns | +75040.71% |
| carrier_setup_tight_optall | 12175456ns | 12149416ns | 12061766ns | 12120234ns | 12315135ns | +563161.31% |
| carrier_setup_tight_parse | 2162ns | 2142ns | 2103ns | 2137ns | 2228ns | base |
| carrier_setup_tight_predecode | 557548ns | 555920ns | 554838ns | 555656ns | 561741ns | +25693.29% |
| carrier_setup_tight_stackcompile | 1229439ns | 1221693ns | 1209518ns | 1219773ns | 1253898ns | +56776.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 3897417ns | 3874459ns | 3925645ns | +0.00% | 0.004 |
| carrier_setup_tight_emitdirect | 1621458ns | 1606108ns | 1641382ns | +0.00% | 0.010 |
| carrier_setup_tight_optall | 12171790ns | 12058289ns | 12311505ns | +0.00% | 0.001 |
| carrier_setup_tight_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_tight_predecode | 555275ns | 552598ns | 559444ns | +0.00% | 0.030 |
| carrier_setup_tight_stackcompile | 1226929ns | 1207245ns | 1251191ns | +0.00% | 0.013 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 24223381 | 106117316 | 0.228 | 1320.02× |
| carrier_setup_tight_emitdirect | 10044777 | 51729518 | 0.194 | 547.37× |
| carrier_setup_tight_optall | 75382027 | 371063115 | 0.203 | 4107.83× |
| carrier_setup_tight_parse | 18351 | 79491 | 0.231 | 1.00× |
| carrier_setup_tight_predecode | 3476160 | 25682450 | 0.135 | 189.43× |
| carrier_setup_tight_stackcompile | 7597261 | 41857645 | 0.182 | 414.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.030 Gops/s** (carrier_setup_tight_predecode; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_tight_emitcopypatch | 0.004 | 14.2% |
| carrier_setup_tight_emitdirect | 0.010 | 34.2% |
| carrier_setup_tight_optall | 0.001 | 4.5% |
| carrier_setup_tight_predecode | 0.030 | 99.8% |
| carrier_setup_tight_stackcompile | 0.013 | 45.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 3900565ns | 3900565ns | +180348.04% |
| carrier_setup_tight_emitdirect | 1624242ns | 1624242ns | +75040.71% |
| carrier_setup_tight_optall | 12175456ns | 12175456ns | +563161.31% |
| carrier_setup_tight_parse | 2162ns | 2162ns | base |
| carrier_setup_tight_predecode | 557548ns | 557548ns | +25693.29% |
| carrier_setup_tight_stackcompile | 1229439ns | 1229439ns | +56776.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_tight_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_tight_emitcopypatch | 3887792ns | +3887791.5ns (+0.0%) | [+3878816, +3925645]ns | [3878816, 3925645] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_emitdirect | 1615876ns | +1615876.4ns (+0.0%) | [+1607115, +1641382]ns | [1607115, 1641382] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_optall | 12145573ns | +12145572.7ns (+0.0%) | [+12058292, +12311505]ns | [12058292, 12311505] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_predecode | 553654ns | +553654.4ns (+0.0%) | [+552727, +559444]ns | [552727, 559444] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_stackcompile | 1219202ns | +1219202.1ns (+0.0%) | [+1210394, +1251191]ns | [1210394, 1251191] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_tight_emitcopypatch | 0.145 | ok |
| carrier_setup_tight_emitdirect | -0.426 | moderate- |
| carrier_setup_tight_optall | -0.541 | HIGH- (thermal bounce) |
| carrier_setup_tight_parse | 0.000 | ok |
| carrier_setup_tight_predecode | -0.567 | HIGH- (thermal bounce) |
| carrier_setup_tight_stackcompile | -0.330 | moderate- |

**Consistency summary:**

- **carrier_setup_tight_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_tight_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_tight_optall**: won 0/6, lost 0/6
- **carrier_setup_tight_predecode**: won 0/6, lost 0/6
- **carrier_setup_tight_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 3895038.9ns | 3897417.4ns | 99.9% | HIGH |
| carrier_setup_tight_emitdirect | 1621641.9ns | 1621457.7ns | 100.0% | HIGH |
| carrier_setup_tight_optall | 12172941.8ns | 12171789.9ns | 100.0% | HIGH |
| carrier_setup_tight_parse | 2765.8ns | 0.0ns | 0.0% |  |
| carrier_setup_tight_predecode | 554591.2ns | 555275.2ns | 99.9% | HIGH |
| carrier_setup_tight_stackcompile | 1227796.2ns | 1226929.0ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_setup_tight_emitcopypatch (n=6, range 3874459.2-3925645.0 ns)
  3874459.2 |####################
  3877018.5 |
  3879577.8 |
  3882137.1 |########################################
  3884696.4 |
  3887255.7 |
  3889814.9 |####################
  3892374.2 |
  3894933.5 |
  3897492.8 |
  3900052.1 |####################
  3902611.4 |
  3905170.7 |
  3907730.0 |
  3910289.3 |
  3912848.5 |
  3915407.8 |
  3917967.1 |
  3920526.4 |
  3923085.7 |
  (0 below, 1 above range)

carrier_setup_tight_emitdirect (n=6, range 1606107.9-1641381.6 ns)
  1606107.9 |########################################
  1607871.6 |########################################
  1609635.3 |
  1611399.0 |
  1613162.6 |########################################
  1614926.3 |
  1616690.0 |########################################
  1618453.7 |
  1620217.4 |
  1621981.1 |
  1623744.8 |
  1625508.5 |
  1627272.1 |
  1629035.8 |
  1630799.5 |
  1632563.2 |
  1634326.9 |
  1636090.6 |
  1637854.3 |
  1639618.0 |########################################
  (0 below, 1 above range)

carrier_setup_tight_optall (n=6, range 12058288.7-12311505.2 ns)
  12058288.7 |########################################
  12070949.5 |####################
  12083610.3 |
  12096271.2 |
  12108932.0 |
  12121592.8 |
  12134253.6 |
  12146914.5 |
  12159575.3 |
  12172236.1 |
  12184896.9 |
  12197557.8 |
  12210218.6 |####################
  12222879.4 |
  12235540.2 |
  12248201.1 |
  12260861.9 |
  12273522.7 |
  12286183.5 |####################
  12298844.4 |
  (0 below, 1 above range)

carrier_setup_tight_predecode (n=6, range 552597.5-559444.2 ns)
  552597.5 |########################################
  552939.8 |
  553282.2 |####################
  553624.5 |
  553966.8 |####################
  554309.2 |
  554651.5 |
  554993.8 |
  555336.2 |
  555678.5 |
  556020.8 |
  556363.2 |
  556705.5 |
  557047.8 |
  557390.2 |
  557732.5 |
  558074.8 |
  558417.2 |
  558759.5 |
  559101.8 |####################
  (0 below, 1 above range)

carrier_setup_tight_stackcompile (n=6, range 1207245.4-1251191.5 ns)
  1207245.4 |########################################
  1209442.7 |
  1211640.0 |########################################
  1213837.3 |
  1216034.6 |########################################
  1218231.9 |
  1220429.2 |########################################
  1222626.5 |
  1224823.8 |
  1227021.1 |
  1229218.4 |
  1231415.7 |
  1233613.0 |
  1235810.3 |
  1238007.6 |
  1240204.9 |########################################
  1242402.2 |
  1244599.5 |
  1246796.8 |
  1248994.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_tight_emitcopypatch**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_tight_emitdirect**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_setup_tight_optall**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_tight_predecode**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_setup_tight_stackcompile**: bridge=100.7% of algo (FFI overhead may distort results)
