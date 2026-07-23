# Predecoded dispatch shape, leaf profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_leaf_null beats baseline by 23% (significant)

carrier_pre_leaf_null is -5.78 us (23%) faster than baseline carrier_pre_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_leaf_fntable is an outlier: 2.7x slower than the field

carrier_pre_leaf_fntable (54.82 us) is 2.7x the fastest (19.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_pre_leaf_direct, carrier_pre_leaf_null) are a dead heat (<1%)

carrier_pre_leaf_direct (19.97 us) and carrier_pre_leaf_null (20.05 us) differ by 0.39%, inside the noise, even though the wider field spreads 174.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_switch, carrier_pre_leaf_threaded, carrier_pre_leaf_regcache} vs {carrier_pre_leaf_fntable} (103% apart)

The field splits into a fast tier {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_switch, carrier_pre_leaf_threaded, carrier_pre_leaf_regcache} and a slow tier {carrier_pre_leaf_fntable} with a 103% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_pre_leaf_threaded's edge over baseline is significant but tiny (-12 ns, 0.05%)

carrier_pre_leaf_threaded differs from baseline carrier_pre_leaf_switch by -12 ns (0.05%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_pre_leaf_direct** at 19972.1 ns median (-22.2% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.74x (fastest 19972.1 ns, slowest 54821.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 22165ns | 22295ns | 21142ns | 22269ns | 22522ns | -20.58% |
| carrier_pre_leaf_fntable | 56618ns | 57156ns | 50917ns | 56418ns | 59769ns | +102.88% |
| carrier_pre_leaf_null | 22281ns | 22350ns | 21605ns | 22116ns | 22866ns | -20.16% |
| carrier_pre_leaf_regcache | 29397ns | 29368ns | 27879ns | 29012ns | 30734ns | +5.34% |
| carrier_pre_leaf_switch | 27908ns | 27970ns | 26630ns | 27927ns | 28518ns | base |
| carrier_pre_leaf_threaded | 28040ns | 28035ns | 27682ns | 27995ns | 28286ns | +0.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 19856ns | 18941ns | 20187ns | -22.49% | 0.052 |
| carrier_pre_leaf_fntable | 54337ns | 48803ns | 57462ns | +112.10% | 0.019 |
| carrier_pre_leaf_null | 19976ns | 19393ns | 20471ns | -22.03% | 0.051 |
| carrier_pre_leaf_regcache | 27106ns | 25692ns | 28364ns | +5.81% | 0.038 |
| carrier_pre_leaf_switch | 25619ns | 24434ns | 26179ns | base | 0.040 |
| carrier_pre_leaf_threaded | 25722ns | 25392ns | 25937ns | +0.40% | 0.040 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 363537 | 1394733 | 0.261 | 0.93× |
| carrier_pre_leaf_fntable | 497037 | 1202210 | 0.413 | 1.27× |
| carrier_pre_leaf_null | 365952 | 1665444 | 0.220 | 0.94× |
| carrier_pre_leaf_regcache | 400557 | 1848598 | 0.217 | 1.03× |
| carrier_pre_leaf_switch | 390208 | 1579328 | 0.247 | 1.00× |
| carrier_pre_leaf_threaded | 387067 | 1646896 | 0.235 | 0.99× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.054 Gops/s** (carrier_pre_leaf_direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_leaf_direct | 0.051 | 94.8% |
| carrier_pre_leaf_fntable | 0.019 | 34.6% |
| carrier_pre_leaf_null | 0.051 | 94.5% |
| carrier_pre_leaf_regcache | 0.038 | 70.0% |
| carrier_pre_leaf_switch | 0.040 | 73.8% |
| carrier_pre_leaf_threaded | 0.040 | 73.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_leaf_direct | 22165ns | 22165ns | -20.58% |
| carrier_pre_leaf_fntable | 56618ns | 56618ns | +102.88% |
| carrier_pre_leaf_null | 22281ns | 22281ns | -20.16% |
| carrier_pre_leaf_regcache | 29397ns | 29397ns | +5.34% |
| carrier_pre_leaf_switch | 27908ns | 27908ns | base |
| carrier_pre_leaf_threaded | 28040ns | 28040ns | +0.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_leaf_switch | 25677ns | base | --- | [25000, 26179] | --- | --- | --- | --- |
| carrier_pre_leaf_direct | 19972ns | -5667.5ns (-22.1%) | [-6710, -4910]ns | [19409, 20187] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_fntable | 54821ns | +28642.7ns (+111.6%) | [+25050, +32461]ns | [50727, 57462] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_null | 20049ns | -5781.0ns (-22.5%) | [-6542, -4605]ns | [19407, 20471] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_regcache | 27062ns | +1325.2ns (+5.2%) | [+238, +2900]ns | [25893, 28364] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_threaded | 25728ns | no significant difference | [-582, +905]ns | [25500, 25937] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_leaf_switch | carrier_pre_leaf_direct | carrier_pre_leaf_fntable | carrier_pre_leaf_null | carrier_pre_leaf_regcache | carrier_pre_leaf_threaded |
|---|---|---|---|---|---|---|
| 1 | 25610ns | -21.7% | +90.6% | -24.2% | +5.3% | +0.7% |
| 2 | 24434ns | -17.4% | +130.5% | -17.2% | +14.8% | +6.7% |
| 3 | 26494ns | -28.5% | +110.1% | -25.0% | +8.2% | -2.6% |
| 4 | 25567ns | -22.2% | +129.2% | -19.5% | +0.5% | +0.2% |
| 5 | 25743ns | -22.8% | +104.5% | -20.9% | +1.4% | -0.3% |
| 6 | 25863ns | -21.9% | +108.7% | -25.0% | +5.0% | -1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_leaf_direct | -0.248 | moderate- |
| carrier_pre_leaf_fntable | -0.162 | ok |
| carrier_pre_leaf_null | -0.172 | ok |
| carrier_pre_leaf_regcache | 0.085 | ok |
| carrier_pre_leaf_switch | -0.466 | moderate- |
| carrier_pre_leaf_threaded | 0.228 | moderate+ |

**Consistency summary:**

- **carrier_pre_leaf_direct**: won 6/6, lost 0/6
- **carrier_pre_leaf_fntable**: won 0/6, lost 6/6
- **carrier_pre_leaf_null**: won 6/6, lost 0/6
- **carrier_pre_leaf_regcache**: won 0/6, lost 6/6
- **carrier_pre_leaf_threaded**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 102732.2ns | 19856.0ns | 517.4% | HIGH |
| carrier_pre_leaf_fntable | 112372.6ns | 54336.5ns | 206.8% | HIGH |
| carrier_pre_leaf_null | 102509.9ns | 19975.9ns | 513.2% | HIGH |
| carrier_pre_leaf_regcache | 107079.5ns | 27106.2ns | 395.0% | HIGH |
| carrier_pre_leaf_switch | 105634.0ns | 25618.5ns | 412.3% | HIGH |
| carrier_pre_leaf_threaded | 105787.8ns | 25722.0ns | 411.3% | HIGH |

## Distribution (algo ns)

```
carrier_pre_leaf_direct (n=6, range 18940.8-20187.0 ns)
  18940.8 |####################
  19003.1 |
  19065.4 |
  19127.7 |
  19190.0 |
  19252.4 |
  19314.7 |
  19377.0 |
  19439.3 |
  19501.6 |
  19563.9 |
  19626.2 |
  19688.5 |
  19750.9 |
  19813.2 |
  19875.5 |########################################
  19937.8 |
  20000.1 |####################
  20062.4 |
  20124.7 |####################
  (0 below, 1 above range)

carrier_pre_leaf_fntable (n=6, range 48803.3-57461.6 ns)
  48803.3 |########################################
  49236.2 |
  49669.1 |
  50102.1 |
  50535.0 |
  50967.9 |
  51400.8 |
  51833.7 |
  52266.6 |########################################
  52699.6 |
  53132.5 |
  53565.4 |########################################
  53998.3 |
  54431.2 |
  54864.1 |
  55297.1 |########################################
  55730.0 |
  56162.9 |########################################
  56595.8 |
  57028.7 |
  (0 below, 1 above range)

carrier_pre_leaf_null (n=6, range 19393.3-20471.0 ns)
  19393.3 |########################################
  19447.2 |
  19501.1 |
  19555.0 |
  19608.8 |
  19662.7 |
  19716.6 |
  19770.5 |
  19824.4 |
  19878.3 |####################
  19932.2 |
  19986.1 |
  20040.0 |
  20093.8 |
  20147.7 |
  20201.6 |####################
  20255.5 |
  20309.4 |
  20363.3 |####################
  20417.2 |
  (0 below, 1 above range)

carrier_pre_leaf_regcache (n=6, range 25691.7-28364.0 ns)
  25691.7 |########################################
  25825.3 |
  25958.9 |
  26092.5 |########################################
  26226.2 |
  26359.8 |
  26493.4 |
  26627.0 |
  26760.6 |
  26894.2 |########################################
  27027.8 |########################################
  27161.4 |
  27295.0 |
  27428.7 |
  27562.3 |
  27695.9 |
  27829.5 |
  27963.1 |########################################
  28096.7 |
  28230.3 |
  (0 below, 1 above range)

carrier_pre_leaf_switch (n=6, range 24433.7-26178.5 ns)
  24433.7 |########################################
  24520.9 |
  24608.2 |
  24695.4 |
  24782.7 |
  24869.9 |
  24957.2 |
  25044.4 |
  25131.6 |
  25218.9 |
  25306.1 |
  25393.4 |
  25480.6 |########################################
  25567.9 |########################################
  25655.1 |
  25742.3 |########################################
  25829.6 |########################################
  25916.8 |
  26004.1 |
  26091.3 |
  (0 below, 1 above range)

carrier_pre_leaf_threaded (n=6, range 25392.1-25937.3 ns)
  25392.1 |####################
  25419.4 |
  25446.6 |
  25473.9 |
  25501.1 |
  25528.4 |
  25555.7 |
  25582.9 |####################
  25610.2 |
  25637.4 |
  25664.7 |####################
  25692.0 |
  25719.2 |
  25746.5 |
  25773.7 |########################################
  25801.0 |
  25828.3 |
  25855.5 |
  25882.8 |
  25910.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_leaf_direct**: bridge=517.3% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_fntable**: bridge=207.0% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_null**: bridge=513.2% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_regcache**: bridge=396.8% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_switch**: bridge=414.4% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_threaded**: bridge=411.0% of algo (FFI overhead may distort results)
