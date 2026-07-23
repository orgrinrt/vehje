# Entropy x locality surface: op_correlation {0,500,900} x locality_window {4,64,unbounded}, fixed predecoded switch dispatch

9 variants, 6 samples per variant.
Baseline: **carrier_ent_c0_w64**

## Highlights

Baseline for all deltas below: **carrier_ent_c0_w64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ent_c0_w64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ent_c0_w64 has the worst median (2.13 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ent_c900_w64 at 855.32 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ent_c900_w4 beats baseline by 60% (significant)

carrier_ent_c900_w4 is -1.27 ms (60%) faster than baseline carrier_ent_c0_w64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_ent_c0_w64 is an outlier: 2.5x slower than the field

carrier_ent_c0_w64 (2.13 ms) is 2.5x the fastest (855.32 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_ent_c900_w64, carrier_ent_c900_w4) are a dead heat (<1%)

carrier_ent_c900_w64 (855.32 us) and carrier_ent_c900_w4 (859.75 us) differ by 0.52%, inside the noise, even though the wider field spreads 148.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {carrier_ent_c900_w64, carrier_ent_c900_w4, carrier_ent_c900_wmax} vs {carrier_ent_c500_wmax, carrier_ent_c500_w4, carrier_ent_c500_w64, carrier_ent_c0_wmax, carrier_ent_c0_w4, carrier_ent_c0_w64} (119% apart)

The field splits into a fast tier {carrier_ent_c900_w64, carrier_ent_c900_w4, carrier_ent_c900_wmax} and a slow tier {carrier_ent_c500_wmax, carrier_ent_c500_w4, carrier_ent_c500_w64, carrier_ent_c0_wmax, carrier_ent_c0_w4, carrier_ent_c0_w64} with a 119% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_ent_c900_w64** at 855319.3 ns median (-59.8% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 2.49x (fastest 855319.3 ns, slowest 2126723.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ent_c0_w4 | 2168557ns | 2125117ns | 2122713ns | 2124339ns | 2257806ns | -0.46% |
| carrier_ent_c0_w64 | 2178588ns | 2129279ns | 2113448ns | 2124520ns | 2292259ns | base |
| carrier_ent_c0_wmax | 2112668ns | 2111201ns | 2109846ns | 2110956ns | 2116648ns | -3.03% |
| carrier_ent_c500_w4 | 1928313ns | 1927242ns | 1920957ns | 1926405ns | 1934852ns | -11.49% |
| carrier_ent_c500_w64 | 1960937ns | 1930155ns | 1913779ns | 1928949ns | 2032497ns | -9.99% |
| carrier_ent_c500_wmax | 1965732ns | 1919079ns | 1897684ns | 1916172ns | 2074096ns | -9.77% |
| carrier_ent_c900_w4 | 871068ns | 862114ns | 850961ns | 858681ns | 899702ns | -60.02% |
| carrier_ent_c900_w64 | 866050ns | 857601ns | 852357ns | 856142ns | 887760ns | -60.25% |
| carrier_ent_c900_wmax | 894475ns | 875993ns | 866916ns | 874424ns | 938331ns | -58.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ent_c0_w4 | 2165879ns | 2120420ns | 2254483ns | -0.45% | 0.008 |
| carrier_ent_c0_w64 | 2175737ns | 2111141ns | 2288550ns | base | 0.008 |
| carrier_ent_c0_wmax | 2110305ns | 2107492ns | 2114206ns | -3.01% | 0.008 |
| carrier_ent_c500_w4 | 1925894ns | 1918423ns | 1932405ns | -11.48% | 0.009 |
| carrier_ent_c500_w64 | 1958207ns | 1911303ns | 2029183ns | -10.00% | 0.008 |
| carrier_ent_c500_wmax | 1962156ns | 1895272ns | 2068286ns | -9.82% | 0.008 |
| carrier_ent_c900_w4 | 868635ns | 848800ns | 896941ns | -60.08% | 0.019 |
| carrier_ent_c900_w64 | 863685ns | 850211ns | 885117ns | -60.30% | 0.019 |
| carrier_ent_c900_wmax | 891996ns | 864633ns | 935413ns | -59.00% | 0.018 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_ent_c0_w4 | 13777584 | 11455801 | 1.203 | 1.00× |
| carrier_ent_c0_w64 | 13753772 | 11468573 | 1.199 | 1.00× |
| carrier_ent_c0_wmax | 13655084 | 11441018 | 1.194 | 0.99× |
| carrier_ent_c500_w4 | 12429722 | 11536063 | 1.077 | 0.90× |
| carrier_ent_c500_w64 | 12381274 | 11555536 | 1.071 | 0.90× |
| carrier_ent_c500_wmax | 12388413 | 11577428 | 1.070 | 0.90× |
| carrier_ent_c900_w4 | 5521956 | 11638642 | 0.474 | 0.40× |
| carrier_ent_c900_w64 | 5553858 | 11621409 | 0.478 | 0.40× |
| carrier_ent_c900_wmax | 5667075 | 11633619 | 0.487 | 0.41× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.019 Gops/s** (carrier_ent_c900_w4; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ent_c0_w4 | 0.008 | 40.0% |
| carrier_ent_c0_w64 | 0.008 | 39.9% |
| carrier_ent_c0_wmax | 0.008 | 40.2% |
| carrier_ent_c500_w4 | 0.009 | 44.1% |
| carrier_ent_c500_w64 | 0.008 | 44.0% |
| carrier_ent_c500_wmax | 0.009 | 44.3% |
| carrier_ent_c900_w4 | 0.019 | 98.7% |
| carrier_ent_c900_w64 | 0.019 | 99.2% |
| carrier_ent_c900_wmax | 0.019 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ent_c0_w4 | 2168557ns | 2168557ns | -0.46% |
| carrier_ent_c0_w64 | 2178588ns | 2178588ns | base |
| carrier_ent_c0_wmax | 2112668ns | 2112668ns | -3.03% |
| carrier_ent_c500_w4 | 1928313ns | 1928313ns | -11.49% |
| carrier_ent_c500_w64 | 1960937ns | 1960937ns | -9.99% |
| carrier_ent_c500_wmax | 1965732ns | 1965732ns | -9.77% |
| carrier_ent_c900_w4 | 871068ns | 871068ns | -60.02% |
| carrier_ent_c900_w64 | 866050ns | 866050ns | -60.25% |
| carrier_ent_c900_wmax | 894475ns | 894475ns | -58.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ent_c0_w64 | 2126724ns | base | --- | [2111936, 2288550] | --- | --- | --- | --- |
| carrier_ent_c0_w4 | 2122731ns | no significant difference | [-122674, +89248]ns | [2120423, 2254483] | no | 0.6875 | 0.6875 | 0 |
| carrier_ent_c0_wmax | 2108903ns | -16042.7ns (-0.8%) | [-176229, -4022]ns | [2107807, 2114206] | YES | 0.0357 | 0.0313 | 0 |
| carrier_ent_c500_w4 | 1924897ns | -197815.4ns (-9.3%) | [-362289, -189424]ns | [1920379, 1932405] | YES | 0.0357 | 0.0313 | 0 |
| carrier_ent_c500_w64 | 1927730ns | -200436.5ns (-9.4%) | [-268975, -183179]ns | [1917707, 2029183] | YES | 0.0357 | 0.0313 | 0 |
| carrier_ent_c500_wmax | 1916602ns | -215753.7ns (-10.1%) | [-240017, -184972]ns | [1901579, 2068286] | YES | 0.0357 | 0.0313 | 0 |
| carrier_ent_c900_w4 | 859749ns | -1271857.1ns (-59.8%) | [-1391610, -1257837]ns | [849217, 896941] | YES | 0.0357 | 0.0313 | 0 |
| carrier_ent_c900_w64 | 855319ns | -1268031.9ns (-59.6%) | [-1414776, -1253347]ns | [850619, 885117] | YES | 0.0357 | 0.0313 | 0 |
| carrier_ent_c900_wmax | 873760ns | -1248575.4ns (-58.7%) | [-1370660, -1231988]ns | [866814, 935413] | YES | 0.0357 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ent_c0_w64 | carrier_ent_c0_w4 | carrier_ent_c0_wmax | carrier_ent_c500_w4 | carrier_ent_c500_w64 | carrier_ent_c500_wmax | carrier_ent_c900_w4 | carrier_ent_c900_w64 | carrier_ent_c900_wmax |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2112731ns | +0.4% | -0.2% | -9.0% | -8.8% | -10.3% | -59.3% | -59.8% | -58.9% |
| 2 | 2111141ns | +0.4% | -0.2% | -8.9% | -8.6% | -8.9% | -59.8% | -59.5% | -59.0% |
| 3 | 2425106ns | -9.8% | -13.1% | -20.2% | -12.3% | -9.7% | -61.7% | -63.0% | -61.3% |
| 4 | 2128915ns | -0.4% | -0.8% | -9.5% | -9.6% | -8.5% | -59.6% | -59.8% | -59.0% |
| 5 | 2124532ns | +0.0% | -0.7% | -9.1% | -9.2% | -10.1% | -60.0% | -58.9% | -58.9% |
| 6 | 2151995ns | +7.9% | -1.6% | -10.9% | -11.2% | -11.3% | -59.8% | -60.5% | -56.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ent_c0_w4 | -0.138 | ok |
| carrier_ent_c0_w64 | -0.266 | moderate- |
| carrier_ent_c0_wmax | 0.029 | ok |
| carrier_ent_c500_w4 | -0.223 | moderate- |
| carrier_ent_c500_w64 | -0.207 | moderate- |
| carrier_ent_c500_wmax | -0.096 | ok |
| carrier_ent_c900_w4 | -0.283 | moderate- |
| carrier_ent_c900_w64 | -0.389 | moderate- |
| carrier_ent_c900_wmax | -0.333 | moderate- |

**Consistency summary:**

- **carrier_ent_c0_w4**: won 2/6, lost 3/6
- **carrier_ent_c0_wmax**: won 6/6, lost 0/6
- **carrier_ent_c500_w4**: won 6/6, lost 0/6
- **carrier_ent_c500_w64**: won 6/6, lost 0/6
- **carrier_ent_c500_wmax**: won 6/6, lost 0/6
- **carrier_ent_c900_w4**: won 6/6, lost 0/6
- **carrier_ent_c900_w64**: won 6/6, lost 0/6
- **carrier_ent_c900_wmax**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ent_c0_w4 | 2312857.2ns | 2165879.0ns | 106.8% | HIGH |
| carrier_ent_c0_w64 | 2320584.9ns | 2175736.6ns | 106.7% | HIGH |
| carrier_ent_c0_wmax | 2260869.7ns | 2110305.3ns | 107.1% | HIGH |
| carrier_ent_c500_w4 | 2061267.8ns | 1925893.8ns | 107.0% | HIGH |
| carrier_ent_c500_w64 | 2081620.3ns | 1958206.6ns | 106.3% | HIGH |
| carrier_ent_c500_wmax | 2112414.3ns | 1962155.9ns | 107.7% | HIGH |
| carrier_ent_c900_w4 | 920308.2ns | 868635.3ns | 105.9% | HIGH |
| carrier_ent_c900_w64 | 923840.9ns | 863684.9ns | 107.0% | HIGH |
| carrier_ent_c900_wmax | 964083.4ns | 891995.8ns | 108.1% | HIGH |

## Distribution (algo ns)

```
carrier_ent_c0_w4 (n=6, range 2120419.6-2254483.1 ns)
  2120419.6 |########################################
  2127122.8 |
  2133826.0 |
  2140529.1 |
  2147232.3 |
  2153935.5 |
  2160638.6 |
  2167341.8 |
  2174045.0 |
  2180748.2 |
  2187451.4 |##########
  2194154.5 |
  2200857.7 |
  2207560.9 |
  2214264.1 |
  2220967.2 |
  2227670.4 |
  2234373.6 |
  2241076.8 |
  2247779.9 |
  (0 below, 1 above range)

carrier_ent_c0_w64 (n=6, range 2111140.8-2288550.2 ns)
  2111140.8 |########################################
  2120011.3 |####################
  2128881.7 |####################
  2137752.2 |
  2146622.7 |####################
  2155493.1 |
  2164363.6 |
  2173234.1 |
  2182104.6 |
  2190975.0 |
  2199845.5 |
  2208716.0 |
  2217586.4 |
  2226456.9 |
  2235327.4 |
  2244197.9 |
  2253068.3 |
  2261938.8 |
  2270809.3 |
  2279679.7 |
  (0 below, 1 above range)

carrier_ent_c0_wmax (n=6, range 2107492.5-2114206.5 ns)
  2107492.5 |########################################
  2107828.2 |########################################
  2108163.9 |########################################
  2108499.6 |
  2108835.3 |
  2109171.0 |########################################
  2109506.7 |
  2109842.4 |
  2110178.1 |
  2110513.8 |
  2110849.5 |
  2111185.2 |
  2111520.9 |
  2111856.6 |########################################
  2112192.3 |
  2112528.0 |
  2112863.7 |
  2113199.4 |
  2113535.1 |
  2113870.8 |
  (0 below, 1 above range)

carrier_ent_c500_w4 (n=6, range 1918422.9-1932405.2 ns)
  1918422.9 |########################################
  1919122.0 |
  1919821.1 |
  1920520.3 |
  1921219.4 |
  1921918.5 |########################################
  1922617.6 |########################################
  1923316.7 |
  1924015.8 |
  1924715.0 |
  1925414.1 |
  1926113.2 |
  1926812.3 |########################################
  1927511.4 |
  1928210.5 |
  1928909.7 |
  1929608.8 |
  1930307.9 |########################################
  1931007.0 |
  1931706.1 |
  (0 below, 1 above range)

carrier_ent_c500_w64 (n=6, range 1911303.3-2029182.9 ns)
  1911303.3 |#############
  1917197.3 |
  1923091.3 |########################################
  1928985.2 |#############
  1934879.2 |
  1940773.2 |
  1946667.2 |
  1952561.2 |
  1958455.1 |
  1964349.1 |
  1970243.1 |
  1976137.1 |
  1982031.1 |
  1987925.0 |
  1993819.0 |
  1999713.0 |
  2005607.0 |
  2011501.0 |
  2017394.9 |
  2023288.9 |
  (0 below, 1 above range)

carrier_ent_c500_wmax (n=6, range 1895271.7-2068286.0 ns)
  1895271.7 |####################
  1903922.4 |########################################
  1912573.1 |
  1921223.9 |####################
  1929874.6 |
  1938525.3 |
  1947176.0 |####################
  1955826.7 |
  1964477.4 |
  1973128.2 |
  1981778.9 |
  1990429.6 |
  1999080.3 |
  2007731.0 |
  2016381.7 |
  2025032.5 |
  2033683.2 |
  2042333.9 |
  2050984.6 |
  2059635.3 |
  (0 below, 1 above range)

carrier_ent_c900_w4 (n=6, range 848799.6-896940.6 ns)
  848799.6 |########################################
  851206.7 |
  853613.7 |
  856020.8 |
  858427.8 |########################################
  860834.8 |
  863241.9 |####################
  865649.0 |
  868056.0 |
  870463.1 |
  872870.1 |
  875277.2 |
  877684.2 |
  880091.2 |
  882498.3 |
  884905.4 |
  887312.4 |
  889719.5 |
  892126.5 |
  894533.6 |
  (0 below, 1 above range)

carrier_ent_c900_w64 (n=6, range 850210.8-885116.6 ns)
  850210.8 |########################################
  851956.1 |
  853701.4 |########################################
  855446.7 |
  857192.0 |
  858937.3 |
  860682.6 |
  862427.8 |
  864173.1 |
  865918.4 |
  867663.7 |
  869409.0 |
  871154.3 |
  872899.6 |####################
  874644.9 |
  876390.2 |
  878135.5 |
  879880.8 |
  881626.1 |
  883371.4 |
  (0 below, 1 above range)

carrier_ent_c900_wmax (n=6, range 864632.9-935413.3 ns)
  864632.9 |####################
  868171.9 |####################
  871710.9 |########################################
  875250.0 |
  878789.0 |
  882328.0 |
  885867.0 |
  889406.0 |
  892945.1 |
  896484.1 |
  900023.1 |
  903562.1 |
  907101.1 |
  910640.2 |
  914179.2 |
  917718.2 |
  921257.2 |
  924796.2 |
  928335.3 |####################
  931874.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ent_c0_w4**: bridge=106.7% of algo (FFI overhead may distort results)
- **carrier_ent_c0_w64**: bridge=107.1% of algo (FFI overhead may distort results)
- **carrier_ent_c0_wmax**: bridge=107.1% of algo (FFI overhead may distort results)
- **carrier_ent_c500_w4**: bridge=107.2% of algo (FFI overhead may distort results)
- **carrier_ent_c500_w64**: bridge=106.3% of algo (FFI overhead may distort results)
- **carrier_ent_c500_wmax**: bridge=107.3% of algo (FFI overhead may distort results)
- **carrier_ent_c900_w4**: bridge=106.1% of algo (FFI overhead may distort results)
- **carrier_ent_c900_w64**: bridge=106.9% of algo (FFI overhead may distort results)
- **carrier_ent_c900_wmax**: bridge=107.8% of algo (FFI overhead may distort results)
