# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), scatter profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_scatter_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_scatter_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_setup_scatter_parse)

The baseline carrier_setup_scatter_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_scatter_parse, carrier_setup_scatter_predecode, carrier_setup_scatter_stackcompile, carrier_setup_scatter_emitdirect, carrier_setup_scatter_emitcopypatch} vs {carrier_setup_scatter_optall} (917% apart)

The field splits into a fast tier {carrier_setup_scatter_parse, carrier_setup_scatter_predecode, carrier_setup_scatter_stackcompile, carrier_setup_scatter_emitdirect, carrier_setup_scatter_emitcopypatch} and a slow tier {carrier_setup_scatter_optall} with a 917% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 12.32 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_scatter_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 1220178ns | 1214042ns | 1205420ns | 1211816ns | 1240099ns | +54708.24% |
| carrier_setup_scatter_emitdirect | 504398ns | 502512ns | 493179ns | 499891ns | 516767ns | +22556.66% |
| carrier_setup_scatter_optall | 12611537ns | 12321067ns | 12225835ns | 12298074ns | 13274583ns | +566388.17% |
| carrier_setup_scatter_parse | 2226ns | 2192ns | 2105ns | 2165ns | 2378ns | base |
| carrier_setup_scatter_predecode | 144388ns | 144402ns | 142812ns | 144090ns | 145621ns | +6385.64% |
| carrier_setup_scatter_stackcompile | 336508ns | 334140ns | 332050ns | 333581ns | 343130ns | +15015.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 1217377ns | 1203056ns | 1236864ns | +0.00% | 0.003 |
| carrier_setup_scatter_emitdirect | 502022ns | 490718ns | 514386ns | +0.00% | 0.008 |
| carrier_setup_scatter_optall | 12607864ns | 12222424ns | 13270406ns | +0.00% | 0.000 |
| carrier_setup_scatter_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_scatter_predecode | 142055ns | 140463ns | 143192ns | +0.00% | 0.029 |
| carrier_setup_scatter_stackcompile | 334208ns | 329817ns | 340902ns | +0.00% | 0.012 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 7569692 | 23447984 | 0.323 | 523.42× |
| carrier_setup_scatter_emitdirect | 3134728 | 9368049 | 0.335 | 216.76× |
| carrier_setup_scatter_optall | 77329030 | 324236257 | 0.238 | 5347.05× |
| carrier_setup_scatter_parse | 14462 | 52953 | 0.273 | 1.00× |
| carrier_setup_scatter_predecode | 891350 | 6265274 | 0.142 | 61.63× |
| carrier_setup_scatter_stackcompile | 2088058 | 11433415 | 0.183 | 144.38× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_setup_scatter_predecode; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_scatter_emitcopypatch | 0.003 | 11.6% |
| carrier_setup_scatter_emitdirect | 0.008 | 28.1% |
| carrier_setup_scatter_optall | 0.000 | 1.1% |
| carrier_setup_scatter_predecode | 0.029 | 98.9% |
| carrier_setup_scatter_stackcompile | 0.012 | 42.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 1220178ns | 1220178ns | +54708.24% |
| carrier_setup_scatter_emitdirect | 504398ns | 504398ns | +22556.66% |
| carrier_setup_scatter_optall | 12611537ns | 12611537ns | +566388.17% |
| carrier_setup_scatter_parse | 2226ns | 2226ns | base |
| carrier_setup_scatter_predecode | 144388ns | 144388ns | +6385.64% |
| carrier_setup_scatter_stackcompile | 336508ns | 336508ns | +15015.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_scatter_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_scatter_emitcopypatch | 1211330ns | +1211330.0ns (+0.0%) | [+1203937, +1236864]ns | [1203937, 1236864] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_emitdirect | 500138ns | +500138.3ns (+0.0%) | [+491540, +514386]ns | [491540, 514386] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_optall | 12317714ns | +12317713.8ns (+0.0%) | [+12235472, +13270406]ns | [12235472, 13270406] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_predecode | 142075ns | +142075.2ns (+0.0%) | [+140897, +143192]ns | [140897, 143192] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_scatter_stackcompile | 331842ns | +331842.2ns (+0.0%) | [+329881, +340902]ns | [329881, 340902] | YES | 0.0313 | 0.0313 | 0 |

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
| carrier_setup_scatter_emitcopypatch | -0.140 | ok |
| carrier_setup_scatter_emitdirect | 0.065 | ok |
| carrier_setup_scatter_optall | -0.053 | ok |
| carrier_setup_scatter_parse | 0.000 | ok |
| carrier_setup_scatter_predecode | 0.112 | ok |
| carrier_setup_scatter_stackcompile | 0.115 | ok |

