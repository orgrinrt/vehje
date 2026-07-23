# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), real profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_real_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_real_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_real_emitdirect shows alternating (throttle bounce) (autocorr -0.50)

carrier_setup_real_emitdirect's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_real_parse)

The baseline carrier_setup_real_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_real_parse, carrier_setup_real_predecode, carrier_setup_real_stackcompile, carrier_setup_real_emitdirect, carrier_setup_real_emitcopypatch} vs {carrier_setup_real_optall} (737% apart)

The field splits into a fast tier {carrier_setup_real_parse, carrier_setup_real_predecode, carrier_setup_real_stackcompile, carrier_setup_real_emitdirect, carrier_setup_real_emitcopypatch} and a slow tier {carrier_setup_real_optall} with a 737% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 9.93 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_real_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 1188504ns | 1189334ns | 1184757ns | 1188831ns | 1189888ns | +54328.66% |
| carrier_setup_real_emitdirect | 497171ns | 501171ns | 473288ns | 498036ns | 507816ns | +22668.41% |
| carrier_setup_real_optall | 9949775ns | 9935345ns | 9919746ns | 9932730ns | 9990357ns | +455559.24% |
| carrier_setup_real_parse | 2184ns | 2151ns | 2103ns | 2136ns | 2295ns | base |
| carrier_setup_real_predecode | 150571ns | 149121ns | 145528ns | 148431ns | 156303ns | +6795.55% |
| carrier_setup_real_stackcompile | 337689ns | 337794ns | 325719ns | 335783ns | 346533ns | +15364.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 1185945ns | 1182225ns | 1187309ns | +0.00% | 0.003 |
| carrier_setup_real_emitdirect | 494879ns | 471032ns | 505653ns | +0.00% | 0.008 |
| carrier_setup_real_optall | 9946521ns | 9916589ns | 9986962ns | +0.00% | 0.000 |
| carrier_setup_real_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_real_predecode | 148358ns | 143366ns | 154039ns | +0.00% | 0.028 |
| carrier_setup_real_stackcompile | 335447ns | 323514ns | 344312ns | +0.00% | 0.012 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 7432144 | 22838374 | 0.325 | 520.82× |
| carrier_setup_real_emitdirect | 3111546 | 9149244 | 0.340 | 218.05× |
| carrier_setup_real_optall | 62168165 | 278676008 | 0.223 | 4356.56× |
| carrier_setup_real_parse | 14270 | 52918 | 0.270 | 1.00× |
| carrier_setup_real_predecode | 924079 | 6150722 | 0.150 | 64.76× |
| carrier_setup_real_stackcompile | 2100713 | 10917666 | 0.192 | 147.21× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_setup_real_predecode; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_real_emitcopypatch | 0.003 | 12.1% |
| carrier_setup_real_emitdirect | 0.008 | 28.7% |
| carrier_setup_real_optall | 0.000 | 1.4% |
| carrier_setup_real_predecode | 0.028 | 97.6% |
| carrier_setup_real_stackcompile | 0.012 | 42.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_real_emitcopypatch | 1188504ns | 1188504ns | +54328.66% |
| carrier_setup_real_emitdirect | 497171ns | 497171ns | +22668.41% |
| carrier_setup_real_optall | 9949775ns | 9949775ns | +455559.24% |
| carrier_setup_real_parse | 2184ns | 2184ns | base |
| carrier_setup_real_predecode | 150571ns | 150571ns | +6795.55% |
| carrier_setup_real_stackcompile | 337689ns | 337689ns | +15364.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_real_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_real_emitcopypatch | 1186805ns | +1186804.8ns (+0.0%) | [+1183720, +1187309]ns | [1183720, 1187309] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_emitdirect | 498814ns | +498813.9ns (+0.0%) | [+480169, +505653]ns | [480169, 505653] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_optall | 9932101ns | +9932101.1ns (+0.0%) | [+9920499, +9986962]ns | [9920499, 9986962] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_predecode | 146918ns | +146917.7ns (+0.0%) | [+144118, +154039]ns | [144118, 154039] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_stackcompile | 335540ns | +335540.4ns (+0.0%) | [+326489, +344312]ns | [326489, 344312] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_real_parse | carrier_setup_real_emitcopypatch | carrier_setup_real_emitdirect | carrier_setup_real_optall | carrier_setup_real_predecode | carrier_setup_real_stackcompile |
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
| carrier_setup_real_emitcopypatch | -0.047 | ok |
| carrier_setup_real_emitdirect | -0.501 | HIGH- (thermal bounce) |
| carrier_setup_real_optall | 0.114 | ok |
| carrier_setup_real_parse | 0.000 | ok |
| carrier_setup_real_predecode | -0.321 | moderate- |
| carrier_setup_real_stackcompile | -0.022 | ok |

