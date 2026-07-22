# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, tight profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_tight_none**

## Highlights

Baseline for all deltas below: **carrier_opt_tight_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_tight_all dominates: 1028% faster than the next best (carrier_opt_tight_fold)

carrier_opt_tight_all (631 ns) leads carrier_opt_tight_fold (7.12 us) by 1028%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_tight_all beats baseline by 93% (significant)

carrier_opt_tight_all is -8.10 us (93%) faster than baseline carrier_opt_tight_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_tight_eqsat is an outlier: 36.9x slower than the field

carrier_opt_tight_eqsat (23.27 us) is 36.9x the fastest (631 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_tight_none shows alternating (throttle bounce) (autocorr -0.65)

carrier_opt_tight_none's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_tight_all} vs {carrier_opt_tight_fold, carrier_opt_tight_cse, carrier_opt_tight_dce, carrier_opt_tight_none, carrier_opt_tight_cseeqsat, carrier_opt_tight_eqsat} (1028% apart)

The field splits into a fast tier {carrier_opt_tight_all} and a slow tier {carrier_opt_tight_fold, carrier_opt_tight_cse, carrier_opt_tight_dce, carrier_opt_tight_none, carrier_opt_tight_cseeqsat, carrier_opt_tight_eqsat} with a 1028% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 36.9x the fastest

Fastest carrier_opt_tight_all (631 ns) to slowest carrier_opt_tight_eqsat (23.27 us): 36.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_tight_all** at 631.2 ns median (-92.8% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 36.86x (fastest 631.2 ns, slowest 23267.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_tight_all | 3184ns | 3174ns | 3063ns | 3138ns | 3314ns | -71.65% |
| carrier_opt_tight_cse | 10772ns | 10895ns | 9570ns | 10873ns | 11221ns | -4.10% |
| carrier_opt_tight_cseeqsat | 25631ns | 25619ns | 25400ns | 25578ns | 25826ns | +128.18% |
| carrier_opt_tight_dce | 11249ns | 11244ns | 11090ns | 11194ns | 11411ns | +0.15% |
| carrier_opt_tight_eqsat | 25743ns | 25760ns | 25256ns | 25612ns | 26184ns | +129.18% |
| carrier_opt_tight_fold | 9615ns | 9618ns | 9592ns | 9613ns | 9628ns | -14.41% |
| carrier_opt_tight_none | 11233ns | 11235ns | 11013ns | 11218ns | 11364ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_tight_all | 631ns | 626ns | 634ns | -92.76% | 0.406 |
| carrier_opt_tight_cse | 8270ns | 7326ns | 8639ns | -5.04% | 0.031 |
| carrier_opt_tight_cseeqsat | 23088ns | 22815ns | 23321ns | +165.09% | 0.011 |
| carrier_opt_tight_dce | 8693ns | 8545ns | 8790ns | -0.18% | 0.029 |
| carrier_opt_tight_eqsat | 23245ns | 22832ns | 23594ns | +166.90% | 0.011 |
| carrier_opt_tight_fold | 7109ns | 7038ns | 7138ns | -18.37% | 0.036 |
| carrier_opt_tight_none | 8709ns | 8572ns | 8791ns | base | 0.029 |

## Performance model

- Peak throughput: **0.409 Gops/s** (carrier_opt_tight_all; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_tight_all | 0.406 | 99.2% |
| carrier_opt_tight_cse | 0.031 | 7.5% |
| carrier_opt_tight_cseeqsat | 0.011 | 2.7% |
| carrier_opt_tight_dce | 0.029 | 7.2% |
| carrier_opt_tight_eqsat | 0.011 | 2.7% |
| carrier_opt_tight_fold | 0.036 | 8.8% |
| carrier_opt_tight_none | 0.029 | 7.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_tight_all | 3184ns | 3184ns | -71.65% |
| carrier_opt_tight_cse | 10772ns | 10772ns | -4.10% |
| carrier_opt_tight_cseeqsat | 25631ns | 25631ns | +128.18% |
| carrier_opt_tight_dce | 11249ns | 11249ns | +0.15% |
| carrier_opt_tight_eqsat | 25743ns | 25743ns | +129.18% |
| carrier_opt_tight_fold | 9615ns | 9615ns | -14.41% |
| carrier_opt_tight_none | 11233ns | 11233ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_tight_none | 8726ns | base | --- | [8611, 8791] | --- | --- | --- | --- |
| carrier_opt_tight_all | 631ns | -8095.6ns (-92.8%) | [-8162, -7979]ns | [627, 634] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_tight_cse | 8350ns | -288.6ns (-3.3%) | [-878, -151]ns | [7820, 8639] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_tight_cseeqsat | 23051ns | +14329.1ns (+164.2%) | [+14271, +14534]ns | [22891, 23321] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_tight_dce | 8713ns | no significant difference | [-165, +179]ns | [8577, 8790] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_tight_eqsat | 23268ns | +14590.0ns (+167.2%) | [+14099, +14918]ns | [22873, 23594] | YES | 0.0375 | 0.0313 | 0 |
| carrier_opt_tight_fold | 7118ns | -1606.2ns (-18.4%) | [-1718, -1476]ns | [7072, 7138] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_tight_none | carrier_opt_tight_all | carrier_opt_tight_cse | carrier_opt_tight_cseeqsat | carrier_opt_tight_dce | carrier_opt_tight_eqsat | carrier_opt_tight_fold |
|---|---|---|---|---|---|---|---|
| 1 | 8714ns | -92.8% | -15.9% | +164.6% | -0.7% | +162.0% | -18.1% |
| 2 | 8572ns | -92.6% | -2.8% | +166.2% | +2.3% | +171.9% | -16.9% |
| 3 | 8835ns | -92.8% | -1.5% | +165.0% | -0.7% | +159.4% | -19.5% |
| 4 | 8650ns | -92.7% | -3.9% | +165.5% | +1.8% | +174.6% | -17.4% |
| 5 | 8747ns | -92.8% | -2.0% | +163.5% | -1.6% | +165.5% | -19.5% |
| 6 | 8739ns | -92.7% | -4.2% | +165.9% | -2.2% | +168.2% | -18.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_tight_all | -0.441 | moderate- |
| carrier_opt_tight_cse | 0.028 | ok |
| carrier_opt_tight_cseeqsat | -0.546 | HIGH- (thermal bounce) |
| carrier_opt_tight_dce | 0.262 | moderate+ |
| carrier_opt_tight_eqsat | -0.396 | moderate- |
| carrier_opt_tight_fold | -0.257 | moderate- |
| carrier_opt_tight_none | -0.655 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_opt_tight_all**: won 6/6, lost 0/6
- **carrier_opt_tight_cse**: won 6/6, lost 0/6
- **carrier_opt_tight_cseeqsat**: won 0/6, lost 6/6
- **carrier_opt_tight_dce**: won 4/6, lost 2/6
- **carrier_opt_tight_eqsat**: won 0/6, lost 6/6
- **carrier_opt_tight_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_tight_all | 85103.0ns | 630.6ns | 13495.2% | HIGH |
| carrier_opt_tight_cse | 90474.5ns | 8270.1ns | 1094.0% | HIGH |
| carrier_opt_tight_cseeqsat | 92588.7ns | 23087.7ns | 401.0% | HIGH |
| carrier_opt_tight_dce | 87402.7ns | 8693.5ns | 1005.4% | HIGH |
| carrier_opt_tight_eqsat | 93222.7ns | 23245.1ns | 401.0% | HIGH |
| carrier_opt_tight_fold | 91103.1ns | 7109.4ns | 1281.4% | HIGH |
| carrier_opt_tight_none | 87483.5ns | 8709.4ns | 1004.5% | HIGH |

## Distribution (algo ns)

```
carrier_opt_tight_all (n=6, range 626.2-634.0 ns)
    626.2 |########################################
    626.6 |
    627.0 |########################################
    627.4 |
    627.8 |
    628.1 |
    628.5 |
    628.9 |
    629.3 |
    629.7 |
    630.1 |
    630.5 |########################################
    630.9 |
    631.2 |
    631.6 |########################################
    632.0 |
    632.4 |
    632.8 |
    633.2 |########################################
    633.6 |
  (0 below, 1 above range)

carrier_opt_tight_cse (n=6, range 7325.8-8639.4 ns)
   7325.8 |#############
   7391.5 |
   7457.2 |
   7522.8 |
   7588.5 |
   7654.2 |
   7719.9 |
   7785.6 |
   7851.2 |
   7916.9 |
   7982.6 |
   8048.3 |
   8114.0 |
   8179.6 |
   8245.3 |
   8311.0 |########################################
   8376.7 |
   8442.4 |
   8508.0 |
   8573.7 |#############
  (0 below, 1 above range)

carrier_opt_tight_cseeqsat (n=6, range 22815.0-23320.8 ns)
  22815.0 |####################
  22840.3 |
  22865.6 |
  22890.9 |
  22916.2 |
  22941.5 |
  22966.8 |####################
  22992.0 |
  23017.3 |
  23042.6 |########################################
  23067.9 |
  23093.2 |
  23118.5 |
  23143.8 |
  23169.1 |
  23194.4 |
  23219.7 |####################
  23245.0 |
  23270.3 |
  23295.6 |
  (0 below, 1 above range)

carrier_opt_tight_dce (n=6, range 8545.0-8790.2 ns)
   8545.0 |####################
   8557.3 |
   8569.5 |
   8581.8 |
   8594.0 |
   8606.3 |####################
   8618.6 |
   8630.8 |
   8643.1 |
   8655.3 |####################
   8667.6 |
   8679.9 |
   8692.1 |
   8704.4 |
   8716.6 |
   8728.9 |
   8741.2 |
   8753.4 |
   8765.7 |########################################
   8777.9 |
  (0 below, 1 above range)

carrier_opt_tight_eqsat (n=6, range 22832.5-23594.4 ns)
  22832.5 |########################################
  22870.6 |
  22908.7 |########################################
  22946.8 |
  22984.9 |
  23023.0 |
  23061.1 |
  23099.2 |
  23137.3 |
  23175.4 |
  23213.5 |########################################
  23251.5 |
  23289.6 |########################################
  23327.7 |
  23365.8 |
  23403.9 |########################################
  23442.0 |
  23480.1 |
  23518.2 |
  23556.3 |
  (0 below, 1 above range)

carrier_opt_tight_fold (n=6, range 7038.3-7138.1 ns)
   7038.3 |####################
   7043.3 |
   7048.3 |
   7053.3 |
   7058.3 |
   7063.3 |
   7068.3 |
   7073.2 |
   7078.2 |
   7083.2 |
   7088.2 |
   7093.2 |
   7098.2 |
   7103.2 |########################################
   7108.2 |
   7113.2 |
   7118.2 |
   7123.2 |####################
   7128.2 |
   7133.2 |####################
  (0 below, 1 above range)

carrier_opt_tight_none (n=6, range 8572.1-8790.7 ns)
   8572.1 |####################
   8583.0 |
   8594.0 |
   8604.9 |
   8615.8 |
   8626.7 |
   8637.7 |
   8648.6 |####################
   8659.5 |
   8670.4 |
   8681.4 |
   8692.3 |
   8703.2 |
   8714.2 |####################
   8725.1 |
   8736.0 |########################################
   8746.9 |
   8757.9 |
   8768.8 |
   8779.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_tight_all**: bridge=13461.2% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cse**: bridge=1088.2% of algo (FFI overhead may distort results)
- **carrier_opt_tight_cseeqsat**: bridge=400.9% of algo (FFI overhead may distort results)
- **carrier_opt_tight_dce**: bridge=1003.4% of algo (FFI overhead may distort results)
- **carrier_opt_tight_eqsat**: bridge=401.3% of algo (FFI overhead may distort results)
- **carrier_opt_tight_fold**: bridge=1284.3% of algo (FFI overhead may distort results)
- **carrier_opt_tight_none**: bridge=1004.2% of algo (FFI overhead may distort results)