**Consistency summary:**

- **carrier_setup_scatter_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_scatter_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_scatter_optall**: won 0/6, lost 0/6
- **carrier_setup_scatter_predecode**: won 0/6, lost 0/6
- **carrier_setup_scatter_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_scatter_emitcopypatch | 1221461.8ns | 1217377.0ns | 100.3% | HIGH |
| carrier_setup_scatter_emitdirect | 503178.8ns | 502021.6ns | 100.2% | HIGH |
| carrier_setup_scatter_optall | 12581568.9ns | 12607863.9ns | 99.8% | HIGH |
| carrier_setup_scatter_parse | 1525.5ns | 0.0ns | 0.0% |  |
| carrier_setup_scatter_predecode | 142092.0ns | 142054.9ns | 100.0% | HIGH |
| carrier_setup_scatter_stackcompile | 333975.6ns | 334208.5ns | 99.9% | HIGH |

## Distribution (algo ns)

```
carrier_setup_scatter_emitcopypatch (n=6, range 1203056.2-1236864.1 ns)
  1203056.2 |########################################
  1204746.6 |########################################
  1206437.0 |
  1208127.4 |
  1209817.8 |########################################
  1211508.2 |########################################
  1213198.6 |
  1214889.0 |########################################
  1216579.4 |
  1218269.8 |
  1219960.2 |
  1221650.6 |
  1223341.0 |
  1225031.4 |
  1226721.8 |
  1228412.2 |
  1230102.6 |
  1231793.0 |
  1233483.4 |
  1235173.8 |
  (0 below, 1 above range)

carrier_setup_scatter_emitdirect (n=6, range 490717.5-514386.4 ns)
  490717.5 |########################################
  491900.9 |########################################
  493084.4 |
  494267.8 |
  495451.3 |########################################
  496634.7 |
  497818.2 |
  499001.6 |
  500185.1 |
  501368.5 |
  502552.0 |
  503735.4 |########################################
  504918.9 |
  506102.3 |
  507285.8 |
  508469.2 |
  509652.7 |
  510836.1 |
  512019.6 |
  513203.0 |########################################
  (0 below, 1 above range)

carrier_setup_scatter_optall (n=6, range 12222423.7-13270405.6 ns)
  12222423.7 |########################################
  12274822.8 |####################
  12327221.9 |####################
  12379621.0 |####################
  12432020.1 |
  12484419.2 |
  12536818.3 |
  12589217.4 |
  12641616.5 |
  12694015.6 |
  12746414.7 |
  12798813.8 |
  12851212.9 |
  12903612.0 |
  12956011.1 |
  13008410.2 |
  13060809.3 |
  13113208.4 |
  13165607.5 |
  13218006.6 |
  (0 below, 1 above range)

carrier_setup_scatter_predecode (n=6, range 140463.3-143192.3 ns)
  140463.3 |####################
  140599.8 |
  140736.2 |
  140872.6 |
  141009.1 |
  141145.5 |
  141282.0 |########################################
  141418.4 |
  141554.9 |
  141691.3 |
  141827.8 |
  141964.2 |
  142100.7 |
  142237.1 |
  142373.6 |
  142510.0 |
  142646.5 |####################
  142782.9 |
  142919.4 |
  143055.8 |####################
  (0 below, 1 above range)

carrier_setup_scatter_stackcompile (n=6, range 329817.1-340902.1 ns)
  329817.1 |########################################
  330371.3 |####################
  330925.6 |
  331479.8 |
  332034.1 |
  332588.3 |####################
  333142.6 |
  333696.8 |
  334251.1 |
  334805.3 |
  335359.6 |
  335913.8 |
  336468.1 |
  337022.3 |
  337576.6 |
  338130.8 |
  338685.1 |
  339239.3 |
  339793.6 |
  340347.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_scatter_emitcopypatch**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_emitdirect**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_optall**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_predecode**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_scatter_stackcompile**: bridge=99.9% of algo (FFI overhead may distort results)
