# Predecoded dispatch shape, leaf profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_leaf_direct beats baseline by 46% (significant)

carrier_pre_leaf_direct is -66.90 us (46%) faster than baseline carrier_pre_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_leaf_fntable is an outlier: 3.1x slower than the field

carrier_pre_leaf_fntable (248.16 us) is 3.1x the fastest (78.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_pre_leaf_direct shows alternating (throttle bounce) (autocorr -0.59)

carrier_pre_leaf_direct's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_threaded, carrier_pre_leaf_switch, carrier_pre_leaf_regcache} vs {carrier_pre_leaf_fntable} (68% apart)

The field splits into a fast tier {carrier_pre_leaf_direct, carrier_pre_leaf_null, carrier_pre_leaf_threaded, carrier_pre_leaf_switch, carrier_pre_leaf_regcache} and a slow tier {carrier_pre_leaf_fntable} with a 68% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.1x the fastest

Fastest carrier_pre_leaf_direct (78.78 us) to slowest carrier_pre_leaf_fntable (248.16 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_pre_leaf_direct** at 78784.8 ns median (-46.3% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.15x (fastest 78784.8 ns, slowest 248164.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 81629ns | 81016ns | 77874ns | 80209ns | 85638ns | -45.30% |
| carrier_pre_leaf_fntable | 250532ns | 250408ns | 246589ns | 250109ns | 253139ns | +67.89% |
| carrier_pre_leaf_null | 83284ns | 84672ns | 77788ns | 83916ns | 85084ns | -44.19% |
| carrier_pre_leaf_regcache | 152429ns | 149831ns | 139154ns | 149163ns | 163966ns | +2.15% |
| carrier_pre_leaf_switch | 149228ns | 148872ns | 145738ns | 148161ns | 152574ns | base |
| carrier_pre_leaf_threaded | 102047ns | 101912ns | 88732ns | 99805ns | 112068ns | -31.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 79383ns | 75664ns | 83310ns | -45.98% | 0.052 |
| carrier_pre_leaf_fntable | 248312ns | 244391ns | 250926ns | +68.97% | 0.016 |
| carrier_pre_leaf_null | 80965ns | 75638ns | 82697ns | -44.91% | 0.051 |
| carrier_pre_leaf_regcache | 150188ns | 136929ns | 161696ns | +2.20% | 0.027 |
| carrier_pre_leaf_switch | 146958ns | 143543ns | 150279ns | base | 0.028 |
| carrier_pre_leaf_threaded | 99778ns | 86544ns | 109905ns | -32.10% | 0.041 |

## Performance model

- Peak throughput: **0.054 Gops/s** (carrier_pre_leaf_null; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_leaf_direct | 0.052 | 96.0% |
| carrier_pre_leaf_fntable | 0.017 | 30.5% |
| carrier_pre_leaf_null | 0.050 | 91.9% |
| carrier_pre_leaf_regcache | 0.028 | 51.2% |
| carrier_pre_leaf_switch | 0.028 | 51.6% |
| carrier_pre_leaf_threaded | 0.041 | 76.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_leaf_direct | 81629ns | 81629ns | -45.30% |
| carrier_pre_leaf_fntable | 250532ns | 250532ns | +67.89% |
| carrier_pre_leaf_null | 83284ns | 83284ns | -44.19% |
| carrier_pre_leaf_regcache | 152429ns | 152429ns | +2.15% |
| carrier_pre_leaf_switch | 149228ns | 149228ns | base |
| carrier_pre_leaf_threaded | 102047ns | 102047ns | -31.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_leaf_switch | 146600ns | base | --- | [143997, 150279] | --- | --- | --- | --- |
| carrier_pre_leaf_direct | 78785ns | -66904.2ns (-45.6%) | [-74224, -61597]ns | [76055, 83310] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_fntable | 248164ns | +103256.6ns (+70.4%) | [+95568, +105237]ns | [245846, 250926] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_null | 82316ns | -66431.6ns (-45.3%) | [-69146, -62404]ns | [77881, 82697] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_regcache | 147594ns | no significant difference | [-5035, +13476]ns | [141273, 161696] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_leaf_threaded | 99570ns | -46118.9ns (-31.5%) | [-57514, -37908]ns | [89859, 109905] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_leaf_switch | carrier_pre_leaf_direct | carrier_pre_leaf_fntable | carrier_pre_leaf_null | carrier_pre_leaf_regcache | carrier_pre_leaf_threaded |
|---|---|---|---|---|---|---|
| 1 | 150296ns | -49.1% | +64.5% | -45.3% | +0.9% | -38.0% |
| 2 | 145365ns | -43.3% | +70.8% | -43.3% | +2.1% | -23.7% |
| 3 | 147835ns | -46.1% | +70.4% | -43.9% | +16.2% | -28.9% |
| 4 | 143543ns | -45.7% | +74.1% | -47.3% | -4.6% | -34.5% |
| 5 | 144450ns | -41.7% | +71.7% | -42.9% | +0.8% | -40.1% |
| 6 | 150262ns | -49.6% | +62.6% | -46.7% | -2.3% | -27.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_leaf_direct | -0.585 | HIGH- (thermal bounce) |
| carrier_pre_leaf_fntable | 0.192 | ok |
| carrier_pre_leaf_null | -0.387 | moderate- |
| carrier_pre_leaf_regcache | -0.370 | moderate- |
| carrier_pre_leaf_switch | -0.218 | moderate- |
| carrier_pre_leaf_threaded | -0.184 | ok |

**Consistency summary:**

- **carrier_pre_leaf_direct**: won 6/6, lost 0/6
- **carrier_pre_leaf_fntable**: won 0/6, lost 6/6
- **carrier_pre_leaf_null**: won 6/6, lost 0/6
- **carrier_pre_leaf_regcache**: won 2/6, lost 4/6
- **carrier_pre_leaf_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 159174.7ns | 79383.1ns | 200.5% | HIGH |
| carrier_pre_leaf_fntable | 256332.5ns | 248312.2ns | 103.2% | HIGH |
| carrier_pre_leaf_null | 145062.9ns | 80964.5ns | 179.2% | HIGH |
| carrier_pre_leaf_regcache | 161388.6ns | 150187.6ns | 107.5% | HIGH |
| carrier_pre_leaf_switch | 161725.4ns | 146958.3ns | 110.0% | HIGH |
| carrier_pre_leaf_threaded | 118753.5ns | 99778.0ns | 119.0% | HIGH |

## Distribution (algo ns)

```
carrier_pre_leaf_direct (n=6, range 75663.8-83310.0 ns)
  75663.8 |########################################
  76046.1 |
  76428.4 |########################################
  76810.7 |
  77193.0 |
  77575.4 |########################################
  77957.7 |
  78340.0 |
  78722.3 |
  79104.6 |
  79486.9 |########################################
  79869.2 |
  80251.5 |
  80633.8 |
  81016.1 |
  81398.4 |
  81780.8 |
  82163.1 |########################################
  82545.4 |
  82927.7 |
  (0 below, 1 above range)

carrier_pre_leaf_fntable (n=6, range 244391.2-250926.2 ns)
  244391.2 |####################
  244718.0 |
  245044.7 |
  245371.5 |
  245698.2 |
  246025.0 |
  246351.7 |
  246678.5 |
  247005.2 |####################
  247332.0 |
  247658.7 |
  247985.5 |########################################
  248312.2 |
  248639.0 |
  248965.7 |
  249292.5 |
  249619.2 |####################
  249946.0 |
  250272.7 |
  250599.5 |
  (0 below, 1 above range)

carrier_pre_leaf_null (n=6, range 75637.9-82697.1 ns)
  75637.9 |####################
  75990.9 |
  76343.8 |
  76696.8 |
  77049.7 |
  77402.7 |
  77755.7 |
  78108.6 |
  78461.6 |
  78814.5 |
  79167.5 |
  79520.5 |
  79873.4 |####################
  80226.4 |
  80579.3 |
  80932.3 |
  81285.3 |
  81638.2 |
  81991.2 |####################
  82344.1 |########################################
  (0 below, 1 above range)

carrier_pre_leaf_regcache (n=6, range 136928.8-161696.2 ns)
  136928.8 |####################
  138167.2 |
  139405.5 |
  140643.9 |
  141882.3 |
  143120.7 |
  144359.0 |
  145597.4 |########################################
  146835.8 |
  148074.2 |####################
  149312.5 |
  150550.9 |####################
  151789.3 |
  153027.6 |
  154266.0 |
  155504.4 |
  156742.8 |
  157981.1 |
  159219.5 |
  160457.9 |
  (0 below, 1 above range)

carrier_pre_leaf_switch (n=6, range 143543.3-150278.8 ns)
  143543.3 |########################################
  143880.1 |
  144216.8 |########################################
  144553.6 |
  144890.4 |
  145227.2 |########################################
  145563.9 |
  145900.7 |
  146237.5 |
  146574.3 |
  146911.0 |
  147247.8 |
  147584.6 |########################################
  147921.3 |
  148258.1 |
  148594.9 |
  148931.7 |
  149268.4 |
  149605.2 |
  149942.0 |########################################
  (0 below, 1 above range)

carrier_pre_leaf_threaded (n=6, range 86543.8-109905.0 ns)
  86543.8 |########################################
  87711.9 |
  88879.9 |
  90048.0 |
  91216.0 |
  92384.1 |########################################
  93552.2 |########################################
  94720.2 |
  95888.3 |
  97056.3 |
  98224.4 |
  99392.5 |
  100560.5 |
  101728.6 |
  102896.6 |
  104064.7 |########################################
  105232.8 |
  106400.8 |
  107568.9 |
  108736.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_leaf_direct**: bridge=209.7% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_fntable**: bridge=103.4% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_null**: bridge=170.9% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_regcache**: bridge=107.6% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_switch**: bridge=110.1% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_threaded**: bridge=119.3% of algo (FFI overhead may distort results)
