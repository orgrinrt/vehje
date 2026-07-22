# Dispatch shape over the wire form, scatter profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_scatter_nullfloor dominates: 149% faster than the next best (carrier_disp_scatter_bittree)

carrier_disp_scatter_nullfloor (117.92 us) leads carrier_disp_scatter_bittree (293.36 us) by 149%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_scatter_nullfloor beats baseline by 80% (significant)

carrier_disp_scatter_nullfloor is -463.56 us (80%) faster than baseline carrier_disp_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_scatter_threaded is an outlier: 5.4x slower than the field

carrier_disp_scatter_threaded (639.60 us) is 5.4x the fastest (117.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_scatter_fntable shows warm-up / thermal drift (autocorr +0.52)

carrier_disp_scatter_fntable's per-pass series has lag-1 autocorrelation +0.52, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_scatter_nullfloor} vs {carrier_disp_scatter_bittree, carrier_disp_scatter_ifchainlin, carrier_disp_scatter_fntable, carrier_disp_scatter_switch, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_ifchain, carrier_disp_scatter_threaded} (149% apart)

The field splits into a fast tier {carrier_disp_scatter_nullfloor} and a slow tier {carrier_disp_scatter_bittree, carrier_disp_scatter_ifchainlin, carrier_disp_scatter_fntable, carrier_disp_scatter_switch, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_ifchain, carrier_disp_scatter_threaded} with a 149% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.4x the fastest

Fastest carrier_disp_scatter_nullfloor (117.92 us) to slowest carrier_disp_scatter_threaded (639.60 us): 5.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_scatter_nullfloor** at 117921.5 ns median (-79.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.42x (fastest 117921.5 ns, slowest 639596.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 311721ns | 296024ns | 281943ns | 291911ns | 356324ns | -46.89% |
| carrier_disp_scatter_fntable | 560514ns | 562309ns | 546022ns | 559253ns | 569653ns | -4.50% |
| carrier_disp_scatter_ifchain | 604206ns | 614764ns | 527472ns | 612266ns | 630483ns | +2.95% |
| carrier_disp_scatter_ifchainasc | 604350ns | 605088ns | 574624ns | 597324ns | 629750ns | +2.97% |
| carrier_disp_scatter_ifchainlin | 558183ns | 551419ns | 510565ns | 538842ns | 611002ns | -4.89% |
| carrier_disp_scatter_nullfloor | 120975ns | 120321ns | 117050ns | 119814ns | 124680ns | -79.39% |
| carrier_disp_scatter_switch | 586907ns | 585039ns | 554858ns | 580342ns | 612780ns | base |
| carrier_disp_scatter_threaded | 637142ns | 642452ns | 547574ns | 630274ns | 692226ns | +8.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 309015ns | 278961ns | 353483ns | -47.10% | 0.013 |
| carrier_disp_scatter_fntable | 557630ns | 543818ns | 566466ns | -4.53% | 0.007 |
| carrier_disp_scatter_ifchain | 601393ns | 525145ns | 627710ns | +2.96% | 0.007 |
| carrier_disp_scatter_ifchainasc | 601230ns | 571548ns | 626803ns | +2.93% | 0.007 |
| carrier_disp_scatter_ifchainlin | 555603ns | 508293ns | 608015ns | -4.88% | 0.007 |
| carrier_disp_scatter_nullfloor | 118540ns | 114855ns | 122201ns | -79.71% | 0.035 |
| carrier_disp_scatter_switch | 584107ns | 551562ns | 609502ns | base | 0.007 |
| carrier_disp_scatter_threaded | 634440ns | 545371ns | 689306ns | +8.62% | 0.006 |

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_disp_scatter_nullfloor; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_scatter_bittree | 0.014 | 39.2% |
| carrier_disp_scatter_fntable | 0.007 | 20.5% |
| carrier_disp_scatter_ifchain | 0.007 | 18.8% |
| carrier_disp_scatter_ifchainasc | 0.007 | 19.1% |
| carrier_disp_scatter_ifchainlin | 0.007 | 20.9% |
| carrier_disp_scatter_nullfloor | 0.035 | 97.4% |
| carrier_disp_scatter_switch | 0.007 | 19.7% |
| carrier_disp_scatter_threaded | 0.006 | 18.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_scatter_bittree | 311721ns | 311721ns | -46.89% |
| carrier_disp_scatter_fntable | 560514ns | 560514ns | -4.50% |
| carrier_disp_scatter_ifchain | 604206ns | 604206ns | +2.95% |
| carrier_disp_scatter_ifchainasc | 604350ns | 604350ns | +2.97% |
| carrier_disp_scatter_ifchainlin | 558183ns | 558183ns | -4.89% |
| carrier_disp_scatter_nullfloor | 120975ns | 120975ns | -79.39% |
| carrier_disp_scatter_switch | 586907ns | 586907ns | base |
| carrier_disp_scatter_threaded | 637142ns | 637142ns | +8.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_scatter_switch | 582761ns | base | --- | [560059, 609502] | --- | --- | --- | --- |
| carrier_disp_scatter_bittree | 293356ns | -291460.8ns (-50.0%) | [-315460, -218357]ns | [280205, 353483] | YES (adj: no) | 0.1094 | 0.0313 | 0 |
| carrier_disp_scatter_fntable | 559078ns | no significant difference | [-50424, +6407]ns | [547345, 566466] | no | 0.3063 | 0.2188 | 0 |
| carrier_disp_scatter_ifchain | 611448ns | no significant difference | [-35458, +62005]ns | [565022, 627710] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_scatter_ifchainasc | 601877ns | no significant difference | [-14446, +52601]ns | [575010, 626803] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_scatter_ifchainlin | 549112ns | no significant difference | [-66213, +12507]ns | [509684, 608015] | no | 0.3063 | 0.2188 | 0 |
| carrier_disp_scatter_nullfloor | 117921ns | -463559.4ns (-79.5%) | [-492468, -440673]ns | [115499, 122201] | YES (adj: no) | 0.1094 | 0.0313 | 0 |
| carrier_disp_scatter_threaded | 639596ns | no significant difference | [-16689, +93798]ns | [574420, 689306] | no | 0.3063 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_scatter_switch | carrier_disp_scatter_bittree | carrier_disp_scatter_fntable | carrier_disp_scatter_ifchain | carrier_disp_scatter_ifchainasc | carrier_disp_scatter_ifchainlin | carrier_disp_scatter_nullfloor | carrier_disp_scatter_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 585673ns | -51.9% | -5.9% | -10.3% | -0.1% | -7.5% | -79.2% | -6.9% |
| 2 | 579848ns | -48.1% | -6.2% | +7.5% | +9.5% | -4.0% | -79.9% | +12.4% |
| 3 | 596544ns | -53.2% | -6.7% | +2.4% | -4.2% | -14.8% | -80.7% | +1.2% |
| 4 | 551562ns | -48.2% | +2.4% | +14.6% | +4.9% | -7.3% | -77.8% | +13.7% |
| 5 | 568556ns | -30.0% | -0.1% | +6.4% | +8.8% | -0.3% | -79.6% | +18.5% |
| 6 | 622460ns | -50.3% | -9.8% | -1.7% | -0.6% | +4.3% | -80.8% | +13.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_scatter_bittree | -0.090 | ok |
| carrier_disp_scatter_fntable | 0.523 | HIGH+ (drift/warm-up) |
| carrier_disp_scatter_ifchain | -0.140 | ok |
| carrier_disp_scatter_ifchainasc | -0.282 | moderate- |
| carrier_disp_scatter_ifchainlin | 0.195 | ok |
| carrier_disp_scatter_nullfloor | -0.506 | HIGH- (thermal bounce) |
| carrier_disp_scatter_switch | -0.188 | ok |
| carrier_disp_scatter_threaded | 0.039 | ok |

**Consistency summary:**

- **carrier_disp_scatter_bittree**: won 6/6, lost 0/6
- **carrier_disp_scatter_fntable**: won 5/6, lost 1/6
- **carrier_disp_scatter_ifchain**: won 2/6, lost 4/6
- **carrier_disp_scatter_ifchainasc**: won 2/6, lost 3/6
- **carrier_disp_scatter_ifchainlin**: won 5/6, lost 1/6
- **carrier_disp_scatter_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_scatter_threaded**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 309721.8ns | 309014.5ns | 100.2% | HIGH |
| carrier_disp_scatter_fntable | 552896.1ns | 557629.6ns | 99.2% | HIGH |
| carrier_disp_scatter_ifchain | 602120.0ns | 601393.2ns | 100.1% | HIGH |
| carrier_disp_scatter_ifchainasc | 601208.8ns | 601230.0ns | 100.0% | HIGH |
| carrier_disp_scatter_ifchainlin | 557255.9ns | 555603.3ns | 100.3% | HIGH |
| carrier_disp_scatter_nullfloor | 119029.7ns | 118540.3ns | 100.4% | HIGH |
| carrier_disp_scatter_switch | 585625.5ns | 584107.1ns | 100.3% | HIGH |
| carrier_disp_scatter_threaded | 634628.7ns | 634440.4ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_disp_scatter_bittree (n=6, range 278961.2-353482.8 ns)
  278961.2 |########################################
  282687.3 |####################
  286413.4 |
  290139.4 |
  293865.5 |
  297591.6 |####################
  301317.7 |
  305043.7 |
  308769.8 |####################
  312495.9 |
  316222.0 |
  319948.1 |
  323674.1 |
  327400.2 |
  331126.3 |
  334852.4 |
  338578.4 |
  342304.5 |
  346030.6 |
  349756.7 |
  (0 below, 1 above range)

carrier_disp_scatter_fntable (n=6, range 543818.3-566466.2 ns)
  543818.3 |########################################
  544950.7 |
  546083.1 |
  547215.5 |
  548347.9 |
  549480.3 |
  550612.7 |########################################
  551745.1 |
  552877.5 |
  554009.9 |
  555142.3 |
  556274.7 |########################################
  557407.1 |
  558539.5 |
  559671.9 |
  560804.3 |########################################
  561936.7 |
  563069.1 |
  564201.5 |########################################
  565333.9 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchain (n=6, range 525145.4-627709.6 ns)
  525145.4 |####################
  530273.6 |
  535401.8 |
  540530.0 |
  545658.2 |
  550786.4 |
  555914.6 |
  561042.9 |
  566171.1 |
  571299.3 |
  576427.5 |
  581555.7 |
  586683.9 |
  591812.1 |
  596940.3 |
  602068.5 |####################
  607196.7 |########################################
  612324.9 |
  617453.1 |
  622581.3 |####################
  (0 below, 1 above range)

carrier_disp_scatter_ifchainasc (n=6, range 571547.5-626802.9 ns)
  571547.5 |####################
  574310.3 |
  577073.0 |####################
  579835.8 |
  582598.6 |####################
  585361.3 |
  588124.1 |
  590886.9 |
  593649.7 |
  596412.4 |
  599175.2 |
  601938.0 |
  604700.7 |
  607463.5 |
  610226.3 |
  612989.1 |
  615751.8 |
  618514.6 |########################################
  621277.4 |
  624040.1 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchainlin (n=6, range 508293.3-608014.6 ns)
  508293.3 |########################################
  513279.4 |
  518265.4 |
  523251.5 |
  528237.6 |
  533223.6 |
  538209.7 |####################
  543195.7 |
  548181.8 |
  553167.9 |####################
  558153.9 |
  563140.0 |####################
  568126.1 |
  573112.1 |
  578098.2 |
  583084.2 |
  588070.3 |
  593056.4 |
  598042.4 |
  603028.5 |
  (0 below, 1 above range)

carrier_disp_scatter_nullfloor (n=6, range 114854.6-122201.1 ns)
  114854.6 |########################################
  115221.9 |
  115589.2 |
  115956.6 |########################################
  116323.9 |########################################
  116691.2 |
  117058.5 |
  117425.9 |
  117793.2 |
  118160.5 |
  118527.8 |
  118895.1 |########################################
  119262.5 |
  119629.8 |
  119997.1 |
  120364.4 |
  120731.8 |
  121099.1 |
  121466.4 |########################################
  121833.7 |
  (0 below, 1 above range)

carrier_disp_scatter_switch (n=6, range 551561.7-609501.6 ns)
  551561.7 |########################################
  554458.7 |
  557355.7 |
  560252.7 |
  563149.7 |
  566046.7 |########################################
  568943.7 |
  571840.7 |
  574737.7 |
  577634.7 |########################################
  580531.7 |
  583428.7 |########################################
  586325.7 |
  589222.7 |
  592119.7 |
  595016.7 |########################################
  597913.7 |
  600810.7 |
  603707.7 |
  606604.7 |
  (0 below, 1 above range)

carrier_disp_scatter_threaded (n=6, range 545370.8-689305.6 ns)
  545370.8 |########################################
  552567.5 |
  559764.3 |
  566961.0 |
  574157.8 |
  581354.5 |
  588551.3 |
  595748.0 |
  602944.7 |########################################
  610141.5 |
  617338.2 |
  624535.0 |########################################
  631731.7 |
  638928.5 |
  646125.2 |########################################
  653321.9 |
  660518.7 |
  667715.4 |########################################
  674912.2 |
  682108.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_scatter_bittree**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_fntable**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **carrier_disp_scatter_fntable**: bridge=98.5% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchain**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainasc**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainlin**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_nullfloor**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_switch**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_threaded**: bridge=100.0% of algo (FFI overhead may distort results)
