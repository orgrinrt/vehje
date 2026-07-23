# Value representation (static / runtime-tagged / NaN-boxed)

3 variants, 6 samples per variant.
Baseline: **carrier_vr_static**

## Highlights

Baseline for all deltas below: **carrier_vr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vr_static) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vr_static has the worst median (1.78 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vr_nanbox at 1.41 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vr_nanbox dominates: 18% faster than the next best (carrier_vr_tagged)

carrier_vr_nanbox (1.41 ms) leads carrier_vr_tagged (1.66 ms) by 18%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vr_nanbox beats baseline by 21% (significant)

carrier_vr_nanbox is -366.73 us (21%) faster than baseline carrier_vr_static, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_vr_nanbox** at 1412617.5 ns median (-20.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.26x (fastest 1412617.5 ns, slowest 1778947.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vr_nanbox | 1427432ns | 1415877ns | 1398295ns | 1414613ns | 1461228ns | -19.74% |
| carrier_vr_static | 1778427ns | 1782334ns | 1732566ns | 1775718ns | 1805422ns | base |
| carrier_vr_tagged | 1667242ns | 1665371ns | 1608603ns | 1663934ns | 1701523ns | -6.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vr_nanbox | 1424103ns | 1395160ns | 1457689ns | -19.77% | 0.012 |
| carrier_vr_static | 1774944ns | 1729512ns | 1801750ns | base | 0.009 |
| carrier_vr_tagged | 1664371ns | 1605858ns | 1698510ns | -6.23% | 0.010 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vr_nanbox | 8781734 | 19676695 | 0.446 | 0.80× |
| carrier_vr_static | 11031079 | 14125731 | 0.781 | 1.00× |
| carrier_vr_tagged | 10331900 | 20791365 | 0.497 | 0.94× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.012 Gops/s** (carrier_vr_nanbox; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vr_nanbox | 0.012 | 98.8% |
| carrier_vr_static | 0.009 | 78.4% |
| carrier_vr_tagged | 0.010 | 83.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vr_nanbox | 1427432ns | 1427432ns | -19.74% |
| carrier_vr_static | 1778427ns | 1778427ns | base |
| carrier_vr_tagged | 1667242ns | 1667242ns | -6.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vr_static | 1778948ns | base | --- | [1744135, 1801750] | --- | --- | --- | --- |
| carrier_vr_nanbox | 1412618ns | -366734.8ns (-20.6%) | [-399342, -286446]ns | [1402004, 1457689] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vr_tagged | 1662620ns | -124587.7ns (-7.0%) | [-151370, -55761]ns | [1631984, 1698510] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vr_static | carrier_vr_nanbox | carrier_vr_tagged |
|---|---|---|---|
| 1 | 1810676ns | -22.9% | -8.3% |
| 2 | 1779030ns | -20.4% | -4.6% |
| 3 | 1778866ns | -20.8% | -6.4% |
| 4 | 1758758ns | -17.7% | -8.7% |
| 5 | 1729512ns | -15.2% | -1.7% |
| 6 | 1792823ns | -21.4% | -7.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vr_nanbox | 0.110 | ok |
| carrier_vr_static | 0.005 | ok |
| carrier_vr_tagged | -0.415 | moderate- |

**Consistency summary:**

- **carrier_vr_nanbox**: won 6/6, lost 0/6
- **carrier_vr_tagged**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vr_nanbox | 1427279.4ns | 1424103.5ns | 100.2% | HIGH |
| carrier_vr_static | 1780397.5ns | 1774944.2ns | 100.3% | HIGH |
| carrier_vr_tagged | 1670674.2ns | 1664371.3ns | 100.4% | HIGH |

## Distribution (algo ns)

```
carrier_vr_nanbox (n=6, range 1395160.4-1457689.4 ns)
  1395160.4 |####################
  1398286.8 |
  1401413.3 |
  1404539.8 |
  1407666.2 |########################################
  1410792.6 |
  1413919.1 |####################
  1417045.5 |
  1420172.0 |
  1423298.4 |
  1426424.9 |
  1429551.3 |
  1432677.8 |
  1435804.2 |
  1438930.7 |
  1442057.1 |
  1445183.6 |####################
  1448310.0 |
  1451436.5 |
  1454562.9 |
  (0 below, 1 above range)

carrier_vr_static (n=6, range 1729511.7-1801749.8 ns)
  1729511.7 |####################
  1733123.6 |
  1736735.5 |
  1740347.4 |
  1743959.3 |
  1747571.2 |
  1751183.1 |
  1754795.0 |
  1758406.9 |####################
  1762018.8 |
  1765630.7 |
  1769242.6 |
  1772854.5 |
  1776466.4 |########################################
  1780078.3 |
  1783690.2 |
  1787302.1 |
  1790914.0 |####################
  1794525.9 |
  1798137.8 |
  (0 below, 1 above range)

carrier_vr_tagged (n=6, range 1605857.5-1698510.0 ns)
  1605857.5 |####################
  1610490.1 |
  1615122.8 |
  1619755.4 |
  1624388.0 |
  1629020.6 |
  1633653.2 |
  1638285.9 |
  1642918.5 |
  1647551.1 |
  1652183.8 |
  1656816.4 |########################################
  1661449.0 |####################
  1666081.6 |
  1670714.2 |
  1675346.9 |
  1679979.5 |
  1684612.1 |
  1689244.8 |
  1693877.4 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vr_nanbox**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_vr_static**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_vr_tagged**: bridge=100.4% of algo (FFI overhead may distort results)
