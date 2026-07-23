# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, leaf profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_leaf_none**

## Highlights

Baseline for all deltas below: **carrier_opt_leaf_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_leaf_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_leaf_none has the worst median (222.02 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_leaf_all at 40.73 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_leaf_all dominates: 125% faster than the next best (carrier_opt_leaf_canon)

carrier_opt_leaf_all (40.73 us) leads carrier_opt_leaf_canon (91.74 us) by 125%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_leaf_all beats baseline by 81% (significant)

carrier_opt_leaf_all is -180.88 us (81%) faster than baseline carrier_opt_leaf_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_leaf_none is an outlier: 5.5x slower than the field

carrier_opt_leaf_none (222.02 us) is 5.5x the fastest (40.73 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_leaf_all shows alternating (throttle bounce) (autocorr -0.63)

carrier_opt_leaf_all's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_leaf_all} vs {carrier_opt_leaf_canon, carrier_opt_leaf_cse, carrier_opt_leaf_fold, carrier_opt_leaf_dce, carrier_opt_leaf_none} (125% apart)

The field splits into a fast tier {carrier_opt_leaf_all} and a slow tier {carrier_opt_leaf_canon, carrier_opt_leaf_cse, carrier_opt_leaf_fold, carrier_opt_leaf_dce, carrier_opt_leaf_none} with a 125% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.5x the fastest

Fastest carrier_opt_leaf_all (40.73 us) to slowest carrier_opt_leaf_none (222.02 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_leaf_all** at 40731.9 ns median (-81.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 5.45x (fastest 40731.9 ns, slowest 222019.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_leaf_all | 43117ns | 43032ns | 41682ns | 42823ns | 44276ns | -80.77% |
| carrier_opt_leaf_canon | 95383ns | 94247ns | 90085ns | 93695ns | 100565ns | -57.46% |
| carrier_opt_leaf_cse | 96024ns | 95366ns | 94040ns | 95038ns | 98495ns | -57.17% |
| carrier_opt_leaf_dce | 222298ns | 222214ns | 219325ns | 221492ns | 224995ns | -0.85% |
| carrier_opt_leaf_fold | 126811ns | 125847ns | 124579ns | 125729ns | 129551ns | -43.44% |
| carrier_opt_leaf_none | 224204ns | 224507ns | 220562ns | 223204ns | 227524ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_leaf_all | 40746ns | 39400ns | 41832ns | -81.61% | 0.101 |
| carrier_opt_leaf_canon | 92932ns | 87815ns | 97999ns | -58.06% | 0.044 |
| carrier_opt_leaf_cse | 93697ns | 91657ns | 96137ns | -57.72% | 0.044 |
| carrier_opt_leaf_dce | 219790ns | 216955ns | 222342ns | -0.82% | 0.019 |
| carrier_opt_leaf_fold | 124359ns | 122024ns | 126966ns | -43.88% | 0.033 |
| carrier_opt_leaf_none | 221599ns | 218053ns | 224615ns | base | 0.018 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_leaf_all | 498567 | 2169345 | 0.230 | 0.36× |
| carrier_opt_leaf_canon | 571981 | 1403160 | 0.408 | 0.41× |
| carrier_opt_leaf_cse | 586259 | 1454307 | 0.403 | 0.42× |
| carrier_opt_leaf_dce | 1379752 | 4507270 | 0.306 | 0.99× |
| carrier_opt_leaf_fold | 781079 | 4425628 | 0.176 | 0.56× |
| carrier_opt_leaf_none | 1387309 | 4507149 | 0.308 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.104 Gops/s** (carrier_opt_leaf_all; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_leaf_all | 0.101 | 96.7% |
| carrier_opt_leaf_canon | 0.045 | 42.9% |
| carrier_opt_leaf_cse | 0.044 | 42.3% |
| carrier_opt_leaf_dce | 0.019 | 17.9% |
| carrier_opt_leaf_fold | 0.033 | 31.9% |
| carrier_opt_leaf_none | 0.018 | 17.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_leaf_all | 43117ns | 43117ns | -80.77% |
| carrier_opt_leaf_canon | 95383ns | 95383ns | -57.46% |
| carrier_opt_leaf_cse | 96024ns | 96024ns | -57.17% |
| carrier_opt_leaf_dce | 222298ns | 222298ns | -0.85% |
| carrier_opt_leaf_fold | 126811ns | 126811ns | -43.44% |
| carrier_opt_leaf_none | 224204ns | 224204ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_leaf_none | 222019ns | base | --- | [218162, 224615] | --- | --- | --- | --- |
| carrier_opt_leaf_all | 40732ns | -180879.8ns (-81.5%) | [-183536, -178142]ns | [39674, 41832] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_leaf_canon | 91741ns | -130241.6ns (-58.7%) | [-134297, -121462]ns | [89054, 97999] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_leaf_cse | 93081ns | -129207.3ns (-58.2%) | [-131798, -122698]ns | [91874, 96137] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_leaf_dce | 219641ns | no significant difference | [-5848, +2831]ns | [217386, 222342] | no | 0.2188 | 0.2188 | 0 |
| carrier_opt_leaf_fold | 123607ns | -97917.4ns (-44.1%) | [-101660, -92142]ns | [122503, 126966] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_leaf_none | carrier_opt_leaf_all | carrier_opt_leaf_canon | carrier_opt_leaf_cse | carrier_opt_leaf_dce | carrier_opt_leaf_fold |
|---|---|---|---|---|---|---|
| 1 | 224072ns | -81.7% | -59.5% | -58.9% | -3.2% | -44.1% |
| 2 | 223170ns | -82.1% | -60.7% | -58.9% | -2.1% | -44.9% |
| 3 | 220869ns | -80.8% | -55.1% | -57.5% | -0.1% | -43.9% |
| 4 | 218053ns | -81.9% | -55.6% | -54.9% | +2.7% | -41.0% |
| 5 | 218270ns | -81.4% | -58.6% | -57.6% | -0.2% | -43.5% |
| 6 | 225157ns | -81.7% | -58.9% | -58.5% | -1.9% | -45.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_leaf_all | -0.628 | HIGH- (thermal bounce) |
| carrier_opt_leaf_canon | -0.072 | ok |
| carrier_opt_leaf_cse | -0.028 | ok |
| carrier_opt_leaf_dce | -0.127 | ok |
| carrier_opt_leaf_fold | -0.164 | ok |
| carrier_opt_leaf_none | 0.116 | ok |

**Consistency summary:**

- **carrier_opt_leaf_all**: won 6/6, lost 0/6
- **carrier_opt_leaf_canon**: won 6/6, lost 0/6
- **carrier_opt_leaf_cse**: won 6/6, lost 0/6
- **carrier_opt_leaf_dce**: won 4/6, lost 1/6
- **carrier_opt_leaf_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_leaf_all | 119166.6ns | 40746.2ns | 292.5% | HIGH |
| carrier_opt_leaf_canon | 93015.7ns | 92931.5ns | 100.1% | HIGH |
| carrier_opt_leaf_cse | 94026.3ns | 93697.2ns | 100.4% | HIGH |
| carrier_opt_leaf_dce | 220704.4ns | 219789.6ns | 100.4% | HIGH |
| carrier_opt_leaf_fold | 124949.2ns | 124358.5ns | 100.5% | HIGH |
| carrier_opt_leaf_none | 222337.5ns | 221598.6ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_opt_leaf_all (n=6, range 39400.4-41832.5 ns)
  39400.4 |########################################
  39522.0 |
  39643.6 |
  39765.2 |
  39886.8 |########################################
  40008.4 |
  40130.0 |
  40251.6 |
  40373.2 |
  40494.8 |########################################
  40616.4 |
  40738.1 |
  40859.7 |########################################
  40981.3 |
  41102.9 |
  41224.5 |########################################
  41346.1 |
  41467.7 |
  41589.3 |
  41710.9 |
  (0 below, 1 above range)

carrier_opt_leaf_canon (n=6, range 87815.0-97998.8 ns)
  87815.0 |########################################
  88324.2 |
  88833.4 |
  89342.6 |
  89851.8 |########################################
  90360.9 |########################################
  90870.1 |
  91379.3 |
  91888.5 |
  92397.7 |########################################
  92906.9 |
  93416.1 |
  93925.2 |
  94434.4 |
  94943.6 |
  95452.8 |
  95962.0 |
  96471.2 |########################################
  96980.4 |
  97489.6 |
  (0 below, 1 above range)

carrier_opt_leaf_cse (n=6, range 91656.7-96137.3 ns)
  91656.7 |########################################
  91880.7 |########################################
  92104.8 |
  92328.8 |
  92552.8 |########################################
  92776.9 |
  93000.9 |
  93224.9 |
  93448.9 |########################################
  93673.0 |
  93897.0 |########################################
  94121.0 |
  94345.1 |
  94569.1 |
  94793.1 |
  95017.1 |
  95241.2 |
  95465.2 |
  95689.2 |
  95913.3 |
  (0 below, 1 above range)

carrier_opt_leaf_dce (n=6, range 216955.0-222341.7 ns)
  216955.0 |########################################
  217224.3 |
  217493.7 |
  217763.0 |########################################
  218032.3 |
  218301.7 |
  218571.0 |########################################
  218840.3 |
  219109.7 |
  219379.0 |
  219648.3 |
  219917.7 |
  220187.0 |
  220456.3 |########################################
  220725.7 |########################################
  220995.0 |
  221264.3 |
  221533.7 |
  221803.0 |
  222072.3 |
  (0 below, 1 above range)

carrier_opt_leaf_fold (n=6, range 122024.2-126965.6 ns)
  122024.2 |########################################
  122271.3 |
  122518.3 |
  122765.4 |########################################
  123012.5 |
  123259.6 |########################################
  123506.6 |
  123753.7 |########################################
  124000.8 |
  124247.8 |
  124494.9 |
  124742.0 |
  124989.0 |########################################
  125236.1 |
  125483.2 |
  125730.2 |
  125977.3 |
  126224.4 |
  126471.5 |
  126718.5 |
  (0 below, 1 above range)

carrier_opt_leaf_none (n=6, range 218053.3-224614.6 ns)
  218053.3 |########################################
  218381.4 |
  218709.4 |
  219037.5 |
  219365.6 |
  219693.6 |
  220021.7 |
  220349.8 |
  220677.8 |####################
  221005.9 |
  221334.0 |
  221662.0 |
  221990.1 |
  222318.1 |
  222646.2 |
  222974.3 |####################
  223302.3 |
  223630.4 |
  223958.5 |####################
  224286.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_leaf_all**: bridge=292.9% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_canon**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cse**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_dce**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_fold**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_none**: bridge=100.4% of algo (FFI overhead may distort results)
