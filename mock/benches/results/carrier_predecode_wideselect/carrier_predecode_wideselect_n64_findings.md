# Predecoded dispatch shape, wideselect profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_wideselect_null beats baseline by 36% (significant)

carrier_pre_wideselect_null is -748 ns (36%) faster than baseline carrier_pre_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_wideselect_fntable is an outlier: 2.2x slower than the field

carrier_pre_wideselect_fntable (2.82 us) is 2.2x the fastest (1.31 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_pre_wideselect_threaded shows alternating (throttle bounce) (autocorr -0.53)

carrier_pre_wideselect_threaded's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_pre_wideselect_null, carrier_pre_wideselect_direct, carrier_pre_wideselect_threaded, carrier_pre_wideselect_switch, carrier_pre_wideselect_regcache} vs {carrier_pre_wideselect_fntable} (27% apart)

The field splits into a fast tier {carrier_pre_wideselect_null, carrier_pre_wideselect_direct, carrier_pre_wideselect_threaded, carrier_pre_wideselect_switch, carrier_pre_wideselect_regcache} and a slow tier {carrier_pre_wideselect_fntable} with a 27% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_wideselect_null** at 1308.2 ns median (-36.7% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.16x (fastest 1308.2 ns, slowest 2819.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 3732ns | 3771ns | 3564ns | 3730ns | 3819ns | -15.02% |
| carrier_pre_wideselect_fntable | 5042ns | 5156ns | 4720ns | 5023ns | 5230ns | +14.80% |
| carrier_pre_wideselect_null | 3632ns | 3658ns | 3414ns | 3618ns | 3761ns | -17.30% |
| carrier_pre_wideselect_regcache | 4513ns | 4516ns | 4355ns | 4473ns | 4651ns | +2.76% |
| carrier_pre_wideselect_switch | 4392ns | 4438ns | 4132ns | 4368ns | 4557ns | base |
| carrier_pre_wideselect_threaded | 3957ns | 3988ns | 3693ns | 3939ns | 4116ns | -9.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 1396ns | 1339ns | 1429ns | -31.41% | 0.046 |
| carrier_pre_wideselect_fntable | 2758ns | 2587ns | 2860ns | +35.54% | 0.023 |
| carrier_pre_wideselect_null | 1294ns | 1223ns | 1328ns | -36.39% | 0.049 |
| carrier_pre_wideselect_regcache | 2204ns | 2132ns | 2253ns | +8.31% | 0.029 |
| carrier_pre_wideselect_switch | 2035ns | 1907ns | 2104ns | base | 0.031 |
| carrier_pre_wideselect_threaded | 1673ns | 1556ns | 1745ns | -17.81% | 0.038 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 261040 | 1060577 | 0.246 | 1.00× |
| carrier_pre_wideselect_fntable | 272018 | 874359 | 0.311 | 1.04× |
| carrier_pre_wideselect_null | 263616 | 1441989 | 0.183 | 1.01× |
| carrier_pre_wideselect_regcache | 268394 | 1195010 | 0.225 | 1.03× |
| carrier_pre_wideselect_switch | 261657 | 903913 | 0.289 | 1.00× |
| carrier_pre_wideselect_threaded | 265950 | 1214608 | 0.219 | 1.02× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.052 Gops/s** (carrier_pre_wideselect_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_wideselect_direct | 0.046 | 87.1% |
| carrier_pre_wideselect_fntable | 0.023 | 43.4% |
| carrier_pre_wideselect_null | 0.049 | 93.5% |
| carrier_pre_wideselect_regcache | 0.029 | 55.1% |
| carrier_pre_wideselect_switch | 0.031 | 59.2% |
| carrier_pre_wideselect_threaded | 0.038 | 72.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_wideselect_direct | 3732ns | 3732ns | -15.02% |
| carrier_pre_wideselect_fntable | 5042ns | 5042ns | +14.80% |
| carrier_pre_wideselect_null | 3632ns | 3632ns | -17.30% |
| carrier_pre_wideselect_regcache | 4513ns | 4513ns | +2.76% |
| carrier_pre_wideselect_switch | 4392ns | 4392ns | base |
| carrier_pre_wideselect_threaded | 3957ns | 3957ns | -9.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_wideselect_switch | 2066ns | base | --- | [1935, 2104] | --- | --- | --- | --- |
| carrier_pre_wideselect_direct | 1404ns | -667.7ns (-32.3%) | [-700, -550]ns | [1354, 1429] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_fntable | 2820ns | +738.1ns (+35.7%) | [+584, +848]ns | [2595, 2860] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_null | 1308ns | -747.7ns (-36.2%) | [-815, -659]ns | [1247, 1328] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_regcache | 2219ns | +169.2ns (+8.2%) | [+106, +232]ns | [2140, 2253] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_threaded | 1682ns | -357.8ns (-17.3%) | [-430, -300]ns | [1591, 1745] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_wideselect_switch | carrier_pre_wideselect_direct | carrier_pre_wideselect_fntable | carrier_pre_wideselect_null | carrier_pre_wideselect_regcache | carrier_pre_wideselect_threaded |
|---|---|---|---|---|---|---|
| 1 | 2117ns | -33.1% | +22.2% | -39.9% | +5.9% | -17.2% |
| 2 | 1907ns | -29.8% | +36.6% | -31.3% | +11.8% | -18.4% |
| 3 | 1963ns | -27.2% | +47.1% | -37.7% | +12.1% | -14.1% |
| 4 | 2072ns | -31.1% | +35.7% | -36.5% | +8.0% | -19.0% |
| 5 | 2091ns | -33.4% | +35.2% | -37.5% | +8.3% | -22.2% |
| 6 | 2060ns | -33.6% | +37.4% | -35.0% | +4.2% | -15.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_wideselect_direct | -0.297 | moderate- |
| carrier_pre_wideselect_fntable | 0.267 | moderate+ |
| carrier_pre_wideselect_null | -0.251 | moderate- |
| carrier_pre_wideselect_regcache | -0.271 | moderate- |
| carrier_pre_wideselect_switch | -0.013 | ok |
| carrier_pre_wideselect_threaded | -0.532 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_pre_wideselect_direct**: won 6/6, lost 0/6
- **carrier_pre_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_pre_wideselect_null**: won 6/6, lost 0/6
- **carrier_pre_wideselect_regcache**: won 0/6, lost 6/6
- **carrier_pre_wideselect_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 86815.5ns | 1395.8ns | 6219.9% | HIGH |
| carrier_pre_wideselect_fntable | 87343.2ns | 2758.3ns | 3166.5% | HIGH |
| carrier_pre_wideselect_null | 86451.6ns | 1294.5ns | 6678.5% | HIGH |
| carrier_pre_wideselect_regcache | 87099.0ns | 2204.1ns | 3951.7% | HIGH |
| carrier_pre_wideselect_switch | 86317.5ns | 2035.1ns | 4241.5% | HIGH |
| carrier_pre_wideselect_threaded | 86298.0ns | 1672.6ns | 5159.6% | HIGH |

## Distribution (algo ns)

```
carrier_pre_wideselect_direct (n=6, range 1339.2-1428.9 ns)
   1339.2 |########################################
   1343.7 |
   1348.2 |
   1352.7 |
   1357.2 |
   1361.6 |
   1366.1 |########################################
   1370.6 |
   1375.1 |
   1379.6 |
   1384.1 |
   1388.6 |
   1393.0 |########################################
   1397.5 |
   1402.0 |
   1406.5 |
   1411.0 |########################################
   1415.5 |
   1420.0 |
   1424.5 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_fntable (n=6, range 2586.7-2859.8 ns)
   2586.7 |####################
   2600.4 |####################
   2614.0 |
   2627.7 |
   2641.3 |
   2655.0 |
   2668.6 |
   2682.3 |
   2695.9 |
   2709.6 |
   2723.2 |
   2736.9 |
   2750.6 |
   2764.2 |
   2777.9 |
   2791.5 |
   2805.2 |####################
   2818.8 |########################################
   2832.5 |
   2846.1 |
  (0 below, 1 above range)

carrier_pre_wideselect_null (n=6, range 1222.9-1328.0 ns)
   1222.9 |####################
   1228.2 |
   1233.4 |
   1238.7 |
   1243.9 |
   1249.2 |
   1254.4 |
   1259.7 |
   1264.9 |
   1270.2 |####################
   1275.4 |
   1280.7 |
   1285.9 |
   1291.2 |
   1296.4 |
   1301.7 |
   1306.9 |########################################
   1312.2 |####################
   1317.4 |
   1322.7 |
  (0 below, 1 above range)

carrier_pre_wideselect_regcache (n=6, range 2132.5-2252.9 ns)
   2132.5 |########################################
   2138.5 |
   2144.5 |########################################
   2150.6 |
   2156.6 |
   2162.6 |
   2168.6 |
   2174.7 |
   2180.7 |
   2186.7 |
   2192.7 |
   2198.7 |########################################
   2204.8 |
   2210.8 |
   2216.8 |
   2222.8 |
   2228.9 |
   2234.9 |########################################
   2240.9 |########################################
   2246.9 |
  (0 below, 1 above range)

carrier_pre_wideselect_switch (n=6, range 1906.7-2104.1 ns)
   1906.7 |########################################
   1916.6 |
   1926.4 |
   1936.3 |
   1946.2 |
   1956.1 |########################################
   1965.9 |
   1975.8 |
   1985.7 |
   1995.6 |
   2005.4 |
   2015.3 |
   2025.2 |
   2035.0 |
   2044.9 |
   2054.8 |########################################
   2064.7 |########################################
   2074.5 |
   2084.4 |########################################
   2094.3 |
  (0 below, 1 above range)

carrier_pre_wideselect_threaded (n=6, range 1555.8-1744.6 ns)
   1555.8 |########################################
   1565.2 |
   1574.7 |
   1584.1 |
   1593.6 |
   1603.0 |
   1612.4 |
   1621.9 |########################################
   1631.3 |
   1640.8 |
   1650.2 |
   1659.6 |
   1669.1 |########################################
   1678.5 |########################################
   1688.0 |
   1697.4 |
   1706.8 |
   1716.3 |
   1725.7 |
   1735.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_wideselect_direct**: bridge=6185.3% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_fntable**: bridge=3094.0% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_null**: bridge=6588.2% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_regcache**: bridge=3928.5% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_switch**: bridge=4170.9% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_threaded**: bridge=5129.3% of algo (FFI overhead may distort results)
