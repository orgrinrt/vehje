# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_real_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_real_nullfloor dominates: 43% faster than the next best (carrier_disp_real_ifchain)

carrier_disp_real_nullfloor (7.69 us) leads carrier_disp_real_ifchain (10.97 us) by 43%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_real_nullfloor beats baseline by 31% (significant)

carrier_disp_real_nullfloor is -3.38 us (31%) faster than baseline carrier_disp_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_real_ifchainlin is an outlier: 2.9x slower than the field

carrier_disp_real_ifchainlin (22.01 us) is 2.9x the fastest (7.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_real_ifchainlin shows alternating (throttle bounce) (autocorr -0.66)

carrier_disp_real_ifchainlin's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_real_nullfloor, carrier_disp_real_ifchain, carrier_disp_real_switch, carrier_disp_real_ifchainasc, carrier_disp_real_threaded, carrier_disp_real_bittree, carrier_disp_real_fntable} vs {carrier_disp_real_ifchainlin} (59% apart)

The field splits into a fast tier {carrier_disp_real_nullfloor, carrier_disp_real_ifchain, carrier_disp_real_switch, carrier_disp_real_ifchainasc, carrier_disp_real_threaded, carrier_disp_real_bittree, carrier_disp_real_fntable} and a slow tier {carrier_disp_real_ifchainlin} with a 59% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_disp_real_ifchain's edge over baseline is significant but tiny (-7 ns, 0.07%)

carrier_disp_real_ifchain differs from baseline carrier_disp_real_switch by -7 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_real_nullfloor** at 7689.6 ns median (-30.4% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 2.86x (fastest 7689.6 ns, slowest 22005.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_real_bittree | 15123ns | 15376ns | 14120ns | 14988ns | 15827ns | +12.59% |
| carrier_disp_real_fntable | 16372ns | 16417ns | 15982ns | 16358ns | 16590ns | +21.90% |
| carrier_disp_real_ifchain | 13357ns | 13584ns | 12094ns | 13489ns | 13792ns | -0.55% |
| carrier_disp_real_ifchainasc | 13216ns | 13611ns | 12141ns | 13218ns | 13750ns | -1.60% |
| carrier_disp_real_ifchainlin | 23983ns | 24445ns | 22412ns | 23808ns | 25030ns | +78.56% |
| carrier_disp_real_nullfloor | 10012ns | 10156ns | 9338ns | 10063ns | 10272ns | -25.46% |
| carrier_disp_real_switch | 13431ns | 13541ns | 12617ns | 13338ns | 13977ns | base |
| carrier_disp_real_threaded | 14384ns | 14697ns | 13321ns | 14243ns | 15127ns | +7.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_real_bittree | 12723ns | 11863ns | 13345ns | +15.97% | 0.020 |
| carrier_disp_real_fntable | 13836ns | 13515ns | 13982ns | +26.11% | 0.019 |
| carrier_disp_real_ifchain | 10864ns | 9880ns | 11224ns | -0.98% | 0.024 |
| carrier_disp_real_ifchainasc | 10727ns | 9902ns | 11167ns | -2.22% | 0.024 |
| carrier_disp_real_ifchainlin | 21590ns | 20260ns | 22476ns | +96.80% | 0.012 |
| carrier_disp_real_nullfloor | 7574ns | 7039ns | 7789ns | -30.97% | 0.034 |
| carrier_disp_real_switch | 10971ns | 10232ns | 11483ns | base | 0.023 |
| carrier_disp_real_threaded | 12003ns | 11162ns | 12583ns | +9.41% | 0.021 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_real_bittree | 320116 | 1287338 | 0.249 | 1.08× |
| carrier_disp_real_fntable | 315452 | 1563502 | 0.202 | 1.06× |
| carrier_disp_real_ifchain | 293643 | 1372905 | 0.214 | 0.99× |
| carrier_disp_real_ifchainasc | 299423 | 1390678 | 0.215 | 1.01× |
| carrier_disp_real_ifchainlin | 355907 | 2112588 | 0.168 | 1.20× |
| carrier_disp_real_nullfloor | 293688 | 1664530 | 0.176 | 0.99× |
| carrier_disp_real_switch | 296846 | 1349180 | 0.220 | 1.00× |
| carrier_disp_real_threaded | 299944 | 1684072 | 0.178 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_disp_real_nullfloor; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_real_bittree | 0.020 | 54.4% |
| carrier_disp_real_fntable | 0.018 | 50.7% |
| carrier_disp_real_ifchain | 0.023 | 64.1% |
| carrier_disp_real_ifchainasc | 0.023 | 63.5% |
| carrier_disp_real_ifchainlin | 0.012 | 32.0% |
| carrier_disp_real_nullfloor | 0.033 | 91.5% |
| carrier_disp_real_switch | 0.023 | 63.7% |
| carrier_disp_real_threaded | 0.021 | 57.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_real_bittree | 15123ns | 15123ns | +12.59% |
| carrier_disp_real_fntable | 16372ns | 16372ns | +21.90% |
| carrier_disp_real_ifchain | 13357ns | 13357ns | -0.55% |
| carrier_disp_real_ifchainasc | 13216ns | 13216ns | -1.60% |
| carrier_disp_real_ifchainlin | 23983ns | 23983ns | +78.56% |
| carrier_disp_real_nullfloor | 10012ns | 10012ns | -25.46% |
| carrier_disp_real_switch | 13431ns | 13431ns | base |
| carrier_disp_real_threaded | 14384ns | 14384ns | +7.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_real_switch | 11049ns | base | --- | [10381, 11483] | --- | --- | --- | --- |
| carrier_disp_real_bittree | 12946ns | +1873.1ns (+17.0%) | [+1202, +2180]ns | [11878, 13345] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_fntable | 13874ns | +2839.8ns (+25.7%) | [+2220, +3535]ns | [13651, 13982] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_ifchain | 10974ns | no significant difference | [-622, +307]ns | [10393, 11224] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_real_ifchainasc | 11090ns | no significant difference | [-1269, +530]ns | [9925, 11167] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_real_ifchainlin | 22006ns | +10754.8ns (+97.3%) | [+9497, +11607]ns | [20290, 22476] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_nullfloor | 7690ns | -3376.3ns (-30.6%) | [-3835, -2980]ns | [7243, 7789] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_real_threaded | 12264ns | +971.1ns (+8.8%) | [+180, +1945]ns | [11163, 12583] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_real_switch | carrier_disp_real_bittree | carrier_disp_real_fntable | carrier_disp_real_ifchain | carrier_disp_real_ifchainasc | carrier_disp_real_ifchainlin | carrier_disp_real_nullfloor | carrier_disp_real_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 11857ns | +13.2% | +14.0% | -7.5% | -16.5% | +86.1% | -34.3% | +2.1% |
| 2 | 11055ns | +7.6% | +25.2% | -0.7% | +0.3% | +83.3% | -32.6% | +1.0% |
| 3 | 11042ns | +20.0% | +26.1% | +2.2% | +1.7% | +102.3% | -29.5% | +13.2% |
| 4 | 10530ns | +20.1% | +30.9% | +3.6% | -5.5% | +93.0% | -26.3% | +6.0% |
| 5 | 11109ns | +19.4% | +25.2% | +0.5% | -0.1% | +103.6% | -31.4% | +11.8% |
| 6 | 10232ns | +15.9% | +37.3% | -3.4% | +8.5% | +114.4% | -31.2% | +23.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_real_bittree | -0.627 | HIGH- (thermal bounce) |
| carrier_disp_real_fntable | 0.045 | ok |
| carrier_disp_real_ifchain | -0.164 | ok |
| carrier_disp_real_ifchainasc | -0.337 | moderate- |
| carrier_disp_real_ifchainlin | -0.660 | HIGH- (thermal bounce) |
| carrier_disp_real_nullfloor | -0.067 | ok |
| carrier_disp_real_switch | -0.073 | ok |
| carrier_disp_real_threaded | -0.442 | moderate- |

**Consistency summary:**

- **carrier_disp_real_bittree**: won 0/6, lost 6/6
- **carrier_disp_real_fntable**: won 0/6, lost 6/6
- **carrier_disp_real_ifchain**: won 3/6, lost 3/6
- **carrier_disp_real_ifchainasc**: won 3/6, lost 3/6
- **carrier_disp_real_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_real_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_real_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_real_bittree | 93685.2ns | 12722.9ns | 736.4% | HIGH |
| carrier_disp_real_fntable | 95490.0ns | 13835.9ns | 690.2% | HIGH |
| carrier_disp_real_ifchain | 89480.3ns | 10863.6ns | 823.7% | HIGH |
| carrier_disp_real_ifchainasc | 89575.2ns | 10727.3ns | 835.0% | HIGH |
| carrier_disp_real_ifchainlin | 96837.2ns | 21590.5ns | 448.5% | HIGH |
| carrier_disp_real_nullfloor | 91761.4ns | 7573.8ns | 1211.6% | HIGH |
| carrier_disp_real_switch | 91122.3ns | 10971.1ns | 830.6% | HIGH |
| carrier_disp_real_threaded | 88609.8ns | 12003.2ns | 738.2% | HIGH |

## Distribution (algo ns)

```
carrier_disp_real_bittree (n=6, range 11862.9-13345.4 ns)
  11862.9 |########################################
  11937.0 |
  12011.1 |
  12085.3 |
  12159.4 |
  12233.5 |
  12307.6 |
  12381.8 |
  12455.9 |
  12530.0 |
  12604.1 |####################
  12678.3 |
  12752.4 |
  12826.5 |
  12900.6 |
  12974.8 |
  13048.9 |
  13123.0 |
  13197.1 |########################################
  13271.3 |
  (0 below, 1 above range)

carrier_disp_real_fntable (n=6, range 13514.6-13982.3 ns)
  13514.6 |########################################
  13538.0 |
  13561.4 |
  13584.8 |
  13608.1 |
  13631.5 |
  13654.9 |
  13678.3 |
  13701.7 |
  13725.1 |
  13748.5 |
  13771.8 |########################################
  13795.2 |
  13818.6 |########################################
  13842.0 |
  13865.4 |
  13888.8 |########################################
  13912.1 |########################################
  13935.5 |
  13958.9 |
  (0 below, 1 above range)

carrier_disp_real_ifchain (n=6, range 9880.4-11224.2 ns)
   9880.4 |####################
   9947.6 |
  10014.8 |
  10082.0 |
  10149.2 |
  10216.4 |
  10283.5 |
  10350.7 |
  10417.9 |
  10485.1 |
  10552.3 |
  10619.5 |
  10686.7 |
  10753.9 |
  10821.1 |
  10888.2 |####################
  10955.4 |########################################
  11022.6 |
  11089.8 |
  11157.0 |####################
  (0 below, 1 above range)

carrier_disp_real_ifchainasc (n=6, range 9901.7-11166.9 ns)
   9901.7 |########################################
   9965.0 |
  10028.2 |
  10091.5 |
  10154.7 |
  10218.0 |
  10281.3 |
  10344.5 |
  10407.8 |
  10471.0 |
  10534.3 |
  10597.6 |
  10660.8 |
  10724.1 |
  10787.3 |
  10850.6 |
  10913.9 |
  10977.1 |
  11040.4 |########################################
  11103.6 |####################
  (0 below, 1 above range)

carrier_disp_real_ifchainlin (n=6, range 20260.4-22475.8 ns)
  20260.4 |########################################
  20371.2 |
  20481.9 |
  20592.7 |
  20703.5 |
  20814.3 |
  20925.0 |
  21035.8 |
  21146.6 |
  21257.4 |
  21368.1 |
  21478.9 |
  21589.7 |
  21700.4 |
  21811.2 |
  21922.0 |####################
  22032.8 |####################
  22143.5 |
  22254.3 |####################
  22365.1 |
  (0 below, 1 above range)

carrier_disp_real_nullfloor (n=6, range 7039.2-7788.8 ns)
   7039.2 |####################
   7076.7 |
   7114.2 |
   7151.6 |
   7189.1 |
   7226.6 |
   7264.1 |
   7301.5 |
   7339.0 |
   7376.5 |
   7414.0 |####################
   7451.5 |
   7488.9 |
   7526.4 |
   7563.9 |
   7601.4 |####################
   7638.8 |
   7676.3 |
   7713.8 |
   7751.3 |########################################
  (0 below, 1 above range)

carrier_disp_real_switch (n=6, range 10232.5-11483.0 ns)
  10232.5 |########################################
  10295.0 |
  10357.5 |
  10420.1 |
  10482.6 |########################################
  10545.1 |
  10607.6 |
  10670.2 |
  10732.7 |
  10795.2 |
  10857.7 |
  10920.2 |
  10982.8 |########################################
  11045.3 |########################################
  11107.8 |########################################
  11170.3 |
  11232.9 |
  11295.4 |
  11357.9 |
  11420.4 |
  (0 below, 1 above range)

carrier_disp_real_threaded (n=6, range 11162.5-12582.7 ns)
  11162.5 |########################################
  11233.5 |
  11304.5 |
  11375.5 |
  11446.5 |
  11517.5 |
  11588.6 |
  11659.6 |
  11730.6 |
  11801.6 |
  11872.6 |
  11943.6 |
  12014.6 |
  12085.6 |####################
  12156.6 |
  12227.7 |
  12298.7 |
  12369.7 |####################
  12440.7 |####################
  12511.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_real_bittree**: bridge=723.9% of algo (FFI overhead may distort results)
- **carrier_disp_real_fntable**: bridge=687.6% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchain**: bridge=816.0% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainasc**: bridge=809.5% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainlin**: bridge=439.6% of algo (FFI overhead may distort results)
- **carrier_disp_real_nullfloor**: bridge=1197.8% of algo (FFI overhead may distort results)
- **carrier_disp_real_switch**: bridge=819.2% of algo (FFI overhead may distort results)
- **carrier_disp_real_threaded**: bridge=724.9% of algo (FFI overhead may distort results)
