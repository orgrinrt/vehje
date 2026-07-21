# Multiway branch strategies, heavy-arm, mw3_uni: 3-way, uniform key

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw3_uni**

## Key findings

- **Fastest: mw_chain_rev_h_mw3_uni** at 19431.0 ns median (-3.0% vs baseline)
- Spread: 1.08x (fastest 19431.0 ns, slowest 21081.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 22461ns | 22434ns | 20160ns | 22345ns | 23784ns | base |
| mw_chain_h_mw3_uni | 22806ns | 23639ns | 20162ns | 22578ns | 24470ns | +1.54% |
| mw_chain_rev_h_mw3_uni | 21916ns | 21779ns | 19854ns | 21260ns | 23930ns | -2.43% |
| mw_jumptable_h_mw3_uni | 22965ns | 23377ns | 20235ns | 23132ns | 24079ns | +2.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 20037ns | 17982ns | 21204ns | base | 0.051 |
| mw_chain_h_mw3_uni | 20357ns | 17987ns | 21876ns | +1.60% | 0.050 |
| mw_chain_rev_h_mw3_uni | 19556ns | 17717ns | 21357ns | -2.40% | 0.052 |
| mw_jumptable_h_mw3_uni | 20476ns | 18053ns | 21480ns | +2.19% | 0.050 |

## Performance model

- Peak throughput: **0.058 Gops/s** (mw_chain_rev_h_mw3_uni; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw3_uni | 0.051 | 88.5% |
| mw_chain_h_mw3_uni | 0.049 | 84.0% |
| mw_chain_rev_h_mw3_uni | 0.053 | 91.2% |
| mw_jumptable_h_mw3_uni | 0.049 | 85.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw3_uni | 22461ns | 22461ns | base |
| mw_chain_h_mw3_uni | 22806ns | 22806ns | +1.54% |
| mw_chain_rev_h_mw3_uni | 21916ns | 21916ns | -2.43% |
| mw_jumptable_h_mw3_uni | 22965ns | 22965ns | +2.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 20026ns | base | --- | [18880, 21204] | --- | --- | --- | --- |
| mw_chain_h_mw3_uni | 21082ns | no significant difference | [-1016, +1981]ns | [18114, 21876] | no | 1.0000 | 1.0000 | 0 |
| mw_chain_rev_h_mw3_uni | 19431ns | no significant difference | [-2019, +856]ns | [17880, 21357] | no | 1.0000 | 1.0000 | 0 |
| mw_jumptable_h_mw3_uni | 20832ns | no significant difference | [-376, +1419]ns | [19115, 21480] | no | 0.6563 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw3_uni | mw_chain_h_mw3_uni | mw_chain_rev_h_mw3_uni | mw_jumptable_h_mw3_uni |
|---|---|---|---|---|
| 1 | 19777ns | +8.6% | -10.4% | +2.4% |
| 2 | 21420ns | -0.1% | -2.9% | +0.3% |
| 3 | 20988ns | -1.1% | +3.0% | -3.9% |
| 4 | 20013ns | +11.3% | +5.5% | +7.0% |
| 5 | 20040ns | -9.0% | -9.9% | +7.2% |
| 6 | 17982ns | +0.0% | +0.3% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw3_uni | 0.130 | ok |
| mw_chain_h_mw3_uni | 0.203 | moderate+ |
| mw_chain_rev_h_mw3_uni | 0.211 | moderate+ |
| mw_jumptable_h_mw3_uni | -0.258 | moderate- |

**Consistency summary:**

- **mw_chain_h_mw3_uni**: won 2/6, lost 2/6
- **mw_chain_rev_h_mw3_uni**: won 3/6, lost 3/6
- **mw_jumptable_h_mw3_uni**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 3.5ns | 20036.7ns | 0.0% |  |
| mw_chain_h_mw3_uni | 3.9ns | 20357.1ns | 0.0% |  |
| mw_chain_rev_h_mw3_uni | 2.8ns | 19556.0ns | 0.0% |  |
| mw_jumptable_h_mw3_uni | 3.4ns | 20475.8ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw3_uni (n=6, range 17982.1-21204.0 ns)
  17982.1 |####################
  18143.2 |
  18304.3 |
  18465.4 |
  18626.5 |
  18787.6 |
  18948.7 |
  19109.7 |
  19270.8 |
  19431.9 |
  19593.0 |
  19754.1 |####################
  19915.2 |########################################
  20076.3 |
  20237.4 |
  20398.5 |
  20559.6 |
  20720.7 |
  20881.8 |####################
  21042.9 |
  (0 below, 1 above range)

mw_chain_h_mw3_uni (n=6, range 17986.7-21875.7 ns)
  17986.7 |####################
  18181.1 |####################
  18375.6 |
  18570.0 |
  18764.5 |
  18958.9 |
  19153.4 |
  19347.8 |
  19542.3 |
  19736.7 |
  19931.2 |
  20125.6 |
  20320.1 |
  20514.5 |
  20709.0 |####################
  20903.4 |
  21097.9 |
  21292.3 |########################################
  21486.8 |
  21681.2 |
  (0 below, 1 above range)

mw_chain_rev_h_mw3_uni (n=6, range 17716.7-21356.7 ns)
  17716.7 |####################
  17898.7 |########################################
  18080.7 |
  18262.7 |
  18444.7 |
  18626.7 |
  18808.7 |
  18990.7 |
  19172.7 |
  19354.7 |
  19536.7 |
  19718.7 |
  19900.7 |
  20082.7 |
  20264.7 |
  20446.7 |
  20628.7 |####################
  20810.7 |
  20992.7 |####################
  21174.7 |
  (0 below, 1 above range)

mw_jumptable_h_mw3_uni (n=6, range 18052.9-21480.2 ns)
  18052.9 |####################
  18224.3 |
  18395.6 |
  18567.0 |
  18738.4 |
  18909.7 |
  19081.1 |
  19252.5 |
  19423.8 |
  19595.2 |
  19766.6 |
  19937.9 |
  20109.3 |########################################
  20280.6 |
  20452.0 |
  20623.4 |
  20794.7 |
  20966.1 |
  21137.5 |
  21308.8 |########################################
  (0 below, 1 above range)

```
