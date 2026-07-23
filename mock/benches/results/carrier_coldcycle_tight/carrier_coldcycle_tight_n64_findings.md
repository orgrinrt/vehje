# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), tight profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Two tiers: {carrier_cold_tight_null, carrier_cold_tight_threaded, carrier_cold_tight_switch} vs {carrier_cold_tight_fntable} (30% apart)

The field splits into a fast tier {carrier_cold_tight_null, carrier_cold_tight_threaded, carrier_cold_tight_switch} and a slow tier {carrier_cold_tight_fntable} with a 30% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cold_tight_null** at 1542.9 ns median (-11.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.46x (fastest 1542.9 ns, slowest 2253.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_tight_fntable | 4553ns | 4586ns | 4297ns | 4509ns | 4748ns | +12.05% |
| carrier_cold_tight_null | 3909ns | 3909ns | 3567ns | 3854ns | 4163ns | -3.80% |
| carrier_cold_tight_switch | 4064ns | 4087ns | 3837ns | 4009ns | 4259ns | base |
| carrier_cold_tight_threaded | 4025ns | 4015ns | 3758ns | 3937ns | 4291ns | -0.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_tight_fntable | 2252ns | 2133ns | 2365ns | +30.47% | 0.028 |
| carrier_cold_tight_null | 1542ns | 1428ns | 1628ns | -10.63% | 0.041 |
| carrier_cold_tight_switch | 1726ns | 1628ns | 1806ns | base | 0.037 |
| carrier_cold_tight_threaded | 1689ns | 1587ns | 1804ns | -2.13% | 0.038 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_tight_fntable | 263878 | 949876 | 0.278 | 0.93× |
| carrier_cold_tight_null | 269693 | 1247610 | 0.216 | 0.95× |
| carrier_cold_tight_switch | 284718 | 1116336 | 0.255 | 1.00× |
| carrier_cold_tight_threaded | 267787 | 1124956 | 0.238 | 0.94× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.045 Gops/s** (carrier_cold_tight_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_tight_fntable | 0.028 | 63.4% |
| carrier_cold_tight_null | 0.041 | 92.5% |
| carrier_cold_tight_switch | 0.037 | 82.2% |
| carrier_cold_tight_threaded | 0.038 | 85.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_tight_fntable | 4553ns | 4553ns | +12.05% |
| carrier_cold_tight_null | 3909ns | 3909ns | -3.80% |
| carrier_cold_tight_switch | 4064ns | 4064ns | base |
| carrier_cold_tight_threaded | 4025ns | 4025ns | -0.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_tight_switch | 1738ns | base | --- | [1634, 1806] | --- | --- | --- | --- |
| carrier_cold_tight_fntable | 2253ns | +530.6ns (+30.5%) | [+485, +562]ns | [2138, 2365] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_tight_null | 1543ns | -178.3ns (-10.3%) | [-225, -148]ns | [1456, 1628] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_tight_threaded | 1676ns | no significant difference | [-102, +29]ns | [1587, 1804] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_tight_switch | carrier_cold_tight_fntable | carrier_cold_tight_null | carrier_cold_tight_threaded |
|---|---|---|---|---|
| 1 | 1678ns | +27.7% | -8.4% | -5.4% |
| 2 | 1812ns | +31.2% | -10.7% | -1.8% |
| 3 | 1801ns | +30.6% | -9.0% | +1.5% |
| 4 | 1798ns | +28.4% | -13.9% | -6.3% |
| 5 | 1640ns | +34.1% | -9.5% | +1.8% |
| 6 | 1628ns | +31.0% | -12.3% | -2.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_tight_fntable | 0.135 | ok |
| carrier_cold_tight_null | 0.437 | moderate+ |
| carrier_cold_tight_switch | 0.265 | moderate+ |
| carrier_cold_tight_threaded | 0.098 | ok |

**Consistency summary:**

- **carrier_cold_tight_fntable**: won 0/6, lost 6/6
- **carrier_cold_tight_null**: won 6/6, lost 0/6
- **carrier_cold_tight_threaded**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_tight_fntable | 82927.9ns | 2251.9ns | 3682.5% | HIGH |
| carrier_cold_tight_null | 86538.5ns | 1542.4ns | 5610.6% | HIGH |
| carrier_cold_tight_switch | 91381.9ns | 1726.0ns | 5294.5% | HIGH |
| carrier_cold_tight_threaded | 85647.1ns | 1689.2ns | 5070.2% | HIGH |

## Distribution (algo ns)

```
carrier_cold_tight_fntable (n=6, range 2132.9-2365.0 ns)
   2132.9 |########################################
   2144.5 |
   2156.1 |
   2167.7 |
   2179.3 |
   2190.9 |####################
   2202.5 |
   2214.1 |
   2225.7 |
   2237.3 |
   2248.9 |
   2260.6 |
   2272.2 |
   2283.8 |
   2295.4 |
   2307.0 |####################
   2318.6 |
   2330.2 |
   2341.8 |####################
   2353.4 |
  (0 below, 1 above range)

carrier_cold_tight_null (n=6, range 1427.9-1628.1 ns)
   1427.9 |########################################
   1437.9 |
   1447.9 |
   1457.9 |
   1467.9 |
   1478.0 |########################################
   1488.0 |
   1498.0 |
   1508.0 |
   1518.0 |
   1528.0 |########################################
   1538.0 |
   1548.0 |########################################
   1558.0 |
   1568.0 |
   1578.0 |
   1588.1 |
   1598.1 |
   1608.1 |########################################
   1618.1 |
  (0 below, 1 above range)

carrier_cold_tight_switch (n=6, range 1627.9-1806.4 ns)
   1627.9 |########################################
   1636.8 |########################################
   1645.8 |
   1654.7 |
   1663.6 |
   1672.5 |########################################
   1681.5 |
   1690.4 |
   1699.3 |
   1708.2 |
   1717.2 |
   1726.1 |
   1735.0 |
   1744.0 |
   1752.9 |
   1761.8 |
   1770.7 |
   1779.7 |
   1788.6 |########################################
   1797.5 |########################################
  (0 below, 1 above range)

carrier_cold_tight_threaded (n=6, range 1587.1-1803.9 ns)
   1587.1 |########################################
   1597.9 |
   1608.8 |
   1619.6 |
   1630.5 |
   1641.3 |
   1652.2 |
   1663.0 |####################
   1673.8 |####################
   1684.7 |
   1695.5 |
   1706.4 |
   1717.2 |
   1728.1 |
   1738.9 |
   1749.7 |
   1760.6 |
   1771.4 |####################
   1782.3 |
   1793.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_tight_fntable**: bridge=3688.9% of algo (FFI overhead may distort results)
- **carrier_cold_tight_null**: bridge=5604.5% of algo (FFI overhead may distort results)
- **carrier_cold_tight_switch**: bridge=5272.1% of algo (FFI overhead may distort results)
- **carrier_cold_tight_threaded**: bridge=5103.3% of algo (FFI overhead may distort results)
