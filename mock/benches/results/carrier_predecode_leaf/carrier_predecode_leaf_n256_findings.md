# Predecoded dispatch shape, leaf profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_leaf_direct beats baseline by 23% (significant)

carrier_pre_leaf_direct is -1.46 us (23%) faster than baseline carrier_pre_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_threaded, carrier_pre_leaf_switch, carrier_pre_leaf_regcache} vs {carrier_pre_leaf_fntable} (37% apart)

The field splits into a fast tier {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_threaded, carrier_pre_leaf_switch, carrier_pre_leaf_regcache} and a slow tier {carrier_pre_leaf_fntable} with a 37% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_pre_leaf_threaded's edge over baseline is significant but tiny (-6 ns, 0.09%)

carrier_pre_leaf_threaded differs from baseline carrier_pre_leaf_switch by -6 ns (0.09%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_pre_leaf_direct** at 4968.4 ns median (-22.4% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.87x (fastest 4968.4 ns, slowest 9267.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 7484ns | 7458ns | 7350ns | 7438ns | 7619ns | -16.13% |
| carrier_pre_leaf_fntable | 11715ns | 11845ns | 10246ns | 11794ns | 12330ns | +31.29% |
| carrier_pre_leaf_null | 7590ns | 7618ns | 7472ns | 7591ns | 7648ns | -14.94% |
| carrier_pre_leaf_regcache | 9325ns | 9272ns | 9159ns | 9253ns | 9517ns | +4.51% |
| carrier_pre_leaf_switch | 8923ns | 8962ns | 8526ns | 8923ns | 9122ns | base |
| carrier_pre_leaf_threaded | 8956ns | 8962ns | 8827ns | 8922ns | 9070ns | +0.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 5006ns | 4910ns | 5124ns | -21.48% | 0.051 |
| carrier_pre_leaf_fntable | 9170ns | 7953ns | 9689ns | +43.83% | 0.028 |
| carrier_pre_leaf_null | 5068ns | 5044ns | 5086ns | -20.51% | 0.051 |
| carrier_pre_leaf_regcache | 6778ns | 6645ns | 6913ns | +6.31% | 0.038 |
| carrier_pre_leaf_switch | 6376ns | 6135ns | 6466ns | base | 0.040 |
| carrier_pre_leaf_threaded | 6421ns | 6392ns | 6472ns | +0.71% | 0.040 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 263663 | 1050418 | 0.251 | 0.95× |
| carrier_pre_leaf_fntable | 292157 | 1076024 | 0.272 | 1.06× |
| carrier_pre_leaf_null | 265364 | 1234909 | 0.215 | 0.96× |
| carrier_pre_leaf_regcache | 270637 | 1315310 | 0.206 | 0.98× |
| carrier_pre_leaf_switch | 276832 | 1164330 | 0.238 | 1.00× |
| carrier_pre_leaf_threaded | 272718 | 1209454 | 0.225 | 0.99× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.052 Gops/s** (carrier_pre_leaf_direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_leaf_direct | 0.052 | 98.8% |
| carrier_pre_leaf_fntable | 0.028 | 53.0% |
| carrier_pre_leaf_null | 0.051 | 96.9% |
| carrier_pre_leaf_regcache | 0.038 | 72.6% |
| carrier_pre_leaf_switch | 0.040 | 76.7% |
| carrier_pre_leaf_threaded | 0.040 | 76.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_leaf_direct | 7484ns | 7484ns | -16.13% |
| carrier_pre_leaf_fntable | 11715ns | 11715ns | +31.29% |
| carrier_pre_leaf_null | 7590ns | 7590ns | -14.94% |
| carrier_pre_leaf_regcache | 9325ns | 9325ns | +4.51% |
| carrier_pre_leaf_switch | 8923ns | 8923ns | base |
| carrier_pre_leaf_threaded | 8956ns | 8956ns | +0.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_leaf_switch | 6400ns | base | --- | [6261, 6466] | --- | --- | --- | --- |
| carrier_pre_leaf_direct | 4968ns | -1457.7ns (-22.8%) | [-1507, -1144]ns | [4927, 5124] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_fntable | 9268ns | +2812.7ns (+43.9%) | [+2161, +3410]ns | [8555, 9689] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_null | 5068ns | -1328.5ns (-20.8%) | [-1387, -1208]ns | [5050, 5086] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_regcache | 6762ns | +367.8ns (+5.7%) | [+204, +635]ns | [6659, 6913] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_threaded | 6399ns | no significant difference | [-49, +191]ns | [6392, 6472] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_leaf_switch | carrier_pre_leaf_direct | carrier_pre_leaf_fntable | carrier_pre_leaf_null | carrier_pre_leaf_regcache | carrier_pre_leaf_threaded |
|---|---|---|---|---|---|---|
| 1 | 6388ns | -23.1% | +24.5% | -20.8% | +5.8% | +2.0% |
| 2 | 6421ns | -22.7% | +54.8% | -21.2% | +5.7% | -0.3% |
| 3 | 6399ns | -22.8% | +44.1% | -20.7% | +10.0% | -0.1% |
| 4 | 6510ns | -23.6% | +43.1% | -21.7% | +2.1% | -1.2% |
| 5 | 6135ns | -18.5% | +53.9% | -17.8% | +10.2% | +4.2% |
| 6 | 6400ns | -18.0% | +43.1% | -20.7% | +4.3% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_leaf_direct | 0.100 | ok |
| carrier_pre_leaf_fntable | -0.394 | moderate- |
| carrier_pre_leaf_null | -0.361 | moderate- |
| carrier_pre_leaf_regcache | -0.302 | moderate- |
| carrier_pre_leaf_switch | -0.423 | moderate- |
| carrier_pre_leaf_threaded | -0.088 | ok |

**Consistency summary:**

- **carrier_pre_leaf_direct**: won 6/6, lost 0/6
- **carrier_pre_leaf_fntable**: won 0/6, lost 6/6
- **carrier_pre_leaf_null**: won 6/6, lost 0/6
- **carrier_pre_leaf_regcache**: won 0/6, lost 6/6
- **carrier_pre_leaf_threaded**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 88671.9ns | 5006.3ns | 1771.2% | HIGH |
| carrier_pre_leaf_fntable | 91863.5ns | 9170.4ns | 1001.7% | HIGH |
| carrier_pre_leaf_null | 87331.2ns | 5068.1ns | 1723.2% | HIGH |
| carrier_pre_leaf_regcache | 88549.1ns | 6777.8ns | 1306.5% | HIGH |
| carrier_pre_leaf_switch | 89913.3ns | 6375.7ns | 1410.3% | HIGH |
| carrier_pre_leaf_threaded | 89478.5ns | 6421.0ns | 1393.5% | HIGH |

## Distribution (algo ns)

```
carrier_pre_leaf_direct (n=6, range 4910.0-5124.0 ns)
   4910.0 |########################################
   4920.7 |
   4931.4 |
   4942.1 |########################################
   4952.8 |########################################
   4963.5 |
   4974.2 |########################################
   4984.9 |
   4995.6 |########################################
   5006.3 |
   5017.0 |
   5027.7 |
   5038.4 |
   5049.1 |
   5059.8 |
   5070.5 |
   5081.2 |
   5091.9 |
   5102.6 |
   5113.3 |
  (0 below, 1 above range)

carrier_pre_leaf_fntable (n=6, range 7952.9-9688.5 ns)
   7952.9 |########################################
   8039.7 |
   8126.5 |
   8213.2 |
   8300.0 |
   8386.8 |
   8473.6 |
   8560.4 |
   8647.2 |
   8733.9 |
   8820.7 |
   8907.5 |
   8994.3 |
   9081.1 |########################################
   9167.9 |########################################
   9254.6 |########################################
   9341.4 |
   9428.2 |########################################
   9515.0 |
   9601.8 |
  (0 below, 1 above range)

carrier_pre_leaf_null (n=6, range 5044.2-5086.2 ns)
   5044.2 |####################
   5046.3 |
   5048.4 |
   5050.5 |
   5052.6 |
   5054.7 |####################
   5056.8 |
   5058.9 |####################
   5061.0 |
   5063.1 |
   5065.2 |
   5067.3 |
   5069.4 |
   5071.5 |
   5073.6 |########################################
   5075.7 |
   5077.8 |
   5079.9 |
   5082.0 |
   5084.1 |
  (0 below, 1 above range)

carrier_pre_leaf_regcache (n=6, range 6645.4-6912.7 ns)
   6645.4 |####################
   6658.8 |
   6672.1 |####################
   6685.5 |
   6698.9 |
   6712.2 |
   6725.6 |
   6739.0 |
   6752.3 |########################################
   6765.7 |
   6779.1 |####################
   6792.4 |
   6805.8 |
   6819.1 |
   6832.5 |
   6845.9 |
   6859.2 |
   6872.6 |
   6886.0 |
   6899.3 |
  (0 below, 1 above range)

carrier_pre_leaf_switch (n=6, range 6135.0-6465.8 ns)
   6135.0 |####################
   6151.5 |
   6168.1 |
   6184.6 |
   6201.2 |
   6217.7 |
   6234.2 |
   6250.8 |
   6267.3 |
   6283.9 |
   6300.4 |
   6316.9 |
   6333.5 |
   6350.0 |
   6366.6 |
   6383.1 |########################################
   6399.6 |####################
   6416.2 |####################
   6432.7 |
   6449.3 |
  (0 below, 1 above range)

carrier_pre_leaf_threaded (n=6, range 6391.7-6471.9 ns)
   6391.7 |########################################
   6395.7 |####################
   6399.7 |####################
   6403.7 |
   6407.7 |
   6411.7 |
   6415.7 |
   6419.8 |
   6423.8 |
   6427.8 |####################
   6431.8 |
   6435.8 |
   6439.8 |
   6443.8 |
   6447.8 |
   6451.8 |
   6455.8 |
   6459.8 |
   6463.8 |
   6467.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_leaf_direct**: bridge=1787.5% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_fntable**: bridge=984.4% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_null**: bridge=1725.4% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_regcache**: bridge=1313.9% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_switch**: bridge=1399.0% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_threaded**: bridge=1397.9% of algo (FFI overhead may distort results)