**Consistency summary:**

- **carrier_setup_real_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_real_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_real_optall**: won 0/6, lost 0/6
- **carrier_setup_real_predecode**: won 0/6, lost 0/6
- **carrier_setup_real_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 1186857.6ns | 1185944.6ns | 100.1% | HIGH |
| carrier_setup_real_emitdirect | 495280.3ns | 494878.8ns | 100.1% | HIGH |
| carrier_setup_real_optall | 9939943.6ns | 9946520.6ns | 99.9% | HIGH |
| carrier_setup_real_parse | 1479.3ns | 0.0ns | 0.0% |  |
| carrier_setup_real_predecode | 148302.1ns | 148358.2ns | 100.0% | HIGH |
| carrier_setup_real_stackcompile | 335670.1ns | 335447.4ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_setup_real_emitcopypatch (n=6, range 1182225.0-1187308.6 ns)
  1182225.0 |####################
  1182479.2 |
  1182733.4 |
  1182987.5 |
  1183241.7 |
  1183495.9 |
  1183750.1 |
  1184004.2 |
  1184258.4 |
  1184512.6 |
  1184766.8 |
  1185021.0 |####################
  1185275.1 |
  1185529.3 |
  1185783.5 |
  1186037.7 |
  1186291.8 |####################
  1186546.0 |
  1186800.2 |
  1187054.4 |########################################
  (0 below, 1 above range)

carrier_setup_real_emitdirect (n=6, range 471031.7-505653.3 ns)
  471031.7 |####################
  472762.8 |
  474493.9 |
  476224.9 |
  477956.0 |
  479687.1 |
  481418.2 |
  483149.3 |
  484880.3 |
  486611.4 |
  488342.5 |####################
  490073.6 |
  491804.7 |
  493535.7 |
  495266.8 |
  496997.9 |####################
  498729.0 |########################################
  500460.1 |
  502191.1 |
  503922.2 |
  (0 below, 1 above range)

carrier_setup_real_optall (n=6, range 9916588.8-9986962.1 ns)
  9916588.8 |####################
  9920107.5 |
  9923626.1 |####################
  9927144.8 |
  9930663.5 |########################################
  9934182.1 |
  9937700.8 |
  9941219.4 |
  9944738.1 |
  9948256.8 |
  9951775.4 |
  9955294.1 |
  9958812.8 |
  9962331.4 |
  9965850.1 |
  9969368.7 |####################
  9972887.4 |
  9976406.1 |
  9979924.7 |
  9983443.4 |
  (0 below, 1 above range)

carrier_setup_real_predecode (n=6, range 143365.8-154038.8 ns)
  143365.8 |########################################
  143899.4 |
  144433.1 |########################################
  144966.7 |
  145500.4 |########################################
  146034.0 |
  146567.7 |
  147101.3 |
  147635.0 |########################################
  148168.6 |
  148702.3 |
  149235.9 |########################################
  149769.6 |
  150303.2 |
  150836.9 |
  151370.5 |
  151904.2 |
  152437.8 |
  152971.5 |
  153505.1 |
  (0 below, 1 above range)

carrier_setup_real_stackcompile (n=6, range 323514.2-344312.5 ns)
  323514.2 |########################################
  324554.1 |
  325594.0 |
  326633.9 |
  327673.9 |
  328713.8 |########################################
  329753.7 |
  330793.6 |
  331833.5 |
  332873.4 |
  333913.3 |########################################
  334953.3 |
  335993.2 |
  337033.1 |########################################
  338073.0 |
  339112.9 |
  340152.8 |
  341192.8 |
  342232.7 |########################################
  343272.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_real_emitcopypatch**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_real_emitdirect**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_setup_real_optall**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_real_predecode**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_real_stackcompile**: bridge=100.0% of algo (FFI overhead may distort results)
