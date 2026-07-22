# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, wideselect profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_wideselect_none**

## Highlights

Baseline for all deltas below: **carrier_opt_wideselect_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_wideselect_cse beats baseline by 22% (significant)

carrier_opt_wideselect_cse is -440.85 us (22%) faster than baseline carrier_opt_wideselect_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_wideselect_all shows alternating (throttle bounce) (autocorr -0.57)

carrier_opt_wideselect_all's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_opt_wideselect_cse** at 1574486.2 ns median (-21.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.28x (fastest 1574486.2 ns, slowest 2010585.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 1630979ns | 1633731ns | 1605592ns | 1628146ns | 1647920ns | -18.92% |
| carrier_opt_wideselect_cse | 1583520ns | 1578433ns | 1541099ns | 1577961ns | 1613068ns | -21.28% |
| carrier_opt_wideselect_cseeqsat | 1649064ns | 1646727ns | 1623475ns | 1640353ns | 1674926ns | -18.02% |
| carrier_opt_wideselect_dce | 2019815ns | 1996446ns | 1974965ns | 1992937ns | 2082558ns | +0.41% |
| carrier_opt_wideselect_eqsat | 1634848ns | 1627076ns | 1617482ns | 1625181ns | 1658031ns | -18.73% |
| carrier_opt_wideselect_fold | 2015150ns | 2013793ns | 1990305ns | 2010833ns | 2034049ns | +0.18% |
| carrier_opt_wideselect_none | 2011564ns | 2014131ns | 1984339ns | 2010086ns | 2027395ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 1627517ns | 1601925ns | 1644480ns | -18.94% | 0.010 |
| carrier_opt_wideselect_cse | 1579972ns | 1538095ns | 1609734ns | -21.31% | 0.010 |
| carrier_opt_wideselect_cseeqsat | 1645570ns | 1620482ns | 1670804ns | -18.05% | 0.010 |
| carrier_opt_wideselect_dce | 2016398ns | 1971963ns | 2079356ns | +0.42% | 0.008 |
| carrier_opt_wideselect_eqsat | 1631485ns | 1613460ns | 1654629ns | -18.75% | 0.010 |
| carrier_opt_wideselect_fold | 2011539ns | 1986085ns | 2030350ns | +0.18% | 0.008 |
| carrier_opt_wideselect_none | 2007907ns | 1981078ns | 2023539ns | base | 0.008 |

## Performance model

