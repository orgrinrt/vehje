# Dispatch shape over the wire form, leaf profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_leaf_nullfloor dominates: 42% faster than the next best (carrier_disp_leaf_bittree)

carrier_disp_leaf_nullfloor (26.88 us) leads carrier_disp_leaf_bittree (38.05 us) by 42%, a clear separation rather than a photo finish. CV 0.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_leaf_nullfloor beats baseline by 38% (significant)

carrier_disp_leaf_nullfloor is -16.17 us (38%) faster than baseline carrier_disp_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_leaf_switch shows alternating (throttle bounce) (autocorr -0.71)

carrier_disp_leaf_switch's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_leaf_nullfloor} vs {carrier_disp_leaf_bittree, carrier_disp_leaf_threaded, carrier_disp_leaf_switch, carrier_disp_leaf_ifchain, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchainlin, carrier_disp_leaf_fntable} (42% apart)

The field splits into a fast tier {carrier_disp_leaf_nullfloor} and a slow tier {carrier_disp_leaf_bittree, carrier_disp_leaf_threaded, carrier_disp_leaf_switch, carrier_disp_leaf_ifchain, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchainlin, carrier_disp_leaf_fntable} with a 42% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_leaf_nullfloor** at 26878.5 ns median (-37.1% vs baseline)
- 2 variants significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 1.93x (fastest 26878.5 ns, slowest 51966.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 40517ns | 40339ns | 39959ns | 40223ns | 41237ns | -10.92% |
| carrier_disp_leaf_fntable | 54577ns | 54276ns | 53388ns | 54140ns | 55827ns | +19.99% |
| carrier_disp_leaf_ifchain | 49981ns | 48374ns | 46614ns | 48054ns | 54554ns | +9.88% |
| carrier_disp_leaf_ifchainasc | 47707ns | 48451ns | 43318ns | 46788ns | 51279ns | +4.88% |
| carrier_disp_leaf_ifchainlin | 50212ns | 50169ns | 49929ns | 50100ns | 50521ns | +10.39% |
| carrier_disp_leaf_nullfloor | 29192ns | 29205ns | 28960ns | 29128ns | 29403ns | -35.82% |
| carrier_disp_leaf_switch | 45486ns | 45049ns | 41541ns | 44423ns | 49052ns | base |
| carrier_disp_leaf_threaded | 45794ns | 44984ns | 44748ns | 44950ns | 47585ns | +0.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 38213ns | 37680ns | 38856ns | -11.51% | 0.027 |
| carrier_disp_leaf_fntable | 52231ns | 51084ns | 53346ns | +20.95% | 0.020 |
| carrier_disp_leaf_ifchain | 47689ns | 44423ns | 52088ns | +10.43% | 0.021 |
| carrier_disp_leaf_ifchainasc | 45382ns | 40968ns | 48900ns | +5.09% | 0.023 |
| carrier_disp_leaf_ifchainlin | 48004ns | 47751ns | 48254ns | +11.16% | 0.021 |
| carrier_disp_leaf_nullfloor | 26840ns | 26577ns | 27055ns | -37.85% | 0.038 |
| carrier_disp_leaf_switch | 43184ns | 39295ns | 46651ns | base | 0.024 |
| carrier_disp_leaf_threaded | 43501ns | 42505ns | 45243ns | +0.74% | 0.024 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 482317 | 2251492 | 0.214 | 1.03× |
| carrier_disp_leaf_fntable | 502937 | 2398710 | 0.210 | 1.08× |
| carrier_disp_leaf_ifchain | 449905 | 1738113 | 0.259 | 0.97× |
| carrier_disp_leaf_ifchainasc | 471786 | 1925014 | 0.245 | 1.01× |
| carrier_disp_leaf_ifchainlin | 458080 | 2274939 | 0.201 | 0.98× |
| carrier_disp_leaf_nullfloor | 428338 | 2323410 | 0.184 | 0.92× |
| carrier_disp_leaf_switch | 466026 | 1955565 | 0.238 | 1.00× |
| carrier_disp_leaf_threaded | 487502 | 2804095 | 0.174 | 1.05× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.039 Gops/s** (carrier_disp_leaf_nullfloor; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_leaf_bittree | 0.027 | 69.9% |
| carrier_disp_leaf_fntable | 0.020 | 51.1% |
| carrier_disp_leaf_ifchain | 0.022 | 57.6% |
| carrier_disp_leaf_ifchainasc | 0.022 | 57.6% |
| carrier_disp_leaf_ifchainlin | 0.021 | 55.4% |
| carrier_disp_leaf_nullfloor | 0.038 | 98.9% |
| carrier_disp_leaf_switch | 0.024 | 62.2% |
| carrier_disp_leaf_threaded | 0.024 | 62.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_leaf_bittree | 40517ns | 40517ns | -10.92% |
| carrier_disp_leaf_fntable | 54577ns | 54577ns | +19.99% |
| carrier_disp_leaf_ifchain | 49981ns | 49981ns | +9.88% |
| carrier_disp_leaf_ifchainasc | 47707ns | 47707ns | +4.88% |
| carrier_disp_leaf_ifchainlin | 50212ns | 50212ns | +10.39% |
| carrier_disp_leaf_nullfloor | 29192ns | 29192ns | -35.82% |
| carrier_disp_leaf_switch | 45486ns | 45486ns | base |
| carrier_disp_leaf_threaded | 45794ns | 45794ns | +0.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_leaf_switch | 42762ns | base | --- | [40139, 46651] | --- | --- | --- | --- |
| carrier_disp_leaf_bittree | 38048ns | -4522.3ns (-10.6%) | [-8249, -2140]ns | [37737, 38856] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_leaf_fntable | 51966ns | +9027.7ns (+21.1%) | [+5832, +12283]ns | [51382, 53346] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_leaf_ifchain | 46136ns | +5206.9ns (+12.2%) | [+665, +7645]ns | [44845, 52088] | YES (adj: no) | 0.2552 | 0.2188 | 0 |
| carrier_disp_leaf_ifchainasc | 46143ns | +2312.3ns (+5.4%) | [+794, +3490]ns | [41105, 48900] | YES (adj: no) | 0.2552 | 0.2188 | 0 |
| carrier_disp_leaf_ifchainlin | 47987ns | +5219.2ns (+12.2%) | [+1394, +7848]ns | [47773, 48254] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_leaf_nullfloor | 26879ns | -16173.5ns (-37.8%) | [-19654, -13202]ns | [26588, 27055] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_leaf_threaded | 42701ns | no significant difference | [-3999, +4675]ns | [42560, 45243] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_leaf_switch | carrier_disp_leaf_bittree | carrier_disp_leaf_fntable | carrier_disp_leaf_ifchain | carrier_disp_leaf_ifchainasc | carrier_disp_leaf_ifchainlin | carrier_disp_leaf_nullfloor | carrier_disp_leaf_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 39295ns | -3.5% | +32.1% | +16.7% | +5.0% | +22.1% | -31.6% | +8.2% |
| 2 | 45926ns | -14.8% | +16.0% | +19.0% | +3.5% | +4.0% | -40.9% | -7.0% |
| 3 | 40982ns | -7.8% | +26.1% | +8.4% | -0.0% | +17.1% | -34.1% | +4.2% |
| 4 | 47377ns | -20.5% | +9.8% | -4.5% | +5.7% | +2.0% | -43.3% | -10.1% |
| 5 | 41472ns | -7.0% | +28.8% | +11.9% | +7.9% | +16.1% | -35.9% | +14.8% |
| 6 | 44051ns | -13.3% | +16.0% | +12.4% | +8.4% | +8.5% | -39.7% | -2.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_leaf_bittree | -0.425 | moderate- |
| carrier_disp_leaf_fntable | -0.562 | HIGH- (thermal bounce) |
| carrier_disp_leaf_ifchain | -0.366 | moderate- |
| carrier_disp_leaf_ifchainasc | -0.628 | HIGH- (thermal bounce) |
| carrier_disp_leaf_ifchainlin | 0.110 | ok |
| carrier_disp_leaf_nullfloor | 0.493 | moderate+ |
| carrier_disp_leaf_switch | -0.709 | HIGH- (thermal bounce) |
| carrier_disp_leaf_threaded | -0.200 | moderate- |

**Consistency summary:**

- **carrier_disp_leaf_bittree**: won 6/6, lost 0/6
- **carrier_disp_leaf_fntable**: won 0/6, lost 6/6
- **carrier_disp_leaf_ifchain**: won 1/6, lost 5/6
- **carrier_disp_leaf_ifchainasc**: won 0/6, lost 5/6
- **carrier_disp_leaf_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_leaf_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_leaf_threaded**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 114531.5ns | 38213.5ns | 299.7% | HIGH |
| carrier_disp_leaf_fntable | 106152.3ns | 52231.3ns | 203.2% | HIGH |
| carrier_disp_leaf_ifchain | 95574.5ns | 47689.4ns | 200.4% | HIGH |
| carrier_disp_leaf_ifchainasc | 104562.9ns | 45382.4ns | 230.4% | HIGH |
| carrier_disp_leaf_ifchainlin | 96171.0ns | 48004.4ns | 200.3% | HIGH |
| carrier_disp_leaf_nullfloor | 107319.2ns | 26840.4ns | 399.8% | HIGH |
| carrier_disp_leaf_switch | 106186.5ns | 43183.9ns | 245.9% | HIGH |
| carrier_disp_leaf_threaded | 110565.5ns | 43501.4ns | 254.2% | HIGH |

## Distribution (algo ns)

```
carrier_disp_leaf_bittree (n=6, range 37680.0-38856.1 ns)
  37680.0 |########################################
  37738.8 |########################################
  37797.6 |
  37856.4 |########################################
  37915.2 |
  37974.0 |
  38032.8 |
  38091.6 |
  38150.4 |########################################
  38209.2 |
  38268.0 |
  38326.8 |
  38385.6 |
  38444.4 |
  38503.2 |
  38562.0 |########################################
  38620.8 |
  38679.6 |
  38738.4 |
  38797.2 |
  (0 below, 1 above range)

carrier_disp_leaf_fntable (n=6, range 51083.8-53345.6 ns)
  51083.8 |########################################
  51196.9 |
  51310.0 |
  51423.1 |
  51536.2 |
  51649.2 |########################################
  51762.3 |
  51875.4 |########################################
  51988.5 |########################################
  52101.6 |
  52214.7 |
  52327.8 |
  52440.9 |
  52554.0 |
  52667.1 |
  52780.2 |
  52893.2 |
  53006.3 |
  53119.4 |
  53232.5 |########################################
  (0 below, 1 above range)

carrier_disp_leaf_ifchain (n=6, range 44423.3-52087.9 ns)
  44423.3 |########################################
  44806.5 |
  45189.8 |########################################
  45573.0 |########################################
  45956.2 |
  46339.5 |########################################
  46722.7 |
  47105.9 |
  47489.1 |
  47872.4 |
  48255.6 |
  48638.8 |
  49022.1 |
  49405.3 |########################################
  49788.5 |
  50171.8 |
  50555.0 |
  50938.2 |
  51321.4 |
  51704.7 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchainasc (n=6, range 40967.9-48899.6 ns)
  40967.9 |########################################
  41364.5 |
  41761.1 |
  42157.7 |
  42554.2 |
  42950.8 |
  43347.4 |
  43744.0 |
  44140.6 |
  44537.2 |####################
  44933.8 |
  45330.3 |
  45726.9 |
  46123.5 |
  46520.1 |
  46916.7 |
  47313.3 |####################
  47709.8 |####################
  48106.4 |
  48503.0 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchainlin (n=6, range 47750.8-48253.6 ns)
  47750.8 |########################################
  47775.9 |########################################
  47801.1 |
  47826.2 |
  47851.4 |
  47876.5 |
  47901.6 |
  47926.8 |
  47951.9 |########################################
  47977.0 |########################################
  48002.2 |
  48027.3 |
  48052.5 |
  48077.6 |
  48102.7 |
  48127.9 |
  48153.0 |########################################
  48178.1 |
  48203.3 |
  48228.4 |
  (0 below, 1 above range)

carrier_disp_leaf_nullfloor (n=6, range 26577.1-27054.6 ns)
  26577.1 |########################################
  26601.0 |
  26624.8 |
  26648.7 |
  26672.6 |
  26696.5 |
  26720.3 |
  26744.2 |
  26768.1 |
  26792.0 |
  26815.8 |
  26839.7 |
  26863.6 |########################################
  26887.5 |
  26911.3 |
  26935.2 |
  26959.1 |
  26983.0 |####################
  27006.8 |
  27030.7 |
  (0 below, 1 above range)

carrier_disp_leaf_switch (n=6, range 39294.6-46651.4 ns)
  39294.6 |########################################
  39662.4 |
  40030.3 |
  40398.1 |
  40766.0 |########################################
  41133.8 |########################################
  41501.7 |
  41869.5 |
  42237.3 |
  42605.2 |
  42973.0 |
  43340.9 |
  43708.7 |########################################
  44076.6 |
  44444.4 |
  44812.2 |
  45180.1 |
  45547.9 |
  45915.8 |########################################
  46283.6 |
  (0 below, 1 above range)

carrier_disp_leaf_threaded (n=6, range 42505.4-45242.9 ns)
  42505.4 |########################################
  42642.3 |########################################
  42779.2 |####################
  42916.0 |
  43052.9 |
  43189.8 |
  43326.7 |
  43463.5 |
  43600.4 |
  43737.3 |
  43874.1 |
  44011.0 |
  44147.9 |
  44284.8 |
  44421.6 |
  44558.5 |
  44695.4 |
  44832.3 |
  44969.1 |
  45106.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_leaf_bittree**: bridge=300.0% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_fntable**: bridge=203.4% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchain**: bridge=200.1% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainasc**: bridge=212.3% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainlin**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_nullfloor**: bridge=400.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_switch**: bridge=249.0% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_threaded**: bridge=265.0% of algo (FFI overhead may distort results)
