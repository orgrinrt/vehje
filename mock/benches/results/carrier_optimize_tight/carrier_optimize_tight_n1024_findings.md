# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, tight profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_tight_none**

## Highlights

Baseline for all deltas below: **carrier_opt_tight_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_tight_all dominates: 1140% faster than the next best (carrier_opt_tight_fold)

carrier_opt_tight_all (2.08 us) leads carrier_opt_tight_fold (25.76 us) by 1140%, a clear separation rather than a photo finish. CV 10.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_tight_all beats baseline by 94% (significant)

carrier_opt_tight_all is -32.16 us (94%) faster than baseline carrier_opt_tight_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_tight_dce is an outlier: 16.9x slower than the field

carrier_opt_tight_dce (35.02 us) is 16.9x the fastest (2.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_tight_all is fastest but the noisiest (CV 10.2%)

carrier_opt_tight_all wins on median (2.08 us) yet has the highest variance (CV 10.2%), while carrier_opt_tight_dce is the steadiest (CV 2.6%, 35.02 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_opt_tight_none shows alternating (throttle bounce) (autocorr -0.53)

carrier_opt_tight_none's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_tight_all} vs {carrier_opt_tight_fold, carrier_opt_tight_canon, carrier_opt_tight_cse, carrier_opt_tight_none, carrier_opt_tight_dce} (1140% apart)

The field splits into a fast tier {carrier_opt_tight_all} and a slow tier {carrier_opt_tight_fold, carrier_opt_tight_canon, carrier_opt_tight_cse, carrier_opt_tight_none, carrier_opt_tight_dce} with a 1140% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 16.9x the fastest

Fastest carrier_opt_tight_all (2.08 us) to slowest carrier_opt_tight_dce (35.02 us): 16.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_tight_all** at 2077.3 ns median (-93.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 16.86x (fastest 2077.3 ns, slowest 35017.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_tight_all | 4559ns | 4372ns | 4169ns | 4318ns | 5116ns | -87.66% |
| carrier_opt_tight_canon | 35478ns | 35171ns | 33813ns | 34900ns | 37178ns | -3.94% |
| carrier_opt_tight_cse | 36154ns | 36498ns | 34346ns | 35852ns | 37511ns | -2.11% |
| carrier_opt_tight_dce | 36903ns | 37405ns | 35290ns | 36925ns | 37675ns | -0.08% |
| carrier_opt_tight_fold | 28261ns | 28146ns | 27258ns | 27915ns | 29282ns | -23.48% |
| carrier_opt_tight_none | 36932ns | 36878ns | 35005ns | 36388ns | 38711ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_tight_all | 2151ns | 2007ns | 2369ns | -93.76% | 0.476 |
| carrier_opt_tight_canon | 33222ns | 31633ns | 34843ns | -3.61% | 0.031 |
| carrier_opt_tight_cse | 33716ns | 31810ns | 34988ns | -2.18% | 0.030 |
| carrier_opt_tight_dce | 34503ns | 32861ns | 35220ns | +0.10% | 0.030 |
| carrier_opt_tight_fold | 25915ns | 25064ns | 26861ns | -24.81% | 0.040 |
| carrier_opt_tight_none | 34467ns | 32800ns | 36170ns | base | 0.030 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_tight_all | 276461 | 1402221 | 0.197 | 0.65× |
| carrier_opt_tight_canon | 406415 | 2080357 | 0.195 | 0.96× |
| carrier_opt_tight_cse | 413770 | 2125568 | 0.195 | 0.98× |
| carrier_opt_tight_dce | 422455 | 2171970 | 0.195 | 1.00× |
| carrier_opt_tight_fold | 400733 | 2465264 | 0.163 | 0.95× |
| carrier_opt_tight_none | 422457 | 2170398 | 0.195 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.510 Gops/s** (carrier_opt_tight_all; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_tight_all | 0.493 | 96.6% |
| carrier_opt_tight_canon | 0.031 | 6.1% |
| carrier_opt_tight_cse | 0.030 | 5.9% |
| carrier_opt_tight_dce | 0.029 | 5.7% |
| carrier_opt_tight_fold | 0.040 | 7.8% |
| carrier_opt_tight_none | 0.030 | 5.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_tight_all | 4559ns | 4559ns | -87.66% |
| carrier_opt_tight_canon | 35478ns | 35478ns | -3.94% |
| carrier_opt_tight_cse | 36154ns | 36154ns | -2.11% |
| carrier_opt_tight_dce | 36903ns | 36903ns | -0.08% |
| carrier_opt_tight_fold | 28261ns | 28261ns | -23.48% |
| carrier_opt_tight_none | 36932ns | 36932ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_tight_none | 34226ns | base | --- | [33006, 36170] | --- | --- | --- | --- |
| carrier_opt_tight_all | 2077ns | -32159.1ns (-94.0%) | [-33819, -30969]ns | [2008, 2369] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_opt_tight_canon | 32936ns | no significant difference | [-2859, +490]ns | [31887, 34843] | no | 0.3646 | 0.2188 | 0 |
| carrier_opt_tight_cse | 34142ns | no significant difference | [-2795, +1472]ns | [32018, 34988] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_tight_dce | 35017ns | no significant difference | [-974, +1443]ns | [33271, 35220] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_tight_fold | 25757ns | -7923.7ns (-23.2%) | [-10187, -7546]ns | [25126, 26861] | YES (adj: no) | 0.0781 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_tight_none | carrier_opt_tight_all | carrier_opt_tight_canon | carrier_opt_tight_cse | carrier_opt_tight_dce | carrier_opt_tight_fold |
|---|---|---|---|---|---|---|
| 1 | 35905ns | -94.2% | -1.4% | -10.2% | -1.9% | -28.4% |
| 2 | 33212ns | -94.0% | -4.8% | -4.2% | +1.4% | -24.5% |
| 3 | 36434ns | -92.8% | -9.4% | -5.2% | -3.5% | -27.9% |
| 4 | 32800ns | -93.7% | +4.5% | +2.9% | +7.4% | -23.2% |
| 5 | 33295ns | -94.0% | -3.5% | +6.0% | -1.3% | -22.5% |
| 6 | 35156ns | -94.0% | -6.6% | -1.3% | -0.8% | -21.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_tight_all | -0.301 | moderate- |
| carrier_opt_tight_canon | -0.421 | moderate- |
| carrier_opt_tight_cse | 0.297 | moderate+ |
| carrier_opt_tight_dce | -0.488 | moderate- |
| carrier_opt_tight_fold | -0.119 | ok |
| carrier_opt_tight_none | -0.528 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_opt_tight_all**: won 6/6, lost 0/6
- **carrier_opt_tight_canon**: won 5/6, lost 1/6
- **carrier_opt_tight_cse**: won 4/6, lost 2/6
- **carrier_opt_tight_dce**: won 4/6, lost 2/6
- **carrier_opt_tight_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_tight_all | 86531.0ns | 2151.5ns | 4022.0% | HIGH |
| carrier_opt_tight_canon | 99852.4ns | 33222.1ns | 300.6% | HIGH |
| carrier_opt_tight_cse | 101384.5ns | 33715.8ns | 300.7% | HIGH |
| carrier_opt_tight_dce | 103633.6ns | 34502.6ns | 300.4% | HIGH |
| carrier_opt_tight_fold | 103424.7ns | 25914.9ns | 399.1% | HIGH |
| carrier_opt_tight_none | 103396.5ns | 34467.2ns | 300.0% | HIGH |

## Distribution (algo ns)

```
carrier_opt_tight_all (n=6, range 2007.1-2368.8 ns)
   2007.1 |########################################
   2025.2 |
   2043.3 |
   2061.3 |####################
   2079.4 |####################
   2097.5 |
   2115.6 |####################
   2133.7 |
   2151.8 |
   2169.8 |
   2187.9 |
   2206.0 |
   2224.1 |
   2242.2 |
   2260.3 |
   2278.3 |
   2296.4 |
   2314.5 |
   2332.6 |
   2350.7 |
  (0 below, 1 above range)

carrier_opt_tight_canon (n=6, range 31633.3-34843.1 ns)
  31633.3 |########################################
  31793.8 |
  31954.3 |
  32114.8 |########################################
  32275.3 |
  32435.8 |
  32596.2 |
  32756.7 |########################################
  32917.2 |########################################
  33077.7 |
  33238.2 |
  33398.7 |
  33559.2 |
  33719.7 |
  33880.2 |
  34040.7 |
  34201.1 |########################################
  34361.6 |
  34522.1 |
  34682.6 |
  (0 below, 1 above range)

carrier_opt_tight_cse (n=6, range 31809.6-34987.5 ns)
  31809.6 |########################################
  31968.5 |
  32127.4 |########################################
  32286.3 |
  32445.2 |
  32604.1 |
  32763.0 |
  32921.9 |
  33080.8 |
  33239.7 |
  33398.6 |
  33557.4 |
  33716.3 |########################################
  33875.2 |
  34034.1 |
  34193.0 |
  34351.9 |
  34510.8 |########################################
  34669.7 |########################################
  34828.6 |
  (0 below, 1 above range)

carrier_opt_tight_dce (n=6, range 32861.2-35219.6 ns)
  32861.2 |####################
  32979.1 |
  33097.0 |
  33215.0 |
  33332.9 |
  33450.8 |
  33568.7 |####################
  33686.6 |
  33804.6 |
  33922.5 |
  34040.4 |
  34158.3 |
  34276.2 |
  34394.2 |
  34512.1 |
  34630.0 |
  34747.9 |####################
  34865.8 |
  34983.8 |
  35101.7 |########################################
  (0 below, 1 above range)

carrier_opt_tight_fold (n=6, range 25063.7-26861.2 ns)
  25063.7 |########################################
  25153.6 |########################################
  25243.5 |
  25333.3 |
  25423.2 |
  25513.1 |
  25603.0 |
  25692.8 |########################################
  25782.7 |########################################
  25872.6 |
  25962.5 |
  26052.4 |
  26142.2 |
  26232.1 |########################################
  26322.0 |
  26411.9 |
  26501.7 |
  26591.6 |
  26681.5 |
  26771.4 |
  (0 below, 1 above range)

carrier_opt_tight_none (n=6, range 32800.4-36169.8 ns)
  32800.4 |####################
  32968.9 |
  33137.3 |########################################
  33305.8 |
  33474.3 |
  33642.8 |
  33811.2 |
  33979.7 |
  34148.2 |
  34316.6 |
  34485.1 |
  34653.6 |
  34822.0 |
  34990.5 |####################
  35159.0 |
  35327.5 |
  35495.9 |
  35664.4 |
  35832.9 |####################
  36001.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_tight_all**: bridge=4141.6% of algo (FFI overhead may distort results)
- **carrier_opt_tight_canon**: bridge=300.7% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cse**: bridge=300.6% of algo (FFI overhead may distort results)
- **carrier_opt_tight_dce**: bridge=300.6% of algo (FFI overhead may distort results)
- **carrier_opt_tight_fold**: bridge=400.5% of algo (FFI overhead may distort results)
- **carrier_opt_tight_none**: bridge=300.0% of algo (FFI overhead may distort results)
