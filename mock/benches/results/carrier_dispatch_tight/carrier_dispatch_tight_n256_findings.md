# Dispatch shape over the wire form, tight profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_tight_nullfloor dominates: 30% faster than the next best (carrier_disp_tight_switch)

carrier_disp_tight_nullfloor (7.83 us) leads carrier_disp_tight_switch (10.18 us) by 30%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_tight_nullfloor beats baseline by 24% (significant)

carrier_disp_tight_nullfloor is -2.44 us (24%) faster than baseline carrier_disp_tight_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_tight_ifchainlin is an outlier: 2.0x slower than the field

carrier_disp_tight_ifchainlin (16.02 us) is 2.0x the fastest (7.83 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_tight_ifchain shows alternating (throttle bounce) (autocorr -0.84)

carrier_disp_tight_ifchain's per-pass series has lag-1 autocorrelation -0.84, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_tight_nullfloor, carrier_disp_tight_switch, carrier_disp_tight_ifchainasc, carrier_disp_tight_ifchain, carrier_disp_tight_bittree, carrier_disp_tight_threaded, carrier_disp_tight_fntable} vs {carrier_disp_tight_ifchainlin} (32% apart)

The field splits into a fast tier {carrier_disp_tight_nullfloor, carrier_disp_tight_switch, carrier_disp_tight_ifchainasc, carrier_disp_tight_ifchain, carrier_disp_tight_bittree, carrier_disp_tight_threaded, carrier_disp_tight_fntable} and a slow tier {carrier_disp_tight_ifchainlin} with a 32% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_tight_nullfloor** at 7831.1 ns median (-23.1% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 2.05x (fastest 7831.1 ns, slowest 16022.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 13291ns | 13218ns | 12505ns | 12994ns | 14129ns | +4.85% |
| carrier_disp_tight_fntable | 14701ns | 14482ns | 13791ns | 14256ns | 15825ns | +15.98% |
| carrier_disp_tight_ifchain | 13062ns | 12967ns | 12220ns | 12719ns | 13996ns | +3.04% |
| carrier_disp_tight_ifchainasc | 12993ns | 12694ns | 12155ns | 12526ns | 14113ns | +2.50% |
| carrier_disp_tight_ifchainlin | 18547ns | 18373ns | 17557ns | 18170ns | 19606ns | +46.31% |
| carrier_disp_tight_nullfloor | 10235ns | 10221ns | 9555ns | 10004ns | 10923ns | -19.25% |
| carrier_disp_tight_switch | 12676ns | 12477ns | 12048ns | 12353ns | 13476ns | base |
| carrier_disp_tight_threaded | 14182ns | 14154ns | 13326ns | 13886ns | 15055ns | +11.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 10945ns | 10344ns | 11651ns | +5.72% | 0.023 |
| carrier_disp_tight_fntable | 12340ns | 11612ns | 13280ns | +19.21% | 0.021 |
| carrier_disp_tight_ifchain | 10671ns | 10035ns | 11346ns | +3.08% | 0.024 |
| carrier_disp_tight_ifchainasc | 10598ns | 9878ns | 11536ns | +2.38% | 0.024 |
| carrier_disp_tight_ifchainlin | 16186ns | 15341ns | 17082ns | +56.36% | 0.016 |
| carrier_disp_tight_nullfloor | 7862ns | 7389ns | 8364ns | -24.06% | 0.033 |
| carrier_disp_tight_switch | 10352ns | 9871ns | 10991ns | base | 0.025 |
| carrier_disp_tight_threaded | 11825ns | 11158ns | 12512ns | +14.23% | 0.022 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 314787 | 1513182 | 0.208 | 1.03× |
| carrier_disp_tight_fntable | 315577 | 1716141 | 0.184 | 1.03× |
| carrier_disp_tight_ifchain | 307234 | 1461129 | 0.210 | 1.00× |
| carrier_disp_tight_ifchainasc | 306206 | 1457103 | 0.210 | 1.00× |
| carrier_disp_tight_ifchainlin | 328287 | 1656806 | 0.198 | 1.07× |
| carrier_disp_tight_nullfloor | 295842 | 1646440 | 0.180 | 0.97× |
| carrier_disp_tight_switch | 306346 | 1443797 | 0.212 | 1.00× |
| carrier_disp_tight_threaded | 304244 | 1760034 | 0.173 | 0.99× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.035 Gops/s** (carrier_disp_tight_nullfloor; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_tight_bittree | 0.024 | 68.2% |
| carrier_disp_tight_fntable | 0.021 | 61.0% |
| carrier_disp_tight_ifchain | 0.024 | 69.6% |
| carrier_disp_tight_ifchainasc | 0.025 | 71.3% |
| carrier_disp_tight_ifchainlin | 0.016 | 46.1% |
| carrier_disp_tight_nullfloor | 0.033 | 94.4% |
| carrier_disp_tight_switch | 0.025 | 72.6% |
| carrier_disp_tight_threaded | 0.022 | 62.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_tight_bittree | 13291ns | 13291ns | +4.85% |
| carrier_disp_tight_fntable | 14701ns | 14701ns | +15.98% |
| carrier_disp_tight_ifchain | 13062ns | 13062ns | +3.04% |
| carrier_disp_tight_ifchainasc | 12993ns | 12993ns | +2.50% |
| carrier_disp_tight_ifchainlin | 18547ns | 18547ns | +46.31% |
| carrier_disp_tight_nullfloor | 10235ns | 10235ns | -19.25% |
| carrier_disp_tight_switch | 12676ns | 12676ns | base |
| carrier_disp_tight_threaded | 14182ns | 14182ns | +11.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_tight_switch | 10183ns | base | --- | [9882, 10991] | --- | --- | --- | --- |
| carrier_disp_tight_bittree | 10838ns | +489.8ns (+4.8%) | [+450, +838]ns | [10345, 11651] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_fntable | 12122ns | +1944.1ns (+19.1%) | [+1732, +2289]ns | [11620, 13280] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_ifchain | 10622ns | +225.7ns (+2.2%) | [+146, +585]ns | [10045, 11346] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_ifchainasc | 10370ns | no significant difference | [-270, +922]ns | [9888, 11536] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_tight_ifchainlin | 16023ns | +5704.7ns (+56.0%) | [+5553, +6245]ns | [15453, 17082] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_nullfloor | 7831ns | -2438.9ns (-24.0%) | [-2781, -2251]ns | [7390, 8364] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_threaded | 11802ns | +1286.0ns (+12.6%) | [+1237, +1897]ns | [11162, 12512] | YES | 0.0365 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_tight_switch | carrier_disp_tight_bittree | carrier_disp_tight_fntable | carrier_disp_tight_ifchain | carrier_disp_tight_ifchainasc | carrier_disp_tight_ifchainlin | carrier_disp_tight_nullfloor | carrier_disp_tight_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 9871ns | +4.8% | +17.8% | +1.9% | +1.8% | +55.4% | -25.1% | +13.1% |
| 2 | 11213ns | +4.5% | +18.8% | +2.3% | -4.7% | +51.4% | -27.2% | +10.9% |
| 3 | 9894ns | +4.5% | +18.8% | +1.6% | -0.2% | +57.3% | -24.2% | +12.9% |
| 4 | 10461ns | +8.2% | +19.4% | +7.2% | +10.5% | +64.3% | -20.7% | +19.4% |
| 5 | 9904ns | +4.5% | +17.2% | +1.3% | -0.1% | +57.9% | -25.4% | +12.7% |
| 6 | 10768ns | +7.6% | +23.0% | +3.9% | +6.9% | +52.3% | -21.7% | +16.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_tight_bittree | -0.792 | HIGH- (thermal bounce) |
| carrier_disp_tight_fntable | -0.668 | HIGH- (thermal bounce) |
| carrier_disp_tight_ifchain | -0.843 | HIGH- (thermal bounce) |
| carrier_disp_tight_ifchainasc | -0.690 | HIGH- (thermal bounce) |
| carrier_disp_tight_ifchainlin | -0.795 | HIGH- (thermal bounce) |
| carrier_disp_tight_nullfloor | -0.745 | HIGH- (thermal bounce) |
| carrier_disp_tight_switch | -0.697 | HIGH- (thermal bounce) |
| carrier_disp_tight_threaded | -0.819 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_tight_bittree**: won 0/6, lost 6/6
- **carrier_disp_tight_fntable**: won 0/6, lost 6/6
- **carrier_disp_tight_ifchain**: won 0/6, lost 6/6
- **carrier_disp_tight_ifchainasc**: won 2/6, lost 3/6
- **carrier_disp_tight_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_tight_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_tight_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 92682.1ns | 10944.6ns | 846.8% | HIGH |
| carrier_disp_tight_fntable | 91595.8ns | 12340.5ns | 742.2% | HIGH |
| carrier_disp_tight_ifchain | 90628.0ns | 10671.0ns | 849.3% | HIGH |
| carrier_disp_tight_ifchainasc | 89682.8ns | 10598.0ns | 846.2% | HIGH |
| carrier_disp_tight_ifchainlin | 91906.6ns | 16186.2ns | 567.8% | HIGH |
| carrier_disp_tight_nullfloor | 89951.9ns | 7861.7ns | 1144.2% | HIGH |
| carrier_disp_tight_switch | 89501.4ns | 10352.0ns | 864.6% | HIGH |
| carrier_disp_tight_threaded | 88529.2ns | 11825.4ns | 748.6% | HIGH |

## Distribution (algo ns)

```
carrier_disp_tight_bittree (n=6, range 10344.2-11650.8 ns)
  10344.2 |########################################
  10409.5 |
  10474.9 |
  10540.2 |
  10605.5 |
  10670.9 |
  10736.2 |
  10801.5 |
  10866.8 |
  10932.2 |
  10997.5 |
  11062.8 |
  11128.2 |
  11193.5 |
  11258.8 |#############
  11324.1 |
  11389.5 |
  11454.8 |
  11520.1 |#############
  11585.5 |
  (0 below, 1 above range)

carrier_disp_tight_fntable (n=6, range 11612.5-13279.8 ns)
  11612.5 |########################################
  11695.9 |####################
  11779.2 |
  11862.6 |
  11946.0 |
  12029.3 |
  12112.7 |
  12196.0 |
  12279.4 |
  12362.8 |
  12446.1 |####################
  12529.5 |
  12612.9 |
  12696.2 |
  12779.6 |
  12862.9 |
  12946.3 |
  13029.7 |
  13113.0 |
  13196.4 |####################
  (0 below, 1 above range)

carrier_disp_tight_ifchain (n=6, range 10034.6-11345.9 ns)
  10034.6 |########################################
  10100.2 |
  10165.7 |
  10231.3 |
  10296.9 |
  10362.4 |
  10428.0 |
  10493.5 |
  10559.1 |
  10624.7 |
  10690.2 |
  10755.8 |
  10821.4 |
  10886.9 |
  10952.5 |
  11018.0 |
  11083.6 |
  11149.2 |#############
  11214.7 |#############
  11280.3 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainasc (n=6, range 9877.9-11536.5 ns)
   9877.9 |########################################
   9960.8 |
  10043.8 |####################
  10126.7 |
  10209.6 |
  10292.5 |
  10375.5 |
  10458.4 |
  10541.3 |
  10624.2 |####################
  10707.2 |
  10790.1 |
  10873.0 |
  10956.0 |
  11038.9 |
  11121.8 |
  11204.7 |
  11287.7 |
  11370.6 |
  11453.5 |####################
  (0 below, 1 above range)

carrier_disp_tight_ifchainlin (n=6, range 15341.2-17082.5 ns)
  15341.2 |########################################
  15428.3 |
  15515.3 |########################################
  15602.4 |########################################
  15689.5 |
  15776.5 |
  15863.6 |
  15950.7 |
  16037.7 |
  16124.8 |
  16211.9 |
  16298.9 |
  16386.0 |########################################
  16473.0 |
  16560.1 |
  16647.2 |
  16734.2 |
  16821.3 |
  16908.4 |########################################
  16995.4 |
  (0 below, 1 above range)

carrier_disp_tight_nullfloor (n=6, range 7388.8-8364.2 ns)
   7388.8 |########################################
   7437.6 |
   7486.3 |####################
   7535.1 |
   7583.9 |
   7632.6 |
   7681.4 |
   7730.2 |
   7778.9 |
   7827.7 |
   7876.5 |
   7925.2 |
   7974.0 |
   8022.8 |
   8071.5 |
   8120.3 |####################
   8169.1 |
   8217.8 |
   8266.6 |####################
   8315.4 |
  (0 below, 1 above range)

carrier_disp_tight_switch (n=6, range 9870.8-10990.8 ns)
   9870.8 |########################################
   9926.8 |
   9982.8 |
  10038.8 |
  10094.8 |
  10150.8 |
  10206.8 |
  10262.8 |
  10318.8 |
  10374.8 |
  10430.8 |#############
  10486.8 |
  10542.8 |
  10598.8 |
  10654.8 |
  10710.8 |
  10766.8 |#############
  10822.8 |
  10878.8 |
  10934.8 |
  (0 below, 1 above range)

carrier_disp_tight_threaded (n=6, range 11158.3-12511.9 ns)
  11158.3 |########################################
  11226.0 |
  11293.7 |
  11361.3 |
  11429.0 |
  11496.7 |
  11564.4 |
  11632.1 |
  11699.7 |
  11767.4 |
  11835.1 |
  11902.8 |
  11970.5 |
  12038.1 |
  12105.8 |
  12173.5 |
  12241.2 |
  12308.9 |
  12376.5 |#############
  12444.2 |#############
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_tight_bittree**: bridge=855.5% of algo (FFI overhead may distort results)
- **carrier_disp_tight_fntable**: bridge=762.1% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchain**: bridge=853.1% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainasc**: bridge=866.0% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainlin**: bridge=579.6% of algo (FFI overhead may distort results)
- **carrier_disp_tight_nullfloor**: bridge=1146.4% of algo (FFI overhead may distort results)
- **carrier_disp_tight_switch**: bridge=877.1% of algo (FFI overhead may distort results)
- **carrier_disp_tight_threaded**: bridge=751.7% of algo (FFI overhead may distort results)
