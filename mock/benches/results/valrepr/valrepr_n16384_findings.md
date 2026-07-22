# Value representation: static/raw vs runtime-tagged vs NaN-boxed (dynamic-typing cost)

3 variants, 6 samples per variant.
Baseline: **valrepr_static**

## Highlights

Baseline for all deltas below: **valrepr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (valrepr_static) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline valrepr_static has the worst median (1.78 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest valrepr_nanbox at 1.47 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### valrepr_nanbox dominates: 11% faster than the next best (valrepr_tagged)

valrepr_nanbox (1.47 ms) leads valrepr_tagged (1.63 ms) by 11%, a clear separation rather than a photo finish. CV 5.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### valrepr_nanbox is fastest but the noisiest (CV 5.4%)

valrepr_nanbox wins on median (1.47 ms) yet has the highest variance (CV 5.4%), while valrepr_tagged is the steadiest (CV 2.3%, 1.63 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: valrepr_nanbox** at 1469230.5 ns median (-17.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.21x (fastest 1469230.5 ns, slowest 1779225.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| valrepr_nanbox | 1464316ns | 1472178ns | 1317153ns | 1462691ns | 1540335ns | -17.29% |
| valrepr_static | 1770372ns | 1782518ns | 1653165ns | 1780383ns | 1813959ns | base |
| valrepr_tagged | 1632276ns | 1629601ns | 1589780ns | 1618239ns | 1674578ns | -7.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| valrepr_nanbox | 1461234ns | 1314549ns | 1536836ns | -17.32% | 0.011 |
| valrepr_static | 1767381ns | 1650723ns | 1810858ns | base | 0.009 |
| valrepr_tagged | 1629787ns | 1587428ns | 1672077ns | -7.79% | 0.010 |

## Performance model

- Peak throughput: **0.012 Gops/s** (valrepr_nanbox; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| valrepr_nanbox | 0.011 | 89.5% |
| valrepr_static | 0.009 | 73.9% |
| valrepr_tagged | 0.010 | 80.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| valrepr_nanbox | 1464316ns | 1464316ns | -17.29% |
| valrepr_static | 1770372ns | 1770372ns | base |
| valrepr_tagged | 1632276ns | 1632276ns | -7.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| valrepr_static | 1779225ns | base | --- | [1712059, 1810858] | --- | --- | --- | --- |
| valrepr_nanbox | 1469230ns | -318068.6ns (-17.9%) | [-417872, -182499]ns | [1377636, 1536836] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| valrepr_tagged | 1627073ns | -154310.0ns (-8.7%) | [-216577, -41894]ns | [1590212, 1672077] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | valrepr_static | valrepr_nanbox | valrepr_tagged |
|---|---|---|---|
| 1 | 1781232ns | -16.7% | -10.9% |
| 2 | 1832345ns | -20.6% | -13.1% |
| 3 | 1773396ns | -25.9% | -7.7% |
| 4 | 1777218ns | -18.9% | -7.5% |
| 5 | 1789371ns | -16.7% | -9.6% |
| 6 | 1650723ns | -4.0% | +3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| valrepr_nanbox | 0.173 | ok |
| valrepr_static | -0.054 | ok |
| valrepr_tagged | 0.049 | ok |

**Consistency summary:**

- **valrepr_nanbox**: won 6/6, lost 0/6
- **valrepr_tagged**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| valrepr_nanbox | 2807.1ns | 1461234.1ns | 0.2% |  |
| valrepr_static | 2822.1ns | 1767380.7ns | 0.2% |  |
| valrepr_tagged | 3577.1ns | 1629787.3ns | 0.2% |  |

## Distribution (algo ns)

```
valrepr_nanbox (n=6, range 1314548.7-1536836.0 ns)
  1314548.7 |####################
  1325663.1 |
  1336777.4 |
  1347891.8 |
  1359006.2 |
  1370120.5 |
  1381234.9 |
  1392349.3 |
  1403463.6 |
  1414578.0 |
  1425692.4 |
  1436806.7 |####################
  1447921.1 |####################
  1459035.4 |
  1470149.8 |
  1481264.2 |########################################
  1492378.5 |
  1503492.9 |
  1514607.3 |
  1525721.6 |
  (0 below, 1 above range)

valrepr_static (n=6, range 1650722.9-1810857.7 ns)
  1650722.9 |####################
  1658729.6 |
  1666736.4 |
  1674743.1 |
  1682749.9 |
  1690756.6 |
  1698763.3 |
  1706770.1 |
  1714776.8 |
  1722783.6 |
  1730790.3 |
  1738797.0 |
  1746803.8 |
  1754810.5 |
  1762817.3 |
  1770824.0 |########################################
  1778830.7 |####################
  1786837.5 |####################
  1794844.2 |
  1802851.0 |
  (0 below, 1 above range)

valrepr_tagged (n=6, range 1587427.5-1672076.9 ns)
  1587427.5 |########################################
  1591660.0 |########################################
  1595892.4 |
  1600124.9 |
  1604357.4 |
  1608589.8 |
  1612822.3 |
  1617054.8 |########################################
  1621287.2 |
  1625519.7 |
  1629752.2 |
  1633984.6 |########################################
  1638217.1 |
  1642449.6 |########################################
  1646682.0 |
  1650914.5 |
  1655147.0 |
  1659379.4 |
  1663611.9 |
  1667844.4 |
  (0 below, 1 above range)

```
