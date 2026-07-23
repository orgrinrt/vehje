# Predecoded dispatch shape, madd profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_madd_null dominates: 10% faster than the next best (carrier_pre_madd_switch)

carrier_pre_madd_null (694.34 us) leads carrier_pre_madd_switch (766.60 us) by 10%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### Two tiers: {carrier_pre_madd_null, carrier_pre_madd_switch, carrier_pre_madd_threaded, carrier_pre_madd_direct, carrier_pre_madd_fntable} vs {carrier_pre_madd_regcache} (37% apart)

The field splits into a fast tier {carrier_pre_madd_null, carrier_pre_madd_switch, carrier_pre_madd_threaded, carrier_pre_madd_direct, carrier_pre_madd_fntable} and a slow tier {carrier_pre_madd_regcache} with a 37% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_madd_null** at 694341.8 ns median (-9.4% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.53x (fastest 694341.8 ns, slowest 1062686.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_madd_direct | 773849ns | 774064ns | 772513ns | 773602ns | 774889ns | +0.51% |
| carrier_pre_madd_fntable | 778692ns | 776897ns | 775273ns | 776844ns | 783172ns | +1.14% |
| carrier_pre_madd_null | 697934ns | 696545ns | 695173ns | 696393ns | 701626ns | -9.35% |
| carrier_pre_madd_regcache | 1055943ns | 1064922ns | 930608ns | 1046985ns | 1132048ns | +37.15% |
| carrier_pre_madd_switch | 769911ns | 768779ns | 767049ns | 768459ns | 773520ns | base |
| carrier_pre_madd_threaded | 770170ns | 769936ns | 768001ns | 769547ns | 772190ns | +0.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_madd_direct | 771666ns | 770374ns | 772687ns | +0.52% | 0.021 |
| carrier_pre_madd_fntable | 776511ns | 773125ns | 780941ns | +1.15% | 0.021 |
| carrier_pre_madd_null | 695711ns | 692974ns | 699361ns | -9.38% | 0.024 |
| carrier_pre_madd_regcache | 1053751ns | 928430ns | 1129881ns | +37.26% | 0.016 |
| carrier_pre_madd_switch | 767700ns | 764919ns | 771213ns | base | 0.021 |
| carrier_pre_madd_threaded | 768005ns | 765861ns | 769977ns | +0.04% | 0.021 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_madd_direct | 5018222 | 8950166 | 0.561 | 1.01× |
| carrier_pre_madd_fntable | 4995072 | 13411315 | 0.372 | 1.01× |
| carrier_pre_madd_null | 4490484 | 12321797 | 0.364 | 0.91× |
| carrier_pre_madd_regcache | 6751002 | 15842172 | 0.426 | 1.37× |
| carrier_pre_madd_switch | 4944345 | 11553469 | 0.428 | 1.00× |
| carrier_pre_madd_threaded | 4951183 | 11831468 | 0.418 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.024 Gops/s** (carrier_pre_madd_null; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_madd_direct | 0.021 | 89.8% |
| carrier_pre_madd_fntable | 0.021 | 89.4% |
| carrier_pre_madd_null | 0.024 | 99.8% |
| carrier_pre_madd_regcache | 0.015 | 65.2% |
| carrier_pre_madd_switch | 0.021 | 90.4% |
| carrier_pre_madd_threaded | 0.021 | 90.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_madd_direct | 773849ns | 773849ns | +0.51% |
| carrier_pre_madd_fntable | 778692ns | 778692ns | +1.14% |
| carrier_pre_madd_null | 697934ns | 697934ns | -9.35% |
| carrier_pre_madd_regcache | 1055943ns | 1055943ns | +37.15% |
| carrier_pre_madd_switch | 769911ns | 769911ns | base |
| carrier_pre_madd_threaded | 770170ns | 770170ns | +0.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_madd_switch | 766604ns | base | --- | [765283, 771213] | --- | --- | --- | --- |
| carrier_pre_madd_direct | 771890ns | +4450.8ns (+0.6%) | [+473, +6974]ns | [770420, 772687] | YES (adj: no) | 0.2734 | 0.2188 | 0 |
| carrier_pre_madd_fntable | 774737ns | +8773.6ns (+1.1%) | [+2747, +14913]ns | [773854, 780941] | YES (adj: no) | 0.2734 | 0.2188 | 0 |
| carrier_pre_madd_null | 694342ns | -72217.7ns (-9.4%) | [-77092, -66656]ns | [693430, 699361] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_pre_madd_regcache | 1062687ns | +292097.5ns (+38.1%) | [+203403, +362654]ns | [968686, 1129881] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_pre_madd_threaded | 767788ns | no significant difference | [-3630, +3495]ns | [766250, 769977] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_madd_switch | carrier_pre_madd_direct | carrier_pre_madd_fntable | carrier_pre_madd_null | carrier_pre_madd_regcache | carrier_pre_madd_threaded |
|---|---|---|---|---|---|---|
| 1 | 767318ns | +0.6% | +0.8% | -9.6% | +48.5% | +0.5% |
| 2 | 767137ns | +0.4% | +1.7% | -9.4% | +46.0% | -0.1% |
| 3 | 764919ns | +1.0% | +2.2% | -9.4% | +21.4% | +0.1% |
| 4 | 775108ns | -0.3% | -0.0% | -10.4% | +36.9% | -0.9% |
| 5 | 765646ns | +0.8% | +1.2% | -8.0% | +31.8% | +0.4% |
| 6 | 766070ns | +0.6% | +1.1% | -9.4% | +39.0% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_madd_direct | -0.064 | ok |
| carrier_pre_madd_fntable | 0.068 | ok |
| carrier_pre_madd_null | -0.167 | ok |
| carrier_pre_madd_regcache | -0.145 | ok |
| carrier_pre_madd_switch | -0.439 | moderate- |
| carrier_pre_madd_threaded | -0.134 | ok |

**Consistency summary:**

- **carrier_pre_madd_direct**: won 1/6, lost 5/6
- **carrier_pre_madd_fntable**: won 0/6, lost 5/6
- **carrier_pre_madd_null**: won 6/6, lost 0/6
- **carrier_pre_madd_regcache**: won 0/6, lost 6/6
- **carrier_pre_madd_threaded**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_madd_direct | 822016.9ns | 771665.8ns | 106.5% | HIGH |
| carrier_pre_madd_fntable | 813830.8ns | 776511.0ns | 104.8% | HIGH |
| carrier_pre_madd_null | 732696.1ns | 695711.0ns | 105.3% | HIGH |
| carrier_pre_madd_regcache | 1092481.4ns | 1053751.3ns | 103.7% | HIGH |
| carrier_pre_madd_switch | 805087.8ns | 767699.7ns | 104.9% | HIGH |
| carrier_pre_madd_threaded | 805316.0ns | 768004.6ns | 104.9% | HIGH |

## Distribution (algo ns)

```
carrier_pre_madd_direct (n=6, range 770374.2-772687.1 ns)
  770374.2 |########################################
  770489.8 |
  770605.5 |
  770721.1 |
  770836.8 |
  770952.4 |
  771068.1 |
  771183.7 |
  771299.3 |
  771415.0 |
  771530.6 |
  771646.3 |
  771761.9 |####################
  771877.6 |####################
  771993.2 |
  772108.8 |
  772224.5 |
  772340.1 |
  772455.8 |####################
  772571.4 |
  (0 below, 1 above range)

carrier_pre_madd_fntable (n=6, range 773125.4-780941.4 ns)
  773125.4 |####################
  773516.2 |
  773907.0 |
  774297.8 |########################################
  774688.6 |####################
  775079.4 |
  775470.2 |
  775861.0 |
  776251.8 |
  776642.6 |
  777033.4 |
  777424.2 |
  777815.0 |
  778205.8 |
  778596.6 |
  778987.4 |
  779378.2 |
  779769.0 |####################
  780159.8 |
  780550.6 |
  (0 below, 1 above range)

carrier_pre_madd_null (n=6, range 692974.2-699361.2 ns)
  692974.2 |####################
  693293.6 |
  693612.9 |####################
  693932.3 |
  694251.6 |########################################
  694571.0 |####################
  694890.3 |
  695209.7 |
  695529.0 |
  695848.4 |
  696167.7 |
  696487.1 |
  696806.4 |
  697125.8 |
  697445.1 |
  697764.5 |
  698083.8 |
  698403.2 |
  698722.5 |
  699041.9 |
  (0 below, 1 above range)

carrier_pre_madd_regcache (n=6, range 928430.0-1129881.2 ns)
  928430.0 |####################
  938502.6 |
  948575.1 |
  958647.7 |
  968720.2 |
  978792.8 |
  988865.4 |
  998937.9 |####################
  1009010.5 |
  1019083.1 |
  1029155.6 |
  1039228.2 |
  1049300.8 |
  1059373.3 |########################################
  1069445.9 |
  1079518.4 |
  1089591.0 |
  1099663.6 |
  1109736.1 |
  1119808.7 |####################
  (0 below, 1 above range)

carrier_pre_madd_switch (n=6, range 764919.2-771212.9 ns)
  764919.2 |####################
  765233.9 |
  765548.6 |####################
  765863.3 |####################
  766177.9 |
  766492.6 |
  766807.3 |
  767122.0 |########################################
  767436.7 |
  767751.4 |
  768066.1 |
  768380.7 |
  768695.4 |
  769010.1 |
  769324.8 |
  769639.5 |
  769954.2 |
  770268.8 |
  770583.5 |
  770898.2 |
  (0 below, 1 above range)

carrier_pre_madd_threaded (n=6, range 765861.2-769976.8 ns)
  765861.2 |########################################
  766067.0 |
  766272.8 |
  766478.5 |########################################
  766684.3 |
  766890.1 |
  767095.9 |########################################
  767301.7 |
  767507.5 |
  767713.2 |
  767919.0 |
  768124.8 |
  768330.6 |########################################
  768536.4 |
  768742.2 |
  768947.9 |########################################
  769153.7 |
  769359.5 |
  769565.3 |
  769771.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_madd_direct**: bridge=106.4% of algo (FFI overhead may distort results)
- **carrier_pre_madd_fntable**: bridge=104.8% of algo (FFI overhead may distort results)
- **carrier_pre_madd_null**: bridge=105.3% of algo (FFI overhead may distort results)
- **carrier_pre_madd_regcache**: bridge=103.5% of algo (FFI overhead may distort results)
- **carrier_pre_madd_switch**: bridge=104.9% of algo (FFI overhead may distort results)
- **carrier_pre_madd_threaded**: bridge=104.9% of algo (FFI overhead may distort results)
