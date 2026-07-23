# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), tight profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_tight_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_tight_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_tight_emitdirect shows alternating (throttle bounce) (autocorr -0.60)

carrier_setup_tight_emitdirect's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_tight_parse)

The baseline carrier_setup_tight_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_tight_parse, carrier_setup_tight_predecode, carrier_setup_tight_emitdirect, carrier_setup_tight_stackcompile, carrier_setup_tight_emitcopypatch} vs {carrier_setup_tight_optall} (269% apart)

The field splits into a fast tier {carrier_setup_tight_parse, carrier_setup_tight_predecode, carrier_setup_tight_emitdirect, carrier_setup_tight_stackcompile, carrier_setup_tight_emitcopypatch} and a slow tier {carrier_setup_tight_optall} with a 269% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 3.23 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_tight_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 881153ns | 879137ns | 874552ns | 878678ns | 888165ns | +40279.10% |
| carrier_setup_tight_emitdirect | 271372ns | 270661ns | 268255ns | 270088ns | 274856ns | +12335.71% |
| carrier_setup_tight_optall | 3235703ns | 3236536ns | 3208988ns | 3233662ns | 3252122ns | +148177.10% |
| carrier_setup_tight_parse | 2182ns | 2149ns | 2101ns | 2135ns | 2294ns | base |
| carrier_setup_tight_predecode | 150809ns | 146100ns | 144303ns | 145965ns | 161328ns | +6810.87% |
| carrier_setup_tight_stackcompile | 333066ns | 333110ns | 326841ns | 331970ns | 337821ns | +15162.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 878595ns | 871954ns | 885707ns | +0.00% | 0.005 |
| carrier_setup_tight_emitdirect | 269106ns | 266059ns | 272470ns | +0.00% | 0.015 |
| carrier_setup_tight_optall | 3232961ns | 3206414ns | 3249233ns | +0.00% | 0.001 |
| carrier_setup_tight_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_tight_predecode | 148496ns | 142123ns | 158806ns | +0.00% | 0.028 |
| carrier_setup_tight_stackcompile | 330794ns | 324630ns | 335509ns | +0.00% | 0.012 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 5473874 | 22966055 | 0.238 | 380.61× |
| carrier_setup_tight_emitdirect | 1680258 | 9564074 | 0.176 | 116.83× |
| carrier_setup_tight_optall | 20077199 | 95200838 | 0.211 | 1396.03× |
| carrier_setup_tight_parse | 14382 | 52910 | 0.272 | 1.00× |
| carrier_setup_tight_predecode | 909722 | 6503796 | 0.140 | 63.26× |
| carrier_setup_tight_stackcompile | 2049833 | 11335817 | 0.181 | 142.53× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_setup_tight_predecode; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_tight_emitcopypatch | 0.005 | 16.2% |
| carrier_setup_tight_emitdirect | 0.015 | 52.9% |
| carrier_setup_tight_optall | 0.001 | 4.4% |
| carrier_setup_tight_predecode | 0.028 | 98.8% |
| carrier_setup_tight_stackcompile | 0.012 | 43.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 881153ns | 881153ns | +40279.10% |
| carrier_setup_tight_emitdirect | 271372ns | 271372ns | +12335.71% |
| carrier_setup_tight_optall | 3235703ns | 3235703ns | +148177.10% |
| carrier_setup_tight_parse | 2182ns | 2182ns | base |
| carrier_setup_tight_predecode | 150809ns | 150809ns | +6810.87% |
| carrier_setup_tight_stackcompile | 333066ns | 333066ns | +15162.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_tight_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_tight_emitcopypatch | 876540ns | +876539.6ns (+0.0%) | [+873539, +885707]ns | [873539, 885707] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_emitdirect | 268459ns | +268459.3ns (+0.0%) | [+266388, +272470]ns | [266388, 272470] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_optall | 3233832ns | +3233831.9ns (+0.0%) | [+3215819, +3249233]ns | [3215819, 3249233] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_predecode | 143901ns | +143901.5ns (+0.0%) | [+142780, +158806]ns | [142780, 158806] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_tight_stackcompile | 330803ns | +330802.9ns (+0.0%) | [+326071, +335509]ns | [326071, 335509] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_tight_emitcopypatch | -0.320 | moderate- |
| carrier_setup_tight_emitdirect | -0.598 | HIGH- (thermal bounce) |
| carrier_setup_tight_optall | -0.244 | moderate- |
| carrier_setup_tight_parse | 0.000 | ok |
| carrier_setup_tight_predecode | 0.018 | ok |
| carrier_setup_tight_stackcompile | 0.427 | moderate+ |

