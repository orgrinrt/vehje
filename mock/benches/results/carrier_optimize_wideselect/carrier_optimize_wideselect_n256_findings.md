# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, wideselect profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_wideselect_none**

## Highlights

Baseline for all deltas below: **carrier_opt_wideselect_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_wideselect_canon is fastest but the noisiest (CV 5.6%)

carrier_opt_wideselect_canon wins on median (7.32 us) yet has the highest variance (CV 5.6%), while carrier_opt_wideselect_cse is the steadiest (CV 4.2%, 7.56 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (carrier_opt_wideselect_canon, carrier_opt_wideselect_all) are a dead heat (<1%)

carrier_opt_wideselect_canon (7.32 us) and carrier_opt_wideselect_all (7.33 us) differ by 0.14%, inside the noise, even though the wider field spreads 7.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_opt_wideselect_none shows alternating (throttle bounce) (autocorr -0.58)

carrier_opt_wideselect_none's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader carrier_opt_wideselect_canon vs stability leader carrier_opt_wideselect_cse (+3% speed for 1.4x steadier)

carrier_opt_wideselect_canon is fastest (7.32 us, CV 5.6%); carrier_opt_wideselect_cse gives up 3.2% median for 1.4x lower variance (CV 4.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### carrier_opt_wideselect_dce's edge over baseline is significant but tiny (9 ns, 0.12%)

carrier_opt_wideselect_dce differs from baseline carrier_opt_wideselect_none by 9 ns (0.12%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_wideselect_canon** at 7323.7 ns median (-6.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.08x (fastest 7323.7 ns, slowest 7896.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 9649ns | 9662ns | 9035ns | 9512ns | 10161ns | -5.98% |
| carrier_opt_wideselect_canon | 9873ns | 9625ns | 9315ns | 9525ns | 10674ns | -3.79% |
| carrier_opt_wideselect_cse | 9960ns | 9795ns | 9477ns | 9737ns | 10536ns | -2.95% |
| carrier_opt_wideselect_dce | 10355ns | 10286ns | 9665ns | 10119ns | 11053ns | +0.91% |
| carrier_opt_wideselect_fold | 10117ns | 9924ns | 9648ns | 9840ns | 10767ns | -1.42% |
| carrier_opt_wideselect_none | 10262ns | 10070ns | 9652ns | 9979ns | 10992ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 7310ns | 6864ns | 7658ns | -7.56% | 0.035 |
| carrier_opt_wideselect_canon | 7511ns | 7119ns | 8077ns | -5.01% | 0.034 |
| carrier_opt_wideselect_cse | 7666ns | 7321ns | 8088ns | -3.06% | 0.033 |
| carrier_opt_wideselect_dce | 7941ns | 7488ns | 8436ns | +0.42% | 0.032 |
| carrier_opt_wideselect_fold | 7797ns | 7432ns | 8313ns | -1.41% | 0.033 |
| carrier_opt_wideselect_none | 7908ns | 7490ns | 8365ns | base | 0.032 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 296446 | 1719848 | 0.172 | 0.98× |
| carrier_opt_wideselect_canon | 293482 | 1694261 | 0.173 | 0.97× |
| carrier_opt_wideselect_cse | 299629 | 1690825 | 0.177 | 0.99× |
| carrier_opt_wideselect_dce | 302102 | 1735374 | 0.174 | 1.00× |
| carrier_opt_wideselect_fold | 305481 | 1747920 | 0.175 | 1.01× |
| carrier_opt_wideselect_none | 301191 | 1732887 | 0.174 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.037 Gops/s** (carrier_opt_wideselect_all; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_wideselect_all | 0.035 | 93.6% |
| carrier_opt_wideselect_canon | 0.035 | 93.7% |
| carrier_opt_wideselect_cse | 0.034 | 90.8% |
| carrier_opt_wideselect_dce | 0.032 | 86.9% |
| carrier_opt_wideselect_fold | 0.033 | 89.8% |
| carrier_opt_wideselect_none | 0.033 | 87.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_wideselect_all | 9649ns | 9649ns | -5.98% |
| carrier_opt_wideselect_canon | 9873ns | 9873ns | -3.79% |
| carrier_opt_wideselect_cse | 9960ns | 9960ns | -2.95% |
| carrier_opt_wideselect_dce | 10355ns | 10355ns | +0.91% |
| carrier_opt_wideselect_fold | 10117ns | 10117ns | -1.42% |
| carrier_opt_wideselect_none | 10262ns | 10262ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_wideselect_none | 7824ns | base | --- | [7535, 8365] | --- | --- | --- | --- |
| carrier_opt_wideselect_all | 7334ns | -656.2ns (-8.4%) | [-955, -182]ns | [6938, 7658] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_opt_wideselect_canon | 7324ns | -385.6ns (-4.9%) | [-587, -217]ns | [7134, 8077] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_opt_wideselect_cse | 7557ns | no significant difference | [-657, +143]ns | [7354, 8088] | no | 0.2734 | 0.2188 | 0 |
| carrier_opt_wideselect_dce | 7897ns | no significant difference | [-298, +389]ns | [7491, 8436] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_wideselect_fold | 7642ns | no significant difference | [-313, +124]ns | [7435, 8313] | no | 0.2734 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_wideselect_none | carrier_opt_wideselect_all | carrier_opt_wideselect_canon | carrier_opt_wideselect_cse | carrier_opt_wideselect_dce | carrier_opt_wideselect_fold |
|---|---|---|---|---|---|---|
| 1 | 7580ns | -9.5% | -5.7% | -3.4% | -1.1% | -2.0% |
| 2 | 8198ns | -7.3% | -2.9% | -2.0% | +5.0% | +3.6% |
| 3 | 7896ns | -11.2% | -6.8% | -5.2% | +4.7% | -1.8% |
| 4 | 7752ns | -0.5% | -8.2% | +5.0% | -3.4% | -2.9% |
| 5 | 8531ns | -12.0% | -4.0% | -10.6% | -3.9% | -4.7% |
| 6 | 7490ns | -4.4% | -2.6% | -1.4% | +1.4% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_wideselect_all | -0.484 | moderate- |
| carrier_opt_wideselect_canon | -0.575 | HIGH- (thermal bounce) |
| carrier_opt_wideselect_cse | -0.486 | moderate- |
| carrier_opt_wideselect_dce | -0.382 | moderate- |
| carrier_opt_wideselect_fold | -0.514 | HIGH- (thermal bounce) |
| carrier_opt_wideselect_none | -0.583 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_opt_wideselect_all**: won 6/6, lost 0/6
- **carrier_opt_wideselect_canon**: won 6/6, lost 0/6
- **carrier_opt_wideselect_cse**: won 5/6, lost 1/6
- **carrier_opt_wideselect_dce**: won 3/6, lost 3/6
- **carrier_opt_wideselect_fold**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 90687.6ns | 7309.9ns | 1240.6% | HIGH |
| carrier_opt_wideselect_canon | 88098.0ns | 7511.4ns | 1172.8% | HIGH |
| carrier_opt_wideselect_cse | 89779.4ns | 7666.1ns | 1171.1% | HIGH |
| carrier_opt_wideselect_dce | 91431.8ns | 7941.3ns | 1151.4% | HIGH |
| carrier_opt_wideselect_fold | 90979.6ns | 7796.5ns | 1166.9% | HIGH |
| carrier_opt_wideselect_none | 90927.1ns | 7908.0ns | 1149.8% | HIGH |

## Distribution (algo ns)

```
carrier_opt_wideselect_all (n=6, range 6863.7-7657.9 ns)
   6863.7 |########################################
   6903.4 |
   6943.1 |
   6982.8 |########################################
   7022.5 |
   7062.2 |
   7102.0 |
   7141.7 |########################################
   7181.4 |
   7221.1 |
   7260.8 |
   7300.5 |
   7340.2 |
   7379.9 |
   7419.6 |
   7459.3 |
   7499.1 |########################################
   7538.8 |
   7578.5 |########################################
   7618.2 |
  (0 below, 1 above range)

carrier_opt_wideselect_canon (n=6, range 7119.2-8077.1 ns)
   7119.2 |########################################
   7167.1 |
   7215.0 |
   7262.9 |####################
   7310.8 |####################
   7358.7 |
   7406.6 |
   7454.5 |
   7502.4 |
   7550.3 |
   7598.1 |
   7646.0 |
   7693.9 |
   7741.8 |
   7789.7 |
   7837.6 |
   7885.5 |
   7933.4 |####################
   7981.3 |
   8029.2 |
  (0 below, 1 above range)

carrier_opt_wideselect_cse (n=6, range 7320.8-8087.7 ns)
   7320.8 |########################################
   7359.1 |########################################
   7397.5 |
   7435.8 |
   7474.2 |########################################
   7512.5 |
   7550.9 |
   7589.2 |
   7627.6 |########################################
   7665.9 |
   7704.2 |
   7742.6 |
   7780.9 |
   7819.3 |
   7857.6 |
   7896.0 |
   7934.3 |
   7972.7 |
   8011.0 |########################################
   8049.4 |
  (0 below, 1 above range)

carrier_opt_wideselect_dce (n=6, range 7488.3-8435.9 ns)
   7488.3 |########################################
   7535.7 |
   7583.1 |####################
   7630.4 |
   7677.8 |
   7725.2 |
   7772.6 |
   7819.9 |
   7867.3 |
   7914.7 |
   7962.1 |
   8009.5 |
   8056.8 |
   8104.2 |
   8151.6 |
   8199.0 |####################
   8246.3 |####################
   8293.7 |
   8341.1 |
   8388.5 |
  (0 below, 1 above range)

carrier_opt_wideselect_fold (n=6, range 7431.7-8313.1 ns)
   7431.7 |########################################
   7475.8 |
   7519.8 |####################
   7563.9 |
   7608.0 |
   7652.1 |
   7696.1 |
   7740.2 |####################
   7784.3 |
   7828.4 |
   7872.4 |
   7916.5 |
   7960.6 |
   8004.6 |
   8048.7 |
   8092.8 |####################
   8136.9 |
   8180.9 |
   8225.0 |
   8269.1 |
  (0 below, 1 above range)

carrier_opt_wideselect_none (n=6, range 7489.6-8364.5 ns)
   7489.6 |########################################
   7533.3 |
   7577.1 |########################################
   7620.8 |
   7664.6 |
   7708.3 |
   7752.1 |########################################
   7795.8 |
   7839.6 |
   7883.3 |########################################
   7927.1 |
   7970.8 |
   8014.6 |
   8058.3 |
   8102.1 |
   8145.8 |
   8189.6 |########################################
   8233.3 |
   8277.1 |
   8320.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_wideselect_all**: bridge=1240.2% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_canon**: bridge=1199.1% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cse**: bridge=1181.5% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_dce**: bridge=1155.6% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_fold**: bridge=1185.9% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_none**: bridge=1158.3% of algo (FFI overhead may distort results)
