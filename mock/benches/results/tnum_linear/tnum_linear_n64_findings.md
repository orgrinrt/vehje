# tnum linear transfers: add/and/or/shl vs concrete u64 (abstract arith)

5 variants, 6 samples per variant.
Baseline: **tl_concrete**

## Highlights

Baseline for all deltas below: **tl_concrete**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### tl_shl dominates: 97% faster than the next best (tl_concrete)

tl_shl (413 ns) leads tl_concrete (814 ns) by 97%, a clear separation rather than a photo finish. CV 8.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### tl_shl beats baseline by 51% (significant)

tl_shl is -414 ns (51%) faster than baseline tl_concrete, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### tl_and is an outlier: 2.5x slower than the field

tl_and (1.02 us) is 2.5x the fastest (413 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### tl_shl is fastest but the noisiest (CV 8.7%)

tl_shl wins on median (413 ns) yet has the highest variance (CV 8.7%), while tl_and is the steadiest (CV 7.6%, 1.02 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### tl_and shows alternating (throttle bounce) (autocorr -0.57)

tl_and's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {tl_shl} vs {tl_concrete, tl_or, tl_add, tl_and} (97% apart)

The field splits into a fast tier {tl_shl} and a slow tier {tl_concrete, tl_or, tl_add, tl_and} with a 97% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### tl_or's comparison is tie-heavy (17% tied pairs)

17% of paired samples for tl_or are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

## Key findings

- **Fastest: tl_shl** at 413.1 ns median (-49.3% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.46x (fastest 413.1 ns, slowest 1016.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| tl_add | 3486ns | 3595ns | 3110ns | 3434ns | 3752ns | +2.26% |
| tl_and | 3578ns | 3620ns | 3022ns | 3616ns | 3798ns | +4.94% |
| tl_concrete | 3409ns | 3420ns | 2846ns | 3414ns | 3684ns | base |
| tl_or | 3213ns | 3391ns | 2844ns | 3211ns | 3400ns | -5.76% |
| tl_shl | 2964ns | 2987ns | 2513ns | 2922ns | 3254ns | -13.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| tl_add | 973ns | 867ns | 1046ns | +19.63% | 0.066 |
| tl_and | 1009ns | 855ns | 1076ns | +24.05% | 0.063 |
| tl_concrete | 813ns | 678ns | 882ns | base | 0.079 |
| tl_or | 773ns | 682ns | 820ns | -4.89% | 0.083 |
| tl_shl | 410ns | 348ns | 450ns | -49.63% | 0.156 |

## Performance model

- Peak throughput: **0.184 Gops/s** (tl_shl; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| tl_add | 0.064 | 34.6% |
| tl_and | 0.063 | 34.2% |
| tl_concrete | 0.079 | 42.7% |
| tl_or | 0.078 | 42.5% |
| tl_shl | 0.155 | 84.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| tl_add | 3486ns | 3486ns | +2.26% |
| tl_and | 3578ns | 3578ns | +4.94% |
| tl_concrete | 3409ns | 3409ns | base |
| tl_or | 3213ns | 3213ns | -5.76% |
| tl_shl | 2964ns | 2964ns | -13.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| tl_concrete | 814ns | base | --- | [744, 882] | --- | --- | --- | --- |
| tl_add | 1005ns | +188.9ns (+23.2%) | [+89, +201]ns | [867, 1046] | YES | 0.0417 | 0.0313 | 0 |
| tl_and | 1016ns | +188.8ns (+23.2%) | [+133, +265]ns | [934, 1076] | YES | 0.0417 | 0.0313 | 0 |
| tl_or | 817ns | no significant difference | [-95, +7]ns | [684, 820] | no | 1.0000 | 1.0000 | **1** (17%, HIGH) |
| tl_shl | 413ns | -413.9ns (-50.8%) | [-450, -346]ns | [366, 450] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | tl_concrete | tl_add | tl_and | tl_or | tl_shl |
|---|---|---|---|---|---|
| 1 | 812ns | +6.8% | +37.0% | -15.6% | -44.6% |
| 2 | 678ns | +28.0% | +26.1% | +0.7% | -48.7% |
| 3 | 809ns | +24.4% | +28.4% | +1.1% | -49.0% |
| 4 | 817ns | +23.1% | +24.6% | +0.0% | -52.9% |
| 5 | 882ns | +13.8% | +14.9% | -7.0% | -53.1% |
| 6 | 881ns | +23.3% | +15.2% | -7.3% | -49.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| tl_add | 0.346 | moderate+ |
| tl_and | -0.571 | HIGH- (thermal bounce) |
| tl_concrete | 0.204 | moderate+ |
| tl_or | 0.415 | moderate+ |
| tl_shl | -0.347 | moderate- |

**Consistency summary:**

- **tl_add**: won 0/6, lost 6/6
- **tl_and**: won 0/6, lost 6/6
- **tl_or**: won 3/6, lost 2/6
- **tl_shl**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| tl_add | 4.5ns | 972.7ns | 0.5% |  |
| tl_and | 3.9ns | 1008.7ns | 0.4% |  |
| tl_concrete | 4.4ns | 813.1ns | 0.5% |  |
| tl_or | 2.9ns | 773.3ns | 0.4% |  |
| tl_shl | 3.9ns | 409.6ns | 0.9% |  |

## Distribution (algo ns)

```
tl_add (n=6, range 867.1-1046.2 ns)
    867.1 |##########################
    876.1 |
    885.0 |
    894.0 |
    902.9 |
    911.9 |
    920.8 |
    929.8 |
    938.8 |
    947.7 |
    956.7 |
    965.6 |
    974.6 |
    983.5 |
    992.5 |
   1001.5 |########################################
   1010.4 |
   1019.4 |
   1028.3 |
   1037.3 |
  (0 below, 1 above range)

tl_and (n=6, range 854.6-1075.7 ns)
    854.6 |#############
    865.7 |
    876.7 |
    887.8 |
    898.8 |
    909.9 |
    920.9 |
    932.0 |
    943.0 |
    954.1 |
    965.1 |
    976.2 |
    987.2 |
    998.3 |
   1009.3 |########################################
   1020.4 |
   1031.4 |#############
   1042.5 |
   1053.5 |
   1064.6 |
  (0 below, 1 above range)

tl_concrete (n=6, range 677.9-881.6 ns)
    677.9 |####################
    688.1 |
    698.3 |
    708.5 |
    718.6 |
    728.8 |
    739.0 |
    749.2 |
    759.4 |
    769.6 |
    779.8 |
    790.0 |
    800.1 |####################
    810.3 |########################################
    820.5 |
    830.7 |
    840.9 |
    851.1 |
    861.3 |
    871.5 |####################
  (0 below, 1 above range)

tl_or (n=6, range 682.5-819.5 ns)
    682.5 |##########################
    689.4 |
    696.2 |
    703.1 |
    709.9 |
    716.8 |
    723.6 |
    730.5 |
    737.3 |
    744.2 |
    751.0 |
    757.9 |
    764.7 |
    771.6 |
    778.4 |
    785.3 |
    792.1 |
    799.0 |
    805.8 |
    812.7 |########################################
  (0 below, 1 above range)

tl_shl (n=6, range 347.5-449.6 ns)
    347.5 |####################
    352.6 |
    357.7 |
    362.8 |
    367.9 |
    373.0 |
    378.1 |
    383.2 |####################
    388.3 |
    393.4 |
    398.6 |
    403.7 |
    408.8 |########################################
    413.9 |
    419.0 |
    424.1 |
    429.2 |
    434.3 |
    439.4 |
    444.5 |
  (0 below, 2 above range)

```