**Consistency summary:**

- **carrier_setup_tight_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_tight_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_tight_optall**: won 0/6, lost 0/6
- **carrier_setup_tight_predecode**: won 0/6, lost 0/6
- **carrier_setup_tight_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_tight_emitcopypatch | 878510.9ns | 878595.3ns | 100.0% | HIGH |
| carrier_setup_tight_emitdirect | 269489.2ns | 269105.8ns | 100.1% | HIGH |
| carrier_setup_tight_optall | 3230636.6ns | 3232961.2ns | 99.9% | HIGH |
| carrier_setup_tight_parse | 1508.5ns | 0.0ns | 0.0% |  |
| carrier_setup_tight_predecode | 148922.8ns | 148496.0ns | 100.3% | HIGH |
| carrier_setup_tight_stackcompile | 326683.4ns | 330794.2ns | 98.8% | HIGH |

## Distribution (algo ns)

```
carrier_setup_tight_emitcopypatch (n=6, range 871953.7-885707.2 ns)
  871953.7 |####################
  872641.4 |
  873329.1 |
  874016.7 |
  874704.4 |########################################
  875392.1 |
  876079.8 |
  876767.4 |
  877455.1 |####################
  878142.8 |
  878830.5 |
  879518.2 |
  880205.8 |
  880893.5 |
  881581.2 |
  882268.9 |
  882956.5 |
  883644.2 |
  884331.9 |
  885019.6 |####################
  (0 below, 1 above range)

carrier_setup_tight_emitdirect (n=6, range 266058.8-272470.4 ns)
  266058.8 |########################################
  266379.4 |
  266700.0 |########################################
  267020.5 |########################################
  267341.1 |
  267661.7 |
  267982.3 |
  268302.9 |
  268623.4 |
  268944.0 |
  269264.6 |
  269585.2 |########################################
  269905.8 |
  270226.3 |
  270546.9 |
  270867.5 |
  271188.1 |
  271508.7 |########################################
  271829.2 |
  272149.8 |
  (0 below, 1 above range)

carrier_setup_tight_optall (n=6, range 3206414.2-3249232.9 ns)
  3206414.2 |####################
  3208555.1 |
  3210696.1 |
  3212837.0 |
  3214977.9 |
  3217118.9 |
  3219259.8 |
  3221400.7 |
  3223541.7 |########################################
  3225682.6 |
  3227823.5 |
  3229964.5 |
  3232105.4 |
  3234246.4 |
  3236387.3 |
  3238528.2 |
  3240669.2 |####################
  3242810.1 |####################
  3244951.0 |
  3247092.0 |
  (0 below, 1 above range)

carrier_setup_tight_predecode (n=6, range 142122.9-158806.5 ns)
  142122.9 |####################
  142957.1 |####################
  143791.3 |########################################
  144625.4 |####################
  145459.6 |
  146293.8 |
  147128.0 |
  147962.1 |
  148796.3 |
  149630.5 |
  150464.7 |
  151298.9 |
  152133.0 |
  152967.2 |
  153801.4 |
  154635.6 |
  155469.7 |
  156303.9 |
  157138.1 |
  157972.3 |
  (0 below, 1 above range)

carrier_setup_tight_stackcompile (n=6, range 324629.6-335509.2 ns)
  324629.6 |########################################
  325173.6 |
  325717.6 |
  326261.5 |
  326805.5 |
  327349.5 |########################################
  327893.5 |
  328437.5 |
  328981.4 |
  329525.4 |
  330069.4 |########################################
  330613.4 |
  331157.4 |########################################
  331701.3 |
  332245.3 |
  332789.3 |
  333333.3 |
  333877.3 |
  334421.2 |
  334965.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_tight_emitcopypatch**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_tight_emitdirect**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_tight_optall**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_tight_predecode**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_setup_tight_stackcompile**: bridge=98.7% of algo (FFI overhead may distort results)
