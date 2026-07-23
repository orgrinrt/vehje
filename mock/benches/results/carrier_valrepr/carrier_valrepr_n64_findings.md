# Value representation (static / runtime-tagged / NaN-boxed)

3 variants, 6 samples per variant.
Baseline: **carrier_vr_static**

## Highlights

Baseline for all deltas below: **carrier_vr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: carrier_vr_nanbox** at 2086.5 ns median (-3.9% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.09x (fastest 2086.5 ns, slowest 2281.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vr_nanbox | 4416ns | 4410ns | 4215ns | 4368ns | 4588ns | -3.58% |
| carrier_vr_static | 4580ns | 4535ns | 4293ns | 4526ns | 4804ns | base |
| carrier_vr_tagged | 4565ns | 4611ns | 4249ns | 4548ns | 4749ns | -0.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vr_nanbox | 2075ns | 1996ns | 2122ns | -3.34% | 0.031 |
| carrier_vr_static | 2146ns | 2054ns | 2199ns | base | 0.030 |
| carrier_vr_tagged | 2243ns | 2110ns | 2330ns | +4.53% | 0.029 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vr_nanbox | 266969 | 1603120 | 0.167 | 1.00× |
| carrier_vr_static | 266360 | 1142690 | 0.233 | 1.00× |
| carrier_vr_tagged | 270822 | 1577392 | 0.172 | 1.02× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_vr_nanbox; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vr_nanbox | 0.031 | 95.7% |
| carrier_vr_static | 0.029 | 92.0% |
| carrier_vr_tagged | 0.028 | 87.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vr_nanbox | 4416ns | 4416ns | -3.58% |
| carrier_vr_static | 4580ns | 4580ns | base |
| carrier_vr_tagged | 4565ns | 4565ns | -0.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vr_static | 2170ns | base | --- | [2070, 2199] | --- | --- | --- | --- |
| carrier_vr_nanbox | 2086ns | no significant difference | [-137, +0]ns | [2016, 2122] | no | 0.2188 | 0.2188 | 0 |
| carrier_vr_tagged | 2281ns | +99.0ns (+4.6%) | [+33, +160]ns | [2119, 2330] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vr_static | carrier_vr_nanbox | carrier_vr_tagged |
|---|---|---|---|
| 1 | 2085ns | +2.8% | +1.2% |
| 2 | 2170ns | -3.3% | +6.9% |
| 3 | 2170ns | -4.0% | +7.8% |
| 4 | 2213ns | -5.6% | +1.9% |
| 5 | 2185ns | -6.8% | +5.6% |
| 6 | 2054ns | -2.8% | +3.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vr_nanbox | 0.340 | moderate+ |
| carrier_vr_static | -0.016 | ok |
| carrier_vr_tagged | -0.172 | ok |

**Consistency summary:**

- **carrier_vr_nanbox**: won 5/6, lost 1/6
- **carrier_vr_tagged**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vr_nanbox | 85716.0ns | 2074.5ns | 4131.8% | HIGH |
| carrier_vr_static | 85729.1ns | 2146.2ns | 3994.4% | HIGH |
| carrier_vr_tagged | 86188.1ns | 2243.4ns | 3841.8% | HIGH |

## Distribution (algo ns)

```
carrier_vr_nanbox (n=6, range 1995.8-2121.5 ns)
   1995.8 |####################
   2002.1 |
   2008.4 |
   2014.7 |
   2020.9 |
   2027.2 |
   2033.5 |####################
   2039.8 |
   2046.1 |
   2052.4 |
   2058.7 |
   2064.9 |
   2071.2 |
   2077.5 |
   2083.8 |########################################
   2090.1 |
   2096.4 |####################
   2102.6 |
   2108.9 |
   2115.2 |
  (0 below, 1 above range)

carrier_vr_static (n=6, range 2053.8-2198.9 ns)
   2053.8 |####################
   2061.1 |
   2068.3 |
   2075.6 |
   2082.8 |####################
   2090.1 |
   2097.3 |
   2104.6 |
   2111.9 |
   2119.1 |
   2126.4 |
   2133.6 |
   2140.9 |
   2148.1 |
   2155.4 |
   2162.7 |
   2169.9 |########################################
   2177.2 |
   2184.4 |####################
   2191.7 |
  (0 below, 1 above range)

carrier_vr_tagged (n=6, range 2109.6-2330.0 ns)
   2109.6 |########################################
   2120.6 |########################################
   2131.6 |
   2142.7 |
   2153.7 |
   2164.7 |
   2175.7 |
   2186.7 |
   2197.8 |
   2208.8 |
   2219.8 |
   2230.8 |
   2241.8 |
   2252.9 |########################################
   2263.9 |
   2274.9 |
   2285.9 |
   2296.9 |########################################
   2308.0 |
   2319.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vr_nanbox**: bridge=4103.7% of algo (FFI overhead may distort results)
- **carrier_vr_static**: bridge=3951.5% of algo (FFI overhead may distort results)
- **carrier_vr_tagged**: bridge=3782.8% of algo (FFI overhead may distort results)
