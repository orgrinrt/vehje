# Predecoded dispatch shape, leaf profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_leaf_direct beats baseline by 46% (significant)

carrier_pre_leaf_direct is -66.61 us (46%) faster than baseline carrier_pre_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_leaf_fntable is an outlier: 3.2x slower than the field

carrier_pre_leaf_fntable (248.68 us) is 3.2x the fastest (77.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_pre_leaf_direct shows alternating (throttle bounce) (autocorr -0.59)

carrier_pre_leaf_direct's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_threaded, carrier_pre_leaf_switch, carrier_pre_leaf_regcache} vs {carrier_pre_leaf_fntable} (61% apart)

The field splits into a fast tier {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_threaded, carrier_pre_leaf_switch, carrier_pre_leaf_regcache} and a slow tier {carrier_pre_leaf_fntable} with a 61% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.2x the fastest

Fastest carrier_pre_leaf_direct (77.07 us) to slowest carrier_pre_leaf_fntable (248.68 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_pre_leaf_direct** at 77068.1 ns median (-47.0% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.23x (fastest 77068.1 ns, slowest 248684.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 81005ns | 79209ns | 78411ns | 79037ns | 85252ns | -44.96% |
| carrier_pre_leaf_fntable | 250430ns | 250835ns | 245606ns | 250612ns | 252568ns | +70.15% |
| carrier_pre_leaf_null | 82478ns | 83305ns | 78000ns | 82697ns | 84390ns | -43.96% |
| carrier_pre_leaf_regcache | 155311ns | 156486ns | 140584ns | 154052ns | 164565ns | +5.53% |
| carrier_pre_leaf_switch | 147179ns | 147532ns | 143400ns | 146333ns | 150337ns | base |
| carrier_pre_leaf_threaded | 118060ns | 117134ns | 105876ns | 113433ns | 131092ns | -19.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 78803ns | 76272ns | 82933ns | -45.63% | 0.052 |
| carrier_pre_leaf_fntable | 248276ns | 243413ns | 250420ns | +71.29% | 0.016 |
| carrier_pre_leaf_null | 80207ns | 75838ns | 82078ns | -44.66% | 0.051 |
| carrier_pre_leaf_regcache | 153126ns | 138409ns | 162353ns | +5.64% | 0.027 |
| carrier_pre_leaf_switch | 144946ns | 141272ns | 148001ns | base | 0.028 |
| carrier_pre_leaf_threaded | 115831ns | 103623ns | 128797ns | -20.09% | 0.035 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 736478 | 2748886 | 0.268 | 0.77× |
| carrier_pre_leaf_fntable | 1596014 | 3223472 | 0.495 | 1.68× |
| carrier_pre_leaf_null | 695147 | 3132569 | 0.222 | 0.73× |
| carrier_pre_leaf_regcache | 988558 | 3146094 | 0.314 | 1.04× |
| carrier_pre_leaf_switch | 950831 | 2656604 | 0.358 | 1.00× |
| carrier_pre_leaf_threaded | 744413 | 2726001 | 0.273 | 0.78× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.054 Gops/s** (carrier_pre_leaf_null; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_leaf_direct | 0.053 | 98.4% |
| carrier_pre_leaf_fntable | 0.016 | 30.5% |
| carrier_pre_leaf_null | 0.051 | 93.6% |
| carrier_pre_leaf_regcache | 0.027 | 49.2% |
| carrier_pre_leaf_switch | 0.028 | 52.2% |
| carrier_pre_leaf_threaded | 0.036 | 66.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_leaf_direct | 81005ns | 81005ns | -44.96% |
| carrier_pre_leaf_fntable | 250430ns | 250430ns | +70.15% |
| carrier_pre_leaf_null | 82478ns | 82478ns | -43.96% |
| carrier_pre_leaf_regcache | 155311ns | 155311ns | +5.53% |
| carrier_pre_leaf_switch | 147179ns | 147179ns | base |
| carrier_pre_leaf_threaded | 118060ns | 118060ns | -19.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_leaf_switch | 145326ns | base | --- | [141511, 148001] | --- | --- | --- | --- |
| carrier_pre_leaf_direct | 77068ns | -66609.8ns (-45.8%) | [-70722, -61097]ns | [76408, 82933] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_fntable | 248685ns | +103056.2ns (+70.9%) | [+98533, +108399]ns | [245722, 250420] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_null | 81013ns | -64579.4ns (-44.4%) | [-67951, -61687]ns | [77530, 82078] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_regcache | 154295ns | no significant difference | [-1330, +16874]ns | [142730, 162353] | no | 0.2188 | 0.2188 | 0 |
| carrier_pre_leaf_threaded | 114947ns | -29060.7ns (-20.0%) | [-39387, -18897]ns | [103749, 128797] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_leaf_switch | carrier_pre_leaf_direct | carrier_pre_leaf_fntable | carrier_pre_leaf_null | carrier_pre_leaf_regcache | carrier_pre_leaf_threaded |
|---|---|---|---|---|---|---|
| 1 | 144284ns | -47.1% | +71.9% | -44.3% | +15.3% | -14.2% |
| 2 | 141272ns | -41.3% | +77.8% | -42.1% | +8.2% | -26.5% |
| 3 | 146368ns | -47.3% | +69.9% | -43.7% | +0.5% | -11.8% |
| 4 | 141750ns | -46.0% | +75.4% | -46.5% | -2.4% | -26.9% |
| 5 | 146789ns | -43.5% | +65.8% | -44.4% | +6.1% | -27.7% |
| 6 | 149212ns | -48.4% | +67.3% | -46.9% | +6.1% | -13.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_leaf_direct | -0.589 | HIGH- (thermal bounce) |
| carrier_pre_leaf_fntable | -0.223 | moderate- |
| carrier_pre_leaf_null | -0.456 | moderate- |
| carrier_pre_leaf_regcache | 0.135 | ok |
| carrier_pre_leaf_switch | -0.112 | ok |
| carrier_pre_leaf_threaded | -0.535 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_pre_leaf_direct**: won 6/6, lost 0/6
- **carrier_pre_leaf_fntable**: won 0/6, lost 6/6
- **carrier_pre_leaf_null**: won 6/6, lost 0/6
- **carrier_pre_leaf_regcache**: won 1/6, lost 5/6
- **carrier_pre_leaf_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 158022.2ns | 78803.0ns | 200.5% | HIGH |
| carrier_pre_leaf_fntable | 259918.2ns | 248275.6ns | 104.7% | HIGH |
| carrier_pre_leaf_null | 148683.8ns | 80206.7ns | 185.4% | HIGH |
| carrier_pre_leaf_regcache | 165724.6ns | 153125.9ns | 108.2% | HIGH |
| carrier_pre_leaf_switch | 161235.1ns | 144946.0ns | 111.2% | HIGH |
| carrier_pre_leaf_threaded | 127942.4ns | 115831.0ns | 110.5% | HIGH |

## Distribution (algo ns)

```
carrier_pre_leaf_direct (n=6, range 76272.1-82933.4 ns)
  76272.1 |########################################
  76605.2 |
  76938.2 |########################################
  77271.3 |
  77604.4 |
  77937.4 |
  78270.5 |
  78603.5 |
  78936.6 |
  79269.7 |
  79602.7 |
  79935.8 |
  80268.9 |
  80601.9 |
  80935.0 |
  81268.0 |
  81601.1 |
  81934.2 |
  82267.2 |
  82600.3 |####################
  (0 below, 1 above range)

carrier_pre_leaf_fntable (n=6, range 243412.9-250420.2 ns)
  243412.9 |########################################
  243763.3 |
  244113.6 |
  244464.0 |
  244814.4 |
  245164.7 |
  245515.1 |
  245865.5 |
  246215.8 |
  246566.2 |
  246916.5 |
  247266.9 |
  247617.3 |
  247967.6 |########################################
  248318.0 |########################################
  248668.4 |########################################
  249018.7 |
  249369.1 |########################################
  249719.5 |
  250069.8 |
  (0 below, 1 above range)

carrier_pre_leaf_null (n=6, range 75838.3-82077.5 ns)
  75838.3 |########################################
  76150.3 |
  76462.2 |
  76774.2 |
  77086.1 |
  77398.1 |
  77710.1 |
  78022.0 |
  78334.0 |
  78645.9 |
  78957.9 |########################################
  79269.9 |
  79581.8 |
  79893.8 |
  80205.7 |########################################
  80517.7 |
  80829.7 |
  81141.6 |
  81453.6 |########################################
  81765.5 |########################################
  (0 below, 1 above range)

carrier_pre_leaf_regcache (n=6, range 138409.2-162352.7 ns)
  138409.2 |########################################
  139606.4 |
  140803.6 |
  142000.7 |
  143197.9 |
  144395.1 |
  145592.2 |
  146789.4 |########################################
  147986.6 |
  149183.8 |
  150381.0 |
  151578.1 |
  152775.3 |########################################
  153972.5 |
  155169.7 |########################################
  156366.8 |
  157564.0 |########################################
  158761.2 |
  159958.4 |
  161155.5 |
  (0 below, 1 above range)

carrier_pre_leaf_switch (n=6, range 141271.7-148000.7 ns)
  141271.7 |########################################
  141608.1 |########################################
  141944.6 |
  142281.0 |
  142617.5 |
  142953.9 |
  143290.4 |
  143626.8 |
  143963.3 |########################################
  144299.7 |
  144636.2 |
  144972.6 |
  145309.1 |
  145645.5 |
  145982.0 |
  146318.4 |########################################
  146654.9 |########################################
  146991.3 |
  147327.8 |
  147664.2 |
  (0 below, 1 above range)

carrier_pre_leaf_threaded (n=6, range 103622.9-128797.3 ns)
  103622.9 |########################################
  104881.6 |
  106140.3 |####################
  107399.1 |
  108657.8 |
  109916.5 |
  111175.2 |
  112433.9 |
  113692.7 |
  114951.4 |
  116210.1 |
  117468.8 |
  118727.5 |
  119986.3 |
  121245.0 |
  122503.7 |####################
  123762.4 |
  125021.1 |
  126279.9 |
  127538.6 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_leaf_direct**: bridge=215.9% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_fntable**: bridge=104.6% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_null**: bridge=182.1% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_regcache**: bridge=108.2% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_switch**: bridge=111.4% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_threaded**: bridge=110.3% of algo (FFI overhead may distort results)
