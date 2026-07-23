# Record layout (REC12..REC32) with fixed switch dispatch, wideselect profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_wideselect_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_wideselect_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Speed leader carrier_lay_wideselect_rec12 vs stability leader carrier_lay_wideselect_rec16 (+8% speed for 1.2x steadier)

carrier_lay_wideselect_rec12 is fastest (423.87 us, CV 3.0%); carrier_lay_wideselect_rec16 gives up 8.3% median for 1.2x lower variance (CV 2.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_lay_wideselect_rec12** at 423873.1 ns median (-4.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.08x (fastest 423873.1 ns, slowest 458966.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 426869ns | 426408ns | 409012ns | 422288ns | 442669ns | -5.19% |
| carrier_lay_wideselect_rec16 | 456311ns | 461730ns | 434807ns | 456913ns | 466160ns | +1.35% |
| carrier_lay_wideselect_rec20 | 448621ns | 446013ns | 431138ns | 443467ns | 465093ns | -0.36% |
| carrier_lay_wideselect_rec24 | 450246ns | 448145ns | 432088ns | 444840ns | 467435ns | base |
| carrier_lay_wideselect_rec32 | 445085ns | 447195ns | 426463ns | 442690ns | 457989ns | -1.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 424321ns | 406762ns | 439901ns | -5.18% | 0.010 |
| carrier_lay_wideselect_rec16 | 453441ns | 432158ns | 463101ns | +1.33% | 0.009 |
| carrier_lay_wideselect_rec20 | 446036ns | 428842ns | 462050ns | -0.33% | 0.009 |
| carrier_lay_wideselect_rec24 | 447496ns | 429893ns | 464611ns | base | 0.009 |
| carrier_lay_wideselect_rec32 | 442429ns | 423651ns | 455390ns | -1.13% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 2635791 | 5220479 | 0.505 | 0.94× |
| carrier_lay_wideselect_rec16 | 2838412 | 4828468 | 0.588 | 1.01× |
| carrier_lay_wideselect_rec20 | 2778442 | 4827824 | 0.576 | 0.99× |
| carrier_lay_wideselect_rec24 | 2802234 | 4828201 | 0.580 | 1.00× |
| carrier_lay_wideselect_rec32 | 2750749 | 4828896 | 0.570 | 0.98× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_lay_wideselect_rec12; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_wideselect_rec12 | 0.010 | 96.0% |
| carrier_lay_wideselect_rec16 | 0.009 | 88.6% |
| carrier_lay_wideselect_rec20 | 0.009 | 91.7% |
| carrier_lay_wideselect_rec24 | 0.009 | 91.4% |
| carrier_lay_wideselect_rec32 | 0.009 | 91.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_wideselect_rec12 | 426869ns | 426869ns | -5.19% |
| carrier_lay_wideselect_rec16 | 456311ns | 456311ns | +1.35% |
| carrier_lay_wideselect_rec20 | 448621ns | 448621ns | -0.36% |
| carrier_lay_wideselect_rec24 | 450246ns | 450246ns | base |
| carrier_lay_wideselect_rec32 | 445085ns | 445085ns | -1.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec24 | 444970ns | base | --- | [432906, 464611] | --- | --- | --- | --- |
| carrier_lay_wideselect_rec12 | 423873ns | -29109.8ns (-6.5%) | [-32393, -8020]ns | [409190, 439901] | YES (adj: no) | 0.8750 | 0.2188 | 0 |
| carrier_lay_wideselect_rec16 | 458966ns | no significant difference | [-4767, +19593]ns | [438256, 463101] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_wideselect_rec20 | 443593ns | no significant difference | [-24528, +17555]ns | [432465, 462050] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_wideselect_rec32 | 444484ns | no significant difference | [-14330, +10913]ns | [427414, 455390] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_wideselect_rec24 | carrier_lay_wideselect_rec12 | carrier_lay_wideselect_rec16 | carrier_lay_wideselect_rec20 | carrier_lay_wideselect_rec32 |
|---|---|---|---|---|---|
| 1 | 469011ns | -5.8% | -0.9% | -5.3% | -3.1% |
| 2 | 429893ns | +1.9% | +3.4% | +3.1% | +0.3% |
| 3 | 437619ns | -7.1% | -1.2% | +5.0% | -3.2% |
| 4 | 452321ns | -6.9% | +2.0% | +2.7% | -2.7% |
| 5 | 460212ns | -7.3% | -0.6% | -5.2% | -2.5% |
| 6 | 435920ns | -5.6% | +5.7% | -1.6% | +4.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_wideselect_rec12 | 0.017 | ok |
| carrier_lay_wideselect_rec16 | -0.025 | ok |
| carrier_lay_wideselect_rec20 | 0.217 | moderate+ |
| carrier_lay_wideselect_rec24 | -0.285 | moderate- |
| carrier_lay_wideselect_rec32 | 0.229 | moderate+ |

**Consistency summary:**

- **carrier_lay_wideselect_rec12**: won 5/6, lost 1/6
- **carrier_lay_wideselect_rec16**: won 3/6, lost 3/6
- **carrier_lay_wideselect_rec20**: won 3/6, lost 3/6
- **carrier_lay_wideselect_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 425207.2ns | 424321.5ns | 100.2% | HIGH |
| carrier_lay_wideselect_rec16 | 453791.6ns | 453441.2ns | 100.1% | HIGH |
| carrier_lay_wideselect_rec20 | 446390.1ns | 446035.9ns | 100.1% | HIGH |
| carrier_lay_wideselect_rec24 | 448006.1ns | 447495.9ns | 100.1% | HIGH |
| carrier_lay_wideselect_rec32 | 441994.9ns | 442429.4ns | 99.9% | HIGH |

## Distribution (algo ns)

```
carrier_lay_wideselect_rec12 (n=6, range 406762.1-439901.5 ns)
  406762.1 |########################################
  408419.1 |
  410076.0 |########################################
  411733.0 |
  413390.0 |
  415046.9 |
  416703.9 |
  418360.9 |
  420017.8 |########################################
  421674.8 |
  423331.8 |
  424988.7 |########################################
  426645.7 |
  428302.7 |
  429959.6 |
  431616.6 |
  433273.6 |
  434930.5 |
  436587.5 |########################################
  438244.5 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec16 (n=6, range 432158.3-463101.2 ns)
  432158.3 |####################
  433705.4 |
  435252.6 |
  436799.7 |
  438346.9 |
  439894.0 |
  441441.2 |
  442988.3 |####################
  444535.5 |
  446082.6 |
  447629.8 |
  449176.9 |
  450724.1 |
  452271.2 |
  453818.4 |
  455365.5 |
  456912.7 |####################
  458459.8 |
  460007.0 |########################################
  461554.1 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec20 (n=6, range 428842.1-462049.8 ns)
  428842.1 |########################################
  430502.5 |
  432162.9 |
  433823.2 |
  435483.6 |########################################
  437144.0 |
  438804.4 |
  440464.8 |
  442125.2 |########################################
  443785.5 |########################################
  445445.9 |
  447106.3 |
  448766.7 |
  450427.1 |
  452087.5 |
  453747.8 |
  455408.2 |
  457068.6 |
  458729.0 |########################################
  460389.4 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec24 (n=6, range 429892.9-464611.5 ns)
  429892.9 |########################################
  431628.8 |
  433364.8 |
  435100.7 |########################################
  436836.6 |########################################
  438572.5 |
  440308.5 |
  442044.4 |
  443780.3 |
  445516.2 |
  447252.2 |
  448988.1 |
  450724.0 |########################################
  452460.0 |
  454195.9 |
  455931.8 |
  457667.7 |
  459403.7 |########################################
  461139.6 |
  462875.5 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec32 (n=6, range 423651.2-455390.0 ns)
  423651.2 |########################################
  425238.1 |
  426825.1 |
  428412.0 |
  429999.0 |########################################
  431585.9 |
  433172.8 |
  434759.8 |
  436346.7 |
  437933.6 |
  439520.6 |########################################
  441107.5 |
  442694.5 |
  444281.4 |
  445868.3 |
  447455.3 |########################################
  449042.2 |
  450629.1 |
  452216.1 |
  453803.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_wideselect_rec12**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec16**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec20**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec24**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec32**: bridge=100.0% of algo (FFI overhead may distort results)
