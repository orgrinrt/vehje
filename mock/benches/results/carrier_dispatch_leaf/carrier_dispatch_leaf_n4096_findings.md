# Dispatch shape over the wire form, leaf profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_leaf_nullfloor dominates: 53% faster than the next best (carrier_disp_leaf_bittree)

carrier_disp_leaf_nullfloor (109.12 us) leads carrier_disp_leaf_bittree (166.52 us) by 53%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_leaf_nullfloor beats baseline by 51% (significant)

carrier_disp_leaf_nullfloor is -115.36 us (51%) faster than baseline carrier_disp_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_leaf_fntable is an outlier: 2.4x slower than the field

carrier_disp_leaf_fntable (263.17 us) is 2.4x the fastest (109.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_disp_leaf_nullfloor} vs {carrier_disp_leaf_bittree, carrier_disp_leaf_ifchainlin, carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_threaded, carrier_disp_leaf_ifchain, carrier_disp_leaf_fntable} (53% apart)

The field splits into a fast tier {carrier_disp_leaf_nullfloor} and a slow tier {carrier_disp_leaf_bittree, carrier_disp_leaf_ifchainlin, carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_threaded, carrier_disp_leaf_ifchain, carrier_disp_leaf_fntable} with a 53% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_leaf_nullfloor** at 109118.9 ns median (-51.7% vs baseline)
- 3 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.41x (fastest 109118.9 ns, slowest 263168.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 169201ns | 169238ns | 167120ns | 168733ns | 170945ns | -25.78% |
| carrier_disp_leaf_fntable | 266155ns | 265846ns | 264958ns | 265708ns | 267423ns | +16.75% |
| carrier_disp_leaf_ifchain | 244819ns | 243169ns | 241042ns | 242687ns | 249907ns | +7.39% |
| carrier_disp_leaf_ifchainasc | 235663ns | 234046ns | 229052ns | 233382ns | 242390ns | +3.37% |
| carrier_disp_leaf_ifchainlin | 215028ns | 215082ns | 210396ns | 213776ns | 219221ns | -5.68% |
| carrier_disp_leaf_nullfloor | 112194ns | 111519ns | 110184ns | 111160ns | 114749ns | -50.79% |
| carrier_disp_leaf_switch | 227977ns | 228335ns | 221439ns | 228081ns | 231089ns | base |
| carrier_disp_leaf_threaded | 233416ns | 235574ns | 216842ns | 234353ns | 240296ns | +2.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 166498ns | 164436ns | 168286ns | -26.15% | 0.025 |
| carrier_disp_leaf_fntable | 263479ns | 262694ns | 264513ns | +16.87% | 0.016 |
| carrier_disp_leaf_ifchain | 242330ns | 238729ns | 247170ns | +7.49% | 0.017 |
| carrier_disp_leaf_ifchainasc | 233044ns | 226807ns | 239530ns | +3.37% | 0.018 |
| carrier_disp_leaf_ifchainlin | 212468ns | 208222ns | 216655ns | -5.75% | 0.019 |
| carrier_disp_leaf_nullfloor | 109747ns | 107722ns | 112282ns | -51.32% | 0.037 |
| carrier_disp_leaf_switch | 225440ns | 218870ns | 228527ns | base | 0.018 |
| carrier_disp_leaf_threaded | 230657ns | 214499ns | 237488ns | +2.31% | 0.018 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 1048361 | 4536177 | 0.231 | 0.74× |
| carrier_disp_leaf_fntable | 1673522 | 6434063 | 0.260 | 1.18× |
| carrier_disp_leaf_ifchain | 1528506 | 4626609 | 0.330 | 1.08× |
| carrier_disp_leaf_ifchainasc | 1461741 | 4626548 | 0.316 | 1.03× |
| carrier_disp_leaf_ifchainlin | 1341332 | 6034642 | 0.222 | 0.95× |
| carrier_disp_leaf_nullfloor | 692320 | 3763165 | 0.184 | 0.49× |
| carrier_disp_leaf_switch | 1415594 | 4495856 | 0.315 | 1.00× |
| carrier_disp_leaf_threaded | 1457254 | 6330475 | 0.230 | 1.03× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.038 Gops/s** (carrier_disp_leaf_nullfloor; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_leaf_bittree | 0.025 | 64.7% |
| carrier_disp_leaf_fntable | 0.016 | 40.9% |
| carrier_disp_leaf_ifchain | 0.017 | 44.7% |
| carrier_disp_leaf_ifchainasc | 0.018 | 46.5% |
| carrier_disp_leaf_ifchainlin | 0.019 | 50.7% |
| carrier_disp_leaf_nullfloor | 0.038 | 98.7% |
| carrier_disp_leaf_switch | 0.018 | 47.7% |
| carrier_disp_leaf_threaded | 0.018 | 46.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_leaf_bittree | 169201ns | 169201ns | -25.78% |
| carrier_disp_leaf_fntable | 266155ns | 266155ns | +16.75% |
| carrier_disp_leaf_ifchain | 244819ns | 244819ns | +7.39% |
| carrier_disp_leaf_ifchainasc | 235663ns | 235663ns | +3.37% |
| carrier_disp_leaf_ifchainlin | 215028ns | 215028ns | -5.68% |
| carrier_disp_leaf_nullfloor | 112194ns | 112194ns | -50.79% |
| carrier_disp_leaf_switch | 227977ns | 227977ns | base |
| carrier_disp_leaf_threaded | 233416ns | 233416ns | +2.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_leaf_switch | 225896ns | base | --- | [221897, 228527] | --- | --- | --- | --- |
| carrier_disp_leaf_bittree | 166519ns | -58049.1ns (-25.7%) | [-63218, -55560]ns | [164688, 168286] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_fntable | 263169ns | +37735.5ns (+16.7%) | [+35289, +41094]ns | [262757, 264513] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_ifchain | 240906ns | +14659.2ns (+6.5%) | [+12105, +23908]ns | [238915, 247170] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_ifchainasc | 231583ns | +4870.6ns (+2.2%) | [+307, +17634]ns | [228017, 239530] | YES (adj: no) | 0.2552 | 0.2188 | 0 |
| carrier_disp_leaf_ifchainlin | 212357ns | -12466.4ns (-5.5%) | [-17000, -9449]ns | [208391, 216655] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_nullfloor | 109119ns | -115360.2ns (-51.1%) | [-119578, -112140]ns | [107839, 112282] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_leaf_threaded | 232720ns | no significant difference | [-2347, +11592]ns | [221763, 237488] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_leaf_switch | carrier_disp_leaf_bittree | carrier_disp_leaf_fntable | carrier_disp_leaf_ifchain | carrier_disp_leaf_ifchainasc | carrier_disp_leaf_ifchainlin | carrier_disp_leaf_nullfloor | carrier_disp_leaf_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 225772ns | -25.1% | +17.0% | +5.9% | +2.4% | -4.9% | -51.3% | +4.8% |
| 2 | 229402ns | -27.8% | +14.6% | +4.7% | -1.1% | -6.0% | -52.8% | -0.1% |
| 3 | 226019ns | -26.0% | +16.4% | +6.9% | +1.4% | -3.7% | -52.2% | +5.5% |
| 4 | 224923ns | -25.5% | +17.0% | +6.1% | +4.3% | -7.3% | -51.1% | +1.8% |
| 5 | 218870ns | -24.9% | +20.0% | +11.9% | +11.7% | -4.9% | -50.8% | -2.0% |
| 6 | 227653ns | -27.5% | +16.4% | +9.6% | +1.9% | -7.7% | -49.7% | +3.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_leaf_bittree | -0.050 | ok |
| carrier_disp_leaf_fntable | -0.271 | moderate- |
| carrier_disp_leaf_ifchain | 0.236 | moderate+ |
| carrier_disp_leaf_ifchainasc | 0.187 | ok |
| carrier_disp_leaf_ifchainlin | 0.360 | moderate+ |
| carrier_disp_leaf_nullfloor | -0.254 | moderate- |
| carrier_disp_leaf_switch | -0.121 | ok |
| carrier_disp_leaf_threaded | -0.254 | moderate- |

**Consistency summary:**

- **carrier_disp_leaf_bittree**: won 6/6, lost 0/6
- **carrier_disp_leaf_fntable**: won 0/6, lost 6/6
- **carrier_disp_leaf_ifchain**: won 0/6, lost 6/6
- **carrier_disp_leaf_ifchainasc**: won 1/6, lost 5/6
- **carrier_disp_leaf_ifchainlin**: won 6/6, lost 0/6
- **carrier_disp_leaf_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_leaf_threaded**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 166996.1ns | 166497.6ns | 100.3% | HIGH |
| carrier_disp_leaf_fntable | 268876.4ns | 263479.4ns | 102.0% | HIGH |
| carrier_disp_leaf_ifchain | 243006.1ns | 242330.4ns | 100.3% | HIGH |
| carrier_disp_leaf_ifchainasc | 233860.9ns | 233043.6ns | 100.4% | HIGH |
| carrier_disp_leaf_ifchainlin | 213060.3ns | 212467.8ns | 100.3% | HIGH |
| carrier_disp_leaf_nullfloor | 110243.8ns | 109746.9ns | 100.5% | HIGH |
| carrier_disp_leaf_switch | 226249.4ns | 225439.8ns | 100.4% | HIGH |
| carrier_disp_leaf_threaded | 231658.6ns | 230656.8ns | 100.4% | HIGH |

## Distribution (algo ns)

```
carrier_disp_leaf_bittree (n=6, range 164435.8-168286.5 ns)
  164435.8 |####################
  164628.3 |
  164820.9 |####################
  165013.4 |
  165205.9 |
  165398.5 |
  165591.0 |####################
  165783.5 |
  165976.1 |
  166168.6 |
  166361.1 |
  166553.7 |
  166746.2 |
  166938.7 |
  167131.3 |
  167323.8 |########################################
  167516.3 |
  167708.9 |
  167901.4 |
  168093.9 |
  (0 below, 1 above range)

carrier_disp_leaf_fntable (n=6, range 262693.7-264512.7 ns)
  262693.7 |####################
  262784.7 |####################
  262875.6 |
  262966.5 |
  263057.5 |
  263148.5 |########################################
  263239.4 |
  263330.4 |
  263421.3 |
  263512.2 |
  263603.2 |
  263694.2 |
  263785.1 |
  263876.0 |
  263967.0 |
  264058.0 |####################
  264148.9 |
  264239.9 |
  264330.8 |
  264421.8 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchain (n=6, range 238729.2-247169.6 ns)
  238729.2 |########################################
  239151.2 |
  239573.2 |
  239995.3 |####################
  240417.3 |
  240839.3 |
  241261.3 |####################
  241683.3 |
  242105.4 |
  242527.4 |
  242949.4 |
  243371.4 |
  243793.4 |
  244215.5 |
  244637.5 |####################
  245059.5 |
  245481.5 |
  245903.5 |
  246325.6 |
  246747.6 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchainasc (n=6, range 226807.1-239530.4 ns)
  226807.1 |########################################
  227443.3 |
  228079.4 |
  228715.6 |########################################
  229351.8 |
  229987.9 |
  230624.1 |########################################
  231260.3 |
  231896.4 |########################################
  232532.6 |
  233168.8 |
  233804.9 |
  234441.1 |########################################
  235077.2 |
  235713.4 |
  236349.6 |
  236985.7 |
  237621.9 |
  238258.1 |
  238894.2 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchainlin (n=6, range 208221.7-216655.4 ns)
  208221.7 |########################################
  208643.4 |
  209065.1 |
  209486.8 |
  209908.4 |####################
  210330.1 |
  210751.8 |
  211173.5 |
  211595.2 |
  212016.9 |
  212438.6 |
  212860.2 |
  213281.9 |
  213703.6 |
  214125.3 |
  214547.0 |####################
  214968.7 |
  215390.3 |####################
  215812.0 |
  216233.7 |
  (0 below, 1 above range)

carrier_disp_leaf_nullfloor (n=6, range 107722.5-112282.5 ns)
  107722.5 |########################################
  107950.5 |########################################
  108178.5 |########################################
  108406.5 |
  108634.5 |
  108862.5 |
  109090.5 |
  109318.5 |
  109546.5 |
  109774.5 |########################################
  110002.5 |########################################
  110230.5 |
  110458.5 |
  110686.5 |
  110914.5 |
  111142.5 |
  111370.5 |
  111598.5 |
  111826.5 |
  112054.5 |
  (0 below, 1 above range)

carrier_disp_leaf_switch (n=6, range 218870.4-228527.3 ns)
  218870.4 |####################
  219353.2 |
  219836.1 |
  220318.9 |
  220801.8 |
  221284.6 |
  221767.5 |
  222250.3 |
  222733.2 |
  223216.0 |
  223698.8 |
  224181.7 |
  224664.5 |####################
  225147.4 |
  225630.2 |########################################
  226113.1 |
  226595.9 |
  227078.8 |
  227561.6 |####################
  228044.5 |
  (0 below, 1 above range)

carrier_disp_leaf_threaded (n=6, range 214498.8-237487.5 ns)
  214498.8 |####################
  215648.2 |
  216797.7 |
  217947.1 |
  219096.5 |
  220246.0 |
  221395.4 |
  222544.8 |
  223694.3 |
  224843.7 |
  225993.1 |
  227142.6 |
  228292.0 |########################################
  229441.5 |
  230590.9 |
  231740.3 |
  232889.8 |
  234039.2 |
  235188.6 |
  236338.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_leaf_bittree**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_fntable**: bridge=102.0% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchain**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainasc**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainlin**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_nullfloor**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_switch**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_threaded**: bridge=100.4% of algo (FFI overhead may distort results)
