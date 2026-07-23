# Dispatch shape over the wire form, wideselect profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_wideselect_nullfloor dominates: 62% faster than the next best (carrier_disp_wideselect_bittree)

carrier_disp_wideselect_nullfloor (118.04 us) leads carrier_disp_wideselect_bittree (191.41 us) by 62%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_wideselect_nullfloor beats baseline by 74% (significant)

carrier_disp_wideselect_nullfloor is -334.66 us (74%) faster than baseline carrier_disp_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_wideselect_ifchainlin is an outlier: 4.5x slower than the field

carrier_disp_wideselect_ifchainlin (532.05 us) is 4.5x the fastest (118.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_bittree} vs {carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_threaded, carrier_disp_wideselect_switch, carrier_disp_wideselect_fntable, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_ifchainlin} (126% apart)

The field splits into a fast tier {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_bittree} and a slow tier {carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_threaded, carrier_disp_wideselect_switch, carrier_disp_wideselect_fntable, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_ifchainlin} with a 126% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.5x the fastest

Fastest carrier_disp_wideselect_nullfloor (118.04 us) to slowest carrier_disp_wideselect_ifchainlin (532.05 us): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_wideselect_nullfloor** at 118038.9 ns median (-73.9% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.51x (fastest 118038.9 ns, slowest 532053.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 198989ns | 193802ns | 189568ns | 192504ns | 213428ns | -55.67% |
| carrier_disp_wideselect_fntable | 456094ns | 455307ns | 445944ns | 453981ns | 464338ns | +1.62% |
| carrier_disp_wideselect_ifchain | 454874ns | 461172ns | 427016ns | 451002ns | 474611ns | +1.34% |
| carrier_disp_wideselect_ifchainasc | 438405ns | 435582ns | 412808ns | 428347ns | 466290ns | -2.32% |
| carrier_disp_wideselect_ifchainlin | 528777ns | 535197ns | 488945ns | 522683ns | 557834ns | +17.81% |
| carrier_disp_wideselect_nullfloor | 121227ns | 120382ns | 119370ns | 120109ns | 123834ns | -72.99% |
| carrier_disp_wideselect_switch | 448839ns | 454749ns | 416686ns | 446100ns | 469024ns | base |
| carrier_disp_wideselect_threaded | 461670ns | 449437ns | 421658ns | 441243ns | 512317ns | +2.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 196521ns | 187289ns | 210654ns | -55.96% | 0.021 |
| carrier_disp_wideselect_fntable | 453327ns | 442859ns | 461998ns | +1.59% | 0.009 |
| carrier_disp_wideselect_ifchain | 452356ns | 424718ns | 471692ns | +1.37% | 0.009 |
| carrier_disp_wideselect_ifchainasc | 435933ns | 410559ns | 463812ns | -2.31% | 0.009 |
| carrier_disp_wideselect_ifchainlin | 525822ns | 486700ns | 554574ns | +17.83% | 0.008 |
| carrier_disp_wideselect_nullfloor | 118788ns | 116938ns | 121167ns | -73.38% | 0.034 |
| carrier_disp_wideselect_switch | 446245ns | 414341ns | 466049ns | base | 0.009 |
| carrier_disp_wideselect_threaded | 458936ns | 418976ns | 509743ns | +2.84% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 1226365 | 5068521 | 0.242 | 0.44× |
| carrier_disp_wideselect_fntable | 2838069 | 6680894 | 0.425 | 1.01× |
| carrier_disp_wideselect_ifchain | 2839279 | 4958355 | 0.573 | 1.01× |
| carrier_disp_wideselect_ifchainasc | 2718967 | 4958361 | 0.548 | 0.97× |
| carrier_disp_wideselect_ifchainlin | 3305626 | 13168878 | 0.251 | 1.18× |
| carrier_disp_wideselect_nullfloor | 739608 | 4189518 | 0.177 | 0.26× |
| carrier_disp_wideselect_switch | 2798296 | 4827425 | 0.580 | 1.00× |
| carrier_disp_wideselect_threaded | 2888103 | 6606078 | 0.437 | 1.03× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.035 Gops/s** (carrier_disp_wideselect_nullfloor; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_wideselect_bittree | 0.021 | 61.1% |
| carrier_disp_wideselect_fntable | 0.009 | 25.8% |
| carrier_disp_wideselect_ifchain | 0.009 | 25.5% |
| carrier_disp_wideselect_ifchainasc | 0.009 | 27.0% |
| carrier_disp_wideselect_ifchainlin | 0.008 | 22.0% |
| carrier_disp_wideselect_nullfloor | 0.035 | 99.1% |
| carrier_disp_wideselect_switch | 0.009 | 25.9% |
| carrier_disp_wideselect_threaded | 0.009 | 26.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_wideselect_bittree | 198989ns | 198989ns | -55.67% |
| carrier_disp_wideselect_fntable | 456094ns | 456094ns | +1.62% |
| carrier_disp_wideselect_ifchain | 454874ns | 454874ns | +1.34% |
| carrier_disp_wideselect_ifchainasc | 438405ns | 438405ns | -2.32% |
| carrier_disp_wideselect_ifchainlin | 528777ns | 528777ns | +17.81% |
| carrier_disp_wideselect_nullfloor | 121227ns | 121227ns | -72.99% |
| carrier_disp_wideselect_switch | 448839ns | 448839ns | base |
| carrier_disp_wideselect_threaded | 461670ns | 461670ns | +2.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_wideselect_switch | 452328ns | base | --- | [420359, 466049] | --- | --- | --- | --- |
| carrier_disp_wideselect_bittree | 191413ns | -249698.6ns (-55.2%) | [-275576, -223897]ns | [187497, 210654] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_wideselect_fntable | 452456ns | no significant difference | [-11254, +25999]ns | [445526, 461998] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_wideselect_ifchain | 458783ns | no significant difference | [-1303, +13401]ns | [426593, 471692] | no | 0.3828 | 0.2188 | 0 |
| carrier_disp_wideselect_ifchainasc | 433014ns | no significant difference | [-48216, +28285]ns | [410972, 463812] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_wideselect_ifchainlin | 532054ns | +69497.1ns (+15.4%) | [+35339, +133894]ns | [490838, 554574] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_wideselect_nullfloor | 118039ns | -334662.8ns (-74.0%) | [-345493, -302216]ns | [117158, 121167] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_wideselect_threaded | 446405ns | no significant difference | [-38967, +74216]ns | [420661, 509743] | no | 0.8021 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_wideselect_switch | carrier_disp_wideselect_bittree | carrier_disp_wideselect_fntable | carrier_disp_wideselect_ifchain | carrier_disp_wideselect_ifchainasc | carrier_disp_wideselect_ifchainlin | carrier_disp_wideselect_nullfloor | carrier_disp_wideselect_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 444677ns | -56.2% | +3.6% | +3.6% | +2.0% | +9.5% | -73.6% | +7.2% |
| 2 | 426378ns | -56.0% | +5.5% | +0.5% | +11.2% | +28.3% | -72.0% | +27.3% |
| 3 | 414341ns | -50.5% | +6.9% | +2.5% | -0.7% | +35.5% | -71.8% | +1.1% |
| 4 | 465775ns | -53.6% | -2.3% | +2.3% | -8.7% | +17.6% | -74.6% | -9.3% |
| 5 | 459979ns | -59.3% | -2.6% | -0.6% | -4.1% | +12.4% | -74.4% | +0.2% |
| 6 | 466322ns | -59.7% | -0.6% | +0.1% | -12.0% | +6.1% | -73.6% | -7.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_wideselect_bittree | 0.009 | ok |
| carrier_disp_wideselect_fntable | -0.221 | moderate- |
| carrier_disp_wideselect_ifchain | -0.013 | ok |
| carrier_disp_wideselect_ifchainasc | -0.059 | ok |
| carrier_disp_wideselect_ifchainlin | 0.164 | ok |
| carrier_disp_wideselect_nullfloor | -0.145 | ok |
| carrier_disp_wideselect_switch | 0.245 | moderate+ |
| carrier_disp_wideselect_threaded | -0.048 | ok |

**Consistency summary:**

- **carrier_disp_wideselect_bittree**: won 6/6, lost 0/6
- **carrier_disp_wideselect_fntable**: won 3/6, lost 3/6
- **carrier_disp_wideselect_ifchain**: won 1/6, lost 4/6
- **carrier_disp_wideselect_ifchainasc**: won 4/6, lost 2/6
- **carrier_disp_wideselect_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_wideselect_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_wideselect_threaded**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 197083.1ns | 196521.4ns | 100.3% | HIGH |
| carrier_disp_wideselect_fntable | 454680.1ns | 453326.6ns | 100.3% | HIGH |
| carrier_disp_wideselect_ifchain | 453273.4ns | 452356.0ns | 100.2% | HIGH |
| carrier_disp_wideselect_ifchainasc | 436645.5ns | 435932.6ns | 100.2% | HIGH |
| carrier_disp_wideselect_ifchainlin | 527956.5ns | 525821.7ns | 100.4% | HIGH |
| carrier_disp_wideselect_nullfloor | 119103.7ns | 118788.0ns | 100.3% | HIGH |
| carrier_disp_wideselect_switch | 446847.4ns | 446245.3ns | 100.1% | HIGH |
| carrier_disp_wideselect_threaded | 460740.7ns | 458936.3ns | 100.4% | HIGH |

## Distribution (algo ns)

```
carrier_disp_wideselect_bittree (n=6, range 187289.2-210654.0 ns)
  187289.2 |########################################
  188457.4 |
  189625.7 |
  190793.9 |
  191962.2 |
  193130.4 |
  194298.6 |#############
  195466.9 |
  196635.1 |
  197803.3 |
  198971.6 |
  200139.8 |
  201308.1 |
  202476.3 |
  203644.5 |
  204812.8 |#############
  205981.0 |
  207149.2 |
  208317.5 |
  209485.7 |
  (0 below, 1 above range)

carrier_disp_wideselect_fntable (n=6, range 442858.8-461997.7 ns)
  442858.8 |########################################
  443815.7 |
  444772.7 |
  445729.6 |
  446686.6 |
  447643.5 |########################################
  448600.5 |
  449557.4 |########################################
  450514.4 |
  451471.3 |
  452428.2 |
  453385.2 |
  454342.1 |########################################
  455299.1 |
  456256.0 |
  457213.0 |
  458169.9 |
  459126.9 |
  460083.8 |########################################
  461040.8 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchain (n=6, range 424717.5-471692.1 ns)
  424717.5 |########################################
  427066.2 |########################################
  429415.0 |
  431763.7 |
  434112.4 |
  436461.2 |
  438809.9 |
  441158.6 |
  443507.3 |
  445856.1 |
  448204.8 |
  450553.5 |
  452902.3 |
  455251.0 |########################################
  457599.7 |
  459948.4 |########################################
  462297.2 |
  464645.9 |########################################
  466994.6 |
  469343.4 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainasc (n=6, range 410559.2-463812.1 ns)
  410559.2 |########################################
  413221.8 |
  415884.5 |
  418547.1 |
  421209.8 |
  423872.4 |####################
  426535.1 |
  429197.7 |
  431860.4 |
  434523.0 |
  437185.7 |
  439848.3 |####################
  442510.9 |
  445173.6 |
  447836.2 |
  450498.9 |
  453161.5 |####################
  455824.2 |
  458486.8 |
  461149.5 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainlin (n=6, range 486699.6-554573.8 ns)
  486699.6 |########################################
  490093.3 |
  493487.0 |########################################
  496880.7 |
  500274.4 |
  503668.1 |
  507061.8 |
  510455.6 |
  513849.3 |########################################
  517243.0 |
  520636.7 |
  524030.4 |
  527424.1 |
  530817.8 |
  534211.5 |
  537605.2 |
  540998.9 |
  544392.6 |########################################
  547786.3 |########################################
  551180.0 |
  (0 below, 1 above range)

carrier_disp_wideselect_nullfloor (n=6, range 116938.3-121167.1 ns)
  116938.3 |########################################
  117149.7 |
  117361.2 |########################################
  117572.6 |
  117784.1 |########################################
  117995.5 |########################################
  118206.9 |
  118418.4 |
  118629.8 |
  118841.2 |
  119052.7 |
  119264.1 |########################################
  119475.6 |
  119687.0 |
  119898.4 |
  120109.9 |
  120321.3 |
  120532.7 |
  120744.2 |
  120955.6 |
  (0 below, 1 above range)

carrier_disp_wideselect_switch (n=6, range 414341.2-466048.6 ns)
  414341.2 |########################################
  416926.6 |
  419511.9 |
  422097.3 |
  424682.7 |########################################
  427268.0 |
  429853.4 |
  432438.8 |
  435024.1 |
  437609.5 |
  440194.9 |
  442780.2 |########################################
  445365.6 |
  447951.0 |
  450536.3 |
  453121.7 |
  455707.1 |
  458292.4 |########################################
  460877.8 |
  463463.2 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_threaded (n=6, range 418976.2-509743.3 ns)
  418976.2 |########################################
  423514.6 |
  428052.9 |####################
  432591.3 |
  437129.6 |
  441668.0 |
  446206.3 |
  450744.7 |
  455283.0 |
  459821.4 |####################
  464359.8 |
  468898.1 |
  473436.5 |####################
  477974.8 |
  482513.2 |
  487051.5 |
  491589.9 |
  496128.2 |
  500666.6 |
  505204.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_wideselect_bittree**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_fntable**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchain**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainasc**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainlin**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_nullfloor**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_switch**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_threaded**: bridge=100.4% of algo (FFI overhead may distort results)
