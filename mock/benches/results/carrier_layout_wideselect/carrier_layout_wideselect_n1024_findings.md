# Record layout (REC12..REC32) with fixed switch dispatch, wideselect profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_wideselect_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_wideselect_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_lay_wideselect_rec20, carrier_lay_wideselect_rec16) are a dead heat (<1%)

carrier_lay_wideselect_rec20 (40.06 us) and carrier_lay_wideselect_rec16 (40.06 us) differ by 0.01%, inside the noise, even though the wider field spreads 5.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: carrier_lay_wideselect_rec20** at 40059.3 ns median (-0.6% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 40059.3 ns, slowest 42206.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 44612ns | 43862ns | 43251ns | 43745ns | 46593ns | +4.37% |
| carrier_lay_wideselect_rec16 | 43187ns | 42291ns | 41805ns | 42227ns | 45318ns | +1.04% |
| carrier_lay_wideselect_rec20 | 42775ns | 42384ns | 41749ns | 42212ns | 44131ns | +0.07% |
| carrier_lay_wideselect_rec24 | 42743ns | 42583ns | 41766ns | 42472ns | 43639ns | base |
| carrier_lay_wideselect_rec32 | 44145ns | 44561ns | 41941ns | 43845ns | 45696ns | +3.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 42230ns | 40851ns | 44154ns | +4.57% | 0.024 |
| carrier_lay_wideselect_rec16 | 40823ns | 39358ns | 42864ns | +1.09% | 0.025 |
| carrier_lay_wideselect_rec20 | 40437ns | 39311ns | 41732ns | +0.13% | 0.025 |
| carrier_lay_wideselect_rec24 | 40383ns | 39431ns | 41204ns | base | 0.025 |
| carrier_lay_wideselect_rec32 | 41713ns | 39517ns | 43126ns | +3.29% | 0.025 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 497174 | 2478492 | 0.201 | 0.98× |
| carrier_lay_wideselect_rec16 | 492032 | 2329270 | 0.211 | 0.97× |
| carrier_lay_wideselect_rec20 | 495692 | 2353031 | 0.211 | 0.98× |
| carrier_lay_wideselect_rec24 | 505017 | 2397353 | 0.211 | 1.00× |
| carrier_lay_wideselect_rec32 | 472547 | 2245421 | 0.210 | 0.94× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_lay_wideselect_rec20; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_wideselect_rec12 | 0.025 | 94.6% |
| carrier_lay_wideselect_rec16 | 0.026 | 98.1% |
| carrier_lay_wideselect_rec20 | 0.026 | 98.1% |
| carrier_lay_wideselect_rec24 | 0.025 | 97.6% |
| carrier_lay_wideselect_rec32 | 0.024 | 93.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_wideselect_rec12 | 44612ns | 44612ns | +4.37% |
| carrier_lay_wideselect_rec16 | 43187ns | 43187ns | +1.04% |
| carrier_lay_wideselect_rec20 | 42775ns | 42775ns | +0.07% |
| carrier_lay_wideselect_rec24 | 42743ns | 42743ns | base |
| carrier_lay_wideselect_rec32 | 44145ns | 44145ns | +3.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec24 | 40289ns | base | --- | [39656, 41204] | --- | --- | --- | --- |
| carrier_lay_wideselect_rec12 | 41571ns | +1514.1ns (+3.8%) | [+274, +3753]ns | [40966, 44154] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| carrier_lay_wideselect_rec16 | 40065ns | no significant difference | [-569, +2164]ns | [39540, 42864] | no | 0.4375 | 0.2188 | 0 |
| carrier_lay_wideselect_rec20 | 40059ns | no significant difference | [-1046, +1156]ns | [39519, 41732] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_wideselect_rec32 | 42206ns | no significant difference | [-253, +3037]ns | [39806, 43126] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_wideselect_rec24 | carrier_lay_wideselect_rec12 | carrier_lay_wideselect_rec16 | carrier_lay_wideselect_rec20 | carrier_lay_wideselect_rec32 |
|---|---|---|---|---|---|
| 1 | 41721ns | -0.2% | -0.7% | -3.7% | -0.3% |
| 2 | 39431ns | +3.6% | -0.2% | +0.8% | +0.2% |
| 3 | 39880ns | +4.0% | +11.0% | -1.4% | +7.4% |
| 4 | 40465ns | +1.5% | -0.7% | +0.7% | -0.9% |
| 5 | 40687ns | +6.6% | -1.8% | +4.9% | +5.7% |
| 6 | 40114ns | +12.0% | -1.0% | -0.5% | +7.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_wideselect_rec12 | 0.351 | moderate+ |
| carrier_lay_wideselect_rec16 | -0.393 | moderate- |
| carrier_lay_wideselect_rec20 | 0.024 | ok |
| carrier_lay_wideselect_rec24 | -0.286 | moderate- |
| carrier_lay_wideselect_rec32 | -0.321 | moderate- |

**Consistency summary:**

- **carrier_lay_wideselect_rec12**: won 1/6, lost 5/6
- **carrier_lay_wideselect_rec16**: won 5/6, lost 1/6
- **carrier_lay_wideselect_rec20**: won 3/6, lost 3/6
- **carrier_lay_wideselect_rec32**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 118297.2ns | 42230.0ns | 280.1% | HIGH |
| carrier_lay_wideselect_rec16 | 116974.4ns | 40823.1ns | 286.5% | HIGH |
| carrier_lay_wideselect_rec20 | 117588.1ns | 40436.8ns | 290.8% | HIGH |
| carrier_lay_wideselect_rec24 | 120682.4ns | 40383.0ns | 298.8% | HIGH |
| carrier_lay_wideselect_rec32 | 113305.6ns | 41712.6ns | 271.6% | HIGH |

## Distribution (algo ns)

```
carrier_lay_wideselect_rec12 (n=6, range 40851.2-44153.8 ns)
  40851.2 |########################################
  41016.3 |########################################
  41181.5 |
  41346.6 |########################################
  41511.7 |########################################
  41676.8 |
  41842.0 |
  42007.1 |
  42172.2 |
  42337.3 |
  42502.5 |
  42667.6 |
  42832.7 |
  42997.9 |
  43163.0 |
  43328.1 |########################################
  43493.2 |
  43658.4 |
  43823.5 |
  43988.6 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec16 (n=6, range 39358.3-42864.2 ns)
  39358.3 |########################################
  39533.6 |
  39708.9 |########################################
  39884.2 |########################################
  40059.5 |########################################
  40234.8 |
  40410.1 |
  40585.4 |
  40760.7 |
  40936.0 |
  41111.2 |
  41286.5 |########################################
  41461.8 |
  41637.1 |
  41812.4 |
  41987.7 |
  42163.0 |
  42338.3 |
  42513.6 |
  42688.9 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec20 (n=6, range 39311.2-41731.6 ns)
  39311.2 |########################################
  39432.2 |
  39553.2 |
  39674.3 |########################################
  39795.3 |
  39916.3 |########################################
  40037.3 |
  40158.4 |########################################
  40279.4 |
  40400.4 |
  40521.4 |
  40642.4 |
  40763.5 |########################################
  40884.5 |
  41005.5 |
  41126.5 |
  41247.6 |
  41368.6 |
  41489.6 |
  41610.6 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec24 (n=6, range 39430.8-41203.9 ns)
  39430.8 |########################################
  39519.5 |
  39608.1 |
  39696.8 |
  39785.4 |
  39874.1 |########################################
  39962.7 |
  40051.4 |########################################
  40140.1 |
  40228.7 |
  40317.4 |
  40406.0 |########################################
  40494.7 |
  40583.3 |
  40672.0 |########################################
  40760.7 |
  40849.3 |
  40938.0 |
  41026.6 |
  41115.3 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec32 (n=6, range 39517.1-43126.1 ns)
  39517.1 |########################################
  39697.5 |
  39878.0 |
  40058.4 |########################################
  40238.9 |
  40419.3 |
  40599.8 |
  40780.2 |
  40960.7 |
  41141.1 |
  41321.6 |
  41502.0 |########################################
  41682.5 |
  41862.9 |
  42043.4 |
  42223.8 |
  42404.3 |
  42584.7 |
  42765.2 |########################################
  42945.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_wideselect_rec12**: bridge=292.5% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec16**: bridge=296.9% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec20**: bridge=296.7% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec24**: bridge=299.2% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec32**: bridge=270.1% of algo (FFI overhead may distort results)
