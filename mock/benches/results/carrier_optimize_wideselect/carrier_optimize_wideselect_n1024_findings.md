# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, wideselect profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_wideselect_none**

## Highlights

Baseline for all deltas below: **carrier_opt_wideselect_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Speed leader carrier_opt_wideselect_cse vs stability leader carrier_opt_wideselect_all (+2% speed for 7.1x steadier)

carrier_opt_wideselect_cse is fastest (28.96 us, CV 4.1%); carrier_opt_wideselect_all gives up 1.8% median for 7.1x lower variance (CV 0.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_opt_wideselect_cse** at 28956.7 ns median (-12.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.15x (fastest 28956.7 ns, slowest 33335.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 31921ns | 31930ns | 31599ns | 31821ns | 32233ns | -10.14% |
| carrier_opt_wideselect_cse | 31040ns | 31353ns | 28370ns | 31339ns | 31926ns | -12.62% |
| carrier_opt_wideselect_cseeqsat | 32632ns | 32641ns | 31907ns | 32471ns | 33238ns | -8.14% |
| carrier_opt_wideselect_dce | 36191ns | 35741ns | 35518ns | 35678ns | 37298ns | +1.88% |
| carrier_opt_wideselect_eqsat | 33036ns | 32891ns | 32317ns | 32824ns | 33714ns | -7.00% |
| carrier_opt_wideselect_fold | 35530ns | 35675ns | 35000ns | 35457ns | 35905ns | +0.02% |
| carrier_opt_wideselect_none | 35523ns | 35674ns | 34942ns | 35520ns | 35817ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 29454ns | 29240ns | 29631ns | -10.90% | 0.035 |
| carrier_opt_wideselect_cse | 28597ns | 26017ns | 29374ns | -13.49% | 0.036 |
| carrier_opt_wideselect_cseeqsat | 30145ns | 29481ns | 30688ns | -8.81% | 0.034 |
| carrier_opt_wideselect_dce | 33749ns | 33159ns | 34750ns | +2.09% | 0.030 |
| carrier_opt_wideselect_eqsat | 30451ns | 29726ns | 31048ns | -7.88% | 0.034 |
| carrier_opt_wideselect_fold | 33007ns | 32607ns | 33275ns | -0.15% | 0.031 |
| carrier_opt_wideselect_none | 33057ns | 32402ns | 33402ns | base | 0.031 |

## Performance model

- Peak throughput: **0.039 Gops/s** (carrier_opt_wideselect_cse; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_wideselect_all | 0.035 | 88.2% |
| carrier_opt_wideselect_cse | 0.035 | 89.8% |
| carrier_opt_wideselect_cseeqsat | 0.034 | 86.4% |
| carrier_opt_wideselect_dce | 0.031 | 78.0% |
| carrier_opt_wideselect_eqsat | 0.034 | 85.8% |
| carrier_opt_wideselect_fold | 0.031 | 78.6% |
| carrier_opt_wideselect_none | 0.031 | 78.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_wideselect_all | 31921ns | 31921ns | -10.14% |
| carrier_opt_wideselect_cse | 31040ns | 31040ns | -12.62% |
| carrier_opt_wideselect_cseeqsat | 32632ns | 32632ns | -8.14% |
| carrier_opt_wideselect_dce | 36191ns | 36191ns | +1.88% |
| carrier_opt_wideselect_eqsat | 33036ns | 33036ns | -7.00% |
| carrier_opt_wideselect_fold | 35530ns | 35530ns | +0.02% |
| carrier_opt_wideselect_none | 35523ns | 35523ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_wideselect_none | 33153ns | base | --- | [32617, 33402] | --- | --- | --- | --- |
| carrier_opt_wideselect_all | 29485ns | -3549.0ns (-10.7%) | [-4157, -3105]ns | [29246, 29631] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_wideselect_cse | 28957ns | -4356.1ns (-13.1%) | [-5689, -3335]ns | [27461, 29374] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_wideselect_cseeqsat | 30126ns | -2998.1ns (-9.0%) | [-3570, -2168]ns | [29622, 30688] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_wideselect_dce | 33336ns | no significant difference | [-217, +1891]ns | [33162, 34750] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_wideselect_eqsat | 30331ns | -2566.1ns (-7.7%) | [-3178, -2074]ns | [29975, 31048] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_wideselect_fold | 33082ns | no significant difference | [-740, +658]ns | [32662, 33275] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_wideselect_none | carrier_opt_wideselect_all | carrier_opt_wideselect_cse | carrier_opt_wideselect_cseeqsat | carrier_opt_wideselect_dce | carrier_opt_wideselect_eqsat | carrier_opt_wideselect_fold |
|---|---|---|---|---|---|---|---|
| 1 | 32832ns | -9.7% | -20.8% | -10.2% | +2.8% | -6.4% | +1.5% |
| 2 | 33469ns | -12.6% | -13.6% | -10.7% | -0.9% | -6.3% | -2.6% |
| 3 | 33335ns | -12.3% | -13.2% | -10.7% | -0.4% | -9.1% | -1.9% |
| 4 | 33015ns | -10.3% | -10.6% | -6.9% | +8.3% | -10.0% | +0.4% |
| 5 | 32402ns | -9.3% | -9.8% | -6.3% | +3.2% | -6.3% | +2.5% |
| 6 | 33290ns | -11.2% | -13.0% | -7.9% | -0.4% | -9.2% | -0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_wideselect_all | -0.290 | moderate- |
| carrier_opt_wideselect_cse | 0.054 | ok |
| carrier_opt_wideselect_cseeqsat | 0.197 | ok |
| carrier_opt_wideselect_dce | -0.232 | moderate- |
| carrier_opt_wideselect_eqsat | 0.212 | moderate+ |
| carrier_opt_wideselect_fold | -0.067 | ok |
| carrier_opt_wideselect_none | -0.147 | ok |

**Consistency summary:**

- **carrier_opt_wideselect_all**: won 6/6, lost 0/6
- **carrier_opt_wideselect_cse**: won 6/6, lost 0/6
- **carrier_opt_wideselect_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_wideselect_dce**: won 3/6, lost 3/6
- **carrier_opt_wideselect_eqsat**: won 6/6, lost 0/6
- **carrier_opt_wideselect_fold**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 98485.3ns | 29453.7ns | 334.4% | HIGH |
| carrier_opt_wideselect_cse | 96942.9ns | 28597.4ns | 339.0% | HIGH |
| carrier_opt_wideselect_cseeqsat | 91341.9ns | 30145.2ns | 303.0% | HIGH |
| carrier_opt_wideselect_dce | 101499.0ns | 33749.3ns | 300.7% | HIGH |
| carrier_opt_wideselect_eqsat | 92262.5ns | 30451.0ns | 303.0% | HIGH |
| carrier_opt_wideselect_fold | 99217.0ns | 33006.6ns | 300.6% | HIGH |
| carrier_opt_wideselect_none | 99350.2ns | 33057.2ns | 300.5% | HIGH |

## Distribution (algo ns)

```
carrier_opt_wideselect_all (n=6, range 29240.4-29631.0 ns)
  29240.4 |########################################
  29259.9 |
  29279.5 |
  29299.0 |
  29318.5 |
  29338.1 |
  29357.6 |
  29377.1 |####################
  29396.6 |
  29416.2 |
  29435.7 |
  29455.2 |
  29474.8 |
  29494.3 |
  29513.8 |
  29533.3 |
  29552.9 |
  29572.4 |####################
  29591.9 |
  29611.5 |####################
  (0 below, 1 above range)

carrier_opt_wideselect_cse (n=6, range 26017.1-29374.0 ns)
  26017.1 |#############
  26184.9 |
  26352.8 |
  26520.6 |
  26688.5 |
  26856.3 |
  27024.2 |
  27192.0 |
  27359.8 |
  27527.7 |
  27695.5 |
  27863.4 |
  28031.2 |
  28199.1 |
  28366.9 |
  28534.7 |
  28702.6 |
  28870.4 |########################################
  29038.3 |
  29206.1 |#############
  (0 below, 1 above range)

carrier_opt_wideselect_cseeqsat (n=6, range 29480.8-30688.2 ns)
  29480.8 |########################################
  29541.2 |
  29601.5 |
  29661.9 |
  29722.3 |########################################
  29782.6 |
  29843.0 |########################################
  29903.4 |
  29963.7 |
  30024.1 |
  30084.5 |
  30144.8 |
  30205.2 |
  30265.6 |
  30325.9 |########################################
  30386.3 |
  30446.7 |
  30507.0 |
  30567.4 |
  30627.8 |########################################
  (0 below, 1 above range)

carrier_opt_wideselect_dce (n=6, range 33159.2-34749.8 ns)
  33159.2 |########################################
  33238.7 |
  33318.3 |
  33397.8 |#############
  33477.3 |
  33556.8 |
  33636.4 |
  33715.9 |#############
  33795.4 |
  33875.0 |
  33954.5 |
  34034.0 |
  34113.6 |
  34193.1 |
  34272.6 |
  34352.2 |
  34431.7 |
  34511.2 |
  34590.7 |
  34670.3 |
  (0 below, 1 above range)

carrier_opt_wideselect_eqsat (n=6, range 29726.2-31047.5 ns)
  29726.2 |########################################
  29792.3 |
  29858.3 |
  29924.4 |
  29990.5 |
  30056.5 |
  30122.6 |
  30188.7 |########################################
  30254.7 |########################################
  30320.8 |########################################
  30386.8 |
  30452.9 |
  30519.0 |
  30585.0 |
  30651.1 |
  30717.2 |########################################
  30783.2 |
  30849.3 |
  30915.4 |
  30981.4 |
  (0 below, 1 above range)

carrier_opt_wideselect_fold (n=6, range 32607.1-33274.8 ns)
  32607.1 |########################################
  32640.5 |
  32673.9 |
  32707.3 |########################################
  32740.6 |
  32774.0 |
  32807.4 |
  32840.8 |
  32874.2 |
  32907.6 |
  32940.9 |
  32974.3 |########################################
  33007.7 |
  33041.1 |
  33074.5 |
  33107.9 |
  33141.3 |########################################
  33174.6 |
  33208.0 |########################################
  33241.4 |
  (0 below, 1 above range)

carrier_opt_wideselect_none (n=6, range 32401.7-33402.1 ns)
  32401.7 |########################################
  32451.7 |
  32501.7 |
  32551.8 |
  32601.8 |
  32651.8 |
  32701.8 |
  32751.8 |
  32801.9 |########################################
  32851.9 |
  32901.9 |
  32951.9 |
  33001.9 |########################################
  33052.0 |
  33102.0 |
  33152.0 |
  33202.0 |
  33252.0 |########################################
  33302.1 |########################################
  33352.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_wideselect_all**: bridge=334.3% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cse**: bridge=330.4% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cseeqsat**: bridge=303.0% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_dce**: bridge=300.8% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_eqsat**: bridge=304.9% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_fold**: bridge=300.8% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_none**: bridge=300.5% of algo (FFI overhead may distort results)
