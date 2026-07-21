# Multiway branch strategies, heavy-arm, mw8_uni: 8-way, uniform key

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw8_uni**

## Key findings

- **Fastest: mw_chain_h_mw8_uni** at 16993.1 ns median (-7.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.08x (fastest 16993.1 ns, slowest 18353.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 21303ns | 20608ns | 20414ns | 20592ns | 22815ns | base |
| mw_chain_h_mw8_uni | 19506ns | 19329ns | 18836ns | 19219ns | 20272ns | -8.44% |
| mw_chain_rev_h_mw8_uni | 21429ns | 20266ns | 18911ns | 19859ns | 25044ns | +0.59% |
| mw_jumptable_h_mw8_uni | 22626ns | 19981ns | 19084ns | 19687ns | 28806ns | +6.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 18974ns | 18188ns | 20334ns | base | 0.054 |
| mw_chain_h_mw8_uni | 17188ns | 16620ns | 17879ns | -9.41% | 0.060 |
| mw_chain_rev_h_mw8_uni | 18967ns | 16676ns | 22289ns | -0.04% | 0.054 |
| mw_jumptable_h_mw8_uni | 20147ns | 16830ns | 25936ns | +6.18% | 0.051 |

## Performance model

- Peak throughput: **0.062 Gops/s** (mw_chain_h_mw8_uni; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw8_uni | 0.056 | 90.6% |
| mw_chain_h_mw8_uni | 0.060 | 97.8% |
| mw_chain_rev_h_mw8_uni | 0.057 | 93.0% |
| mw_jumptable_h_mw8_uni | 0.058 | 94.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw8_uni | 21303ns | 21303ns | base |
| mw_chain_h_mw8_uni | 19506ns | 19506ns | -8.44% |
| mw_chain_rev_h_mw8_uni | 21429ns | 21429ns | +0.59% |
| mw_jumptable_h_mw8_uni | 22626ns | 22626ns | +6.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 18354ns | base | --- | [18235, 20334] | --- | --- | --- | --- |
| mw_chain_h_mw8_uni | 16993ns | -1590.2ns (-8.7%) | [-2789, -979]ns | [16693, 17879] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| mw_chain_rev_h_mw8_uni | 17870ns | no significant difference | [-1665, +3068]ns | [16741, 22289] | no | 0.3281 | 0.2188 | 0 |
| mw_jumptable_h_mw8_uni | 17665ns | no significant difference | [-2213, +6348]ns | [16839, 25936] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw8_uni | mw_chain_h_mw8_uni | mw_chain_rev_h_mw8_uni | mw_jumptable_h_mw8_uni |
|---|---|---|---|---|
| 1 | 18329ns | -4.2% | -8.3% | +17.0% |
| 2 | 18283ns | -6.5% | -4.0% | -7.9% |
| 3 | 18378ns | -9.6% | -9.3% | -5.0% |
| 4 | 18188ns | -7.8% | +37.8% | -1.8% |
| 5 | 19820ns | -14.8% | -8.2% | -15.1% |
| 6 | 20848ns | -12.7% | -6.4% | +45.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw8_uni | 0.369 | moderate+ |
| mw_chain_h_mw8_uni | 0.050 | ok |
| mw_chain_rev_h_mw8_uni | -0.256 | moderate- |
| mw_jumptable_h_mw8_uni | -0.112 | ok |

**Consistency summary:**

- **mw_chain_h_mw8_uni**: won 6/6, lost 0/6
- **mw_chain_rev_h_mw8_uni**: won 5/6, lost 1/6
- **mw_jumptable_h_mw8_uni**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 3.5ns | 18974.3ns | 0.0% |  |
| mw_chain_h_mw8_uni | 4.5ns | 17188.3ns | 0.0% |  |
| mw_chain_rev_h_mw8_uni | 5.4ns | 18966.9ns | 0.0% |  |
| mw_jumptable_h_mw8_uni | 7.2ns | 20146.5ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw8_uni (n=6, range 18187.5-20334.0 ns)
  18187.5 |########################################
  18294.8 |########################################
  18402.1 |
  18509.5 |
  18616.8 |
  18724.1 |
  18831.4 |
  18938.8 |
  19046.1 |
  19153.4 |
  19260.7 |
  19368.0 |
  19475.4 |
  19582.7 |
  19690.0 |
  19797.3 |####################
  19904.7 |
  20012.0 |
  20119.3 |
  20226.6 |
  (0 below, 1 above range)

mw_chain_h_mw8_uni (n=6, range 16619.6-17879.2 ns)
  16619.6 |########################################
  16682.6 |
  16745.6 |########################################
  16808.5 |
  16871.5 |########################################
  16934.5 |
  16997.5 |
  17060.4 |########################################
  17123.4 |
  17186.4 |
  17249.4 |
  17312.4 |
  17375.3 |
  17438.3 |
  17501.3 |########################################
  17564.3 |
  17627.2 |
  17690.2 |
  17753.2 |
  17816.2 |
  (0 below, 1 above range)

mw_chain_rev_h_mw8_uni (n=6, range 16675.8-22289.2 ns)
  16675.8 |########################################
  16956.5 |
  17237.1 |
  17517.8 |####################
  17798.5 |
  18079.1 |####################
  18359.8 |
  18640.5 |
  18921.1 |
  19201.8 |
  19482.5 |####################
  19763.1 |
  20043.8 |
  20324.5 |
  20605.1 |
  20885.8 |
  21166.5 |
  21447.1 |
  21727.8 |
  22008.5 |
  (0 below, 1 above range)

mw_jumptable_h_mw8_uni (n=6, range 16830.0-25935.8 ns)
  16830.0 |########################################
  17285.3 |####################
  17740.6 |####################
  18195.9 |
  18651.2 |
  19106.5 |
  19561.8 |
  20017.0 |
  20472.3 |
  20927.6 |
  21382.9 |####################
  21838.2 |
  22293.5 |
  22748.8 |
  23204.1 |
  23659.4 |
  24114.7 |
  24570.0 |
  25025.3 |
  25480.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **mw_jumptable_h_mw8_uni**: CV=24.1% (high variance, measurements may be unstable)
