# Partial-eval specialization: fold ratio on a random static/dynamic mix (reduction metric)

4 variants, 6 samples per variant.
Baseline: **pe_rand_sf30**

## Highlights

Baseline for all deltas below: **pe_rand_sf30**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### pe_rand_sf90 dominates: 92% faster than the next best (pe_rand_sf70)

pe_rand_sf90 (831.19 us) leads pe_rand_sf70 (1.59 ms) by 92%, a clear separation rather than a photo finish. CV 5.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### pe_rand_sf90 beats baseline by 56% (significant)

pe_rand_sf90 is -1.05 ms (56%) faster than baseline pe_rand_sf30, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### pe_rand_sf50 is an outlier: 2.3x slower than the field

pe_rand_sf50 (1.94 ms) is 2.3x the fastest (831.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### pe_rand_sf90 is fastest but the noisiest (CV 5.3%)

pe_rand_sf90 wins on median (831.19 us) yet has the highest variance (CV 5.3%), while pe_rand_sf50 is the steadiest (CV 1.1%, 1.94 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {pe_rand_sf90} vs {pe_rand_sf70, pe_rand_sf30, pe_rand_sf50} (92% apart)

The field splits into a fast tier {pe_rand_sf90} and a slow tier {pe_rand_sf70, pe_rand_sf30, pe_rand_sf50} with a 92% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: pe_rand_sf90** at 831193.3 ns median (-55.2% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.33x (fastest 831193.3 ns, slowest 1938074.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| pe_rand_sf30 | 1857012ns | 1859410ns | 1806655ns | 1847675ns | 1896194ns | base |
| pe_rand_sf50 | 1936445ns | 1940805ns | 1898714ns | 1934198ns | 1958681ns | +4.28% |
| pe_rand_sf70 | 1582850ns | 1594841ns | 1514096ns | 1574724ns | 1629417ns | -14.76% |
| pe_rand_sf90 | 829677ns | 834023ns | 761778ns | 821434ns | 875991ns | -55.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| pe_rand_sf30 | 1853992ns | 1804026ns | 1893596ns | base | 0.009 |
| pe_rand_sf50 | 1933745ns | 1896291ns | 1955845ns | +4.30% | 0.008 |
| pe_rand_sf70 | 1580266ns | 1511374ns | 1626854ns | -14.76% | 0.010 |
| pe_rand_sf90 | 827061ns | 759388ns | 873324ns | -55.39% | 0.020 |

## Performance model

- Peak throughput: **0.022 Gops/s** (pe_rand_sf90; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| pe_rand_sf30 | 0.009 | 40.9% |
| pe_rand_sf50 | 0.008 | 39.2% |
| pe_rand_sf70 | 0.010 | 47.7% |
| pe_rand_sf90 | 0.020 | 91.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| pe_rand_sf30 | 1857012ns | 1857012ns | base |
| pe_rand_sf50 | 1936445ns | 1936445ns | +4.28% |
| pe_rand_sf70 | 1582850ns | 1582850ns | -14.76% |
| pe_rand_sf90 | 829677ns | 829677ns | -55.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| pe_rand_sf30 | 1855620ns | base | --- | [1812759, 1893596] | --- | --- | --- | --- |
| pe_rand_sf50 | 1938074ns | +73526.4ns (+4.0%) | [+22646, +143085]ns | [1907315, 1955845] | YES | 0.0313 | 0.0313 | 0 |
| pe_rand_sf70 | 1592306ns | -278235.2ns (-15.0%) | [-312426, -230517]ns | [1521638, 1626854] | YES | 0.0313 | 0.0313 | 0 |
| pe_rand_sf90 | 831193ns | -1046114.6ns (-56.4%) | [-1095242, -939435]ns | [776667, 873324] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | pe_rand_sf30 | pe_rand_sf50 | pe_rand_sf70 | pe_rand_sf90 |
|---|---|---|---|---|
| 1 | 1846635ns | +5.1% | -18.2% | -58.9% |
| 2 | 1804026ns | +9.0% | -11.2% | -51.3% |
| 3 | 1907224ns | +1.5% | -13.6% | -57.8% |
| 4 | 1879969ns | +0.9% | -14.6% | -54.3% |
| 5 | 1821492ns | +6.8% | -15.9% | -52.3% |
| 6 | 1864605ns | +2.9% | -15.1% | -57.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| pe_rand_sf30 | -0.290 | moderate- |
| pe_rand_sf50 | -0.163 | ok |
| pe_rand_sf70 | 0.029 | ok |
| pe_rand_sf90 | -0.471 | moderate- |

**Consistency summary:**

- **pe_rand_sf50**: won 0/6, lost 6/6
- **pe_rand_sf70**: won 6/6, lost 0/6
- **pe_rand_sf90**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| pe_rand_sf30 | 33.4ns | 1853991.9ns | 0.0% |  |
| pe_rand_sf50 | 34.7ns | 1933744.5ns | 0.0% |  |
| pe_rand_sf70 | 25.1ns | 1580265.7ns | 0.0% |  |
| pe_rand_sf90 | 9.8ns | 827061.4ns | 0.0% |  |

## Distribution (algo ns)

```
pe_rand_sf30 (n=6, range 1804025.8-1893596.5 ns)
  1804025.8 |########################################
  1808504.3 |
  1812982.9 |
  1817461.4 |########################################
  1821939.9 |
  1826418.5 |
  1830897.0 |
  1835375.5 |
  1839854.1 |
  1844332.6 |########################################
  1848811.1 |
  1853289.7 |
  1857768.2 |
  1862246.8 |########################################
  1866725.3 |
  1871203.8 |
  1875682.4 |########################################
  1880160.9 |
  1884639.4 |
  1889118.0 |
  (0 below, 1 above range)

pe_rand_sf50 (n=6, range 1896291.2-1955844.6 ns)
  1896291.2 |########################################
  1899268.9 |
  1902246.5 |
  1905224.2 |
  1908201.9 |
  1911179.6 |
  1914157.2 |
  1917134.9 |########################################
  1920112.6 |
  1923090.2 |
  1926067.9 |
  1929045.6 |
  1932023.2 |
  1935000.9 |########################################
  1937978.6 |########################################
  1940956.2 |
  1943933.9 |########################################
  1946911.6 |
  1949889.3 |
  1952866.9 |
  (0 below, 1 above range)

pe_rand_sf70 (n=6, range 1511373.8-1626853.8 ns)
  1511373.8 |########################################
  1517147.8 |
  1522921.8 |
  1528695.8 |########################################
  1534469.8 |
  1540243.8 |
  1546017.8 |
  1551791.8 |
  1557565.8 |
  1563339.8 |
  1569113.8 |
  1574887.8 |
  1580661.8 |########################################
  1586435.8 |
  1592209.8 |
  1597983.8 |########################################
  1603757.8 |########################################
  1609531.8 |
  1615305.8 |
  1621079.8 |
  (0 below, 1 above range)

pe_rand_sf90 (n=6, range 759388.3-873323.9 ns)
  759388.3 |########################################
  765085.1 |
  770781.9 |
  776478.6 |
  782175.4 |
  787872.2 |
  793569.0 |########################################
  799265.8 |########################################
  804962.6 |
  810659.3 |
  816356.1 |
  822052.9 |
  827749.7 |
  833446.5 |
  839143.3 |
  844840.0 |
  850536.8 |
  856233.6 |########################################
  861930.4 |
  867627.2 |########################################
  (0 below, 1 above range)

```
