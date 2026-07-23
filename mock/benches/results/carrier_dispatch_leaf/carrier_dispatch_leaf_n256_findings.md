# Dispatch shape over the wire form, leaf profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_leaf_nullfloor dominates: 33% faster than the next best (carrier_disp_leaf_switch)

carrier_disp_leaf_nullfloor (6.97 us) leads carrier_disp_leaf_switch (9.28 us) by 33%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_leaf_nullfloor beats baseline by 23% (significant)

carrier_disp_leaf_nullfloor is -2.18 us (23%) faster than baseline carrier_disp_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_leaf_threaded shows alternating (throttle bounce) (autocorr -0.61)

carrier_disp_leaf_threaded's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_leaf_nullfloor} vs {carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchain, carrier_disp_leaf_bittree, carrier_disp_leaf_threaded, carrier_disp_leaf_fntable, carrier_disp_leaf_ifchainlin} (33% apart)

The field splits into a fast tier {carrier_disp_leaf_nullfloor} and a slow tier {carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchain, carrier_disp_leaf_bittree, carrier_disp_leaf_threaded, carrier_disp_leaf_fntable, carrier_disp_leaf_ifchainlin} with a 33% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_leaf_nullfloor** at 6975.0 ns median (-24.8% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.72x (fastest 6975.0 ns, slowest 12020.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 12289ns | 12496ns | 11429ns | 12346ns | 12633ns | +4.08% |
| carrier_disp_leaf_fntable | 14104ns | 14001ns | 13045ns | 13755ns | 15157ns | +19.45% |
| carrier_disp_leaf_ifchain | 12149ns | 12038ns | 10647ns | 11971ns | 13168ns | +2.90% |
| carrier_disp_leaf_ifchainasc | 11721ns | 11803ns | 10997ns | 11692ns | 12125ns | -0.73% |
| carrier_disp_leaf_ifchainlin | 14409ns | 14502ns | 13555ns | 14392ns | 14864ns | +22.04% |
| carrier_disp_leaf_nullfloor | 9325ns | 9409ns | 8693ns | 9299ns | 9679ns | -21.03% |
| carrier_disp_leaf_switch | 11807ns | 11688ns | 10728ns | 11650ns | 12583ns | base |
| carrier_disp_leaf_threaded | 13255ns | 13539ns | 12485ns | 13226ns | 13685ns | +12.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 9862ns | 9240ns | 10106ns | +4.95% | 0.026 |
| carrier_disp_leaf_fntable | 11727ns | 10874ns | 12567ns | +24.79% | 0.022 |
| carrier_disp_leaf_ifchain | 9740ns | 8484ns | 10676ns | +3.65% | 0.026 |
| carrier_disp_leaf_ifchainasc | 9284ns | 8729ns | 9591ns | -1.21% | 0.028 |
| carrier_disp_leaf_ifchainlin | 12005ns | 11298ns | 12428ns | +27.74% | 0.021 |
| carrier_disp_leaf_nullfloor | 6948ns | 6530ns | 7235ns | -26.07% | 0.037 |
| carrier_disp_leaf_switch | 9397ns | 8552ns | 10071ns | base | 0.027 |
| carrier_disp_leaf_threaded | 10895ns | 10218ns | 11252ns | +15.93% | 0.023 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 303255 | 1436853 | 0.211 | 1.01× |
| carrier_disp_leaf_fntable | 313594 | 1711849 | 0.183 | 1.05× |
| carrier_disp_leaf_ifchain | 301396 | 1504025 | 0.200 | 1.00× |
| carrier_disp_leaf_ifchainasc | 300230 | 1534039 | 0.196 | 1.00× |
| carrier_disp_leaf_ifchainlin | 316011 | 1648433 | 0.192 | 1.05× |
| carrier_disp_leaf_nullfloor | 291252 | 1605100 | 0.181 | 0.97× |
| carrier_disp_leaf_switch | 299962 | 1503709 | 0.199 | 1.00× |
| carrier_disp_leaf_threaded | 309843 | 1836368 | 0.169 | 1.03× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.039 Gops/s** (carrier_disp_leaf_nullfloor; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_leaf_bittree | 0.026 | 65.2% |
| carrier_disp_leaf_fntable | 0.022 | 56.0% |
| carrier_disp_leaf_ifchain | 0.027 | 68.0% |
| carrier_disp_leaf_ifchainasc | 0.027 | 69.7% |
| carrier_disp_leaf_ifchainlin | 0.021 | 54.3% |
| carrier_disp_leaf_nullfloor | 0.037 | 93.6% |
| carrier_disp_leaf_switch | 0.028 | 70.4% |
| carrier_disp_leaf_threaded | 0.023 | 58.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_leaf_bittree | 12289ns | 12289ns | +4.08% |
| carrier_disp_leaf_fntable | 14104ns | 14104ns | +19.45% |
| carrier_disp_leaf_ifchain | 12149ns | 12149ns | +2.90% |
| carrier_disp_leaf_ifchainasc | 11721ns | 11721ns | -0.73% |
| carrier_disp_leaf_ifchainlin | 14409ns | 14409ns | +22.04% |
| carrier_disp_leaf_nullfloor | 9325ns | 9325ns | -21.03% |
| carrier_disp_leaf_switch | 11807ns | 11807ns | base |
| carrier_disp_leaf_threaded | 13255ns | 13255ns | +12.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_leaf_switch | 9280ns | base | --- | [8841, 10071] | --- | --- | --- | --- |
| carrier_disp_leaf_bittree | 10016ns | no significant difference | [-434, +1070]ns | [9464, 10106] | no | 0.2552 | 0.2188 | 0 |
| carrier_disp_leaf_fntable | 11658ns | +2693.1ns (+29.0%) | [+958, +3337]ns | [10954, 12567] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_leaf_ifchain | 9598ns | no significant difference | [-20, +773]ns | [8947, 10676] | no | 0.2552 | 0.2188 | 0 |
| carrier_disp_leaf_ifchainasc | 9370ns | no significant difference | [-1179, +698]ns | [8892, 9591] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_leaf_ifchainlin | 12021ns | +2750.9ns (+29.6%) | [+1847, +3224]ns | [11565, 12428] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_leaf_nullfloor | 6975ns | -2180.4ns (-23.5%) | [-3317, -1851]ns | [6633, 7235] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_leaf_threaded | 11105ns | +1883.4ns (+20.3%) | [+587, +2021]ns | [10328, 11252] | YES (adj: no) | 0.2552 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_leaf_switch | carrier_disp_leaf_bittree | carrier_disp_leaf_fntable | carrier_disp_leaf_ifchain | carrier_disp_leaf_ifchainasc | carrier_disp_leaf_ifchainlin | carrier_disp_leaf_nullfloor | carrier_disp_leaf_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 9330ns | +7.5% | +34.6% | +12.7% | -2.9% | +26.8% | -24.8% | +20.6% |
| 2 | 8552ns | +13.3% | +29.0% | -0.8% | +10.3% | +32.1% | -21.2% | +22.0% |
| 3 | 9277ns | +8.6% | +31.3% | +2.9% | +2.8% | +35.6% | -22.1% | +19.1% |
| 4 | 9283ns | +7.7% | +20.0% | +3.9% | +0.3% | +29.7% | -25.3% | +20.3% |
| 5 | 10811ns | -14.5% | +0.6% | +0.3% | -19.3% | +11.0% | -39.6% | -5.5% |
| 6 | 9130ns | +11.0% | +37.8% | +3.1% | +5.7% | +34.4% | -20.7% | +23.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_leaf_bittree | -0.499 | moderate- |
| carrier_disp_leaf_fntable | -0.434 | moderate- |
| carrier_disp_leaf_ifchain | -0.333 | moderate- |
| carrier_disp_leaf_ifchainasc | -0.356 | moderate- |
| carrier_disp_leaf_ifchainlin | -0.285 | moderate- |
| carrier_disp_leaf_nullfloor | -0.500 | HIGH- (thermal bounce) |
| carrier_disp_leaf_switch | -0.130 | ok |
| carrier_disp_leaf_threaded | -0.605 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_leaf_bittree**: won 1/6, lost 5/6
- **carrier_disp_leaf_fntable**: won 0/6, lost 6/6
- **carrier_disp_leaf_ifchain**: won 1/6, lost 5/6
- **carrier_disp_leaf_ifchainasc**: won 2/6, lost 4/6
- **carrier_disp_leaf_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_leaf_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_leaf_threaded**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 90645.2ns | 9862.1ns | 919.1% | HIGH |
| carrier_disp_leaf_fntable | 90107.1ns | 11726.7ns | 768.4% | HIGH |
| carrier_disp_leaf_ifchain | 91204.9ns | 9740.2ns | 936.4% | HIGH |
| carrier_disp_leaf_ifchainasc | 89625.7ns | 9284.1ns | 965.4% | HIGH |
| carrier_disp_leaf_ifchainlin | 91956.9ns | 12004.6ns | 766.0% | HIGH |
| carrier_disp_leaf_nullfloor | 89049.7ns | 6947.8ns | 1281.7% | HIGH |
| carrier_disp_leaf_switch | 91312.9ns | 9397.4ns | 971.7% | HIGH |
| carrier_disp_leaf_threaded | 90475.7ns | 10894.8ns | 830.4% | HIGH |

## Distribution (algo ns)

```
carrier_disp_leaf_bittree (n=6, range 9239.6-10106.5 ns)
   9239.6 |########################################
   9282.9 |
   9326.3 |
   9369.6 |
   9413.0 |
   9456.3 |
   9499.7 |
   9543.0 |
   9586.3 |
   9629.7 |
   9673.0 |########################################
   9716.4 |
   9759.7 |
   9803.1 |
   9846.4 |
   9889.7 |
   9933.1 |
   9976.4 |########################################
  10019.8 |########################################
  10063.1 |########################################
  (0 below, 1 above range)

carrier_disp_leaf_fntable (n=6, range 10874.2-12567.3 ns)
  10874.2 |########################################
  10958.9 |########################################
  11043.5 |
  11128.2 |########################################
  11212.8 |
  11297.5 |
  11382.1 |
  11466.8 |
  11551.4 |
  11636.1 |
  11720.8 |
  11805.4 |
  11890.1 |
  11974.7 |
  12059.4 |
  12144.0 |########################################
  12228.7 |
  12313.3 |
  12398.0 |
  12482.6 |########################################
  (0 below, 1 above range)

carrier_disp_leaf_ifchain (n=6, range 8483.8-10675.6 ns)
   8483.8 |########################################
   8593.4 |
   8703.0 |
   8812.6 |
   8922.2 |
   9031.8 |
   9141.3 |
   9250.9 |
   9360.5 |########################################
   9470.1 |########################################
   9579.7 |########################################
   9689.3 |
   9798.9 |
   9908.5 |
  10018.1 |
  10127.6 |
  10237.2 |
  10346.8 |
  10456.4 |########################################
  10566.0 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchainasc (n=6, range 8728.8-9590.7 ns)
   8728.8 |########################################
   8771.9 |
   8815.0 |
   8858.1 |
   8901.2 |
   8944.3 |
   8987.4 |
   9030.4 |########################################
   9073.5 |
   9116.6 |
   9159.7 |
   9202.8 |
   9245.9 |
   9289.0 |########################################
   9332.1 |
   9375.2 |
   9418.3 |########################################
   9461.4 |
   9504.5 |########################################
   9547.6 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchainlin (n=6, range 11297.9-12428.0 ns)
  11297.9 |########################################
  11354.4 |
  11410.9 |
  11467.4 |
  11523.9 |
  11580.4 |
  11636.9 |
  11693.4 |
  11749.9 |
  11806.4 |########################################
  11862.9 |
  11919.4 |
  11975.9 |########################################
  12032.4 |########################################
  12088.9 |
  12145.4 |
  12201.9 |
  12258.4 |########################################
  12314.9 |
  12371.4 |
  (0 below, 1 above range)

carrier_disp_leaf_nullfloor (n=6, range 6529.6-7235.2 ns)
   6529.6 |########################################
   6564.9 |
   6600.2 |
   6635.4 |
   6670.7 |
   6706.0 |########################################
   6741.3 |
   6776.6 |
   6811.8 |
   6847.1 |
   6882.4 |
   6917.7 |########################################
   6953.0 |
   6988.2 |########################################
   7023.5 |
   7058.8 |
   7094.1 |
   7129.4 |
   7164.6 |
   7199.9 |########################################
  (0 below, 1 above range)

carrier_disp_leaf_switch (n=6, range 8552.5-10070.6 ns)
   8552.5 |####################
   8628.4 |
   8704.3 |
   8780.2 |
   8856.1 |
   8932.0 |
   9007.9 |
   9083.8 |####################
   9159.7 |
   9235.6 |########################################
   9311.5 |####################
   9387.5 |
   9463.4 |
   9539.3 |
   9615.2 |
   9691.1 |
   9767.0 |
   9842.9 |
   9918.8 |
   9994.7 |
  (0 below, 1 above range)

carrier_disp_leaf_threaded (n=6, range 10217.9-11251.7 ns)
  10217.9 |########################################
  10269.6 |
  10321.3 |
  10373.0 |
  10424.6 |########################################
  10476.3 |
  10528.0 |
  10579.7 |
  10631.4 |
  10683.1 |
  10734.8 |
  10786.5 |
  10838.2 |
  10889.8 |
  10941.5 |
  10993.2 |
  11044.9 |########################################
  11096.6 |
  11148.3 |########################################
  11200.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_leaf_bittree**: bridge=904.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_fntable**: bridge=770.5% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchain**: bridge=945.0% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainasc**: bridge=956.1% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainlin**: bridge=763.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_nullfloor**: bridge=1271.6% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_switch**: bridge=986.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_threaded**: bridge=812.4% of algo (FFI overhead may distort results)
