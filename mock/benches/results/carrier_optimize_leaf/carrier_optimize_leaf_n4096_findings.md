# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, leaf profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_leaf_none**

## Highlights

Baseline for all deltas below: **carrier_opt_leaf_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_leaf_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_leaf_none has the worst median (226.30 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_leaf_all at 43.58 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_leaf_all dominates: 127% faster than the next best (carrier_opt_leaf_cse)

carrier_opt_leaf_all (43.58 us) leads carrier_opt_leaf_cse (98.83 us) by 127%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_leaf_all beats baseline by 81% (significant)

carrier_opt_leaf_all is -182.72 us (81%) faster than baseline carrier_opt_leaf_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_leaf_none is an outlier: 5.2x slower than the field

carrier_opt_leaf_none (226.30 us) is 5.2x the fastest (43.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_leaf_dce shows alternating (throttle bounce) (autocorr -0.56)

carrier_opt_leaf_dce's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_leaf_all} vs {carrier_opt_leaf_cse, carrier_opt_leaf_cseeqsat, carrier_opt_leaf_eqsat, carrier_opt_leaf_fold, carrier_opt_leaf_dce, carrier_opt_leaf_none} (127% apart)

The field splits into a fast tier {carrier_opt_leaf_all} and a slow tier {carrier_opt_leaf_cse, carrier_opt_leaf_cseeqsat, carrier_opt_leaf_eqsat, carrier_opt_leaf_fold, carrier_opt_leaf_dce, carrier_opt_leaf_none} with a 127% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.2x the fastest

Fastest carrier_opt_leaf_all (43.58 us) to slowest carrier_opt_leaf_none (226.30 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_leaf_all** at 43583.5 ns median (-80.7% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 5.19x (fastest 43583.5 ns, slowest 226300.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_leaf_all | 45411ns | 46078ns | 41870ns | 45723ns | 46713ns | -80.12% |
| carrier_opt_leaf_cse | 100153ns | 101407ns | 95657ns | 99506ns | 103371ns | -56.15% |
| carrier_opt_leaf_cseeqsat | 102502ns | 102552ns | 99638ns | 102408ns | 104075ns | -55.12% |
| carrier_opt_leaf_dce | 227576ns | 225405ns | 219535ns | 224755ns | 235826ns | -0.35% |
| carrier_opt_leaf_eqsat | 103576ns | 103525ns | 98833ns | 103045ns | 106744ns | -54.65% |
| carrier_opt_leaf_fold | 131153ns | 130796ns | 124686ns | 130496ns | 135373ns | -42.57% |
| carrier_opt_leaf_none | 228383ns | 229031ns | 221160ns | 227000ns | 234069ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_leaf_all | 42869ns | 39487ns | 43947ns | -80.99% | 0.096 |
| carrier_opt_leaf_cse | 97600ns | 93172ns | 100769ns | -56.73% | 0.042 |
| carrier_opt_leaf_cseeqsat | 99824ns | 97116ns | 101367ns | -55.74% | 0.041 |
| carrier_opt_leaf_dce | 224660ns | 216762ns | 232950ns | -0.39% | 0.018 |
| carrier_opt_leaf_eqsat | 100997ns | 96439ns | 104016ns | -55.22% | 0.041 |
| carrier_opt_leaf_fold | 128420ns | 122081ns | 132591ns | -43.06% | 0.032 |
| carrier_opt_leaf_none | 225542ns | 218425ns | 231090ns | base | 0.018 |

## Performance model

- Peak throughput: **0.104 Gops/s** (carrier_opt_leaf_all; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_leaf_all | 0.094 | 90.6% |
| carrier_opt_leaf_cse | 0.041 | 40.0% |
| carrier_opt_leaf_cseeqsat | 0.041 | 39.6% |
| carrier_opt_leaf_dce | 0.018 | 17.8% |
| carrier_opt_leaf_eqsat | 0.041 | 39.1% |
| carrier_opt_leaf_fold | 0.032 | 30.8% |
| carrier_opt_leaf_none | 0.018 | 17.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_leaf_all | 45411ns | 45411ns | -80.12% |
| carrier_opt_leaf_cse | 100153ns | 100153ns | -56.15% |
| carrier_opt_leaf_cseeqsat | 102502ns | 102502ns | -55.12% |
| carrier_opt_leaf_dce | 227576ns | 227576ns | -0.35% |
| carrier_opt_leaf_eqsat | 103576ns | 103576ns | -54.65% |
| carrier_opt_leaf_fold | 131153ns | 131153ns | -42.57% |
| carrier_opt_leaf_none | 228383ns | 228383ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_leaf_none | 226300ns | base | --- | [219235, 231090] | --- | --- | --- | --- |
| carrier_opt_leaf_all | 43584ns | -182716.9ns (-80.7%) | [-187832, -177470]ns | [41077, 43947] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_cse | 98834ns | -126883.8ns (-56.1%) | [-131390, -125554]ns | [93196, 100769] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_cseeqsat | 99816ns | -124991.0ns (-55.2%) | [-131217, -120945]ns | [98290, 101367] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_dce | 222442ns | no significant difference | [-11701, +10269]ns | [218587, 232950] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_leaf_eqsat | 100948ns | -123591.0ns (-54.6%) | [-130142, -119901]ns | [98028, 104016] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_leaf_fold | 128031ns | -96632.5ns (-42.7%) | [-102803, -91932]ns | [124637, 132591] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_leaf_none | carrier_opt_leaf_all | carrier_opt_leaf_cse | carrier_opt_leaf_cseeqsat | carrier_opt_leaf_dce | carrier_opt_leaf_eqsat | carrier_opt_leaf_fold |
|---|---|---|---|---|---|---|---|
| 1 | 220046ns | -80.0% | -57.7% | -54.8% | +4.9% | -56.2% | -42.2% |
| 2 | 227400ns | -81.2% | -56.0% | -55.3% | -4.7% | -55.1% | -43.4% |
| 3 | 225315ns | -80.7% | -56.3% | -55.2% | +4.3% | -53.7% | -45.8% |
| 4 | 227285ns | -80.8% | -55.4% | -56.2% | -1.9% | -54.4% | -41.6% |
| 5 | 234780ns | -81.3% | -57.7% | -57.4% | -5.4% | -57.5% | -43.6% |
| 6 | 218425ns | -81.9% | -57.3% | -55.5% | +0.9% | -54.4% | -41.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_leaf_all | -0.162 | ok |
| carrier_opt_leaf_cse | -0.113 | ok |
| carrier_opt_leaf_cseeqsat | 0.025 | ok |
| carrier_opt_leaf_dce | -0.564 | HIGH- (thermal bounce) |
| carrier_opt_leaf_eqsat | 0.140 | ok |
| carrier_opt_leaf_fold | -0.206 | moderate- |
| carrier_opt_leaf_none | -0.351 | moderate- |

**Consistency summary:**

- **carrier_opt_leaf_all**: won 6/6, lost 0/6
- **carrier_opt_leaf_cse**: won 6/6, lost 0/6
- **carrier_opt_leaf_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_leaf_dce**: won 3/6, lost 3/6
- **carrier_opt_leaf_eqsat**: won 6/6, lost 0/6
- **carrier_opt_leaf_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_leaf_all | 104412.8ns | 42869.2ns | 243.6% | HIGH |
| carrier_opt_leaf_cse | 97816.0ns | 97599.6ns | 100.2% | HIGH |
| carrier_opt_leaf_cseeqsat | 99929.2ns | 99824.2ns | 100.1% | HIGH |
| carrier_opt_leaf_dce | 225378.9ns | 224659.7ns | 100.3% | HIGH |
| carrier_opt_leaf_eqsat | 101231.2ns | 100997.2ns | 100.2% | HIGH |
| carrier_opt_leaf_fold | 128971.6ns | 128419.5ns | 100.4% | HIGH |
| carrier_opt_leaf_none | 226132.1ns | 225542.0ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_opt_leaf_all (n=6, range 39486.7-43947.3 ns)
  39486.7 |####################
  39709.7 |
  39932.8 |
  40155.8 |
  40378.8 |
  40601.8 |
  40824.9 |
  41047.9 |
  41270.9 |
  41494.0 |
  41717.0 |
  41940.0 |
  42163.1 |
  42386.1 |
  42609.1 |####################
  42832.2 |
  43055.2 |
  43278.2 |
  43501.2 |########################################
  43724.3 |####################
  (0 below, 1 above range)

carrier_opt_leaf_cse (n=6, range 93172.5-100769.1 ns)
  93172.5 |########################################
  93552.3 |
  93932.2 |
  94312.0 |
  94691.8 |
  95071.7 |
  95451.5 |
  95831.3 |
  96211.2 |
  96591.0 |
  96970.8 |
  97350.7 |
  97730.5 |
  98110.3 |####################
  98490.2 |
  98870.0 |####################
  99249.8 |
  99629.7 |
  100009.5 |####################
  100389.3 |
  (0 below, 1 above range)

carrier_opt_leaf_cseeqsat (n=6, range 97116.2-101366.9 ns)
  97116.2 |####################
  97328.7 |
  97541.3 |
  97753.8 |
  97966.3 |
  98178.9 |
  98391.4 |
  98603.9 |
  98816.5 |
  99029.0 |
  99241.5 |
  99454.1 |########################################
  99666.6 |
  99879.2 |
  100091.7 |####################
  100304.2 |
  100516.8 |
  100729.3 |
  100941.8 |####################
  101154.4 |
  (0 below, 1 above range)

carrier_opt_leaf_dce (n=6, range 216762.1-232950.0 ns)
  216762.1 |########################################
  217571.5 |
  218380.9 |
  219190.3 |
  219999.7 |########################################
  220809.1 |
  221618.5 |########################################
  222427.9 |########################################
  223237.3 |
  224046.7 |
  224856.0 |
  225665.4 |
  226474.8 |
  227284.2 |
  228093.6 |
  228903.0 |
  229712.4 |
  230521.8 |########################################
  231331.2 |
  232140.6 |
  (0 below, 1 above range)

carrier_opt_leaf_eqsat (n=6, range 96438.8-104016.2 ns)
  96438.8 |####################
  96817.7 |
  97196.5 |
  97575.4 |
  97954.3 |
  98333.2 |
  98712.0 |
  99090.9 |
  99469.8 |########################################
  99848.7 |
  100227.5 |
  100606.4 |
  100985.3 |
  101364.1 |
  101743.0 |####################
  102121.9 |
  102500.8 |
  102879.6 |
  103258.5 |
  103637.4 |####################
  (0 below, 1 above range)

carrier_opt_leaf_fold (n=6, range 122080.8-132590.6 ns)
  122080.8 |########################################
  122606.3 |
  123131.8 |
  123657.3 |
  124182.8 |
  124708.3 |
  125233.8 |
  125759.2 |
  126284.7 |
  126810.2 |########################################
  127335.7 |########################################
  127861.2 |
  128386.7 |########################################
  128912.2 |
  129437.7 |
  129963.2 |
  130488.7 |
  131014.2 |
  131539.7 |
  132065.2 |########################################
  (0 below, 1 above range)

carrier_opt_leaf_none (n=6, range 218424.6-231090.4 ns)
  218424.6 |########################################
  219057.9 |
  219691.2 |########################################
  220324.5 |
  220957.8 |
  221591.0 |
  222224.3 |
  222857.6 |
  223490.9 |
  224124.2 |
  224757.5 |########################################
  225390.8 |
  226024.1 |
  226657.4 |########################################
  227290.7 |########################################
  227924.0 |
  228557.2 |
  229190.5 |
  229823.8 |
  230457.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_leaf_all**: bridge=231.3% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cse**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cseeqsat**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_dce**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_eqsat**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_fold**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_none**: bridge=100.3% of algo (FFI overhead may distort results)