- Peak throughput: **0.011 Gops/s** (carrier_opt_wideselect_cse; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_wideselect_all | 0.010 | 94.4% |
| carrier_opt_wideselect_cse | 0.010 | 97.7% |
| carrier_opt_wideselect_cseeqsat | 0.010 | 93.6% |
| carrier_opt_wideselect_dce | 0.008 | 77.2% |
| carrier_opt_wideselect_eqsat | 0.010 | 94.7% |
| carrier_opt_wideselect_fold | 0.008 | 76.5% |
| carrier_opt_wideselect_none | 0.008 | 76.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_wideselect_all | 1630979ns | 1630979ns | -18.92% |
| carrier_opt_wideselect_cse | 1583520ns | 1583520ns | -21.28% |
| carrier_opt_wideselect_cseeqsat | 1649064ns | 1649064ns | -18.02% |
| carrier_opt_wideselect_dce | 2019815ns | 2019815ns | +0.41% |
| carrier_opt_wideselect_eqsat | 1634848ns | 1634848ns | -18.73% |
| carrier_opt_wideselect_fold | 2015150ns | 2015150ns | +0.18% |
| carrier_opt_wideselect_none | 2011564ns | 2011564ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_wideselect_none | 2010466ns | base | --- | [1989715, 2023539] | --- | --- | --- | --- |
| carrier_opt_wideselect_all | 1630096ns | -370837.1ns (-18.4%) | [-406567, -363765]ns | [1607974, 1644480] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_wideselect_cse | 1574486ns | -440854.0ns (-21.9%) | [-449119, -393832]ns | [1555695, 1609734] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_wideselect_cseeqsat | 1643768ns | -362719.4ns (-18.0%) | [-393029, -331261]ns | [1622139, 1670804] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_wideselect_dce | 1992854ns | no significant difference | [-37126, +89641]ns | [1976983, 2079356] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_wideselect_eqsat | 1624255ns | -382368.0ns (-19.0%) | [-400223, -346673]ns | [1615572, 1654629] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_wideselect_fold | 2010585ns | no significant difference | [-26214, +40585]ns | [1993682, 2030350] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_wideselect_none | carrier_opt_wideselect_all | carrier_opt_wideselect_cse | carrier_opt_wideselect_cseeqsat | carrier_opt_wideselect_dce | carrier_opt_wideselect_eqsat | carrier_opt_wideselect_fold |
|---|---|---|---|---|---|---|---|
| 1 | 1998352ns | -18.5% | -18.7% | -18.7% | +0.8% | -17.1% | +1.0% |
| 2 | 2012152ns | -17.9% | -21.8% | -16.9% | -1.3% | -19.6% | -1.3% |
| 3 | 2019439ns | -20.7% | -22.1% | -17.4% | -1.9% | -20.1% | -0.0% |
| 4 | 2008780ns | -18.5% | -20.6% | -19.3% | -1.8% | -19.4% | -0.3% |
| 5 | 2027640ns | -19.5% | -22.3% | -19.6% | -1.4% | -18.5% | -1.3% |
| 6 | 1981078ns | -18.5% | -22.4% | -16.3% | +8.3% | -17.8% | +3.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_wideselect_all | -0.566 | HIGH- (thermal bounce) |
| carrier_opt_wideselect_cse | -0.055 | ok |
| carrier_opt_wideselect_cseeqsat | -0.122 | ok |
| carrier_opt_wideselect_dce | 0.061 | ok |
| carrier_opt_wideselect_eqsat | -0.113 | ok |
| carrier_opt_wideselect_fold | -0.358 | moderate- |
| carrier_opt_wideselect_none | -0.365 | moderate- |

**Consistency summary:**

- **carrier_opt_wideselect_all**: won 6/6, lost 0/6
- **carrier_opt_wideselect_cse**: won 6/6, lost 0/6
- **carrier_opt_wideselect_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_wideselect_dce**: won 4/6, lost 2/6
- **carrier_opt_wideselect_eqsat**: won 6/6, lost 0/6
- **carrier_opt_wideselect_fold**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 1630662.9ns | 1627516.9ns | 100.2% | HIGH |
| carrier_opt_wideselect_cse | 1582169.5ns | 1579971.7ns | 100.1% | HIGH |
| carrier_opt_wideselect_cseeqsat | 1647602.1ns | 1645570.1ns | 100.1% | HIGH |
| carrier_opt_wideselect_dce | 2366676.5ns | 2016397.8ns | 117.4% | HIGH |
| carrier_opt_wideselect_eqsat | 1633686.5ns | 1631485.2ns | 100.1% | HIGH |
| carrier_opt_wideselect_fold | 2015891.0ns | 2011539.0ns | 100.2% | HIGH |
| carrier_opt_wideselect_none | 2011399.0ns | 2007906.7ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_opt_wideselect_all (n=6, range 1601925.0-1644480.4 ns)
  1601925.0 |########################################
  1604052.8 |
  1606180.5 |
  1608308.3 |
  1610436.1 |
  1612563.9 |########################################
  1614691.6 |
  1616819.4 |
  1618947.2 |
  1621075.0 |
  1623202.7 |
  1625330.5 |
  1627458.3 |########################################
  1629586.0 |
  1631713.8 |########################################
  1633841.6 |
  1635969.4 |########################################
  1638097.1 |
  1640224.9 |
  1642352.7 |
  (0 below, 1 above range)

carrier_opt_wideselect_cse (n=6, range 1538095.4-1609733.8 ns)
  1538095.4 |####################
  1541677.3 |
  1545259.2 |
  1548841.2 |
  1552423.1 |
  1556005.0 |
  1559586.9 |
  1563168.8 |
  1566750.7 |
  1570332.7 |########################################
  1573914.6 |####################
  1577496.5 |
  1581078.4 |
  1584660.3 |
  1588242.2 |
  1591824.2 |####################
  1595406.1 |
  1598988.0 |
  1602569.9 |
  1606151.8 |
  (0 below, 1 above range)

carrier_opt_wideselect_cseeqsat (n=6, range 1620482.5-1670803.5 ns)
  1620482.5 |########################################
  1622998.6 |########################################
  1625514.6 |
  1628030.6 |########################################
  1630546.7 |
  1633062.8 |
  1635578.8 |
  1638094.9 |
  1640610.9 |
  1643126.9 |
  1645643.0 |
  1648159.1 |
  1650675.1 |
  1653191.1 |
  1655707.2 |########################################
  1658223.2 |
  1660739.3 |
  1663255.4 |
  1665771.4 |
  1668287.4 |########################################
  (0 below, 1 above range)

carrier_opt_wideselect_dce (n=6, range 1971963.3-2079356.1 ns)
  1971963.3 |########################################
  1977332.9 |########################################
  1982702.6 |########################################
  1988072.2 |
  1993441.9 |
  1998811.5 |########################################
  2004181.1 |
  2009550.8 |########################################
  2014920.4 |
  2020290.0 |
  2025659.7 |
  2031029.3 |
  2036398.9 |
  2041768.6 |
  2047138.2 |
  2052507.9 |
  2057877.5 |
  2063247.1 |
  2068616.8 |
  2073986.4 |
  (0 below, 1 above range)

carrier_opt_wideselect_eqsat (n=6, range 1613459.6-1654628.8 ns)
  1613459.6 |########################################
  1615518.1 |
  1617576.5 |########################################
  1619635.0 |########################################
  1621693.4 |
  1623751.9 |
  1625810.3 |
  1627868.8 |########################################
  1629927.3 |
  1631985.7 |
  1634044.2 |
  1636102.6 |
  1638161.1 |
  1640219.5 |
  1642278.0 |
  1644336.5 |
  1646394.9 |
  1648453.4 |
  1650511.8 |########################################
  1652570.3 |
  (0 below, 1 above range)

carrier_opt_wideselect_fold (n=6, range 1986085.0-2030350.0 ns)
  1986085.0 |####################
  1988298.2 |
  1990511.5 |
  1992724.8 |
  1994938.0 |
  1997151.2 |
  1999364.5 |####################
  2001577.8 |####################
  2003791.0 |
  2006004.2 |
  2008217.5 |
  2010430.8 |
  2012644.0 |
  2014857.2 |
  2017070.5 |########################################
  2019283.8 |
  2021497.0 |
  2023710.2 |
  2025923.5 |
  2028136.8 |
  (0 below, 1 above range)

carrier_opt_wideselect_none (n=6, range 1981078.3-2023539.4 ns)
  1981078.3 |########################################
  1983201.4 |
  1985324.4 |
  1987447.5 |
  1989570.5 |
  1991693.6 |
  1993816.6 |
  1995939.7 |
  1998062.7 |########################################
  2000185.8 |
  2002308.8 |
  2004431.9 |
  2006554.9 |
  2008678.0 |########################################
  2010801.0 |########################################
  2012924.1 |
  2015047.1 |
  2017170.2 |
  2019293.2 |########################################
  2021416.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_wideselect_all**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cse**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cseeqsat**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_dce**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_eqsat**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_fold**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_none**: bridge=100.2% of algo (FFI overhead may distort results)
