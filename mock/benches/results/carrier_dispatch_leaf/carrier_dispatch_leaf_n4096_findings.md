# Dispatch shape over the wire form, leaf profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_leaf_nullfloor dominates: 47% faster than the next best (carrier_disp_leaf_bittree)

carrier_disp_leaf_nullfloor (114.16 us) leads carrier_disp_leaf_bittree (167.71 us) by 47%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_leaf_nullfloor beats baseline by 51% (significant)

carrier_disp_leaf_nullfloor is -115.14 us (51%) faster than baseline carrier_disp_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_leaf_fntable is an outlier: 2.3x slower than the field

carrier_disp_leaf_fntable (264.74 us) is 2.3x the fastest (114.16 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_disp_leaf_nullfloor} vs {carrier_disp_leaf_bittree, carrier_disp_leaf_ifchainlin, carrier_disp_leaf_threaded, carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchain, carrier_disp_leaf_fntable} (47% apart)

The field splits into a fast tier {carrier_disp_leaf_nullfloor} and a slow tier {carrier_disp_leaf_bittree, carrier_disp_leaf_ifchainlin, carrier_disp_leaf_threaded, carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchain, carrier_disp_leaf_fntable} with a 47% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_leaf_nullfloor** at 114155.0 ns median (-49.5% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.32x (fastest 114155.0 ns, slowest 264744.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 169780ns | 170407ns | 164557ns | 169004ns | 173556ns | -26.31% |
| carrier_disp_leaf_fntable | 267383ns | 267554ns | 264844ns | 267441ns | 268566ns | +16.06% |
| carrier_disp_leaf_ifchain | 252577ns | 254960ns | 242812ns | 251820ns | 258596ns | +9.63% |
| carrier_disp_leaf_ifchainasc | 242372ns | 243230ns | 232973ns | 240855ns | 249345ns | +5.20% |
| carrier_disp_leaf_ifchainlin | 220882ns | 218839ns | 214364ns | 217973ns | 228505ns | -4.13% |
| carrier_disp_leaf_nullfloor | 116599ns | 116821ns | 110864ns | 114977ns | 121899ns | -49.39% |
| carrier_disp_leaf_switch | 230389ns | 228735ns | 224744ns | 228210ns | 236481ns | base |
| carrier_disp_leaf_threaded | 223618ns | 221622ns | 215844ns | 221453ns | 230751ns | -2.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 167152ns | 162325ns | 170816ns | -26.52% | 0.025 |
| carrier_disp_leaf_fntable | 264371ns | 261664ns | 265362ns | +16.22% | 0.015 |
| carrier_disp_leaf_ifchain | 249684ns | 239895ns | 255590ns | +9.77% | 0.016 |
| carrier_disp_leaf_ifchainasc | 239496ns | 230680ns | 246254ns | +5.29% | 0.017 |
| carrier_disp_leaf_ifchainlin | 218061ns | 211386ns | 225760ns | -4.13% | 0.019 |
| carrier_disp_leaf_nullfloor | 113974ns | 108205ns | 119247ns | -49.89% | 0.036 |
| carrier_disp_leaf_switch | 227466ns | 221841ns | 233459ns | base | 0.018 |
| carrier_disp_leaf_threaded | 220742ns | 212975ns | 227924ns | -2.96% | 0.019 |

## Performance model

- Peak throughput: **0.038 Gops/s** (carrier_disp_leaf_nullfloor; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_leaf_bittree | 0.024 | 64.5% |
| carrier_disp_leaf_fntable | 0.015 | 40.9% |
| carrier_disp_leaf_ifchain | 0.016 | 42.9% |
| carrier_disp_leaf_ifchainasc | 0.017 | 45.0% |
| carrier_disp_leaf_ifchainlin | 0.019 | 50.1% |
| carrier_disp_leaf_nullfloor | 0.036 | 94.8% |
| carrier_disp_leaf_switch | 0.018 | 47.9% |
| carrier_disp_leaf_threaded | 0.019 | 49.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_leaf_bittree | 169780ns | 169780ns | -26.31% |
| carrier_disp_leaf_fntable | 267383ns | 267383ns | +16.06% |
| carrier_disp_leaf_ifchain | 252577ns | 252577ns | +9.63% |
| carrier_disp_leaf_ifchainasc | 242372ns | 242372ns | +5.20% |
| carrier_disp_leaf_ifchainlin | 220882ns | 220882ns | -4.13% |
| carrier_disp_leaf_nullfloor | 116599ns | 116599ns | -49.39% |
| carrier_disp_leaf_switch | 230389ns | 230389ns | base |
| carrier_disp_leaf_threaded | 223618ns | 223618ns | -2.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_leaf_switch | 225905ns | base | --- | [223034, 233459] | --- | --- | --- | --- |
| carrier_disp_leaf_bittree | 167709ns | -58166.4ns (-25.7%) | [-70485, -52291]ns | [162931, 170816] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_fntable | 264744ns | +39100.0ns (+17.3%) | [+30211, +41404]ns | [263007, 265362] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_ifchain | 251970ns | +20870.0ns (+9.2%) | [+14283, +31503]ns | [241493, 255590] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_ifchainasc | 240310ns | +11677.7ns (+5.2%) | [+2247, +22167]ns | [231925, 246254] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_ifchainlin | 216014ns | no significant difference | [-19252, +1716]ns | [212410, 225760] | no | 0.2188 | 0.2188 | 0 |
| carrier_disp_leaf_nullfloor | 114155ns | -115144.0ns (-51.0%) | [-119503, -105828]ns | [108520, 119247] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_threaded | 218774ns | no significant difference | [-16080, +3038]ns | [215527, 227924] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_leaf_switch | carrier_disp_leaf_bittree | carrier_disp_leaf_fntable | carrier_disp_leaf_ifchain | carrier_disp_leaf_ifchainasc | carrier_disp_leaf_ifchainlin | carrier_disp_leaf_nullfloor | carrier_disp_leaf_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 225563ns | -25.3% | +17.5% | +7.8% | +6.4% | -5.4% | -52.0% | -3.2% |
| 2 | 221841ns | -24.1% | +19.2% | +15.2% | +10.0% | +5.2% | -47.3% | -1.7% |
| 3 | 224226ns | -22.8% | +18.0% | +10.8% | +4.0% | -4.1% | -50.4% | -5.0% |
| 4 | 226334ns | -26.2% | +17.1% | +12.9% | +9.8% | -6.6% | -51.9% | +4.3% |
| 5 | 240585ns | -32.0% | +10.4% | +6.2% | +0.0% | -9.8% | -50.5% | -8.7% |
| 6 | 226247ns | -28.3% | +15.7% | +6.0% | +2.0% | -3.5% | -47.2% | -3.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_leaf_bittree | 0.351 | moderate+ |
| carrier_disp_leaf_fntable | -0.257 | moderate- |
| carrier_disp_leaf_ifchain | -0.313 | moderate- |
| carrier_disp_leaf_ifchainasc | -0.378 | moderate- |
| carrier_disp_leaf_ifchainlin | -0.292 | moderate- |
| carrier_disp_leaf_nullfloor | -0.079 | ok |
| carrier_disp_leaf_switch | 0.008 | ok |
| carrier_disp_leaf_threaded | -0.343 | moderate- |

**Consistency summary:**

- **carrier_disp_leaf_bittree**: won 6/6, lost 0/6
- **carrier_disp_leaf_fntable**: won 0/6, lost 6/6
- **carrier_disp_leaf_ifchain**: won 0/6, lost 6/6
- **carrier_disp_leaf_ifchainasc**: won 0/6, lost 5/6
- **carrier_disp_leaf_ifchainlin**: won 5/6, lost 1/6
- **carrier_disp_leaf_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_leaf_threaded**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 167703.9ns | 167151.7ns | 100.3% | HIGH |
| carrier_disp_leaf_fntable | 269475.8ns | 264371.0ns | 101.9% | HIGH |
| carrier_disp_leaf_ifchain | 250657.3ns | 249684.4ns | 100.4% | HIGH |
| carrier_disp_leaf_ifchainasc | 240389.1ns | 239496.3ns | 100.4% | HIGH |
| carrier_disp_leaf_ifchainlin | 219147.3ns | 218061.4ns | 100.5% | HIGH |
| carrier_disp_leaf_nullfloor | 114667.0ns | 113974.1ns | 100.6% | HIGH |
| carrier_disp_leaf_switch | 228305.4ns | 227466.0ns | 100.4% | HIGH |
| carrier_disp_leaf_threaded | 221333.1ns | 220741.8ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_disp_leaf_bittree (n=6, range 162325.4-170816.0 ns)
  162325.4 |####################
  162749.9 |
  163174.5 |####################
  163599.0 |
  164023.5 |
  164448.1 |
  164872.6 |
  165297.1 |
  165721.7 |
  166146.2 |
  166570.7 |
  166995.3 |####################
  167419.8 |
  167844.3 |
  168268.9 |########################################
  168693.4 |
  169117.9 |
  169542.5 |
  169967.0 |
  170391.5 |
  (0 below, 1 above range)

carrier_disp_leaf_fntable (n=6, range 261663.8-265361.8 ns)
  261663.8 |########################################
  261848.7 |
  262033.6 |
  262218.5 |
  262403.4 |
  262588.3 |
  262773.2 |
  262958.1 |
  263143.0 |
  263327.9 |
  263512.8 |
  263697.7 |
  263882.6 |
  264067.5 |
  264252.4 |########################################
  264437.3 |########################################
  264622.2 |
  264807.1 |########################################
  264992.0 |########################################
  265176.9 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchain (n=6, range 239895.4-255590.0 ns)
  239895.4 |####################
  240680.1 |
  241464.9 |
  242249.6 |
  243034.3 |####################
  243819.0 |
  244603.8 |
  245388.5 |
  246173.2 |
  246958.0 |
  247742.7 |####################
  248527.4 |
  249312.2 |
  250096.9 |
  250881.6 |
  251666.4 |
  252451.1 |
  253235.8 |
  254020.5 |
  254805.3 |########################################
  (0 below, 1 above range)

carrier_disp_leaf_ifchainasc (n=6, range 230680.4-246254.0 ns)
  230680.4 |########################################
  231459.1 |
  232237.8 |
  233016.4 |########################################
  233795.1 |
  234573.8 |
  235352.5 |
  236131.2 |
  236909.8 |
  237688.5 |
  238467.2 |
  239245.9 |########################################
  240024.6 |########################################
  240803.2 |
  241581.9 |
  242360.6 |
  243139.3 |
  243918.0 |########################################
  244696.6 |
  245475.3 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchainlin (n=6, range 211385.8-225759.6 ns)
  211385.8 |########################################
  212104.5 |
  212823.2 |########################################
  213541.9 |
  214260.6 |
  214979.2 |########################################
  215697.9 |
  216416.6 |########################################
  217135.3 |
  217854.0 |########################################
  218572.7 |
  219291.4 |
  220010.1 |
  220728.8 |
  221447.5 |
  222166.1 |
  222884.8 |
  223603.5 |
  224322.2 |
  225040.9 |
  (0 below, 1 above range)

carrier_disp_leaf_nullfloor (n=6, range 108205.4-119247.3 ns)
  108205.4 |########################################
  108757.5 |########################################
  109309.6 |
  109861.7 |
  110413.8 |
  110965.9 |########################################
  111518.0 |
  112070.1 |
  112622.2 |
  113174.3 |
  113726.4 |
  114278.4 |
  114830.5 |
  115382.6 |
  115934.7 |
  116486.8 |########################################
  117038.9 |
  117591.0 |
  118143.1 |
  118695.2 |########################################
  (0 below, 1 above range)

carrier_disp_leaf_switch (n=6, range 221840.8-233459.4 ns)
  221840.8 |####################
  222421.7 |
  223002.7 |
  223583.6 |
  224164.5 |####################
  224745.4 |
  225326.4 |####################
  225907.3 |########################################
  226488.2 |
  227069.2 |
  227650.1 |
  228231.0 |
  228812.0 |
  229392.9 |
  229973.8 |
  230554.8 |
  231135.7 |
  231716.6 |
  232297.5 |
  232878.5 |
  (0 below, 1 above range)

carrier_disp_leaf_threaded (n=6, range 212974.6-227924.2 ns)
  212974.6 |####################
  213722.1 |
  214469.6 |
  215217.0 |
  215964.5 |
  216712.0 |
  217459.5 |####################
  218207.0 |####################
  218954.4 |########################################
  219701.9 |
  220449.4 |
  221196.9 |
  221944.4 |
  222691.8 |
  223439.3 |
  224186.8 |
  224934.3 |
  225681.8 |
  226429.2 |
  227176.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_leaf_bittree**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_fntable**: bridge=101.9% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchain**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainasc**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainlin**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_nullfloor**: bridge=100.7% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_switch**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_threaded**: bridge=100.3% of algo (FFI overhead may distort results)
