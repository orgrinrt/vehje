# Match lowering: hot-first if-chain vs the rest, K=64 arms, 90% hit one arm

4 variants, 6 samples per variant.
Baseline: **ml_jumptable_h64**

## Highlights

Baseline for all deltas below: **ml_jumptable_h64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_h64 is an outlier: 7.7x slower than the field

ml_tree_h64 (7.75 ms) is 7.7x the fastest (1.01 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ml_hotfirst_h64, ml_ifchain_h64) are a dead heat (<1%)

ml_hotfirst_h64 (1.01 ms) and ml_ifchain_h64 (1.01 ms) differ by 0.11%, inside the noise, even though the wider field spreads 666.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {ml_hotfirst_h64, ml_ifchain_h64, ml_jumptable_h64} vs {ml_tree_h64} (662% apart)

The field splits into a fast tier {ml_hotfirst_h64, ml_ifchain_h64, ml_jumptable_h64} and a slow tier {ml_tree_h64} with a 662% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.7x the fastest

Fastest ml_hotfirst_h64 (1.01 ms) to slowest ml_tree_h64 (7.75 ms): 7.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ml_hotfirst_h64** at 1011119.2 ns median (-0.6% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 7.67x (fastest 1011119.2 ns, slowest 7750590.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_hotfirst_h64 | 1014672ns | 1014567ns | 999871ns | 1013238ns | 1024224ns | -0.59% |
| ml_ifchain_h64 | 1016929ns | 1015518ns | 1004144ns | 1014070ns | 1027608ns | -0.37% |
| ml_jumptable_h64 | 1020685ns | 1020795ns | 1008805ns | 1019989ns | 1027669ns | base |
| ml_tree_h64 | 7758174ns | 7754471ns | 7742750ns | 7752524ns | 7774361ns | +660.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_hotfirst_h64 | 1011530ns | 996704ns | 1021416ns | -0.57% | 0.016 |
| ml_ifchain_h64 | 1013964ns | 1001300ns | 1025105ns | -0.33% | 0.016 |
| ml_jumptable_h64 | 1017365ns | 1005204ns | 1024630ns | base | 0.016 |
| ml_tree_h64 | 7754114ns | 7738628ns | 7770199ns | +662.18% | 0.002 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_hotfirst_h64; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_hotfirst_h64 | 0.016 | 98.6% |
| ml_ifchain_h64 | 0.016 | 98.5% |
| ml_jumptable_h64 | 0.016 | 98.0% |
| ml_tree_h64 | 0.002 | 12.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_hotfirst_h64 | 1014672ns | 1014672ns | -0.59% |
| ml_ifchain_h64 | 1016929ns | 1016929ns | -0.37% |
| ml_jumptable_h64 | 1020685ns | 1020685ns | base |
| ml_tree_h64 | 7758174ns | 7758174ns | +660.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_h64 | 1017462ns | base | --- | [1010004, 1024630] | --- | --- | --- | --- |
| ml_hotfirst_h64 | 1011119ns | no significant difference | [-16817, +9473]ns | [1002055, 1021416] | no | 0.3281 | 0.2188 | 0 |
| ml_ifchain_h64 | 1012233ns | no significant difference | [-16504, +14490]ns | [1004555, 1025105] | no | 0.6875 | 0.6875 | 0 |
| ml_tree_h64 | 7750590ns | +6734691.3ns (+661.9%) | [+6717446, +6758108]ns | [7741551, 7770199] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_h64 | ml_hotfirst_h64 | ml_ifchain_h64 | ml_tree_h64 |
|---|---|---|---|---|
| 1 | 1018898ns | -0.9% | -1.7% | +660.2% |
| 2 | 1014803ns | -0.3% | -0.5% | +663.2% |
| 3 | 1005204ns | +2.2% | +2.2% | +672.5% |
| 4 | 1030281ns | -1.4% | -1.5% | +651.1% |
| 5 | 1018979ns | -1.1% | -1.1% | +663.1% |
| 6 | 1016026ns | -1.9% | +0.7% | +663.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_hotfirst_h64 | 0.240 | moderate+ |
| ml_ifchain_h64 | -0.108 | ok |
| ml_jumptable_h64 | -0.339 | moderate- |
| ml_tree_h64 | -0.493 | moderate- |

**Consistency summary:**

- **ml_hotfirst_h64**: won 5/6, lost 1/6
- **ml_ifchain_h64**: won 4/6, lost 2/6
- **ml_tree_h64**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_hotfirst_h64 | 17.2ns | 1011529.9ns | 0.0% |  |
| ml_ifchain_h64 | 26.1ns | 1013964.4ns | 0.0% |  |
| ml_jumptable_h64 | 44.3ns | 1017365.1ns | 0.0% |  |
| ml_tree_h64 | 229.2ns | 7754113.5ns | 0.0% |  |

## Distribution (algo ns)

```
ml_hotfirst_h64 (n=6, range 996703.7-1021416.1 ns)
  996703.7 |########################################
  997939.3 |
  999174.9 |
  1000410.6 |
  1001646.2 |
  1002881.8 |
  1004117.4 |
  1005353.0 |
  1006588.6 |########################################
  1007824.3 |
  1009059.9 |########################################
  1010295.5 |
  1011531.1 |########################################
  1012766.7 |
  1014002.3 |
  1015238.0 |########################################
  1016473.6 |
  1017709.2 |
  1018944.8 |
  1020180.4 |
  (0 below, 1 above range)

ml_ifchain_h64 (n=6, range 1001299.6-1025104.8 ns)
  1001299.6 |########################################
  1002489.9 |
  1003680.1 |
  1004870.4 |
  1006060.6 |
  1007250.9 |########################################
  1008441.2 |########################################
  1009631.4 |
  1010821.7 |
  1012011.9 |
  1013202.2 |
  1014392.5 |########################################
  1015582.7 |
  1016773.0 |
  1017963.2 |
  1019153.5 |
  1020343.8 |
  1021534.0 |
  1022724.3 |########################################
  1023914.5 |
  (0 below, 1 above range)

ml_jumptable_h64 (n=6, range 1005203.7-1024630.0 ns)
  1005203.7 |####################
  1006175.0 |
  1007146.3 |
  1008117.6 |
  1009089.0 |
  1010060.3 |
  1011031.6 |
  1012002.9 |
  1012974.2 |
  1013945.5 |####################
  1014916.8 |
  1015888.2 |####################
  1016859.5 |
  1017830.8 |
  1018802.1 |########################################
  1019773.4 |
  1020744.7 |
  1021716.1 |
  1022687.4 |
  1023658.7 |
  (0 below, 1 above range)

ml_tree_h64 (n=6, range 7738627.5-7770199.4 ns)
  7738627.5 |########################################
  7740206.1 |
  7741784.7 |
  7743363.3 |########################################
  7744941.9 |########################################
  7746520.5 |
  7748099.1 |
  7749677.7 |
  7751256.3 |
  7752834.9 |
  7754413.5 |########################################
  7755992.0 |
  7757570.6 |
  7759149.2 |
  7760727.8 |
  7762306.4 |
  7763885.0 |########################################
  7765463.6 |
  7767042.2 |
  7768620.8 |
  (0 below, 1 above range)

```
