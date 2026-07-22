# Predecoded dispatch shape, wideselect profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_wideselect_null dominates: 158% faster than the next best (carrier_pre_wideselect_direct)

carrier_pre_wideselect_null (522.58 us) leads carrier_pre_wideselect_direct (1.35 ms) by 158%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_wideselect_null beats baseline by 69% (significant)

carrier_pre_wideselect_null is -1.17 ms (69%) faster than baseline carrier_pre_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_wideselect_fntable is an outlier: 3.7x slower than the field

carrier_pre_wideselect_fntable (1.94 ms) is 3.7x the fastest (522.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_wideselect_null} vs {carrier_pre_wideselect_direct, carrier_pre_wideselect_threaded, carrier_pre_wideselect_switch, carrier_pre_wideselect_regcache, carrier_pre_wideselect_fntable} (158% apart)

The field splits into a fast tier {carrier_pre_wideselect_null} and a slow tier {carrier_pre_wideselect_direct, carrier_pre_wideselect_threaded, carrier_pre_wideselect_switch, carrier_pre_wideselect_regcache, carrier_pre_wideselect_fntable} with a 158% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.7x the fastest

Fastest carrier_pre_wideselect_null (522.58 us) to slowest carrier_pre_wideselect_fntable (1.94 ms): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_pre_wideselect_null** at 522576.2 ns median (-69.1% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.70x (fastest 522576.2 ns, slowest 1936067.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 1354236ns | 1351433ns | 1328930ns | 1345349ns | 1380220ns | -19.94% |
| carrier_pre_wideselect_fntable | 1935275ns | 1938582ns | 1918763ns | 1934360ns | 1944903ns | +14.41% |
| carrier_pre_wideselect_null | 520317ns | 524858ns | 475256ns | 524141ns | 537111ns | -69.24% |
| carrier_pre_wideselect_regcache | 1807060ns | 1807767ns | 1798551ns | 1806725ns | 1811818ns | +6.83% |
| carrier_pre_wideselect_switch | 1691513ns | 1695530ns | 1663690ns | 1690558ns | 1706857ns | base |
| carrier_pre_wideselect_threaded | 1692323ns | 1692799ns | 1658000ns | 1687398ns | 1716872ns | +0.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 1351895ns | 1326536ns | 1377809ns | -19.96% | 0.012 |
| carrier_pre_wideselect_fntable | 1932849ns | 1916527ns | 1942602ns | +14.43% | 0.008 |
| carrier_pre_wideselect_null | 518051ns | 473048ns | 534850ns | -69.33% | 0.032 |
| carrier_pre_wideselect_regcache | 1804664ns | 1796203ns | 1809549ns | +6.84% | 0.009 |
| carrier_pre_wideselect_switch | 1689079ns | 1661527ns | 1704283ns | base | 0.010 |
| carrier_pre_wideselect_threaded | 1689851ns | 1655685ns | 1714324ns | +0.05% | 0.010 |

## Performance model

- Peak throughput: **0.035 Gops/s** (carrier_pre_wideselect_null; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_wideselect_direct | 0.012 | 35.1% |
| carrier_pre_wideselect_fntable | 0.008 | 24.4% |
| carrier_pre_wideselect_null | 0.031 | 90.5% |
| carrier_pre_wideselect_regcache | 0.009 | 26.2% |
| carrier_pre_wideselect_switch | 0.010 | 27.9% |
| carrier_pre_wideselect_threaded | 0.010 | 28.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_wideselect_direct | 1354236ns | 1354236ns | -19.94% |
| carrier_pre_wideselect_fntable | 1935275ns | 1935275ns | +14.41% |
| carrier_pre_wideselect_null | 520317ns | 520317ns | -69.24% |
| carrier_pre_wideselect_regcache | 1807060ns | 1807060ns | +6.83% |
| carrier_pre_wideselect_switch | 1691513ns | 1691513ns | base |
| carrier_pre_wideselect_threaded | 1692323ns | 1692323ns | +0.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_wideselect_switch | 1693088ns | base | --- | [1669865, 1704283] | --- | --- | --- | --- |
| carrier_pre_wideselect_direct | 1349153ns | -349228.5ns (-20.6%) | [-370269, -292056]ns | [1328722, 1377809] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_wideselect_fntable | 1936067ns | +235589.2ns (+13.9%) | [+222983, +272737]ns | [1919876, 1942602] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_wideselect_null | 522576ns | -1174317.2ns (-69.4%) | [-1203750, -1135015]ns | [496728, 534850] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_wideselect_regcache | 1805263ns | +111763.8ns (+6.6%) | [+99319, +135672]ns | [1799180, 1809549] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_wideselect_threaded | 1690355ns | no significant difference | [-26760, +31059]ns | [1664874, 1714324] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_wideselect_switch | carrier_pre_wideselect_direct | carrier_pre_wideselect_fntable | carrier_pre_wideselect_null | carrier_pre_wideselect_regcache | carrier_pre_wideselect_threaded |
|---|---|---|---|---|---|---|
| 1 | 1691601ns | -21.3% | +13.3% | -69.2% | +6.9% | -2.1% |
| 2 | 1694576ns | -20.4% | +14.3% | -72.1% | +6.3% | +0.1% |
| 3 | 1702186ns | -20.7% | +13.0% | -69.2% | +5.5% | -1.0% |
| 4 | 1706380ns | -22.3% | +13.4% | -69.5% | +6.1% | -0.3% |
| 5 | 1678204ns | -17.5% | +15.7% | -68.3% | +7.4% | +3.0% |
| 6 | 1661527ns | -17.4% | +17.0% | -67.6% | +8.8% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_wideselect_direct | -0.020 | ok |
| carrier_pre_wideselect_fntable | -0.055 | ok |
| carrier_pre_wideselect_null | -0.033 | ok |
| carrier_pre_wideselect_regcache | -0.398 | moderate- |
| carrier_pre_wideselect_switch | 0.306 | moderate+ |
| carrier_pre_wideselect_threaded | -0.160 | ok |

**Consistency summary:**

- **carrier_pre_wideselect_direct**: won 6/6, lost 0/6
- **carrier_pre_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_pre_wideselect_null**: won 6/6, lost 0/6
- **carrier_pre_wideselect_regcache**: won 0/6, lost 6/6
- **carrier_pre_wideselect_threaded**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 1447536.9ns | 1351894.6ns | 107.1% | HIGH |
| carrier_pre_wideselect_fntable | 2008520.5ns | 1932848.6ns | 103.9% | HIGH |
| carrier_pre_wideselect_null | 553858.0ns | 518051.4ns | 106.9% | HIGH |
| carrier_pre_wideselect_regcache | 1886566.9ns | 1804664.0ns | 104.5% | HIGH |
| carrier_pre_wideselect_switch | 1840090.8ns | 1689079.0ns | 108.9% | HIGH |
| carrier_pre_wideselect_threaded | 1773833.1ns | 1689851.0ns | 105.0% | HIGH |

## Distribution (algo ns)

```
carrier_pre_wideselect_direct (n=6, range 1326535.8-1377809.4 ns)
  1326535.8 |########################################
  1329099.5 |########################################
  1331663.2 |
  1334226.8 |
  1336790.5 |
  1339354.2 |
  1341917.9 |
  1344481.6 |
  1347045.2 |########################################
  1349608.9 |########################################
  1352172.6 |
  1354736.3 |
  1357300.0 |
  1359863.6 |
  1362427.3 |
  1364991.0 |
  1367554.7 |
  1370118.4 |########################################
  1372682.0 |
  1375245.7 |
  (0 below, 1 above range)

carrier_pre_wideselect_fntable (n=6, range 1916527.1-1942602.3 ns)
  1916527.1 |########################################
  1917830.9 |
  1919134.6 |
  1920438.4 |
  1921742.1 |
  1923045.9 |########################################
  1924349.7 |
  1925653.4 |
  1926957.2 |
  1928260.9 |
  1929564.7 |
  1930868.5 |
  1932172.2 |
  1933476.0 |########################################
  1934779.7 |
  1936083.5 |
  1937387.3 |########################################
  1938691.0 |
  1939994.8 |
  1941298.5 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_null (n=6, range 473047.9-534850.2 ns)
  473047.9 |####################
  476138.0 |
  479228.1 |
  482318.2 |
  485408.4 |
  488498.5 |
  491588.6 |
  494678.7 |
  497768.8 |
  500858.9 |
  503949.0 |
  507039.2 |
  510129.3 |
  513219.4 |
  516309.5 |
  519399.6 |########################################
  522489.7 |####################
  525579.9 |
  528670.0 |
  531760.1 |####################
  (0 below, 1 above range)

carrier_pre_wideselect_regcache (n=6, range 1796203.3-1809549.1 ns)
  1796203.3 |####################
  1796870.6 |
  1797537.9 |
  1798205.2 |
  1798872.5 |
  1799539.8 |
  1800207.1 |
  1800874.3 |
  1801541.6 |####################
  1802208.9 |
  1802876.2 |####################
  1803543.5 |
  1804210.8 |
  1804878.1 |
  1805545.4 |
  1806212.7 |
  1806880.0 |
  1807547.3 |########################################
  1808214.6 |
  1808881.9 |
  (0 below, 1 above range)

carrier_pre_wideselect_switch (n=6, range 1661526.7-1704283.1 ns)
  1661526.7 |########################################
  1663664.5 |
  1665802.3 |
  1667940.2 |
  1670078.0 |
  1672215.8 |
  1674353.6 |
  1676491.4 |########################################
  1678629.3 |
  1680767.1 |
  1682904.9 |
  1685042.7 |
  1687180.5 |
  1689318.4 |
  1691456.2 |########################################
  1693594.0 |########################################
  1695731.8 |
  1697869.6 |
  1700007.5 |
  1702145.3 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_threaded (n=6, range 1655685.4-1714323.8 ns)
  1655685.4 |########################################
  1658617.3 |
  1661549.2 |
  1664481.2 |
  1667413.1 |
  1670345.0 |
  1673276.9 |########################################
  1676208.8 |
  1679140.7 |
  1682072.7 |########################################
  1685004.6 |
  1687936.5 |
  1690868.4 |
  1693800.3 |########################################
  1696732.2 |
  1699664.2 |########################################
  1702596.1 |
  1705528.0 |
  1708459.9 |
  1711391.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_wideselect_direct**: bridge=107.0% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_fntable**: bridge=103.8% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_null**: bridge=105.3% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_regcache**: bridge=104.5% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_switch**: bridge=109.0% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_threaded**: bridge=104.9% of algo (FFI overhead may distort results)
