# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), wideselect profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_wideselect_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_wideselect_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_wideselect_predecode shows alternating (throttle bounce) (autocorr -0.63)

carrier_setup_wideselect_predecode's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_wideselect_parse)

The baseline carrier_setup_wideselect_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_wideselect_parse, carrier_setup_wideselect_predecode, carrier_setup_wideselect_emitdirect, carrier_setup_wideselect_stackcompile, carrier_setup_wideselect_emitcopypatch} vs {carrier_setup_wideselect_optall} (747% apart)

The field splits into a fast tier {carrier_setup_wideselect_parse, carrier_setup_wideselect_predecode, carrier_setup_wideselect_emitdirect, carrier_setup_wideselect_stackcompile, carrier_setup_wideselect_emitcopypatch} and a slow tier {carrier_setup_wideselect_optall} with a 747% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 10.82 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_wideselect_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 1288041ns | 1280347ns | 1269294ns | 1278541ns | 1311664ns | +58550.39% |
| carrier_setup_wideselect_emitdirect | 385351ns | 378808ns | 373892ns | 377796ns | 402414ns | +17446.81% |
| carrier_setup_wideselect_optall | 10832607ns | 10826120ns | 10687469ns | 10791234ns | 10967236ns | +493158.17% |
| carrier_setup_wideselect_parse | 2196ns | 2146ns | 2124ns | 2139ns | 2318ns | base |
| carrier_setup_wideselect_predecode | 166117ns | 167434ns | 155571ns | 164379ns | 173996ns | +7464.06% |
| carrier_setup_wideselect_stackcompile | 401298ns | 398892ns | 384263ns | 394494ns | 420021ns | +18172.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 1285262ns | 1266760ns | 1308686ns | +0.00% | 0.003 |
| carrier_setup_wideselect_emitdirect | 382999ns | 371587ns | 400144ns | +0.00% | 0.011 |
| carrier_setup_wideselect_optall | 10829135ns | 10684110ns | 10963439ns | +0.00% | 0.000 |
| carrier_setup_wideselect_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_wideselect_predecode | 163939ns | 153400ns | 171824ns | +0.00% | 0.025 |
| carrier_setup_wideselect_stackcompile | 399022ns | 382017ns | 417721ns | +0.00% | 0.010 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 7920699 | 28094050 | 0.282 | 554.10× |
| carrier_setup_wideselect_emitdirect | 2365599 | 10135905 | 0.233 | 165.49× |
| carrier_setup_wideselect_optall | 66962018 | 277129211 | 0.242 | 4684.41× |
| carrier_setup_wideselect_parse | 14295 | 52926 | 0.270 | 1.00× |
| carrier_setup_wideselect_predecode | 1030488 | 6664876 | 0.155 | 72.09× |
| carrier_setup_wideselect_stackcompile | 2469414 | 11566556 | 0.213 | 172.75× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.027 Gops/s** (carrier_setup_wideselect_predecode; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 0.003 | 12.0% |
| carrier_setup_wideselect_emitdirect | 0.011 | 40.7% |
| carrier_setup_wideselect_optall | 0.000 | 1.4% |
| carrier_setup_wideselect_predecode | 0.025 | 92.8% |
| carrier_setup_wideselect_stackcompile | 0.010 | 38.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 1288041ns | 1288041ns | +58550.39% |
| carrier_setup_wideselect_emitdirect | 385351ns | 385351ns | +17446.81% |
| carrier_setup_wideselect_optall | 10832607ns | 10832607ns | +493158.17% |
| carrier_setup_wideselect_parse | 2196ns | 2196ns | base |
| carrier_setup_wideselect_predecode | 166117ns | 166117ns | +7464.06% |
| carrier_setup_wideselect_stackcompile | 401298ns | 401298ns | +18172.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_wideselect_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_wideselect_emitcopypatch | 1277679ns | +1277679.4ns (+0.0%) | [+1269422, +1308686]ns | [1269422, 1308686] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_emitdirect | 376527ns | +376526.7ns (+0.0%) | [+372327, +400144]ns | [372327, 400144] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_optall | 10822842ns | +10822842.3ns (+0.0%) | [+10701124, +10963439]ns | [10701124, 10963439] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_predecode | 165251ns | +165251.2ns (+0.0%) | [+154743, +171824]ns | [154743, 171824] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_wideselect_stackcompile | 396628ns | +396627.5ns (+0.0%) | [+382718, +417721]ns | [382718, 417721] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_wideselect_emitcopypatch | -0.168 | ok |
| carrier_setup_wideselect_emitdirect | -0.275 | moderate- |
| carrier_setup_wideselect_optall | -0.418 | moderate- |
| carrier_setup_wideselect_parse | 0.000 | ok |
| carrier_setup_wideselect_predecode | -0.626 | HIGH- (thermal bounce) |
| carrier_setup_wideselect_stackcompile | -0.478 | moderate- |

**Consistency summary:**

- **carrier_setup_wideselect_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_wideselect_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_wideselect_optall**: won 0/6, lost 0/6
- **carrier_setup_wideselect_predecode**: won 0/6, lost 0/6
- **carrier_setup_wideselect_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_wideselect_emitcopypatch | 1284730.9ns | 1285262.3ns | 100.0% | HIGH |
| carrier_setup_wideselect_emitdirect | 382488.2ns | 382999.2ns | 99.9% | HIGH |
| carrier_setup_wideselect_optall | 10818943.6ns | 10829134.9ns | 99.9% | HIGH |
| carrier_setup_wideselect_parse | 1495.4ns | 0.0ns | 0.0% |  |
| carrier_setup_wideselect_predecode | 164097.7ns | 163939.4ns | 100.1% | HIGH |
| carrier_setup_wideselect_stackcompile | 399106.4ns | 399022.1ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_setup_wideselect_emitcopypatch (n=6, range 1266759.6-1308685.6 ns)
  1266759.6 |########################################
  1268855.9 |
  1270952.2 |########################################
  1273048.5 |
  1275144.8 |########################################
  1277241.1 |########################################
  1279337.4 |
  1281433.7 |
  1283530.0 |
  1285626.3 |
  1287722.6 |
  1289818.9 |
  1291915.2 |
  1294011.5 |########################################
  1296107.8 |
  1298204.1 |
  1300300.4 |
  1302396.7 |
  1304493.0 |
  1306589.3 |
  (0 below, 1 above range)

carrier_setup_wideselect_emitdirect (n=6, range 371587.1-400143.8 ns)
  371587.1 |####################
  373014.9 |####################
  374442.8 |
  375870.6 |########################################
  377298.4 |
  378726.3 |
  380154.1 |
  381581.9 |####################
  383009.8 |
  384437.6 |
  385865.4 |
  387293.3 |
  388721.1 |
  390148.9 |
  391576.8 |
  393004.6 |
  394432.4 |
  395860.3 |
  397288.1 |
  398715.9 |
  (0 below, 1 above range)

carrier_setup_wideselect_optall (n=6, range 10684109.6-10963438.6 ns)
  10684109.6 |########################################
  10698076.0 |
  10712042.5 |########################################
  10726008.9 |
  10739975.4 |########################################
  10753941.8 |
  10767908.3 |
  10781874.7 |
  10795841.2 |
  10809807.6 |
  10823774.1 |
  10837740.5 |
  10851707.0 |
  10865673.4 |
  10879639.9 |
  10893606.3 |########################################
  10907572.8 |
  10921539.2 |########################################
  10935505.7 |
  10949472.1 |
  (0 below, 1 above range)

carrier_setup_wideselect_predecode (n=6, range 153399.6-171824.0 ns)
  153399.6 |####################
  154320.8 |
  155242.0 |####################
  156163.3 |
  157084.5 |
  158005.7 |
  158926.9 |
  159848.1 |
  160769.3 |
  161690.6 |####################
  162611.8 |
  163533.0 |
  164454.2 |
  165375.4 |
  166296.6 |
  167217.9 |
  168139.1 |########################################
  169060.3 |
  169981.5 |
  170902.7 |
  (0 below, 1 above range)

carrier_setup_wideselect_stackcompile (n=6, range 382017.1-417721.0 ns)
  382017.1 |########################################
  383802.3 |
  385587.5 |
  387372.7 |
  389157.9 |
  390943.1 |
  392728.3 |####################
  394513.5 |
  396298.7 |
  398083.9 |####################
  399869.0 |
  401654.2 |####################
  403439.4 |
  405224.6 |
  407009.8 |
  408795.0 |
  410580.2 |
  412365.4 |
  414150.6 |
  415935.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_wideselect_emitcopypatch**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_emitdirect**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_optall**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_predecode**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_setup_wideselect_stackcompile**: bridge=99.7% of algo (FFI overhead may distort results)
