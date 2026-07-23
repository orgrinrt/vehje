# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, leaf profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_leaf_none**

## Highlights

Baseline for all deltas below: **carrier_opt_leaf_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_leaf_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_leaf_none has the worst median (8.62 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_leaf_all at 3.49 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_leaf_all beats baseline by 61% (significant)

carrier_opt_leaf_all is -5.28 us (61%) faster than baseline carrier_opt_leaf_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_leaf_none is an outlier: 2.5x slower than the field

carrier_opt_leaf_none (8.62 us) is 2.5x the fastest (3.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_leaf_canon shows alternating (throttle bounce) (autocorr -0.85)

carrier_opt_leaf_canon's per-pass series has lag-1 autocorrelation -0.85, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_leaf_all, carrier_opt_leaf_canon, carrier_opt_leaf_cse} vs {carrier_opt_leaf_dce, carrier_opt_leaf_fold, carrier_opt_leaf_none} (121% apart)

The field splits into a fast tier {carrier_opt_leaf_all, carrier_opt_leaf_canon, carrier_opt_leaf_cse} and a slow tier {carrier_opt_leaf_dce, carrier_opt_leaf_fold, carrier_opt_leaf_none} with a 121% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_opt_leaf_all** at 3493.5 ns median (-59.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.47x (fastest 3493.5 ns, slowest 8622.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_leaf_all | 5807ns | 5892ns | 5421ns | 5742ns | 6097ns | -47.00% |
| carrier_opt_leaf_canon | 6078ns | 6072ns | 5678ns | 5952ns | 6466ns | -44.53% |
| carrier_opt_leaf_cse | 6052ns | 6087ns | 5679ns | 5965ns | 6368ns | -44.77% |
| carrier_opt_leaf_dce | 10758ns | 10656ns | 10000ns | 10453ns | 11594ns | -1.81% |
| carrier_opt_leaf_fold | 10708ns | 10907ns | 9842ns | 10632ns | 11254ns | -2.27% |
| carrier_opt_leaf_none | 10956ns | 11094ns | 9887ns | 10847ns | 11654ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_leaf_all | 3422ns | 3196ns | 3559ns | -59.97% | 0.075 |
| carrier_opt_leaf_canon | 3749ns | 3525ns | 3982ns | -56.14% | 0.068 |
| carrier_opt_leaf_cse | 3709ns | 3462ns | 3924ns | -56.61% | 0.069 |
| carrier_opt_leaf_dce | 8369ns | 7764ns | 9042ns | -2.10% | 0.031 |
| carrier_opt_leaf_fold | 8305ns | 7686ns | 8707ns | -2.85% | 0.031 |
| carrier_opt_leaf_none | 8549ns | 7703ns | 9220ns | base | 0.030 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_leaf_all | 270817 | 1326576 | 0.204 | 0.92× |
| carrier_opt_leaf_canon | 276172 | 1441162 | 0.192 | 0.94× |
| carrier_opt_leaf_cse | 278906 | 1453434 | 0.192 | 0.95× |
| carrier_opt_leaf_dce | 295670 | 1676430 | 0.176 | 1.01× |
| carrier_opt_leaf_fold | 288140 | 1642682 | 0.175 | 0.98× |
| carrier_opt_leaf_none | 293557 | 1643290 | 0.179 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.080 Gops/s** (carrier_opt_leaf_all; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_leaf_all | 0.073 | 91.5% |
| carrier_opt_leaf_canon | 0.068 | 85.5% |
| carrier_opt_leaf_cse | 0.068 | 85.4% |
| carrier_opt_leaf_dce | 0.031 | 38.6% |
| carrier_opt_leaf_fold | 0.030 | 37.7% |
| carrier_opt_leaf_none | 0.030 | 37.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_leaf_all | 5807ns | 5807ns | -47.00% |
| carrier_opt_leaf_canon | 6078ns | 6078ns | -44.53% |
| carrier_opt_leaf_cse | 6052ns | 6052ns | -44.77% |
| carrier_opt_leaf_dce | 10758ns | 10758ns | -1.81% |
| carrier_opt_leaf_fold | 10708ns | 10708ns | -2.27% |
| carrier_opt_leaf_none | 10956ns | 10956ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_leaf_none | 8622ns | base | --- | [7804, 9220] | --- | --- | --- | --- |
| carrier_opt_leaf_all | 3494ns | -5277.6ns (-61.2%) | [-5661, -4440]ns | [3215, 3559] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_leaf_canon | 3739ns | -4670.9ns (-54.2%) | [-5460, -4268]ns | [3526, 3982] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_leaf_cse | 3742ns | -4698.8ns (-54.5%) | [-5560, -4259]ns | [3463, 3924] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_leaf_dce | 8272ns | no significant difference | [-1077, +606]ns | [7793, 9042] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_leaf_fold | 8471ns | no significant difference | [-930, +433]ns | [7738, 8707] | no | 0.8594 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_leaf_none | carrier_opt_leaf_all | carrier_opt_leaf_canon | carrier_opt_leaf_cse | carrier_opt_leaf_dce | carrier_opt_leaf_fold |
|---|---|---|---|---|---|---|
| 1 | 7904ns | -55.8% | -55.4% | -56.2% | +10.2% | +9.9% |
| 2 | 8620ns | -62.9% | -54.0% | -54.2% | -9.3% | -10.8% |
| 3 | 9189ns | -61.6% | -61.6% | -62.3% | -14.8% | -5.0% |
| 4 | 9252ns | -61.2% | -56.8% | -58.3% | -2.1% | -10.0% |
| 5 | 7703ns | -58.0% | -54.0% | -52.9% | +0.8% | +1.1% |
| 6 | 8624ns | -59.5% | -54.4% | -54.7% | +4.7% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_leaf_all | -0.495 | moderate- |
| carrier_opt_leaf_canon | -0.848 | HIGH- (thermal bounce) |
| carrier_opt_leaf_cse | -0.745 | HIGH- (thermal bounce) |
| carrier_opt_leaf_dce | -0.545 | HIGH- (thermal bounce) |
| carrier_opt_leaf_fold | -0.616 | HIGH- (thermal bounce) |
| carrier_opt_leaf_none | -0.102 | ok |

**Consistency summary:**

- **carrier_opt_leaf_all**: won 6/6, lost 0/6
- **carrier_opt_leaf_canon**: won 6/6, lost 0/6
- **carrier_opt_leaf_cse**: won 6/6, lost 0/6
- **carrier_opt_leaf_dce**: won 3/6, lost 3/6
- **carrier_opt_leaf_fold**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_leaf_all | 86994.9ns | 3422.5ns | 2541.9% | HIGH |
| carrier_opt_leaf_canon | 87376.4ns | 3749.1ns | 2330.6% | HIGH |
| carrier_opt_leaf_cse | 87290.5ns | 3709.4ns | 2353.2% | HIGH |
| carrier_opt_leaf_dce | 89426.7ns | 8369.2ns | 1068.5% | HIGH |
| carrier_opt_leaf_fold | 88479.0ns | 8305.3ns | 1065.3% | HIGH |
| carrier_opt_leaf_none | 89163.8ns | 8548.8ns | 1043.0% | HIGH |

## Distribution (algo ns)

```
carrier_opt_leaf_all (n=6, range 3196.2-3559.2 ns)
   3196.2 |####################
   3214.3 |
   3232.5 |####################
   3250.6 |
   3268.8 |
   3286.9 |
   3305.1 |
   3323.2 |
   3341.4 |
   3359.5 |
   3377.7 |
   3395.8 |
   3414.0 |
   3432.1 |
   3450.3 |
   3468.4 |
   3486.6 |########################################
   3504.7 |
   3522.9 |####################
   3541.0 |
  (0 below, 1 above range)

carrier_opt_leaf_canon (n=6, range 3525.4-3981.7 ns)
   3525.4 |########################################
   3548.2 |
   3571.0 |
   3593.8 |
   3616.7 |
   3639.5 |
   3662.3 |
   3685.1 |
   3707.9 |
   3730.7 |
   3753.5 |
   3776.3 |
   3799.2 |
   3822.0 |
   3844.8 |
   3867.6 |
   3890.4 |
   3913.2 |#############
   3936.0 |
   3958.8 |#############
  (0 below, 1 above range)

carrier_opt_leaf_cse (n=6, range 3462.5-3923.6 ns)
   3462.5 |########################################
   3485.6 |
   3508.6 |
   3531.7 |
   3554.7 |
   3577.8 |
   3600.8 |
   3623.9 |####################
   3646.9 |
   3670.0 |
   3693.0 |
   3716.1 |
   3739.1 |
   3762.2 |
   3785.2 |
   3808.3 |
   3831.3 |
   3854.4 |####################
   3877.4 |
   3900.5 |####################
  (0 below, 1 above range)

carrier_opt_leaf_dce (n=6, range 7764.2-9041.7 ns)
   7764.2 |########################################
   7828.1 |####################
   7891.9 |
   7955.8 |
   8019.7 |
   8083.6 |
   8147.4 |
   8211.3 |
   8275.2 |
   8339.1 |
   8403.0 |
   8466.8 |
   8530.7 |
   8594.6 |
   8658.5 |####################
   8722.3 |
   8786.2 |
   8850.1 |
   8914.0 |
   8977.8 |####################
  (0 below, 1 above range)

carrier_opt_leaf_fold (n=6, range 7686.2-8707.1 ns)
   7686.2 |########################################
   7737.2 |
   7788.3 |########################################
   7839.3 |
   7890.4 |
   7941.4 |
   7992.5 |
   8043.5 |
   8094.6 |
   8145.6 |
   8196.6 |
   8247.7 |
   8298.7 |########################################
   8349.8 |
   8400.8 |
   8451.9 |
   8502.9 |
   8554.0 |
   8605.0 |########################################
   8656.1 |########################################
  (0 below, 1 above range)

carrier_opt_leaf_none (n=6, range 7703.3-9220.5 ns)
   7703.3 |####################
   7779.2 |
   7855.0 |####################
   7930.9 |
   8006.7 |
   8082.6 |
   8158.4 |
   8234.3 |
   8310.2 |
   8386.0 |
   8461.9 |
   8537.7 |
   8613.6 |########################################
   8689.4 |
   8765.3 |
   8841.2 |
   8917.0 |
   8992.9 |
   9068.7 |
   9144.6 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_leaf_all**: bridge=2491.2% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_canon**: bridge=2338.5% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cse**: bridge=2332.4% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_dce**: bridge=1089.3% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_fold**: bridge=1033.6% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_none**: bridge=1039.1% of algo (FFI overhead may distort results)
