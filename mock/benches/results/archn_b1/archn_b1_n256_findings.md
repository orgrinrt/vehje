# Per-branch strategy (NATIVE tier): archetype 1

5 variants, 6 samples per variant.
Baseline: **an_b1_table**

## Highlights

Baseline for all deltas below: **an_b1_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: an_b1_prof** at 2018.8 ns median (-14.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.32x (fastest 2018.8 ns, slowest 2673.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b1_pred | 5105ns | 5353ns | 4279ns | 5000ns | 5675ns | +1.72% |
| an_b1_prof | 4403ns | 4383ns | 3914ns | 4249ns | 4879ns | -12.26% |
| an_b1_seq | 4703ns | 4754ns | 4113ns | 4553ns | 5222ns | -6.29% |
| an_b1_table | 5018ns | 5156ns | 4121ns | 5031ns | 5449ns | base |
| an_b1_tree | 4796ns | 4949ns | 4013ns | 4774ns | 5222ns | -4.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b1_pred | 2562ns | 2132ns | 2875ns | +11.64% | 0.100 |
| an_b1_prof | 2032ns | 1809ns | 2264ns | -11.46% | 0.126 |
| an_b1_seq | 2165ns | 1895ns | 2385ns | -5.65% | 0.118 |
| an_b1_table | 2295ns | 1889ns | 2480ns | base | 0.112 |
| an_b1_tree | 2216ns | 1845ns | 2436ns | -3.46% | 0.116 |

## Performance model

- Peak throughput: **0.141 Gops/s** (an_b1_prof; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b1_pred | 0.096 | 67.7% |
| an_b1_prof | 0.127 | 89.6% |
| an_b1_seq | 0.117 | 82.6% |
| an_b1_table | 0.108 | 76.4% |
| an_b1_tree | 0.113 | 79.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b1_pred | 5105ns | 5105ns | +1.72% |
| an_b1_prof | 4403ns | 4403ns | -12.26% |
| an_b1_seq | 4703ns | 4703ns | -6.29% |
| an_b1_table | 5018ns | 5018ns | base |
| an_b1_tree | 4796ns | 4796ns | -4.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b1_table | 2368ns | base | --- | [2037, 2480] | --- | --- | --- | --- |
| an_b1_pred | 2673ns | no significant difference | [-23, +544]ns | [2138, 2875] | no | 0.4375 | 0.2188 | 0 |
| an_b1_prof | 2019ns | -216.9ns (-9.2%) | [-456, -116]ns | [1814, 2264] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b1_seq | 2191ns | no significant difference | [-374, +35]ns | [1920, 2385] | no | 1.0000 | 1.0000 | 0 |
| an_b1_tree | 2274ns | no significant difference | [-310, +151]ns | [1937, 2436] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b1_table | an_b1_pred | an_b1_prof | an_b1_seq | an_b1_tree |
|---|---|---|---|---|---|
| 1 | 2185ns | +35.3% | -17.2% | +0.1% | +8.3% |
| 2 | 2484ns | +12.3% | -9.5% | -11.6% | -4.6% |
| 3 | 2380ns | +7.5% | -7.3% | +0.6% | +5.1% |
| 4 | 2355ns | -9.5% | -22.8% | -19.5% | -13.8% |
| 5 | 1889ns | +13.5% | -3.1% | +3.0% | -2.3% |
| 6 | 2477ns | +12.8% | -8.0% | -4.1% | -11.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b1_pred | 0.280 | moderate+ |
| an_b1_prof | -0.199 | ok |
| an_b1_seq | -0.188 | ok |
| an_b1_table | -0.381 | moderate- |
| an_b1_tree | 0.317 | moderate+ |

**Consistency summary:**

- **an_b1_pred**: won 1/6, lost 5/6
- **an_b1_prof**: won 6/6, lost 0/6
- **an_b1_seq**: won 3/6, lost 2/6
- **an_b1_tree**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b1_pred | 6.2ns | 2562.1ns | 0.2% |  |
| an_b1_prof | 5.7ns | 2032.2ns | 0.3% |  |
| an_b1_seq | 5.6ns | 2165.4ns | 0.3% |  |
| an_b1_table | 6.6ns | 2295.1ns | 0.3% |  |
| an_b1_tree | 6.6ns | 2215.5ns | 0.3% |  |

## Distribution (algo ns)

```
an_b1_pred (n=6, range 2131.7-2875.2 ns)
   2131.7 |########################################
   2168.9 |
   2206.0 |
   2243.2 |
   2280.4 |
   2317.6 |
   2354.8 |
   2391.9 |
   2429.1 |
   2466.3 |
   2503.4 |
   2540.6 |####################
   2577.8 |
   2615.0 |
   2652.1 |
   2689.3 |
   2726.5 |
   2763.7 |########################################
   2800.8 |
   2838.0 |
  (0 below, 1 above range)

an_b1_prof (n=6, range 1809.2-2263.6 ns)
   1809.2 |########################################
   1831.9 |
   1854.6 |
   1877.4 |
   1900.1 |
   1922.8 |
   1945.5 |
   1968.2 |
   1990.9 |
   2013.7 |
   2036.4 |
   2059.1 |
   2081.8 |
   2104.5 |
   2127.2 |
   2150.0 |
   2172.7 |
   2195.4 |#############
   2218.1 |
   2240.8 |#############
  (0 below, 1 above range)

an_b1_seq (n=6, range 1895.4-2385.4 ns)
   1895.4 |########################################
   1919.9 |
   1944.4 |########################################
   1968.9 |
   1993.4 |
   2017.9 |
   2042.4 |
   2066.9 |
   2091.4 |
   2115.9 |
   2140.4 |
   2164.9 |########################################
   2189.4 |########################################
   2213.9 |
   2238.4 |
   2262.9 |
   2287.4 |
   2311.9 |
   2336.4 |
   2360.9 |########################################
  (0 below, 1 above range)

an_b1_table (n=6, range 1888.7-2480.4 ns)
   1888.7 |########################################
   1918.3 |
   1947.9 |
   1977.5 |
   2007.0 |
   2036.6 |
   2066.2 |
   2095.8 |
   2125.4 |
   2155.0 |
   2184.6 |########################################
   2214.2 |
   2243.8 |
   2273.3 |
   2302.9 |
   2332.5 |########################################
   2362.1 |########################################
   2391.7 |
   2421.3 |
   2450.9 |########################################
  (0 below, 1 above range)

an_b1_tree (n=6, range 1844.6-2435.8 ns)
   1844.6 |####################
   1874.2 |
   1903.7 |
   1933.3 |
   1962.8 |
   1992.4 |
   2022.0 |####################
   2051.5 |
   2081.1 |
   2110.6 |
   2140.2 |
   2169.8 |####################
   2199.3 |
   2228.9 |
   2258.4 |
   2288.0 |
   2317.6 |
   2347.1 |########################################
   2376.7 |
   2406.2 |
  (0 below, 1 above range)

```
