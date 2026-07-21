# Multiway branch strategies, cheap-arm, mw8_uni: 8-way, uniform key

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw8_uni**

## Key findings

- **Fastest: mw_jumptable_c_mw8_uni** at 18917.9 ns median (-40.2% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.39x (fastest 18917.9 ns, slowest 120905.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 34107ns | 34008ns | 32466ns | 33596ns | 35696ns | base |
| mw_chain_c_mw8_uni | 22365ns | 22174ns | 20530ns | 21636ns | 24375ns | -34.43% |
| mw_chain_rev_c_mw8_uni | 21872ns | 21969ns | 20343ns | 21467ns | 23245ns | -35.87% |
| mw_jumptable_c_mw8_uni | 21608ns | 21171ns | 20486ns | 21165ns | 22834ns | -36.65% |
| mw_predicate_all_c_mw8_uni | 125951ns | 123365ns | 118646ns | 122041ns | 135469ns | +269.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 31733ns | 30204ns | 33205ns | base | 0.516 |
| mw_chain_c_mw8_uni | 19978ns | 18345ns | 21750ns | -37.04% | 0.820 |
| mw_chain_rev_c_mw8_uni | 19536ns | 18203ns | 20763ns | -38.44% | 0.839 |
| mw_jumptable_c_mw8_uni | 19301ns | 18304ns | 20382ns | -39.18% | 0.849 |
| mw_predicate_all_c_mw8_uni | 123440ns | 116302ns | 132711ns | +288.99% | 0.133 |

## Performance model

- Peak throughput: **0.900 Gops/s** (mw_chain_rev_c_mw8_uni; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw8_uni | 0.518 | 57.5% |
| mw_chain_c_mw8_uni | 0.826 | 91.8% |
| mw_chain_rev_c_mw8_uni | 0.835 | 92.8% |
| mw_jumptable_c_mw8_uni | 0.866 | 96.2% |
| mw_predicate_all_c_mw8_uni | 0.136 | 15.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw8_uni | 34107ns | 34107ns | base |
| mw_chain_c_mw8_uni | 22365ns | 22365ns | -34.43% |
| mw_chain_rev_c_mw8_uni | 21872ns | 21872ns | -35.87% |
| mw_jumptable_c_mw8_uni | 21608ns | 21608ns | -36.65% |
| mw_predicate_all_c_mw8_uni | 125951ns | 125951ns | +269.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 31644ns | base | --- | [30350, 33205] | --- | --- | --- | --- |
| mw_chain_c_mw8_uni | 19831ns | -11807.1ns (-37.3%) | [-12248, -11210]ns | [18353, 21750] | YES | 0.0313 | 0.0313 | 0 |
| mw_chain_rev_c_mw8_uni | 19618ns | -12121.1ns (-38.3%) | [-12647, -11824]ns | [18226, 20763] | YES | 0.0313 | 0.0313 | 0 |
| mw_jumptable_c_mw8_uni | 18918ns | -11776.5ns (-37.2%) | [-14033, -11488]ns | [18602, 20382] | YES | 0.0313 | 0.0313 | 0 |
| mw_predicate_all_c_mw8_uni | 120906ns | +88208.5ns (+278.8%) | [+85959, +100954]ns | [116704, 132711] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw8_uni | mw_chain_c_mw8_uni | mw_chain_rev_c_mw8_uni | mw_jumptable_c_mw8_uni | mw_predicate_all_c_mw8_uni |
|---|---|---|---|---|---|
| 1 | 30496ns | -39.8% | -40.3% | -37.9% | +347.2% |
| 2 | 33018ns | -33.8% | -36.2% | -34.6% | +290.9% |
| 3 | 30552ns | -36.9% | -38.3% | -38.1% | +283.3% |
| 4 | 32736ns | -37.8% | -37.5% | -41.5% | +262.2% |
| 5 | 30204ns | -39.3% | -39.6% | -39.4% | +285.1% |
| 6 | 33393ns | -35.2% | -38.9% | -43.4% | +269.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw8_uni | -0.783 | HIGH- (thermal bounce) |
| mw_chain_c_mw8_uni | -0.653 | HIGH- (thermal bounce) |
| mw_chain_rev_c_mw8_uni | -0.768 | HIGH- (thermal bounce) |
| mw_jumptable_c_mw8_uni | -0.173 | ok |
| mw_predicate_all_c_mw8_uni | 0.332 | moderate+ |

**Consistency summary:**

- **mw_chain_c_mw8_uni**: won 6/6, lost 0/6
- **mw_chain_rev_c_mw8_uni**: won 6/6, lost 0/6
- **mw_jumptable_c_mw8_uni**: won 6/6, lost 0/6
- **mw_predicate_all_c_mw8_uni**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 4.1ns | 31733.2ns | 0.0% |  |
| mw_chain_c_mw8_uni | 3.8ns | 19978.0ns | 0.0% |  |
| mw_chain_rev_c_mw8_uni | 4.2ns | 19536.0ns | 0.0% |  |
| mw_jumptable_c_mw8_uni | 3.6ns | 19300.6ns | 0.0% |  |
| mw_predicate_all_c_mw8_uni | 7.6ns | 123440.3ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw8_uni (n=6, range 30204.2-33205.4 ns)
  30204.2 |########################################
  30354.3 |########################################
  30504.3 |########################################
  30654.4 |
  30804.4 |
  30954.5 |
  31104.6 |
  31254.6 |
  31404.7 |
  31554.7 |
  31704.8 |
  31854.9 |
  32004.9 |
  32155.0 |
  32305.0 |
  32455.1 |
  32605.2 |########################################
  32755.2 |
  32905.3 |########################################
  33055.3 |
  (0 below, 1 above range)

mw_chain_c_mw8_uni (n=6, range 18345.4-21750.2 ns)
  18345.4 |########################################
  18515.6 |
  18685.9 |
  18856.1 |
  19026.4 |
  19196.6 |####################
  19366.8 |
  19537.1 |
  19707.3 |
  19877.6 |
  20047.8 |
  20218.0 |####################
  20388.3 |
  20558.5 |
  20728.8 |
  20899.0 |
  21069.2 |
  21239.5 |
  21409.7 |
  21580.0 |####################
  (0 below, 1 above range)

mw_chain_rev_c_mw8_uni (n=6, range 18203.3-20763.3 ns)
  18203.3 |########################################
  18331.3 |
  18459.3 |
  18587.3 |
  18715.3 |
  18843.3 |####################
  18971.3 |
  19099.3 |
  19227.3 |
  19355.3 |
  19483.3 |
  19611.3 |
  19739.3 |
  19867.3 |
  19995.3 |
  20123.3 |
  20251.3 |
  20379.3 |########################################
  20507.3 |
  20635.3 |
  (0 below, 1 above range)

mw_jumptable_c_mw8_uni (n=6, range 18303.8-20381.9 ns)
  18303.8 |####################
  18407.7 |
  18511.6 |
  18615.5 |
  18719.4 |
  18823.3 |########################################
  18927.2 |####################
  19031.1 |
  19135.0 |####################
  19238.9 |
  19342.8 |
  19446.8 |
  19550.7 |
  19654.6 |
  19758.5 |
  19862.4 |
  19966.3 |
  20070.2 |
  20174.1 |
  20278.0 |
  (0 below, 1 above range)

mw_predicate_all_c_mw8_uni (n=6, range 116302.1-132711.2 ns)
  116302.1 |########################################
  117122.6 |
  117943.0 |####################
  118763.5 |
  119583.9 |
  120404.4 |
  121224.8 |
  122045.3 |
  122865.8 |####################
  123686.2 |
  124506.7 |
  125327.1 |
  126147.6 |
  126968.0 |
  127788.5 |
  128609.0 |####################
  129429.4 |
  130249.9 |
  131070.3 |
  131890.8 |
  (0 below, 1 above range)

```
