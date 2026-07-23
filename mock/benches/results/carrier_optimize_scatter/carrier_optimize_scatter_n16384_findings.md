# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, scatter profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_scatter_none**

## Highlights

Baseline for all deltas below: **carrier_opt_scatter_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_scatter_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_scatter_none has the worst median (2.52 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_scatter_all at 2.33 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (carrier_opt_scatter_all, carrier_opt_scatter_cse) are a dead heat (<1%)

carrier_opt_scatter_all (2.33 ms) and carrier_opt_scatter_cse (2.33 ms) differ by 0.09%, inside the noise, even though the wider field spreads 8.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: carrier_opt_scatter_all** at 2325688.0 ns median (-7.7% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.08x (fastest 2325688.0 ns, slowest 2519021.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_scatter_all | 2330279ns | 2329331ns | 2318553ns | 2327381ns | 2340488ns | -7.69% |
| carrier_opt_scatter_canon | 2345780ns | 2346901ns | 2321113ns | 2342598ns | 2362886ns | -7.08% |
| carrier_opt_scatter_cse | 2332988ns | 2331136ns | 2327730ns | 2330503ns | 2339343ns | -7.58% |
| carrier_opt_scatter_dce | 2520149ns | 2517020ns | 2510030ns | 2515125ns | 2532744ns | -0.17% |
| carrier_opt_scatter_fold | 2520619ns | 2518949ns | 2506052ns | 2517684ns | 2532306ns | -0.15% |
| carrier_opt_scatter_none | 2524442ns | 2522500ns | 2517122ns | 2520977ns | 2533299ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_scatter_all | 2326873ns | 2314933ns | 2337308ns | -7.70% | 0.007 |
| carrier_opt_scatter_canon | 2342548ns | 2317551ns | 2359666ns | -7.08% | 0.007 |
| carrier_opt_scatter_cse | 2329568ns | 2324630ns | 2335837ns | -7.60% | 0.007 |
| carrier_opt_scatter_dce | 2516536ns | 2506158ns | 2529273ns | -0.18% | 0.007 |
| carrier_opt_scatter_fold | 2517066ns | 2502334ns | 2529087ns | -0.16% | 0.007 |
| carrier_opt_scatter_none | 2521048ns | 2514170ns | 2529747ns | base | 0.006 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_scatter_all | 14485388 | 16530545 | 0.876 | 0.92× |
| carrier_opt_scatter_canon | 14563966 | 16587927 | 0.878 | 0.93× |
| carrier_opt_scatter_cse | 14486417 | 16588187 | 0.873 | 0.92× |
| carrier_opt_scatter_dce | 15697188 | 17710422 | 0.886 | 1.00× |
| carrier_opt_scatter_fold | 15719474 | 17723766 | 0.887 | 1.00× |
| carrier_opt_scatter_none | 15725509 | 17710729 | 0.888 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_opt_scatter_all; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_scatter_all | 0.007 | 99.5% |
| carrier_opt_scatter_canon | 0.007 | 98.8% |
| carrier_opt_scatter_cse | 0.007 | 99.4% |
| carrier_opt_scatter_dce | 0.007 | 92.1% |
| carrier_opt_scatter_fold | 0.007 | 92.0% |
| carrier_opt_scatter_none | 0.007 | 91.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_scatter_all | 2330279ns | 2330279ns | -7.69% |
| carrier_opt_scatter_canon | 2345780ns | 2345780ns | -7.08% |
| carrier_opt_scatter_cse | 2332988ns | 2332988ns | -7.58% |
| carrier_opt_scatter_dce | 2520149ns | 2520149ns | -0.17% |
| carrier_opt_scatter_fold | 2520619ns | 2520619ns | -0.15% |
| carrier_opt_scatter_none | 2524442ns | 2524442ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_scatter_none | 2519021ns | base | --- | [2514376, 2529747] | --- | --- | --- | --- |
| carrier_opt_scatter_all | 2325688ns | -194247.2ns (-7.7%) | [-201166, -187110]ns | [2317624, 2337308] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_scatter_canon | 2343869ns | -174921.2ns (-6.9%) | [-205639, -154940]ns | [2324108, 2359666] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_scatter_cse | 2327753ns | -192690.4ns (-7.6%) | [-202981, -178769]ns | [2325113, 2335837] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_scatter_dce | 2513366ns | no significant difference | [-15916, +9706]ns | [2506969, 2529273] | no | 0.2734 | 0.2188 | 0 |
| carrier_opt_scatter_fold | 2515186ns | no significant difference | [-16753, +10487]ns | [2506926, 2529087] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_scatter_none | carrier_opt_scatter_all | carrier_opt_scatter_canon | carrier_opt_scatter_cse | carrier_opt_scatter_dce | carrier_opt_scatter_fold |
|---|---|---|---|---|---|---|
| 1 | 2514581ns | -7.6% | -6.4% | -7.3% | +1.0% | -0.5% |
| 2 | 2514631ns | -7.4% | -5.9% | -6.9% | -0.3% | +0.8% |
| 3 | 2523411ns | -8.0% | -6.9% | -7.8% | -0.2% | +0.0% |
| 4 | 2532778ns | -7.4% | -8.5% | -8.2% | -0.5% | -0.8% |
| 5 | 2526717ns | -7.8% | -7.8% | -7.8% | -0.7% | -0.5% |
| 6 | 2514170ns | -7.9% | -7.0% | -7.5% | -0.3% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_scatter_all | -0.224 | moderate- |
| carrier_opt_scatter_canon | 0.388 | moderate+ |
| carrier_opt_scatter_cse | 0.017 | ok |
| carrier_opt_scatter_dce | -0.182 | ok |
| carrier_opt_scatter_fold | -0.272 | moderate- |
| carrier_opt_scatter_none | 0.267 | moderate+ |

**Consistency summary:**

- **carrier_opt_scatter_all**: won 6/6, lost 0/6
- **carrier_opt_scatter_canon**: won 6/6, lost 0/6
- **carrier_opt_scatter_cse**: won 6/6, lost 0/6
- **carrier_opt_scatter_dce**: won 5/6, lost 1/6
- **carrier_opt_scatter_fold**: won 3/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_scatter_all | 2331259.8ns | 2326873.3ns | 100.2% | HIGH |
| carrier_opt_scatter_canon | 2345853.2ns | 2342547.8ns | 100.1% | HIGH |
| carrier_opt_scatter_cse | 2333154.0ns | 2329567.9ns | 100.2% | HIGH |
| carrier_opt_scatter_dce | 2521011.2ns | 2516536.1ns | 100.2% | HIGH |
| carrier_opt_scatter_fold | 2521288.0ns | 2517066.4ns | 100.2% | HIGH |
| carrier_opt_scatter_none | 2525419.4ns | 2521048.0ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_opt_scatter_all (n=6, range 2314933.3-2337308.0 ns)
  2314933.3 |########################################
  2316052.0 |
  2317170.8 |
  2318289.5 |
  2319408.2 |########################################
  2320527.0 |
  2321645.7 |########################################
  2322764.4 |
  2323883.2 |
  2325001.9 |
  2326120.6 |
  2327239.4 |
  2328358.1 |########################################
  2329476.8 |########################################
  2330595.6 |
  2331714.3 |
  2332833.0 |
  2333951.8 |
  2335070.5 |
  2336189.2 |
  (0 below, 1 above range)

carrier_opt_scatter_canon (n=6, range 2317550.8-2359665.6 ns)
  2317550.8 |########################################
  2319656.5 |
  2321762.3 |
  2323868.0 |
  2325973.8 |
  2328079.5 |
  2330185.2 |########################################
  2332291.0 |
  2334396.7 |
  2336502.5 |
  2338608.2 |########################################
  2340713.9 |
  2342819.7 |
  2344925.4 |
  2347031.2 |########################################
  2349136.9 |
  2351242.6 |########################################
  2353348.4 |
  2355454.1 |
  2357559.9 |
  (0 below, 1 above range)

carrier_opt_scatter_cse (n=6, range 2324629.6-2335837.2 ns)
  2324629.6 |########################################
  2325190.0 |########################################
  2325750.4 |
  2326310.7 |########################################
  2326871.1 |
  2327431.5 |
  2327991.9 |
  2328552.3 |########################################
  2329112.7 |
  2329673.0 |
  2330233.4 |
  2330793.8 |
  2331354.2 |########################################
  2331914.6 |
  2332475.0 |
  2333035.3 |
  2333595.7 |
  2334156.1 |
  2334716.5 |
  2335276.9 |
  (0 below, 1 above range)

carrier_opt_scatter_dce (n=6, range 2506157.5-2529273.2 ns)
  2506157.5 |####################
  2507313.3 |########################################
  2508469.1 |
  2509624.8 |
  2510780.6 |
  2511936.4 |
  2513092.2 |
  2514248.0 |
  2515403.8 |
  2516559.5 |
  2517715.3 |####################
  2518871.1 |####################
  2520026.9 |
  2521182.7 |
  2522338.5 |
  2523494.2 |
  2524650.0 |
  2525805.8 |
  2526961.6 |
  2528117.4 |
  (0 below, 1 above range)

carrier_opt_scatter_fold (n=6, range 2502333.8-2529087.3 ns)
  2502333.8 |####################
  2503671.5 |
  2505009.1 |
  2506346.8 |
  2507684.5 |
  2509022.2 |
  2510359.8 |####################
  2511697.5 |
  2513035.2 |
  2514372.9 |########################################
  2515710.5 |
  2517048.2 |
  2518385.9 |
  2519723.6 |
  2521061.2 |
  2522398.9 |####################
  2523736.6 |
  2525074.3 |
  2526411.9 |
  2527749.6 |
  (0 below, 1 above range)

carrier_opt_scatter_none (n=6, range 2514170.0-2529747.3 ns)
  2514170.0 |########################################
  2514948.9 |
  2515727.7 |
  2516506.6 |
  2517285.5 |
  2518064.3 |
  2518843.2 |
  2519622.1 |
  2520400.9 |
  2521179.8 |
  2521958.6 |
  2522737.5 |#############
  2523516.4 |
  2524295.2 |
  2525074.1 |
  2525853.0 |
  2526631.8 |#############
  2527410.7 |
  2528189.6 |
  2528968.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_scatter_all**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_canon**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cse**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_dce**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_fold**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_none**: bridge=100.2% of algo (FFI overhead may distort results)
