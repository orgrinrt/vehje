# Record width: wire-format stride and inline-vs-pool decode cost (carrier)

5 variants, 6 samples per variant.
Baseline: **carrier_rec24**

## Highlights

Baseline for all deltas below: **carrier_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_rec24)

The baseline carrier_rec24 is the fastest (1.79 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 3.6% of the fastest

All 5 variants sit between 1.79 ms and 1.85 ms - a 3.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_rec24) is the fastest** at 1785806.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 1785806.2 ns, slowest 1849955.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_rec12 | 1854004ns | 1853588ns | 1810568ns | 1843830ns | 1890983ns | +3.85% |
| carrier_rec16 | 1822047ns | 1828022ns | 1791454ns | 1817098ns | 1844768ns | +2.06% |
| carrier_rec20 | 1838737ns | 1830671ns | 1792477ns | 1819498ns | 1890726ns | +2.99% |
| carrier_rec24 | 1785288ns | 1789439ns | 1709219ns | 1780828ns | 1830013ns | base |
| carrier_rec32 | 1830224ns | 1827492ns | 1800554ns | 1820230ns | 1860049ns | +2.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_rec12 | 1850300ns | 1807004ns | 1887170ns | +3.85% | 0.009 |
| carrier_rec16 | 1818229ns | 1787542ns | 1841075ns | +2.05% | 0.009 |
| carrier_rec20 | 1835222ns | 1788912ns | 1886942ns | +3.00% | 0.009 |
| carrier_rec24 | 1781738ns | 1706434ns | 1826304ns | base | 0.009 |
| carrier_rec32 | 1826596ns | 1796777ns | 1856683ns | +2.52% | 0.009 |

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_rec24; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_rec12 | 0.009 | 92.2% |
| carrier_rec16 | 0.009 | 93.5% |
| carrier_rec20 | 0.009 | 93.4% |
| carrier_rec24 | 0.009 | 95.6% |
| carrier_rec32 | 0.009 | 93.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_rec12 | 1854004ns | 1854004ns | +3.85% |
| carrier_rec16 | 1822047ns | 1822047ns | +2.06% |
| carrier_rec20 | 1838737ns | 1838737ns | +2.99% |
| carrier_rec24 | 1785288ns | 1785288ns | base |
| carrier_rec32 | 1830224ns | 1830224ns | +2.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_rec24 | 1785806ns | base | --- | [1733103, 1826304] | --- | --- | --- | --- |
| carrier_rec12 | 1849955ns | +50145.6ns (+2.8%) | [+5154, +150386]ns | [1813773, 1887170] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| carrier_rec16 | 1824146ns | no significant difference | [-16506, +76410]ns | [1789465, 1841075] | no | 0.6875 | 0.6875 | 0 |
| carrier_rec20 | 1827174ns | no significant difference | [-8304, +88767]ns | [1791549, 1886942] | no | 0.2917 | 0.2188 | 0 |
| carrier_rec32 | 1823716ns | no significant difference | [-2588, +101227]ns | [1799390, 1856683] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_rec24 | carrier_rec12 | carrier_rec16 | carrier_rec20 | carrier_rec32 |
|---|---|---|---|---|---|
| 1 | 1839847ns | +1.4% | -1.1% | +4.3% | -0.7% |
| 2 | 1759772ns | +4.2% | +4.1% | +5.4% | +2.1% |
| 3 | 1812762ns | -0.3% | +2.1% | -1.0% | +0.4% |
| 4 | 1804477ns | +0.9% | -0.7% | +0.1% | +2.9% |
| 5 | 1706434ns | +11.2% | +4.8% | +4.8% | +8.8% |
| 6 | 1767135ns | +6.2% | +3.5% | +4.6% | +2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_rec12 | 0.261 | moderate+ |
| carrier_rec16 | 0.031 | ok |
| carrier_rec20 | 0.226 | moderate+ |
| carrier_rec24 | -0.166 | ok |
| carrier_rec32 | 0.039 | ok |

**Consistency summary:**

- **carrier_rec12**: won 1/6, lost 5/6
- **carrier_rec16**: won 2/6, lost 4/6
- **carrier_rec20**: won 1/6, lost 5/6
- **carrier_rec32**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_rec12 | 1901.1ns | 1850299.6ns | 0.1% |  |
| carrier_rec16 | 2037.2ns | 1818228.6ns | 0.1% |  |
| carrier_rec20 | 2196.1ns | 1835221.5ns | 0.1% |  |
| carrier_rec24 | 1654.1ns | 1781738.0ns | 0.1% |  |
| carrier_rec32 | 1706.6ns | 1826596.3ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_rec12 (n=6, range 1807004.2-1887170.4 ns)
  1807004.2 |########################################
  1811012.5 |
  1815020.8 |
  1819029.1 |########################################
  1823037.4 |
  1827045.8 |
  1831054.1 |########################################
  1835062.4 |
  1839070.7 |
  1843079.0 |
  1847087.3 |
  1851095.6 |
  1855103.9 |
  1859112.2 |
  1863120.5 |########################################
  1867128.8 |
  1871137.2 |
  1875145.5 |########################################
  1879153.8 |
  1883162.1 |
  (0 below, 1 above range)

carrier_rec16 (n=6, range 1787541.7-1841074.8 ns)
  1787541.7 |########################################
  1790218.4 |########################################
  1792895.0 |
  1795571.7 |
  1798248.3 |
  1800925.0 |
  1803601.6 |
  1806278.3 |
  1808954.9 |
  1811631.6 |
  1814308.2 |
  1816984.9 |
  1819661.6 |########################################
  1822338.2 |
  1825014.9 |
  1827691.5 |########################################
  1830368.2 |########################################
  1833044.8 |
  1835721.5 |
  1838398.1 |
  (0 below, 1 above range)

carrier_rec20 (n=6, range 1788912.1-1886941.6 ns)
  1788912.1 |########################################
  1793813.6 |########################################
  1798715.1 |
  1803616.5 |########################################
  1808518.0 |
  1813419.5 |
  1818321.0 |
  1823222.4 |
  1828123.9 |
  1833025.4 |
  1837926.9 |
  1842828.4 |
  1847729.8 |########################################
  1852631.3 |########################################
  1857532.8 |
  1862434.3 |
  1867335.7 |
  1872237.2 |
  1877138.7 |
  1882040.2 |
  (0 below, 1 above range)

carrier_rec24 (n=6, range 1706434.2-1826304.4 ns)
  1706434.2 |########################################
  1712427.7 |
  1718421.2 |
  1724414.7 |
  1730408.2 |
  1736401.8 |
  1742395.3 |
  1748388.8 |
  1754382.3 |########################################
  1760375.8 |
  1766369.3 |########################################
  1772362.8 |
  1778356.3 |
  1784349.8 |
  1790343.3 |
  1796336.8 |
  1802330.4 |########################################
  1808323.9 |########################################
  1814317.4 |
  1820310.9 |
  (0 below, 1 above range)

carrier_rec32 (n=6, range 1796777.1-1856682.7 ns)
  1796777.1 |########################################
  1799772.4 |########################################
  1802767.7 |
  1805762.9 |
  1808758.2 |
  1811753.5 |
  1814748.8 |
  1817744.1 |########################################
  1820739.3 |
  1823734.6 |
  1826729.9 |########################################
  1829725.2 |
  1832720.5 |
  1835715.7 |
  1838711.0 |
  1841706.3 |
  1844701.6 |
  1847696.9 |
  1850692.1 |
  1853687.4 |########################################
  (0 below, 1 above range)

```
