# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, tight profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_tight_none**

## Highlights

Baseline for all deltas below: **carrier_opt_tight_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_tight_all dominates: 1077% faster than the next best (carrier_opt_tight_fold)

carrier_opt_tight_all (605 ns) leads carrier_opt_tight_fold (7.12 us) by 1077%, a clear separation rather than a photo finish. CV 5.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_tight_all beats baseline by 93% (significant)

carrier_opt_tight_all is -7.54 us (93%) faster than baseline carrier_opt_tight_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_tight_dce is an outlier: 14.2x slower than the field

carrier_opt_tight_dce (8.61 us) is 14.2x the fastest (605 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_tight_dce shows alternating (throttle bounce) (autocorr -0.58)

carrier_opt_tight_dce's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_tight_all} vs {carrier_opt_tight_fold, carrier_opt_tight_cse, carrier_opt_tight_canon, carrier_opt_tight_none, carrier_opt_tight_dce} (1077% apart)

The field splits into a fast tier {carrier_opt_tight_all} and a slow tier {carrier_opt_tight_fold, carrier_opt_tight_cse, carrier_opt_tight_canon, carrier_opt_tight_none, carrier_opt_tight_dce} with a 1077% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 14.2x the fastest

Fastest carrier_opt_tight_all (605 ns) to slowest carrier_opt_tight_dce (8.61 us): 14.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_opt_tight_dce's edge over baseline is significant but tiny (11 ns, 0.13%)

carrier_opt_tight_dce differs from baseline carrier_opt_tight_none by 11 ns (0.13%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_tight_all** at 605.0 ns median (-92.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 14.22x (fastest 605.0 ns, slowest 8605.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_tight_all | 2923ns | 2917ns | 2686ns | 2877ns | 3111ns | -72.20% |
| carrier_opt_tight_canon | 10259ns | 10408ns | 9647ns | 10301ns | 10503ns | -2.41% |
| carrier_opt_tight_cse | 10406ns | 10318ns | 9520ns | 10098ns | 11311ns | -1.02% |
| carrier_opt_tight_dce | 10765ns | 11095ns | 9800ns | 10725ns | 11309ns | +2.40% |
| carrier_opt_tight_fold | 9488ns | 9607ns | 8741ns | 9528ns | 9801ns | -9.75% |
| carrier_opt_tight_none | 10513ns | 10434ns | 9878ns | 10258ns | 11214ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_tight_all | 603ns | 554ns | 636ns | -92.64% | 0.424 |
| carrier_opt_tight_canon | 7864ns | 7347ns | 8066ns | -4.06% | 0.033 |
| carrier_opt_tight_cse | 7978ns | 7301ns | 8647ns | -2.66% | 0.032 |
| carrier_opt_tight_dce | 8345ns | 7677ns | 8739ns | +1.81% | 0.031 |
| carrier_opt_tight_fold | 7016ns | 6509ns | 7202ns | -14.40% | 0.036 |
| carrier_opt_tight_none | 8196ns | 7700ns | 8732ns | base | 0.031 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_tight_all | 258881 | 1627658 | 0.159 | 0.88× |
| carrier_opt_tight_canon | 288944 | 1584748 | 0.182 | 0.98× |
| carrier_opt_tight_cse | 300420 | 1669235 | 0.180 | 1.02× |
| carrier_opt_tight_dce | 291004 | 1591560 | 0.183 | 0.99× |
| carrier_opt_tight_fold | 284905 | 1745113 | 0.163 | 0.97× |
| carrier_opt_tight_none | 295019 | 1619114 | 0.182 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.462 Gops/s** (carrier_opt_tight_all; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_tight_all | 0.423 | 91.5% |
| carrier_opt_tight_canon | 0.032 | 7.0% |
| carrier_opt_tight_cse | 0.032 | 7.0% |
| carrier_opt_tight_dce | 0.030 | 6.4% |
| carrier_opt_tight_fold | 0.036 | 7.8% |
| carrier_opt_tight_none | 0.031 | 6.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_tight_all | 2923ns | 2923ns | -72.20% |
| carrier_opt_tight_canon | 10259ns | 10259ns | -2.41% |
| carrier_opt_tight_cse | 10406ns | 10406ns | -1.02% |
| carrier_opt_tight_dce | 10765ns | 10765ns | +2.40% |
| carrier_opt_tight_fold | 9488ns | 9488ns | -9.75% |
| carrier_opt_tight_none | 10513ns | 10513ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_tight_none | 8141ns | base | --- | [7716, 8732] | --- | --- | --- | --- |
| carrier_opt_tight_all | 605ns | -7539.4ns (-92.6%) | [-8128, -7112]ns | [568, 636] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_opt_tight_canon | 7945ns | no significant difference | [-771, +257]ns | [7580, 8066] | no | 0.8594 | 0.6875 | 0 |
| carrier_opt_tight_cse | 7916ns | no significant difference | [-944, +526]ns | [7371, 8647] | no | 0.8594 | 0.6875 | 0 |
| carrier_opt_tight_dce | 8606ns | no significant difference | [-29, +464]ns | [7691, 8739] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_tight_fold | 7124ns | -1300.2ns (-16.0%) | [-1602, -639]ns | [6722, 7202] | YES (adj: no) | 0.0781 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_tight_none | carrier_opt_tight_all | carrier_opt_tight_canon | carrier_opt_tight_cse | carrier_opt_tight_dce | carrier_opt_tight_fold |
|---|---|---|---|---|---|---|
| 1 | 8662ns | -93.3% | -8.4% | -15.7% | +0.5% | -18.3% |
| 2 | 7731ns | -92.8% | -5.0% | -3.7% | -0.3% | -6.6% |
| 3 | 8802ns | -92.9% | -9.2% | -6.0% | -0.4% | -18.4% |
| 4 | 7740ns | -92.4% | +5.2% | -2.4% | +10.4% | -15.9% |
| 5 | 7700ns | -91.8% | +1.5% | +10.7% | -0.3% | -9.9% |
| 6 | 8542ns | -92.5% | -6.8% | +2.6% | +1.4% | -16.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_tight_all | 0.002 | ok |
| carrier_opt_tight_canon | -0.224 | moderate- |
| carrier_opt_tight_cse | 0.145 | ok |
| carrier_opt_tight_dce | -0.585 | HIGH- (thermal bounce) |
| carrier_opt_tight_fold | -0.034 | ok |
| carrier_opt_tight_none | -0.524 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_opt_tight_all**: won 6/6, lost 0/6
- **carrier_opt_tight_canon**: won 4/6, lost 2/6
- **carrier_opt_tight_cse**: won 4/6, lost 2/6
- **carrier_opt_tight_dce**: won 3/6, lost 3/6
- **carrier_opt_tight_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_tight_all | 84403.5ns | 603.2ns | 13991.5% | HIGH |
| carrier_opt_tight_canon | 88657.1ns | 7863.8ns | 1127.4% | HIGH |
| carrier_opt_tight_cse | 91056.5ns | 7978.2ns | 1141.3% | HIGH |
| carrier_opt_tight_dce | 88848.7ns | 8345.0ns | 1064.7% | HIGH |
| carrier_opt_tight_fold | 91348.0ns | 7016.0ns | 1302.0% | HIGH |
| carrier_opt_tight_none | 88895.5ns | 8196.3ns | 1084.6% | HIGH |

## Distribution (algo ns)

```
carrier_opt_tight_all (n=6, range 553.7-636.2 ns)
    553.7 |####################
    557.8 |
    562.0 |
    566.1 |
    570.2 |
    574.3 |
    578.5 |
    582.6 |########################################
    586.7 |
    590.8 |
    595.0 |
    599.1 |
    603.2 |
    607.4 |
    611.5 |
    615.6 |
    619.7 |
    623.9 |####################
    628.0 |####################
    632.1 |
  (0 below, 1 above range)

carrier_opt_tight_canon (n=6, range 7347.1-8065.8 ns)
   7347.1 |####################
   7383.0 |
   7419.0 |
   7454.9 |
   7490.8 |
   7526.8 |
   7562.7 |
   7598.6 |
   7634.6 |
   7670.5 |
   7706.5 |
   7742.4 |
   7778.3 |####################
   7814.3 |
   7850.2 |
   7886.1 |
   7922.1 |####################
   7958.0 |########################################
   7993.9 |
   8029.9 |
  (0 below, 1 above range)

carrier_opt_tight_cse (n=6, range 7300.8-8647.3 ns)
   7300.8 |########################################
   7368.1 |
   7435.4 |########################################
   7502.8 |########################################
   7570.1 |
   7637.4 |
   7704.8 |
   7772.1 |
   7839.4 |
   7906.7 |
   7974.0 |
   8041.4 |
   8108.7 |
   8176.0 |
   8243.4 |########################################
   8310.7 |
   8378.0 |
   8445.3 |
   8512.6 |########################################
   8580.0 |
  (0 below, 1 above range)

carrier_opt_tight_dce (n=6, range 7677.1-8738.5 ns)
   7677.1 |########################################
   7730.2 |
   7783.2 |
   7836.3 |
   7889.4 |
   7942.5 |
   7995.5 |
   8048.6 |
   8101.7 |
   8154.8 |
   8207.8 |
   8260.9 |
   8314.0 |
   8367.0 |
   8420.1 |
   8473.2 |
   8526.3 |####################
   8579.3 |
   8632.4 |####################
   8685.5 |####################
  (0 below, 1 above range)

carrier_opt_tight_fold (n=6, range 6508.8-7201.6 ns)
   6508.8 |####################
   6543.4 |
   6578.1 |
   6612.7 |
   6647.4 |
   6682.0 |
   6716.7 |
   6751.3 |
   6785.9 |
   6820.6 |
   6855.2 |
   6889.9 |
   6924.5 |####################
   6959.2 |
   6993.8 |
   7028.4 |
   7063.1 |####################
   7097.7 |
   7132.4 |
   7167.0 |########################################
  (0 below, 1 above range)

carrier_opt_tight_none (n=6, range 7700.4-8732.1 ns)
   7700.4 |########################################
   7752.0 |
   7803.6 |
   7855.2 |
   7906.7 |
   7958.3 |
   8009.9 |
   8061.5 |
   8113.1 |
   8164.7 |
   8216.2 |
   8267.8 |
   8319.4 |
   8371.0 |
   8422.6 |
   8474.2 |
   8525.8 |#############
   8577.3 |
   8628.9 |#############
   8680.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_tight_all**: bridge=13956.1% of algo (FFI overhead may distort results)
- **carrier_opt_tight_canon**: bridge=1119.1% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cse**: bridge=1144.6% of algo (FFI overhead may distort results)
- **carrier_opt_tight_dce**: bridge=1026.6% of algo (FFI overhead may distort results)
- **carrier_opt_tight_fold**: bridge=1285.5% of algo (FFI overhead may distort results)
- **carrier_opt_tight_none**: bridge=1091.4% of algo (FFI overhead may distort results)
