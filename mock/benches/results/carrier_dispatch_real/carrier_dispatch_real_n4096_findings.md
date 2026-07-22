# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_real**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_real**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_nullfloor_real dominates: 121% faster than the next best (carrier_disp_bittree_real)

carrier_disp_nullfloor_real (116.69 us) leads carrier_disp_bittree_real (257.52 us) by 121%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_nullfloor_real beats baseline by 80% (significant)

carrier_disp_nullfloor_real is -473.95 us (80%) faster than baseline carrier_disp_switch_real, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_threaded_real is an outlier: 5.7x slower than the field

carrier_disp_threaded_real (662.02 us) is 5.7x the fastest (116.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_ifchain_real shows alternating (throttle bounce) (autocorr -0.52)

carrier_disp_ifchain_real's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_nullfloor_real} vs {carrier_disp_bittree_real, carrier_disp_ifchainlin_real, carrier_disp_fntable_real, carrier_disp_ifchainasc_real, carrier_disp_switch_real, carrier_disp_ifchain_real, carrier_disp_threaded_real} (121% apart)

The field splits into a fast tier {carrier_disp_nullfloor_real} and a slow tier {carrier_disp_bittree_real, carrier_disp_ifchainlin_real, carrier_disp_fntable_real, carrier_disp_ifchainasc_real, carrier_disp_switch_real, carrier_disp_ifchain_real, carrier_disp_threaded_real} with a 121% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.7x the fastest

Fastest carrier_disp_nullfloor_real (116.69 us) to slowest carrier_disp_threaded_real (662.02 us): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_nullfloor_real** at 116692.7 ns median (-80.2% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.67x (fastest 116692.7 ns, slowest 662021.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_bittree_real | 257560ns | 260000ns | 225051ns | 257716ns | 273581ns | -56.37% |
| carrier_disp_fntable_real | 539161ns | 541168ns | 506390ns | 538574ns | 556428ns | -8.67% |
| carrier_disp_ifchain_real | 600828ns | 607245ns | 573991ns | 597984ns | 618512ns | +1.77% |
| carrier_disp_ifchainasc_real | 591958ns | 588260ns | 570429ns | 582987ns | 616178ns | +0.27% |
| carrier_disp_ifchainlin_real | 513201ns | 502354ns | 455100ns | 487884ns | 580228ns | -13.07% |
| carrier_disp_nullfloor_real | 119762ns | 119170ns | 114051ns | 117920ns | 125381ns | -79.71% |
| carrier_disp_switch_real | 590360ns | 592847ns | 559882ns | 587204ns | 610334ns | base |
| carrier_disp_threaded_real | 661550ns | 664781ns | 626950ns | 659709ns | 681610ns | +12.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_bittree_real | 255073ns | 222746ns | 270988ns | -56.59% | 0.016 |
| carrier_disp_fntable_real | 536228ns | 503973ns | 553437ns | -8.75% | 0.008 |
| carrier_disp_ifchain_real | 597952ns | 570755ns | 615797ns | +1.75% | 0.007 |
| carrier_disp_ifchainasc_real | 589345ns | 567328ns | 613816ns | +0.29% | 0.007 |
| carrier_disp_ifchainlin_real | 510561ns | 452722ns | 577270ns | -13.12% | 0.008 |
| carrier_disp_nullfloor_real | 117340ns | 111884ns | 122818ns | -80.03% | 0.035 |
| carrier_disp_switch_real | 587644ns | 557608ns | 607595ns | base | 0.007 |
| carrier_disp_threaded_real | 658780ns | 624405ns | 678774ns | +12.11% | 0.006 |

## Performance model

- Peak throughput: **0.037 Gops/s** (carrier_disp_nullfloor_real; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_bittree_real | 0.016 | 43.4% |
| carrier_disp_fntable_real | 0.008 | 20.8% |
| carrier_disp_ifchain_real | 0.007 | 18.5% |
| carrier_disp_ifchainasc_real | 0.007 | 19.1% |
| carrier_disp_ifchainlin_real | 0.008 | 22.4% |
| carrier_disp_nullfloor_real | 0.035 | 95.9% |
| carrier_disp_switch_real | 0.007 | 19.0% |
| carrier_disp_threaded_real | 0.006 | 16.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_bittree_real | 257560ns | 257560ns | -56.37% |
| carrier_disp_fntable_real | 539161ns | 539161ns | -8.67% |
| carrier_disp_ifchain_real | 600828ns | 600828ns | +1.77% |
| carrier_disp_ifchainasc_real | 591958ns | 591958ns | +0.27% |
| carrier_disp_ifchainlin_real | 513201ns | 513201ns | -13.07% |
| carrier_disp_nullfloor_real | 119762ns | 119762ns | -79.71% |
| carrier_disp_switch_real | 590360ns | 590360ns | base |
| carrier_disp_threaded_real | 661550ns | 661550ns | +12.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_real | 590009ns | base | --- | [565327, 607595] | --- | --- | --- | --- |
| carrier_disp_bittree_real | 257518ns | -328338.2ns (-55.6%) | [-360567, -308806]ns | [236714, 270988] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_fntable_real | 537901ns | -60376.2ns (-10.2%) | [-81979, -11891]ns | [517348, 553437] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_ifchain_real | 604554ns | no significant difference | [-22389, +43554]ns | [573506, 615797] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_ifchainasc_real | 585530ns | no significant difference | [-34285, +30523]ns | [568689, 613816] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_ifchainlin_real | 499721ns | -99396.9ns (-16.8%) | [-129310, -2542]ns | [454691, 577270] | YES (adj: no) | 0.3063 | 0.2188 | 0 |
| carrier_disp_nullfloor_real | 116693ns | -473949.3ns (-80.3%) | [-488688, -448274]ns | [112510, 122818] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_threaded_real | 662022ns | +68841.0ns (+11.7%) | [+52066, +92502]ns | [635544, 678774] | YES (adj: no) | 0.0547 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_real | carrier_disp_bittree_real | carrier_disp_fntable_real | carrier_disp_ifchain_real | carrier_disp_ifchainasc_real | carrier_disp_ifchainlin_real | carrier_disp_nullfloor_real | carrier_disp_threaded_real |
|---|---|---|---|---|---|---|---|---|
| 1 | 609978ns | -63.5% | -11.6% | +1.2% | -5.5% | -25.8% | -79.6% | +11.6% |
| 2 | 605212ns | -54.7% | -12.3% | -5.7% | -5.8% | -16.2% | -81.3% | +6.9% |
| 3 | 573047ns | -56.3% | -3.5% | +5.3% | -1.0% | +10.6% | -80.3% | +17.8% |
| 4 | 586577ns | -56.9% | -8.5% | -1.8% | +4.0% | -11.3% | -80.9% | +10.7% |
| 5 | 557608ns | -52.9% | -0.7% | +10.2% | +6.6% | -18.1% | -78.3% | +12.0% |
| 6 | 593440ns | -54.8% | -15.1% | +2.1% | +4.1% | -17.1% | -79.7% | +14.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_bittree_real | -0.367 | moderate- |
| carrier_disp_fntable_real | -0.397 | moderate- |
| carrier_disp_ifchain_real | -0.518 | HIGH- (thermal bounce) |
| carrier_disp_ifchainasc_real | 0.206 | moderate+ |
| carrier_disp_ifchainlin_real | 0.067 | ok |
| carrier_disp_nullfloor_real | 0.004 | ok |
| carrier_disp_switch_real | 0.005 | ok |
| carrier_disp_threaded_real | -0.363 | moderate- |

**Consistency summary:**

- **carrier_disp_bittree_real**: won 6/6, lost 0/6
- **carrier_disp_fntable_real**: won 6/6, lost 0/6
- **carrier_disp_ifchain_real**: won 2/6, lost 4/6
- **carrier_disp_ifchainasc_real**: won 3/6, lost 3/6
- **carrier_disp_ifchainlin_real**: won 5/6, lost 1/6
- **carrier_disp_nullfloor_real**: won 6/6, lost 0/6
- **carrier_disp_threaded_real**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_bittree_real | 451.7ns | 255073.4ns | 0.2% |  |
| carrier_disp_fntable_real | 483.5ns | 536228.4ns | 0.1% |  |
| carrier_disp_ifchain_real | 457.1ns | 597952.3ns | 0.1% |  |
| carrier_disp_ifchainasc_real | 447.0ns | 589344.9ns | 0.1% |  |
| carrier_disp_ifchainlin_real | 478.1ns | 510560.5ns | 0.1% |  |
| carrier_disp_nullfloor_real | 447.1ns | 117340.1ns | 0.4% |  |
| carrier_disp_switch_real | 452.4ns | 587643.6ns | 0.1% |  |
| carrier_disp_threaded_real | 444.1ns | 658779.9ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_disp_bittree_real (n=6, range 222746.2-270988.1 ns)
  222746.2 |########################################
  225158.3 |
  227570.4 |
  229982.5 |
  232394.6 |
  234806.7 |
  237218.8 |
  239630.9 |
  242043.0 |
  244455.1 |
  246867.1 |
  249279.2 |########################################
  251691.3 |########################################
  254103.4 |
  256515.5 |
  258927.6 |
  261339.7 |########################################
  263751.8 |
  266163.9 |########################################
  268576.0 |
  (0 below, 1 above range)

carrier_disp_fntable_real (n=6, range 503972.9-553436.7 ns)
  503972.9 |########################################
  506446.1 |
  508919.3 |
  511392.5 |
  513865.7 |
  516338.8 |
  518812.0 |
  521285.2 |
  523758.4 |
  526231.6 |
  528704.8 |########################################
  531178.0 |
  533651.2 |
  536124.3 |########################################
  538597.5 |########################################
  541070.7 |
  543543.9 |
  546017.1 |
  548490.3 |
  550963.5 |########################################
  (0 below, 1 above range)

carrier_disp_ifchain_real (n=6, range 570755.4-615797.1 ns)
  570755.4 |########################################
  573007.5 |
  575259.6 |########################################
  577511.7 |
  579763.7 |
  582015.8 |
  584267.9 |
  586520.0 |
  588772.1 |
  591024.2 |
  593276.2 |
  595528.3 |
  597780.4 |
  600032.5 |
  602284.6 |########################################
  604536.7 |########################################
  606788.8 |
  609040.8 |
  611292.9 |
  613545.0 |########################################
  (0 below, 1 above range)

carrier_disp_ifchainasc_real (n=6, range 567327.9-613815.6 ns)
  567327.9 |########################################
  569652.3 |########################################
  571976.7 |
  574301.1 |########################################
  576625.4 |
  578949.8 |
  581274.2 |
  583598.6 |
  585923.0 |
  588247.4 |
  590571.8 |
  592896.2 |########################################
  595220.5 |
  597544.9 |
  599869.3 |
  602193.7 |
  604518.1 |
  606842.5 |
  609166.9 |########################################
  611491.3 |
  (0 below, 1 above range)

carrier_disp_ifchainlin_real (n=6, range 452721.7-577269.6 ns)
  452721.7 |########################################
  458949.1 |
  465176.5 |
  471403.9 |
  477631.3 |
  483858.7 |
  490086.1 |####################
  496313.4 |
  502540.8 |####################
  508768.2 |
  514995.6 |####################
  521223.0 |
  527450.4 |
  533677.8 |
  539905.2 |
  546132.6 |
  552360.0 |
  558587.4 |
  564814.8 |
  571042.2 |
  (0 below, 1 above range)

carrier_disp_nullfloor_real (n=6, range 111883.8-122817.7 ns)
  111883.8 |####################
  112430.5 |
  112977.2 |########################################
  113523.9 |
  114070.6 |
  114617.3 |
  115164.0 |
  115710.7 |
  116257.4 |
  116804.1 |
  117350.8 |
  117897.4 |
  118444.1 |
  118990.8 |
  119537.5 |
  120084.2 |####################
  120630.9 |####################
  121177.6 |
  121724.3 |
  122271.0 |
  (0 below, 1 above range)

carrier_disp_switch_real (n=6, range 557607.5-607594.8 ns)
  557607.5 |########################################
  560106.9 |
  562606.2 |
  565105.6 |
  567605.0 |
  570104.3 |
  572603.7 |########################################
  575103.1 |
  577602.4 |
  580101.8 |
  582601.2 |
  585100.5 |########################################
  587599.9 |
  590099.2 |
  592598.6 |########################################
  595098.0 |
  597597.3 |
  600096.7 |
  602596.1 |
  605095.4 |########################################
  (0 below, 1 above range)

carrier_disp_threaded_real (n=6, range 624405.4-678773.9 ns)
  624405.4 |########################################
  627123.8 |
  629842.3 |
  632560.7 |
  635279.1 |
  637997.5 |
  640716.0 |
  643434.4 |
  646152.8 |########################################
  648871.2 |########################################
  651589.7 |
  654308.1 |
  657026.5 |
  659745.0 |
  662463.4 |
  665181.8 |
  667900.2 |
  670618.7 |
  673337.1 |########################################
  676055.5 |########################################
  (0 below, 1 above range)

```
