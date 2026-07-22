# Dispatch shape over the wire form, scatter profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_scatter_nullfloor dominates: 41% faster than the next best (carrier_disp_scatter_ifchainasc)

carrier_disp_scatter_nullfloor (30.52 us) leads carrier_disp_scatter_ifchainasc (42.97 us) by 41%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_scatter_nullfloor beats baseline by 32% (significant)

carrier_disp_scatter_nullfloor is -14.13 us (32%) faster than baseline carrier_disp_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_scatter_ifchainlin is an outlier: 3.1x slower than the field

carrier_disp_scatter_ifchainlin (94.69 us) is 3.1x the fastest (30.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_scatter_bittree shows alternating (throttle bounce) (autocorr -0.68)

carrier_disp_scatter_bittree's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_scatter_nullfloor, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_switch, carrier_disp_scatter_ifchain, carrier_disp_scatter_threaded, carrier_disp_scatter_bittree, carrier_disp_scatter_fntable} vs {carrier_disp_scatter_ifchainlin} (69% apart)

The field splits into a fast tier {carrier_disp_scatter_nullfloor, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_switch, carrier_disp_scatter_ifchain, carrier_disp_scatter_threaded, carrier_disp_scatter_bittree, carrier_disp_scatter_fntable} and a slow tier {carrier_disp_scatter_ifchainlin} with a 69% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.1x the fastest

Fastest carrier_disp_scatter_nullfloor (30.52 us) to slowest carrier_disp_scatter_ifchainlin (94.69 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_disp_scatter_switch is inconsistent: worst-20% is 1.5x its best-20%

carrier_disp_scatter_switch's best 20% of batches run at 41.12 us but its worst 20% at 63.37 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: carrier_disp_scatter_nullfloor** at 30519.6 ns median (-30.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.10x (fastest 30519.6 ns, slowest 94694.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 55842ns | 55769ns | 54625ns | 55562ns | 56872ns | +7.32% |
| carrier_disp_scatter_fntable | 57965ns | 58532ns | 55118ns | 58198ns | 59038ns | +11.40% |
| carrier_disp_scatter_ifchain | 50029ns | 46976ns | 45002ns | 46679ns | 57566ns | -3.85% |
| carrier_disp_scatter_ifchainasc | 45366ns | 45441ns | 43624ns | 45299ns | 46338ns | -12.82% |
| carrier_disp_scatter_ifchainlin | 97467ns | 97333ns | 92623ns | 96222ns | 101757ns | +87.31% |
| carrier_disp_scatter_nullfloor | 32732ns | 33012ns | 31121ns | 32711ns | 33569ns | -37.10% |
| carrier_disp_scatter_switch | 52034ns | 46525ns | 43298ns | 45596ns | 66061ns | base |
| carrier_disp_scatter_threaded | 50751ns | 50503ns | 48384ns | 50116ns | 52886ns | -2.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 53296ns | 52042ns | 54376ns | +7.60% | 0.019 |
| carrier_disp_scatter_fntable | 55410ns | 52666ns | 56528ns | +11.87% | 0.018 |
| carrier_disp_scatter_ifchain | 47549ns | 42760ns | 55166ns | -4.00% | 0.022 |
| carrier_disp_scatter_ifchainasc | 42990ns | 41440ns | 43915ns | -13.21% | 0.024 |
| carrier_disp_scatter_ifchainlin | 94891ns | 90193ns | 99098ns | +91.58% | 0.011 |
| carrier_disp_scatter_nullfloor | 30268ns | 28737ns | 31044ns | -38.89% | 0.034 |
| carrier_disp_scatter_switch | 49531ns | 41125ns | 63373ns | base | 0.021 |
| carrier_disp_scatter_threaded | 48252ns | 45947ns | 50367ns | -2.58% | 0.021 |

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_disp_scatter_nullfloor; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_scatter_bittree | 0.019 | 54.0% |
| carrier_disp_scatter_fntable | 0.018 | 51.3% |
| carrier_disp_scatter_ifchain | 0.023 | 64.9% |
| carrier_disp_scatter_ifchainasc | 0.024 | 66.9% |
| carrier_disp_scatter_ifchainlin | 0.011 | 30.3% |
| carrier_disp_scatter_nullfloor | 0.034 | 94.2% |
| carrier_disp_scatter_switch | 0.023 | 65.3% |
| carrier_disp_scatter_threaded | 0.021 | 59.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_scatter_bittree | 55842ns | 55842ns | +7.32% |
| carrier_disp_scatter_fntable | 57965ns | 57965ns | +11.40% |
| carrier_disp_scatter_ifchain | 50029ns | 50029ns | -3.85% |
| carrier_disp_scatter_ifchainasc | 45366ns | 45366ns | -12.82% |
| carrier_disp_scatter_ifchainlin | 97467ns | 97467ns | +87.31% |
| carrier_disp_scatter_nullfloor | 32732ns | 32732ns | -37.10% |
| carrier_disp_scatter_switch | 52034ns | 52034ns | base |
| carrier_disp_scatter_threaded | 50751ns | 50751ns | -2.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_scatter_switch | 43982ns | base | --- | [41237, 63373] | --- | --- | --- | --- |
| carrier_disp_scatter_bittree | 53205ns | no significant difference | [-9799, +12056]ns | [52306, 54376] | no | 0.3063 | 0.2188 | 0 |
| carrier_disp_scatter_fntable | 55998ns | no significant difference | [-9670, +14825]ns | [53703, 56528] | no | 0.3063 | 0.2188 | 0 |
| carrier_disp_scatter_ifchain | 44310ns | no significant difference | [-19334, +11084]ns | [43170, 55166] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_scatter_ifchainasc | 42966ns | no significant difference | [-20609, +2024]ns | [42090, 43915] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_scatter_ifchainlin | 94695ns | +51641.0ns (+117.4%) | [+28183, +56257]ns | [90882, 99098] | YES (adj: no) | 0.1094 | 0.0313 | 0 |
| carrier_disp_scatter_nullfloor | 30520ns | -14134.4ns (-32.1%) | [-32936, -10718]ns | [29240, 31044] | YES (adj: no) | 0.1094 | 0.0313 | 0 |
| carrier_disp_scatter_threaded | 47977ns | no significant difference | [-16235, +6905]ns | [46413, 50367] | no | 0.3063 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_scatter_switch | carrier_disp_scatter_bittree | carrier_disp_scatter_fntable | carrier_disp_scatter_ifchain | carrier_disp_scatter_ifchainasc | carrier_disp_scatter_ifchainlin | carrier_disp_scatter_nullfloor | carrier_disp_scatter_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 43408ns | +21.1% | +28.9% | +32.5% | -4.5% | +107.8% | -33.8% | +8.2% |
| 2 | 81989ns | -32.8% | -35.8% | -46.0% | -47.9% | +11.7% | -62.3% | -44.0% |
| 3 | 44757ns | +16.3% | +22.3% | +18.0% | -4.1% | +111.4% | -30.4% | +11.7% |
| 4 | 44556ns | +20.0% | +27.9% | -2.2% | -0.5% | +126.4% | -33.2% | +13.9% |
| 5 | 41125ns | +30.4% | +36.4% | +4.0% | +5.8% | +136.6% | -25.0% | +14.0% |
| 6 | 41350ns | +28.0% | +35.5% | +7.2% | +4.0% | +129.1% | -27.0% | +18.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_scatter_bittree | -0.684 | HIGH- (thermal bounce) |
| carrier_disp_scatter_fntable | 0.067 | ok |
| carrier_disp_scatter_ifchain | -0.193 | ok |
| carrier_disp_scatter_ifchainasc | 0.223 | moderate+ |
| carrier_disp_scatter_ifchainlin | 0.387 | moderate+ |
| carrier_disp_scatter_nullfloor | -0.296 | moderate- |
| carrier_disp_scatter_switch | -0.172 | ok |
| carrier_disp_scatter_threaded | -0.062 | ok |

**Consistency summary:**

- **carrier_disp_scatter_bittree**: won 1/6, lost 5/6
- **carrier_disp_scatter_fntable**: won 1/6, lost 5/6
- **carrier_disp_scatter_ifchain**: won 2/6, lost 4/6
- **carrier_disp_scatter_ifchainasc**: won 4/6, lost 2/6
- **carrier_disp_scatter_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_scatter_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_scatter_threaded**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 106753.2ns | 53296.0ns | 200.3% | HIGH |
| carrier_disp_scatter_fntable | 111259.8ns | 55409.7ns | 200.8% | HIGH |
| carrier_disp_scatter_ifchain | 97157.0ns | 47548.6ns | 204.3% | HIGH |
| carrier_disp_scatter_ifchainasc | 97724.7ns | 42990.1ns | 227.3% | HIGH |
| carrier_disp_scatter_ifchainlin | 95268.5ns | 94891.4ns | 100.4% | HIGH |
| carrier_disp_scatter_nullfloor | 92480.5ns | 30268.0ns | 305.5% | HIGH |
| carrier_disp_scatter_switch | 106591.7ns | 49530.8ns | 215.2% | HIGH |
| carrier_disp_scatter_threaded | 96677.9ns | 48252.4ns | 200.4% | HIGH |

## Distribution (algo ns)

```
carrier_disp_scatter_bittree (n=6, range 52041.7-54376.4 ns)
  52041.7 |########################################
  52158.4 |
  52275.2 |
  52391.9 |
  52508.6 |########################################
  52625.4 |
  52742.1 |
  52858.9 |########################################
  52975.6 |
  53092.3 |
  53209.1 |
  53325.8 |
  53442.5 |########################################
  53559.3 |########################################
  53676.0 |
  53792.8 |
  53909.5 |
  54026.2 |
  54143.0 |
  54259.7 |
  (0 below, 1 above range)

carrier_disp_scatter_fntable (n=6, range 52666.2-56527.9 ns)
  52666.2 |#############
  52859.3 |
  53052.4 |
  53245.5 |
  53438.5 |
  53631.6 |
  53824.7 |
  54017.8 |
  54210.9 |
  54404.0 |
  54597.1 |#############
  54790.2 |
  54983.2 |
  55176.3 |
  55369.4 |
  55562.5 |
  55755.6 |
  55948.7 |########################################
  56141.8 |
  56334.9 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchain (n=6, range 42759.6-55166.4 ns)
  42759.6 |####################
  43379.9 |####################
  44000.3 |########################################
  44620.6 |
  45241.0 |
  45861.3 |
  46481.7 |
  47102.0 |
  47722.3 |
  48342.7 |
  48963.0 |
  49583.4 |
  50203.7 |
  50824.1 |
  51444.4 |
  52064.7 |
  52685.1 |####################
  53305.4 |
  53925.8 |
  54546.1 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchainasc (n=6, range 41440.0-43915.0 ns)
  41440.0 |########################################
  41563.8 |
  41687.5 |
  41811.2 |
  41935.0 |
  42058.8 |
  42182.5 |
  42306.2 |
  42430.0 |
  42553.8 |
  42677.5 |########################################
  42801.2 |########################################
  42925.0 |########################################
  43048.8 |
  43172.5 |
  43296.2 |
  43420.0 |########################################
  43543.8 |
  43667.5 |
  43791.2 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchainlin (n=6, range 90192.9-99097.5 ns)
  90192.9 |########################################
  90638.1 |
  91083.4 |
  91528.6 |########################################
  91973.8 |
  92419.0 |
  92864.3 |
  93309.5 |
  93754.7 |
  94200.0 |########################################
  94645.2 |########################################
  95090.4 |
  95535.7 |
  95980.9 |
  96426.1 |
  96871.4 |########################################
  97316.6 |
  97761.8 |
  98207.0 |
  98652.3 |
  (0 below, 1 above range)

carrier_disp_scatter_nullfloor (n=6, range 28737.1-31044.4 ns)
  28737.1 |########################################
  28852.5 |
  28967.8 |
  29083.2 |
  29198.6 |
  29313.9 |
  29429.3 |
  29544.7 |
  29660.0 |########################################
  29775.4 |
  29890.8 |
  30006.1 |
  30121.5 |########################################
  30236.8 |
  30352.2 |
  30467.6 |
  30582.9 |
  30698.3 |
  30813.7 |########################################
  30929.0 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_switch (n=6, range 41124.6-63373.1 ns)
  41124.6 |########################################
  42237.0 |
  43349.5 |####################
  44461.9 |########################################
  45574.3 |
  46686.7 |
  47799.2 |
  48911.6 |
  50024.0 |
  51136.4 |
  52248.9 |
  53361.3 |
  54473.7 |
  55586.2 |
  56698.6 |
  57811.0 |
  58923.4 |
  60035.9 |
  61148.3 |
  62260.7 |
  (0 below, 1 above range)

carrier_disp_scatter_threaded (n=6, range 45946.7-50367.3 ns)
  45946.7 |####################
  46167.7 |
  46388.8 |
  46609.8 |
  46830.8 |########################################
  47051.8 |
  47272.9 |
  47493.9 |
  47714.9 |
  47936.0 |
  48157.0 |
  48378.0 |
  48599.1 |
  48820.1 |####################
  49041.1 |
  49262.2 |
  49483.2 |
  49704.2 |
  49925.2 |####################
  50146.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_scatter_bittree**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_fntable**: bridge=201.2% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchain**: bridge=206.3% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainasc**: bridge=219.6% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainlin**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_nullfloor**: bridge=304.5% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_switch**: CV=29.4% (high variance, measurements may be unstable)
- **carrier_disp_scatter_switch**: bridge=236.0% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_threaded**: bridge=200.6% of algo (FFI overhead may distort results)
