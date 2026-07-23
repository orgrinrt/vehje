# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, leaf profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_leaf_none**

## Highlights

Baseline for all deltas below: **carrier_opt_leaf_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_leaf_all dominates: 101% faster than the next best (carrier_opt_leaf_canon)

carrier_opt_leaf_all (150.50 us) leads carrier_opt_leaf_canon (302.84 us) by 101%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_leaf_all beats baseline by 85% (significant)

carrier_opt_leaf_all is -882.99 us (85%) faster than baseline carrier_opt_leaf_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_leaf_dce is an outlier: 6.9x slower than the field

carrier_opt_leaf_dce (1.04 ms) is 6.9x the fastest (150.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_leaf_cse shows alternating (throttle bounce) (autocorr -0.53)

carrier_opt_leaf_cse's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_leaf_all, carrier_opt_leaf_canon, carrier_opt_leaf_cse, carrier_opt_leaf_fold} vs {carrier_opt_leaf_none, carrier_opt_leaf_dce} (109% apart)

The field splits into a fast tier {carrier_opt_leaf_all, carrier_opt_leaf_canon, carrier_opt_leaf_cse, carrier_opt_leaf_fold} and a slow tier {carrier_opt_leaf_none, carrier_opt_leaf_dce} with a 109% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.9x the fastest

Fastest carrier_opt_leaf_all (150.50 us) to slowest carrier_opt_leaf_dce (1.04 ms): 6.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_leaf_all** at 150503.8 ns median (-85.4% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 6.89x (fastest 150503.8 ns, slowest 1036240.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_leaf_all | 152762ns | 153132ns | 150285ns | 152845ns | 153875ns | -85.27% |
| carrier_opt_leaf_canon | 305684ns | 305455ns | 301834ns | 304513ns | 309366ns | -70.53% |
| carrier_opt_leaf_cse | 347085ns | 346515ns | 343823ns | 345768ns | 350692ns | -66.54% |
| carrier_opt_leaf_dce | 1038770ns | 1039278ns | 1029481ns | 1036346ns | 1047050ns | +0.14% |
| carrier_opt_leaf_fold | 497524ns | 496475ns | 493602ns | 496242ns | 501407ns | -52.04% |
| carrier_opt_leaf_none | 1037348ns | 1036023ns | 1026401ns | 1035527ns | 1045554ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_leaf_all | 150244ns | 147935ns | 151303ns | -85.47% | 0.109 |
| carrier_opt_leaf_canon | 303045ns | 299188ns | 306856ns | -70.70% | 0.054 |
| carrier_opt_leaf_cse | 344433ns | 341125ns | 347909ns | -66.70% | 0.048 |
| carrier_opt_leaf_dce | 1035622ns | 1026039ns | 1044208ns | +0.14% | 0.016 |
| carrier_opt_leaf_fold | 494584ns | 490526ns | 498610ns | -52.18% | 0.033 |
| carrier_opt_leaf_none | 1034210ns | 1023027ns | 1042543ns | base | 0.016 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_leaf_all | 945545 | 3943901 | 0.240 | 0.15× |
| carrier_opt_leaf_canon | 1890977 | 4633163 | 0.408 | 0.29× |
| carrier_opt_leaf_cse | 2154665 | 5044290 | 0.427 | 0.33× |
| carrier_opt_leaf_dce | 6438140 | 18004550 | 0.358 | 1.00× |
| carrier_opt_leaf_fold | 3101108 | 17689170 | 0.175 | 0.48× |
| carrier_opt_leaf_none | 6437654 | 18004732 | 0.358 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.111 Gops/s** (carrier_opt_leaf_all; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_leaf_all | 0.109 | 98.3% |
| carrier_opt_leaf_canon | 0.054 | 48.8% |
| carrier_opt_leaf_cse | 0.048 | 43.0% |
| carrier_opt_leaf_dce | 0.016 | 14.3% |
| carrier_opt_leaf_fold | 0.033 | 30.0% |
| carrier_opt_leaf_none | 0.016 | 14.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_leaf_all | 152762ns | 152762ns | -85.27% |
| carrier_opt_leaf_canon | 305684ns | 305684ns | -70.53% |
| carrier_opt_leaf_cse | 347085ns | 347085ns | -66.54% |
| carrier_opt_leaf_dce | 1038770ns | 1038770ns | +0.14% |
| carrier_opt_leaf_fold | 497524ns | 497524ns | -52.04% |
| carrier_opt_leaf_none | 1037348ns | 1037348ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_leaf_none | 1032823ns | base | --- | [1027264, 1042543] | --- | --- | --- | --- |
| carrier_opt_leaf_all | 150504ns | -882994.4ns (-85.5%) | [-891699, -877204]ns | [148926, 151303] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_leaf_canon | 302842ns | -729263.7ns (-70.6%) | [-743107, -721125]ns | [299436, 306856] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_leaf_cse | 344100ns | -689074.4ns (-66.7%) | [-700364, -679894]ns | [341289, 347909] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_leaf_dce | 1036240ns | no significant difference | [-16123, +14491]ns | [1026419, 1044208] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_leaf_fold | 493490ns | -538407.3ns (-52.1%) | [-548076, -532395]ns | [491652, 498610] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_leaf_none | carrier_opt_leaf_all | carrier_opt_leaf_canon | carrier_opt_leaf_cse | carrier_opt_leaf_dce | carrier_opt_leaf_fold |
|---|---|---|---|---|---|---|
| 1 | 1036012ns | -85.5% | -71.1% | -66.9% | -0.9% | -51.7% |
| 2 | 1031586ns | -85.5% | -70.9% | -66.1% | +0.0% | -51.8% |
| 3 | 1034060ns | -85.3% | -70.3% | -67.0% | +1.1% | -52.3% |
| 4 | 1023027ns | -85.3% | -70.1% | -66.3% | +1.7% | -51.8% |
| 5 | 1031502ns | -85.7% | -70.3% | -66.5% | +1.1% | -52.4% |
| 6 | 1049074ns | -85.6% | -71.5% | -67.5% | -2.2% | -52.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_leaf_all | -0.286 | moderate- |
| carrier_opt_leaf_canon | 0.081 | ok |
| carrier_opt_leaf_cse | -0.535 | HIGH- (thermal bounce) |
| carrier_opt_leaf_dce | 0.026 | ok |
| carrier_opt_leaf_fold | 0.361 | moderate+ |
| carrier_opt_leaf_none | -0.035 | ok |

**Consistency summary:**

- **carrier_opt_leaf_all**: won 6/6, lost 0/6
- **carrier_opt_leaf_canon**: won 6/6, lost 0/6
- **carrier_opt_leaf_cse**: won 6/6, lost 0/6
- **carrier_opt_leaf_dce**: won 2/6, lost 3/6
- **carrier_opt_leaf_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_leaf_all | 150741.8ns | 150244.3ns | 100.3% | HIGH |
| carrier_opt_leaf_canon | 303501.5ns | 303044.6ns | 100.2% | HIGH |
| carrier_opt_leaf_cse | 345056.5ns | 344432.6ns | 100.2% | HIGH |
| carrier_opt_leaf_dce | 1036061.5ns | 1035622.4ns | 100.0% | HIGH |
| carrier_opt_leaf_fold | 496639.2ns | 494583.9ns | 100.4% | HIGH |
| carrier_opt_leaf_none | 1035311.5ns | 1034210.0ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_opt_leaf_all (n=6, range 147935.0-151303.3 ns)
  147935.0 |########################################
  148103.4 |
  148271.8 |
  148440.3 |
  148608.7 |
  148777.1 |
  148945.5 |
  149113.9 |
  149282.3 |
  149450.8 |
  149619.2 |
  149787.6 |########################################
  149956.0 |
  150124.4 |########################################
  150292.8 |
  150461.3 |
  150629.7 |########################################
  150798.1 |
  150966.5 |########################################
  151134.9 |
  (0 below, 1 above range)

carrier_opt_leaf_canon (n=6, range 299187.9-306856.2 ns)
  299187.9 |####################
  299571.3 |########################################
  299954.7 |
  300338.2 |
  300721.6 |
  301105.0 |
  301488.4 |
  301871.8 |
  302255.2 |
  302638.7 |
  303022.1 |
  303405.5 |
  303788.9 |
  304172.3 |
  304555.7 |
  304939.2 |
  305322.6 |
  305706.0 |####################
  306089.4 |####################
  306472.8 |
  (0 below, 1 above range)

carrier_opt_leaf_cse (n=6, range 341125.0-347908.5 ns)
  341125.0 |########################################
  341464.2 |
  341803.4 |
  342142.5 |
  342481.7 |
  342820.9 |
  343160.1 |####################
  343499.2 |
  343838.4 |
  344177.6 |
  344516.8 |
  344856.0 |####################
  345195.1 |
  345534.3 |
  345873.5 |####################
  346212.7 |
  346551.8 |
  346891.0 |
  347230.2 |
  347569.4 |
  (0 below, 1 above range)

carrier_opt_leaf_dce (n=6, range 1026039.2-1044207.7 ns)
  1026039.2 |########################################
  1026947.6 |
  1027856.0 |
  1028764.5 |
  1029672.9 |
  1030581.3 |
  1031489.8 |####################
  1032398.2 |
  1033306.6 |
  1034215.0 |
  1035123.4 |
  1036031.9 |
  1036940.3 |
  1037848.7 |
  1038757.1 |
  1039665.6 |####################
  1040574.0 |
  1041482.4 |
  1042390.8 |####################
  1043299.3 |
  (0 below, 1 above range)

carrier_opt_leaf_fold (n=6, range 490526.2-498609.6 ns)
  490526.2 |####################
  490930.4 |
  491334.5 |
  491738.7 |
  492142.9 |
  492547.0 |########################################
  492951.2 |
  493355.4 |
  493759.5 |####################
  494163.7 |
  494567.9 |
  494972.0 |
  495376.2 |
  495780.4 |
  496184.5 |
  496588.7 |
  496992.9 |####################
  497397.0 |
  497801.2 |
  498205.4 |
  (0 below, 1 above range)

carrier_opt_leaf_none (n=6, range 1023026.7-1042542.8 ns)
  1023026.7 |####################
  1024002.5 |
  1024978.3 |
  1025954.1 |
  1026929.9 |
  1027905.7 |
  1028881.5 |
  1029857.3 |
  1030833.1 |########################################
  1031808.9 |
  1032784.7 |
  1033760.5 |####################
  1034736.3 |
  1035712.1 |####################
  1036687.9 |
  1037663.7 |
  1038639.5 |
  1039615.3 |
  1040591.1 |
  1041566.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_leaf_all**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_canon**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cse**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_dce**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_fold**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_none**: bridge=100.1% of algo (FFI overhead may distort results)
