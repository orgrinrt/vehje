# Predecoded dispatch shape, wideselect profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_wideselect_null dominates: 192% faster than the next best (carrier_pre_wideselect_direct)

carrier_pre_wideselect_null (473.59 us) leads carrier_pre_wideselect_direct (1.38 ms) by 192%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_wideselect_null beats baseline by 73% (significant)

carrier_pre_wideselect_null is -1.26 ms (73%) faster than baseline carrier_pre_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_wideselect_fntable is an outlier: 4.1x slower than the field

carrier_pre_wideselect_fntable (1.94 ms) is 4.1x the fastest (473.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_wideselect_null} vs {carrier_pre_wideselect_direct, carrier_pre_wideselect_switch, carrier_pre_wideselect_threaded, carrier_pre_wideselect_regcache, carrier_pre_wideselect_fntable} (192% apart)

The field splits into a fast tier {carrier_pre_wideselect_null} and a slow tier {carrier_pre_wideselect_direct, carrier_pre_wideselect_switch, carrier_pre_wideselect_threaded, carrier_pre_wideselect_regcache, carrier_pre_wideselect_fntable} with a 192% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.1x the fastest

Fastest carrier_pre_wideselect_null (473.59 us) to slowest carrier_pre_wideselect_fntable (1.94 ms): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_pre_wideselect_null** at 473591.2 ns median (-72.7% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.10x (fastest 473591.2 ns, slowest 1943358.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 1381897ns | 1383053ns | 1372820ns | 1381873ns | 1386471ns | -20.41% |
| carrier_pre_wideselect_fntable | 1946361ns | 1945944ns | 1935156ns | 1943956ns | 1955573ns | +12.10% |
| carrier_pre_wideselect_null | 479280ns | 475825ns | 452893ns | 473937ns | 500489ns | -72.40% |
| carrier_pre_wideselect_regcache | 1822777ns | 1821084ns | 1813580ns | 1819115ns | 1832870ns | +4.98% |
| carrier_pre_wideselect_switch | 1736271ns | 1734921ns | 1728034ns | 1733215ns | 1744973ns | base |
| carrier_pre_wideselect_threaded | 1747807ns | 1748916ns | 1734820ns | 1747933ns | 1754112ns | +0.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 1379497ns | 1370387ns | 1384026ns | -20.43% | 0.012 |
| carrier_pre_wideselect_fntable | 1943768ns | 1932748ns | 1952721ns | +12.12% | 0.008 |
| carrier_pre_wideselect_null | 477047ns | 450688ns | 498228ns | -72.48% | 0.034 |
| carrier_pre_wideselect_regcache | 1820229ns | 1811076ns | 1830321ns | +4.99% | 0.009 |
| carrier_pre_wideselect_switch | 1733661ns | 1725506ns | 1742272ns | base | 0.009 |
| carrier_pre_wideselect_threaded | 1745099ns | 1731835ns | 1751558ns | +0.66% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 8963318 | 9823894 | 0.912 | 0.79× |
| carrier_pre_wideselect_fntable | 12443319 | 14356041 | 0.867 | 1.10× |
| carrier_pre_wideselect_null | 3365758 | 11870900 | 0.284 | 0.30× |
| carrier_pre_wideselect_regcache | 11677016 | 15907184 | 0.734 | 1.03× |
| carrier_pre_wideselect_switch | 11310554 | 11584695 | 0.976 | 1.00× |
| carrier_pre_wideselect_threaded | 11168647 | 12709338 | 0.879 | 0.99× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_pre_wideselect_null; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_wideselect_direct | 0.012 | 32.6% |
| carrier_pre_wideselect_fntable | 0.008 | 23.2% |
| carrier_pre_wideselect_null | 0.035 | 95.2% |
| carrier_pre_wideselect_regcache | 0.009 | 24.8% |
| carrier_pre_wideselect_switch | 0.009 | 26.0% |
| carrier_pre_wideselect_threaded | 0.009 | 25.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_wideselect_direct | 1381897ns | 1381897ns | -20.41% |
| carrier_pre_wideselect_fntable | 1946361ns | 1946361ns | +12.10% |
| carrier_pre_wideselect_null | 479280ns | 479280ns | -72.40% |
| carrier_pre_wideselect_regcache | 1822777ns | 1822777ns | +4.98% |
| carrier_pre_wideselect_switch | 1736271ns | 1736271ns | base |
| carrier_pre_wideselect_threaded | 1747807ns | 1747807ns | +0.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_wideselect_switch | 1732307ns | base | --- | [1726404, 1742272] | --- | --- | --- | --- |
| carrier_pre_wideselect_direct | 1380686ns | -351082.1ns (-20.3%) | [-363164, -348246]ns | [1373779, 1384026] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_fntable | 1943358ns | +208520.6ns (+12.0%) | [+200116, +221684]ns | [1935224, 1952721] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_null | 473591ns | -1257152.7ns (-72.6%) | [-1279408, -1233281]ns | [459321, 498228] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_regcache | 1818438ns | +85762.5ns (+5.0%) | [+77216, +96726]ns | [1811927, 1830321] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_threaded | 1746303ns | +10522.8ns (+0.6%) | [+4881, +18912]ns | [1737437, 1751558] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_wideselect_switch | carrier_pre_wideselect_direct | carrier_pre_wideselect_fntable | carrier_pre_wideselect_null | carrier_pre_wideselect_regcache | carrier_pre_wideselect_threaded |
|---|---|---|---|---|---|---|
| 1 | 1727301ns | -20.2% | +13.0% | -72.9% | +5.6% | +0.3% |
| 2 | 1725506ns | -20.2% | +12.7% | -71.1% | +5.1% | +1.1% |
| 3 | 1732703ns | -20.1% | +11.8% | -72.4% | +5.6% | +1.0% |
| 4 | 1747032ns | -20.9% | +11.8% | -74.2% | +4.8% | +0.3% |
| 5 | 1731910ns | -20.9% | +12.2% | -72.9% | +4.7% | +0.6% |
| 6 | 1737512ns | -20.3% | +11.2% | -71.3% | +4.2% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_wideselect_direct | -0.475 | moderate- |
| carrier_pre_wideselect_fntable | -0.173 | ok |
| carrier_pre_wideselect_null | -0.090 | ok |
| carrier_pre_wideselect_regcache | -0.022 | ok |
| carrier_pre_wideselect_switch | 0.055 | ok |
| carrier_pre_wideselect_threaded | 0.078 | ok |

**Consistency summary:**

- **carrier_pre_wideselect_direct**: won 6/6, lost 0/6
- **carrier_pre_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_pre_wideselect_null**: won 6/6, lost 0/6
- **carrier_pre_wideselect_regcache**: won 0/6, lost 6/6
- **carrier_pre_wideselect_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 1478627.8ns | 1379497.0ns | 107.2% | HIGH |
| carrier_pre_wideselect_fntable | 2030811.4ns | 1943767.7ns | 104.5% | HIGH |
| carrier_pre_wideselect_null | 593567.4ns | 477046.9ns | 124.4% | HIGH |
| carrier_pre_wideselect_regcache | 1910035.9ns | 1820228.9ns | 104.9% | HIGH |
| carrier_pre_wideselect_switch | 1881225.3ns | 1733660.9ns | 108.5% | HIGH |
| carrier_pre_wideselect_threaded | 1831844.7ns | 1745099.4ns | 105.0% | HIGH |

## Distribution (algo ns)

```
carrier_pre_wideselect_direct (n=6, range 1370387.1-1384025.8 ns)
  1370387.1 |########################################
  1371069.0 |
  1371751.0 |
  1372432.9 |
  1373114.8 |
  1373796.8 |
  1374478.7 |
  1375160.6 |
  1375842.6 |
  1376524.5 |########################################
  1377206.4 |
  1377888.4 |
  1378570.3 |########################################
  1379252.3 |
  1379934.2 |
  1380616.1 |
  1381298.1 |
  1381980.0 |########################################
  1382661.9 |
  1383343.9 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_fntable (n=6, range 1932747.9-1952721.4 ns)
  1932747.9 |########################################
  1933746.6 |
  1934745.3 |
  1935743.9 |
  1936742.6 |########################################
  1937741.3 |
  1938740.0 |
  1939738.6 |
  1940737.3 |
  1941736.0 |
  1942734.7 |########################################
  1943733.4 |########################################
  1944732.0 |
  1945730.7 |
  1946729.4 |
  1947728.1 |
  1948726.7 |
  1949725.4 |
  1950724.1 |
  1951722.8 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_null (n=6, range 450688.3-498227.9 ns)
  450688.3 |####################
  453065.3 |
  455442.3 |
  457819.2 |
  460196.2 |
  462573.2 |
  464950.2 |
  467327.2 |########################################
  469704.1 |
  472081.1 |
  474458.1 |
  476835.1 |####################
  479212.1 |
  481589.0 |
  483966.0 |
  486343.0 |
  488720.0 |
  491097.0 |
  493473.9 |
  495850.9 |####################
  (0 below, 1 above range)

carrier_pre_wideselect_regcache (n=6, range 1811076.2-1830321.4 ns)
  1811076.2 |####################
  1812038.5 |########################################
  1813000.7 |
  1813963.0 |
  1814925.2 |
  1815887.5 |
  1816849.8 |
  1817812.0 |
  1818774.3 |
  1819736.6 |
  1820698.8 |
  1821661.1 |
  1822623.3 |
  1823585.6 |####################
  1824547.9 |
  1825510.1 |
  1826472.4 |
  1827434.7 |
  1828396.9 |
  1829359.2 |####################
  (0 below, 1 above range)

carrier_pre_wideselect_switch (n=6, range 1725506.2-1742272.1 ns)
  1725506.2 |########################################
  1726344.5 |
  1727182.8 |########################################
  1728021.1 |
  1728859.4 |
  1729697.7 |
  1730536.0 |
  1731374.3 |########################################
  1732212.6 |########################################
  1733050.9 |
  1733889.1 |
  1734727.4 |
  1735565.7 |
  1736404.0 |
  1737242.3 |########################################
  1738080.6 |
  1738918.9 |
  1739757.2 |
  1740595.5 |
  1741433.8 |
  (0 below, 1 above range)

carrier_pre_wideselect_threaded (n=6, range 1731834.6-1751557.9 ns)
  1731834.6 |########################################
  1732820.8 |
  1733806.9 |
  1734793.1 |
  1735779.3 |
  1736765.4 |
  1737751.6 |
  1738737.8 |
  1739723.9 |
  1740710.1 |
  1741696.2 |
  1742682.4 |########################################
  1743668.6 |
  1744654.7 |########################################
  1745640.9 |
  1746627.1 |########################################
  1747613.2 |
  1748599.4 |
  1749585.6 |
  1750571.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_wideselect_direct**: bridge=107.2% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_fntable**: bridge=104.6% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_null**: bridge=125.6% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_regcache**: bridge=104.9% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_switch**: bridge=108.5% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_threaded**: bridge=105.1% of algo (FFI overhead may distort results)
