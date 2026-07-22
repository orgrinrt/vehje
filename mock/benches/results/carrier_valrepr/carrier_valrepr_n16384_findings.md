# Value representation (static / runtime-tagged / NaN-boxed)

3 variants, 6 samples per variant.
Baseline: **carrier_vr_static**

## Highlights

Baseline for all deltas below: **carrier_vr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vr_static) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vr_static has the worst median (1.92 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vr_nanbox at 1.52 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vr_nanbox beats baseline by 22% (significant)

carrier_vr_nanbox is -428.04 us (22%) faster than baseline carrier_vr_static, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vr_nanbox is fastest but the noisiest (CV 7.1%)

carrier_vr_nanbox wins on median (1.52 ms) yet has the highest variance (CV 7.1%), while carrier_vr_static is the steadiest (CV 3.3%, 1.92 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: carrier_vr_nanbox** at 1520962.1 ns median (-20.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.26x (fastest 1520962.1 ns, slowest 1920255.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vr_nanbox | 1545540ns | 1523945ns | 1434808ns | 1494824ns | 1676980ns | -19.01% |
| carrier_vr_static | 1908224ns | 1924117ns | 1822337ns | 1890732ns | 1977404ns | base |
| carrier_vr_tagged | 1595004ns | 1593697ns | 1496142ns | 1579247ns | 1668072ns | -16.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vr_nanbox | 1542007ns | 1431284ns | 1673051ns | -19.04% | 0.011 |
| carrier_vr_static | 1904549ns | 1818885ns | 1973668ns | base | 0.009 |
| carrier_vr_tagged | 1592546ns | 1493712ns | 1665771ns | -16.38% | 0.010 |

## Performance model

- Peak throughput: **0.011 Gops/s** (carrier_vr_nanbox; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vr_nanbox | 0.011 | 94.1% |
| carrier_vr_static | 0.009 | 74.5% |
| carrier_vr_tagged | 0.010 | 89.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vr_nanbox | 1545540ns | 1545540ns | -19.01% |
| carrier_vr_static | 1908224ns | 1908224ns | base |
| carrier_vr_tagged | 1595004ns | 1595004ns | -16.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vr_static | 1920256ns | base | --- | [1819724, 1973668] | --- | --- | --- | --- |
| carrier_vr_nanbox | 1520962ns | -428038.3ns (-22.3%) | [-512917, -146673]ns | [1432006, 1673051] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vr_tagged | 1591301ns | -316691.8ns (-16.5%) | [-366216, -253103]ns | [1520564, 1665771] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vr_static | carrier_vr_nanbox | carrier_vr_tagged |
|---|---|---|---|
| 1 | 1818885ns | -9.4% | -17.9% |
| 2 | 1917448ns | -25.4% | -19.3% |
| 3 | 1986901ns | -27.0% | -18.2% |
| 4 | 1960435ns | -18.9% | -15.7% |
| 5 | 1820564ns | -6.7% | -14.4% |
| 6 | 1923064ns | -25.5% | -12.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vr_nanbox | -0.222 | moderate- |
| carrier_vr_static | -0.068 | ok |
| carrier_vr_tagged | -0.005 | ok |

**Consistency summary:**

- **carrier_vr_nanbox**: won 6/6, lost 0/6
- **carrier_vr_tagged**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vr_nanbox | 1545363.1ns | 1542006.5ns | 100.2% | HIGH |
| carrier_vr_static | 1911363.7ns | 1904549.3ns | 100.4% | HIGH |
| carrier_vr_tagged | 1596242.3ns | 1592545.7ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_vr_nanbox (n=6, range 1431283.7-1673051.2 ns)
  1431283.7 |########################################
  1443372.1 |####################
  1455460.5 |
  1467548.8 |
  1479637.2 |
  1491725.6 |
  1503814.0 |
  1515902.3 |
  1527990.7 |
  1540079.1 |
  1552167.5 |
  1564255.9 |
  1576344.2 |
  1588432.6 |####################
  1600521.0 |
  1612609.4 |
  1624697.7 |
  1636786.1 |####################
  1648874.5 |
  1660962.9 |
  (0 below, 1 above range)

carrier_vr_static (n=6, range 1818884.6-1973668.3 ns)
  1818884.6 |########################################
  1826623.8 |
  1834363.0 |
  1842102.2 |
  1849841.3 |
  1857580.5 |
  1865319.7 |
  1873058.9 |
  1880798.1 |
  1888537.3 |
  1896276.4 |
  1904015.6 |
  1911754.8 |####################
  1919494.0 |####################
  1927233.2 |
  1934972.4 |
  1942711.6 |
  1950450.7 |
  1958189.9 |####################
  1965929.1 |
  (0 below, 1 above range)

carrier_vr_tagged (n=6, range 1493712.5-1665771.3 ns)
  1493712.5 |########################################
  1502315.4 |
  1510918.4 |
  1519521.3 |
  1528124.3 |
  1536727.2 |
  1545330.1 |########################################
  1553933.1 |########################################
  1562536.0 |
  1571139.0 |
  1579741.9 |
  1588344.8 |
  1596947.8 |
  1605550.7 |
  1614153.7 |
  1622756.6 |########################################
  1631359.5 |
  1639962.5 |
  1648565.4 |########################################
  1657168.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vr_nanbox**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_vr_static**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_vr_tagged**: bridge=100.2% of algo (FFI overhead may distort results)
