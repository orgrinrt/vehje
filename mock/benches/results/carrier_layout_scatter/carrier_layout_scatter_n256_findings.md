# Record layout (REC12..REC32) with fixed switch dispatch, scatter profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_scatter_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_scatter_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.8% of the fastest

All 5 variants sit between 11.13 us and 11.21 us - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_scatter_rec12's edge over baseline is significant but tiny (38 ns, 0.34%)

carrier_lay_scatter_rec12 differs from baseline carrier_lay_scatter_rec24 by 38 ns (0.34%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_scatter_rec20** at 11127.5 ns median (-0.5% vs baseline)
- Spread: 1.01x (fastest 11127.5 ns, slowest 11212.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 13581ns | 13834ns | 12087ns | 13810ns | 13985ns | -1.63% |
| carrier_lay_scatter_rec16 | 13710ns | 13733ns | 13606ns | 13694ns | 13786ns | -0.70% |
| carrier_lay_scatter_rec20 | 13717ns | 13697ns | 13554ns | 13679ns | 13856ns | -0.65% |
| carrier_lay_scatter_rec24 | 13807ns | 13741ns | 13621ns | 13722ns | 14027ns | base |
| carrier_lay_scatter_rec32 | 13745ns | 13786ns | 13495ns | 13754ns | 13858ns | -0.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 11004ns | 9890ns | 11263ns | -1.77% | 0.023 |
| carrier_lay_scatter_rec16 | 11158ns | 11125ns | 11212ns | -0.39% | 0.023 |
| carrier_lay_scatter_rec20 | 11128ns | 11022ns | 11196ns | -0.67% | 0.023 |
| carrier_lay_scatter_rec24 | 11202ns | 11087ns | 11311ns | base | 0.023 |
| carrier_lay_scatter_rec32 | 11189ns | 11003ns | 11299ns | -0.12% | 0.023 |

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_lay_scatter_rec12; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_scatter_rec12 | 0.023 | 88.2% |
| carrier_lay_scatter_rec16 | 0.023 | 88.8% |
| carrier_lay_scatter_rec20 | 0.023 | 88.9% |
| carrier_lay_scatter_rec24 | 0.023 | 88.5% |
| carrier_lay_scatter_rec32 | 0.023 | 88.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_scatter_rec12 | 13581ns | 13581ns | -1.63% |
| carrier_lay_scatter_rec16 | 13710ns | 13710ns | -0.70% |
| carrier_lay_scatter_rec20 | 13717ns | 13717ns | -0.65% |
| carrier_lay_scatter_rec24 | 13807ns | 13807ns | base |
| carrier_lay_scatter_rec32 | 13745ns | 13745ns | -0.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec24 | 11180ns | base | --- | [11116, 11311] | --- | --- | --- | --- |
| carrier_lay_scatter_rec12 | 11213ns | no significant difference | [-775, +142]ns | [10535, 11263] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_scatter_rec16 | 11136ns | no significant difference | [-176, +67]ns | [11127, 11212] | no | 0.8750 | 0.2188 | 0 |
| carrier_lay_scatter_rec20 | 11128ns | no significant difference | [-252, +42]ns | [11060, 11196] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_scatter_rec32 | 11204ns | no significant difference | [-246, +160]ns | [11065, 11299] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_scatter_rec24 | carrier_lay_scatter_rec12 | carrier_lay_scatter_rec16 | carrier_lay_scatter_rec20 | carrier_lay_scatter_rec32 |
|---|---|---|---|---|---|
| 1 | 11418ns | -13.4% | -2.5% | -3.5% | -2.5% |
| 2 | 11146ns | +0.7% | -0.2% | -0.3% | +1.6% |
| 3 | 11087ns | +1.8% | +1.2% | +0.5% | +1.3% |
| 4 | 11203ns | -0.2% | -0.0% | +0.3% | +0.7% |
| 5 | 11156ns | +0.4% | -0.2% | +0.0% | +0.2% |
| 6 | 11205ns | +0.3% | -0.6% | -1.0% | -1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_scatter_rec12 | -0.036 | ok |
| carrier_lay_scatter_rec16 | 0.082 | ok |
| carrier_lay_scatter_rec20 | 0.203 | moderate+ |
| carrier_lay_scatter_rec24 | -0.089 | ok |
| carrier_lay_scatter_rec32 | 0.047 | ok |

**Consistency summary:**

- **carrier_lay_scatter_rec12**: won 2/6, lost 4/6
- **carrier_lay_scatter_rec16**: won 4/6, lost 1/6
- **carrier_lay_scatter_rec20**: won 3/6, lost 2/6
- **carrier_lay_scatter_rec32**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 89932.5ns | 11003.8ns | 817.3% | HIGH |
| carrier_lay_scatter_rec16 | 89431.8ns | 11158.2ns | 801.5% | HIGH |
| carrier_lay_scatter_rec20 | 89361.9ns | 11127.6ns | 803.1% | HIGH |
| carrier_lay_scatter_rec24 | 89601.0ns | 11202.5ns | 799.8% | HIGH |
| carrier_lay_scatter_rec32 | 89693.8ns | 11189.2ns | 801.6% | HIGH |

## Distribution (algo ns)

```
carrier_lay_scatter_rec12 (n=6, range 9890.0-11263.4 ns)
   9890.0 |#############
   9958.7 |
  10027.3 |
  10096.0 |
  10164.7 |
  10233.3 |
  10302.0 |
  10370.7 |
  10439.3 |
  10508.0 |
  10576.7 |
  10645.3 |
  10714.0 |
  10782.7 |
  10851.3 |
  10920.0 |
  10988.7 |
  11057.3 |
  11126.0 |#############
  11194.7 |########################################
  (0 below, 1 above range)

carrier_lay_scatter_rec16 (n=6, range 11125.0-11212.1 ns)
  11125.0 |########################################
  11129.4 |####################
  11133.7 |
  11138.1 |####################
  11142.4 |
  11146.8 |
  11151.1 |
  11155.5 |
  11159.8 |
  11164.2 |
  11168.5 |
  11172.9 |
  11177.3 |
  11181.6 |
  11186.0 |
  11190.3 |
  11194.7 |
  11199.0 |####################
  11203.4 |
  11207.7 |
  (0 below, 1 above range)

carrier_lay_scatter_rec20 (n=6, range 11022.1-11195.9 ns)
  11022.1 |########################################
  11030.8 |
  11039.5 |
  11048.2 |
  11056.9 |
  11065.5 |
  11074.2 |
  11082.9 |
  11091.6 |########################################
  11100.3 |
  11109.0 |########################################
  11117.7 |
  11126.4 |
  11135.0 |########################################
  11143.7 |
  11152.4 |########################################
  11161.1 |
  11169.8 |
  11178.5 |
  11187.2 |
  (0 below, 1 above range)

carrier_lay_scatter_rec24 (n=6, range 11086.7-11311.5 ns)
  11086.7 |####################
  11097.9 |
  11109.2 |
  11120.4 |
  11131.7 |
  11142.9 |####################
  11154.1 |####################
  11165.4 |
  11176.6 |
  11187.8 |
  11199.1 |########################################
  11210.3 |
  11221.6 |
  11232.8 |
  11244.0 |
  11255.3 |
  11266.5 |
  11277.7 |
  11289.0 |
  11300.2 |
  (0 below, 1 above range)

carrier_lay_scatter_rec32 (n=6, range 11003.3-11298.8 ns)
  11003.3 |########################################
  11018.1 |
  11032.8 |
  11047.6 |
  11062.4 |
  11077.2 |
  11091.9 |
  11106.7 |
  11121.5 |########################################
  11136.3 |
  11151.0 |
  11165.8 |########################################
  11180.6 |
  11195.3 |
  11210.1 |
  11224.9 |########################################
  11239.7 |
  11254.4 |
  11269.2 |########################################
  11284.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_scatter_rec12**: bridge=802.0% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec16**: bridge=801.9% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec20**: bridge=803.6% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec24**: bridge=800.1% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec32**: bridge=801.3% of algo (FFI overhead may distort results)
