# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, wideselect profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_wideselect_none**

## Highlights

Baseline for all deltas below: **carrier_opt_wideselect_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_wideselect_all beats baseline by 22% (significant)

carrier_opt_wideselect_all is -438.48 us (22%) faster than baseline carrier_opt_wideselect_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (carrier_opt_wideselect_all, carrier_opt_wideselect_canon) are a dead heat (<1%)

carrier_opt_wideselect_all (1.56 ms) and carrier_opt_wideselect_canon (1.57 ms) differ by 0.59%, inside the noise, even though the wider field spreads 29.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {carrier_opt_wideselect_all, carrier_opt_wideselect_canon, carrier_opt_wideselect_cse} vs {carrier_opt_wideselect_none, carrier_opt_wideselect_fold, carrier_opt_wideselect_dce} (26% apart)

The field splits into a fast tier {carrier_opt_wideselect_all, carrier_opt_wideselect_canon, carrier_opt_wideselect_cse} and a slow tier {carrier_opt_wideselect_none, carrier_opt_wideselect_fold, carrier_opt_wideselect_dce} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_opt_wideselect_all** at 1560935.4 ns median (-21.8% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.29x (fastest 1560935.4 ns, slowest 2017791.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 1560940ns | 1564535ns | 1531855ns | 1563874ns | 1571082ns | -22.15% |
| carrier_opt_wideselect_canon | 1574153ns | 1573036ns | 1559435ns | 1569312ns | 1588775ns | -21.49% |
| carrier_opt_wideselect_cse | 1590280ns | 1586174ns | 1562556ns | 1582767ns | 1615411ns | -20.69% |
| carrier_opt_wideselect_dce | 2021095ns | 2021630ns | 2002310ns | 2018633ns | 2034180ns | +0.80% |
| carrier_opt_wideselect_fold | 2019935ns | 2017738ns | 2002402ns | 2012764ns | 2039458ns | +0.74% |
| carrier_opt_wideselect_none | 2005083ns | 2000258ns | 1993589ns | 1998379ns | 2020885ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 1557673ns | 1528940ns | 1568289ns | -22.18% | 0.011 |
| carrier_opt_wideselect_canon | 1570933ns | 1555703ns | 1585553ns | -21.51% | 0.010 |
| carrier_opt_wideselect_cse | 1586862ns | 1558948ns | 1612100ns | -20.72% | 0.010 |
| carrier_opt_wideselect_dce | 2017485ns | 1998741ns | 2030481ns | +0.80% | 0.008 |
| carrier_opt_wideselect_fold | 2016342ns | 1999036ns | 2035786ns | +0.74% | 0.008 |
| carrier_opt_wideselect_none | 2001562ns | 1990246ns | 2017364ns | base | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 9703461 | 15044244 | 0.645 | 0.77× |
| carrier_opt_wideselect_canon | 9799260 | 15192490 | 0.645 | 0.78× |
| carrier_opt_wideselect_cse | 9946370 | 15196112 | 0.655 | 0.79× |
| carrier_opt_wideselect_dce | 12626232 | 17324107 | 0.729 | 1.01× |
| carrier_opt_wideselect_fold | 12615879 | 17299769 | 0.729 | 1.01× |
| carrier_opt_wideselect_none | 12534691 | 17324195 | 0.724 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.011 Gops/s** (carrier_opt_wideselect_all; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_wideselect_all | 0.010 | 98.0% |
| carrier_opt_wideselect_canon | 0.010 | 97.4% |
| carrier_opt_wideselect_cse | 0.010 | 96.6% |
| carrier_opt_wideselect_dce | 0.008 | 75.8% |
| carrier_opt_wideselect_fold | 0.008 | 75.9% |
| carrier_opt_wideselect_none | 0.008 | 76.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_wideselect_all | 1560940ns | 1560940ns | -22.15% |
| carrier_opt_wideselect_canon | 1574153ns | 1574153ns | -21.49% |
| carrier_opt_wideselect_cse | 1590280ns | 1590280ns | -20.69% |
| carrier_opt_wideselect_dce | 2021095ns | 2021095ns | +0.80% |
| carrier_opt_wideselect_fold | 2019935ns | 2019935ns | +0.74% |
| carrier_opt_wideselect_none | 2005083ns | 2005083ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_wideselect_none | 1996627ns | base | --- | [1990695, 2017364] | --- | --- | --- | --- |
| carrier_opt_wideselect_all | 1560935ns | -438480.7ns (-22.0%) | [-460182, -433004]ns | [1543795, 1568289] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_wideselect_canon | 1570139ns | -433007.3ns (-21.7%) | [-446044, -412835]ns | [1557107, 1585553] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_wideselect_cse | 1582830ns | -422079.5ns (-21.1%) | [-440900, -381121]ns | [1565655, 1612100] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_wideselect_dce | 2017791ns | +15818.5ns (+0.8%) | [+2938, +29013]ns | [2004183, 2030481] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_wideselect_fold | 2014138ns | +8407.3ns (+0.4%) | [+1511, +34423]ns | [1999102, 2035786] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_wideselect_none | carrier_opt_wideselect_all | carrier_opt_wideselect_canon | carrier_opt_wideselect_cse | carrier_opt_wideselect_dce | carrier_opt_wideselect_fold |
|---|---|---|---|---|---|---|
| 1 | 2028197ns | -22.6% | -22.4% | -21.9% | +0.2% | -0.1% |
| 2 | 2006531ns | -21.9% | -20.5% | -21.2% | +0.7% | +1.1% |
| 3 | 1991144ns | -23.2% | -21.9% | -21.0% | +0.9% | +0.4% |
| 4 | 1997059ns | -21.9% | -21.6% | -21.9% | +1.6% | +0.3% |
| 5 | 1996195ns | -21.8% | -21.9% | -20.5% | +0.1% | +2.3% |
| 6 | 1990246ns | -21.7% | -20.9% | -17.8% | +1.3% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_wideselect_all | -0.186 | ok |
| carrier_opt_wideselect_canon | -0.225 | moderate- |
| carrier_opt_wideselect_cse | 0.149 | ok |
| carrier_opt_wideselect_dce | -0.361 | moderate- |
| carrier_opt_wideselect_fold | -0.395 | moderate- |
| carrier_opt_wideselect_none | 0.208 | moderate+ |

**Consistency summary:**

- **carrier_opt_wideselect_all**: won 6/6, lost 0/6
- **carrier_opt_wideselect_canon**: won 6/6, lost 0/6
- **carrier_opt_wideselect_cse**: won 6/6, lost 0/6
- **carrier_opt_wideselect_dce**: won 0/6, lost 6/6
- **carrier_opt_wideselect_fold**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 1560388.0ns | 1557673.1ns | 100.2% | HIGH |
| carrier_opt_wideselect_canon | 1573431.5ns | 1570933.0ns | 100.2% | HIGH |
| carrier_opt_wideselect_cse | 1590738.6ns | 1586861.7ns | 100.2% | HIGH |
| carrier_opt_wideselect_dce | 2021043.5ns | 2017485.0ns | 100.2% | HIGH |
| carrier_opt_wideselect_fold | 2017863.7ns | 2016342.1ns | 100.1% | HIGH |
| carrier_opt_wideselect_none | 2003581.2ns | 2001561.9ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_opt_wideselect_all (n=6, range 1528939.6-1568288.6 ns)
  1528939.6 |####################
  1530907.0 |
  1532874.5 |
  1534841.9 |
  1536809.4 |
  1538776.8 |
  1540744.3 |
  1542711.7 |
  1544679.2 |
  1546646.6 |
  1548614.1 |
  1550581.5 |
  1552549.0 |
  1554516.4 |
  1556483.9 |
  1558451.3 |########################################
  1560418.8 |####################
  1562386.2 |
  1564353.7 |
  1566321.1 |####################
  (0 below, 1 above range)

carrier_opt_wideselect_canon (n=6, range 1555702.9-1585553.4 ns)
  1555702.9 |####################
  1557195.4 |####################
  1558687.9 |
  1560180.5 |
  1561673.0 |
  1563165.5 |
  1564658.0 |
  1566150.6 |####################
  1567643.1 |
  1569135.6 |
  1570628.1 |
  1572120.6 |
  1573613.2 |########################################
  1575105.7 |
  1576598.2 |
  1578090.7 |
  1579583.3 |
  1581075.8 |
  1582568.3 |
  1584060.8 |
  (0 below, 1 above range)

carrier_opt_wideselect_cse (n=6, range 1558948.3-1612099.6 ns)
  1558948.3 |########################################
  1561605.9 |
  1564263.4 |
  1566921.0 |
  1569578.6 |
  1572236.1 |########################################
  1574893.7 |
  1577551.2 |
  1580208.8 |########################################
  1582866.4 |########################################
  1585523.9 |########################################
  1588181.5 |
  1590839.1 |
  1593496.6 |
  1596154.2 |
  1598811.7 |
  1601469.3 |
  1604126.9 |
  1606784.4 |
  1609442.0 |
  (0 below, 1 above range)

carrier_opt_wideselect_dce (n=6, range 1998741.2-2030480.6 ns)
  1998741.2 |########################################
  2000328.2 |
  2001915.1 |
  2003502.1 |
  2005089.1 |
  2006676.1 |
  2008263.0 |########################################
  2009850.0 |
  2011437.0 |
  2013023.9 |
  2014610.9 |########################################
  2016197.9 |
  2017784.8 |
  2019371.8 |########################################
  2020958.8 |
  2022545.8 |
  2024132.7 |
  2025719.7 |
  2027306.7 |
  2028893.6 |########################################
  (0 below, 1 above range)

carrier_opt_wideselect_fold (n=6, range 1999036.2-2035785.6 ns)
  1999036.2 |########################################
  2000873.7 |####################
  2002711.1 |
  2004548.6 |
  2006386.1 |
  2008223.6 |
  2010061.0 |
  2011898.5 |
  2013736.0 |
  2015573.5 |
  2017410.9 |
  2019248.4 |
  2021085.9 |
  2022923.3 |
  2024760.8 |####################
  2026598.3 |
  2028435.8 |####################
  2030273.2 |
  2032110.7 |
  2033948.2 |
  (0 below, 1 above range)

carrier_opt_wideselect_none (n=6, range 1990246.2-2017363.8 ns)
  1990246.2 |########################################
  1991602.1 |
  1992958.0 |
  1994313.8 |
  1995669.7 |####################
  1997025.6 |####################
  1998381.5 |
  1999737.3 |
  2001093.2 |
  2002449.1 |
  2003805.0 |
  2005160.9 |
  2006516.7 |####################
  2007872.6 |
  2009228.5 |
  2010584.4 |
  2011940.2 |
  2013296.1 |
  2014652.0 |
  2016007.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_wideselect_all**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_canon**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cse**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_dce**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_fold**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_none**: bridge=100.0% of algo (FFI overhead may distort results)
