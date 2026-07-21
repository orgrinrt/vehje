# Multiway branch strategies, heavy-arm, mw8_uni: 8-way, uniform key

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw8_uni**

## Key findings

- **Fastest: mw_chain_h_mw8_uni** at 258326.5 ns median (-8.0% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.09x (fastest 258326.5 ns, slowest 280939.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 284099ns | 283541ns | 282728ns | 283306ns | 285975ns | base |
| mw_chain_h_mw8_uni | 261264ns | 260866ns | 260031ns | 260697ns | 262730ns | -8.04% |
| mw_chain_rev_h_mw8_uni | 262544ns | 262719ns | 259518ns | 261856ns | 265090ns | -7.59% |
| mw_jumptable_h_mw8_uni | 265458ns | 263705ns | 260658ns | 262970ns | 271590ns | -6.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 281606ns | 280075ns | 283697ns | base | 0.058 |
| mw_chain_h_mw8_uni | 258799ns | 257683ns | 260201ns | -8.10% | 0.063 |
| mw_chain_rev_h_mw8_uni | 259999ns | 256891ns | 262632ns | -7.67% | 0.063 |
| mw_jumptable_h_mw8_uni | 262845ns | 258062ns | 268990ns | -6.66% | 0.062 |

## Performance model

- Peak throughput: **0.064 Gops/s** (mw_chain_rev_h_mw8_uni; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw8_uni | 0.058 | 91.4% |
| mw_chain_h_mw8_uni | 0.063 | 99.4% |
| mw_chain_rev_h_mw8_uni | 0.063 | 98.7% |
| mw_jumptable_h_mw8_uni | 0.063 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw8_uni | 284099ns | 284099ns | base |
| mw_chain_h_mw8_uni | 261264ns | 261264ns | -8.04% |
| mw_chain_rev_h_mw8_uni | 262544ns | 262544ns | -7.59% |
| mw_jumptable_h_mw8_uni | 265458ns | 265458ns | -6.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 280940ns | base | --- | [280182, 283697] | --- | --- | --- | --- |
| mw_chain_h_mw8_uni | 258326ns | -22549.8ns (-8.0%) | [-25755, -20117]ns | [257870, 260201] | YES | 0.0313 | 0.0313 | 0 |
| mw_chain_rev_h_mw8_uni | 260152ns | -22780.2ns (-8.1%) | [-23981, -18060]ns | [257215, 262632] | YES | 0.0313 | 0.0313 | 0 |
| mw_jumptable_h_mw8_uni | 261174ns | -21796.0ns (-7.8%) | [-23296, -11192]ns | [258371, 268990] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw8_uni | mw_chain_h_mw8_uni | mw_chain_rev_h_mw8_uni | mw_jumptable_h_mw8_uni |
|---|---|---|---|---|
| 1 | 280289ns | -7.8% | -7.7% | -3.3% |
| 2 | 281636ns | -8.3% | -8.6% | -8.2% |
| 3 | 285759ns | -9.8% | -8.3% | -8.3% |
| 4 | 281125ns | -8.2% | -6.9% | -7.4% |
| 5 | 280755ns | -7.8% | -8.5% | -8.1% |
| 6 | 280075ns | -6.6% | -6.0% | -4.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw8_uni | -0.009 | ok |
| mw_chain_h_mw8_uni | 0.146 | ok |
| mw_chain_rev_h_mw8_uni | -0.385 | moderate- |
| mw_jumptable_h_mw8_uni | -0.278 | moderate- |

**Consistency summary:**

- **mw_chain_h_mw8_uni**: won 6/6, lost 0/6
- **mw_chain_rev_h_mw8_uni**: won 6/6, lost 0/6
- **mw_jumptable_h_mw8_uni**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw8_uni | 3.9ns | 281606.3ns | 0.0% |  |
| mw_chain_h_mw8_uni | 4.6ns | 258799.0ns | 0.0% |  |
| mw_chain_rev_h_mw8_uni | 3.5ns | 259999.4ns | 0.0% |  |
| mw_jumptable_h_mw8_uni | 5.5ns | 262845.1ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw8_uni (n=6, range 280075.0-283697.3 ns)
  280075.0 |########################################
  280256.1 |########################################
  280437.2 |
  280618.3 |########################################
  280799.5 |
  280980.6 |########################################
  281161.7 |
  281342.8 |
  281523.9 |########################################
  281705.0 |
  281886.2 |
  282067.3 |
  282248.4 |
  282429.5 |
  282610.6 |
  282791.7 |
  282972.8 |
  283154.0 |
  283335.1 |
  283516.2 |
  (0 below, 1 above range)

mw_chain_h_mw8_uni (n=6, range 257682.9-260200.6 ns)
  257682.9 |########################################
  257808.8 |
  257934.7 |########################################
  258060.6 |
  258186.4 |########################################
  258312.3 |
  258438.2 |########################################
  258564.1 |
  258690.0 |########################################
  258815.9 |
  258941.8 |
  259067.6 |
  259193.5 |
  259319.4 |
  259445.3 |
  259571.2 |
  259697.1 |
  259822.9 |
  259948.8 |
  260074.7 |
  (0 below, 1 above range)

mw_chain_rev_h_mw8_uni (n=6, range 256891.2-262631.7 ns)
  256891.2 |########################################
  257178.2 |
  257465.2 |########################################
  257752.3 |
  258039.3 |
  258326.3 |########################################
  258613.3 |
  258900.4 |
  259187.4 |
  259474.4 |
  259761.4 |
  260048.4 |
  260335.5 |
  260622.5 |
  260909.5 |
  261196.5 |
  261483.6 |########################################
  261770.6 |########################################
  262057.6 |
  262344.6 |
  (0 below, 1 above range)

mw_jumptable_h_mw8_uni (n=6, range 258062.5-268990.4 ns)
  258062.5 |########################################
  258608.9 |########################################
  259155.3 |
  259701.7 |########################################
  260248.1 |
  260794.5 |
  261340.9 |
  261887.3 |########################################
  262433.7 |
  262980.1 |
  263526.5 |
  264072.8 |
  264619.2 |
  265165.6 |
  265712.0 |
  266258.4 |
  266804.8 |########################################
  267351.2 |
  267897.6 |
  268444.0 |
  (0 below, 1 above range)

```
