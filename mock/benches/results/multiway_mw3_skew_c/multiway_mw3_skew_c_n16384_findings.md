# Multiway branch strategies, cheap-arm, mw3_skew: 3-way, skewed to arm 0 (~80%)

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw3_skew**

## Key findings

- **Fastest: mw_chain_rev_c_mw3_skew** at 22165.8 ns median (-6.5% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.24x (fastest 22165.8 ns, slowest 49677.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 26508ns | 26321ns | 22452ns | 25189ns | 30513ns | base |
| mw_chain_c_mw3_skew | 25320ns | 26275ns | 22809ns | 25123ns | 26870ns | -4.48% |
| mw_chain_rev_c_mw3_skew | 24972ns | 24516ns | 22994ns | 24209ns | 27104ns | -5.79% |
| mw_jumptable_c_mw3_skew | 24885ns | 25363ns | 22872ns | 24544ns | 26402ns | -6.12% |
| mw_predicate_all_c_mw3_skew | 52577ns | 52116ns | 48932ns | 51227ns | 56423ns | +98.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 23960ns | 20316ns | 27640ns | base | 0.684 |
| mw_chain_c_mw3_skew | 22850ns | 20637ns | 24305ns | -4.63% | 0.717 |
| mw_chain_rev_c_mw3_skew | 22580ns | 20810ns | 24509ns | -5.76% | 0.726 |
| mw_jumptable_c_mw3_skew | 22521ns | 20688ns | 23892ns | -6.01% | 0.728 |
| mw_predicate_all_c_mw3_skew | 50154ns | 46732ns | 53798ns | +109.33% | 0.327 |

## Performance model

- Peak throughput: **0.806 Gops/s** (mw_bintree_c_mw3_skew; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw3_skew | 0.691 | 85.7% |
| mw_chain_c_mw3_skew | 0.694 | 86.1% |
| mw_chain_rev_c_mw3_skew | 0.739 | 91.7% |
| mw_jumptable_c_mw3_skew | 0.713 | 88.4% |
| mw_predicate_all_c_mw3_skew | 0.330 | 40.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw3_skew | 26508ns | 26508ns | base |
| mw_chain_c_mw3_skew | 25320ns | 25320ns | -4.48% |
| mw_chain_rev_c_mw3_skew | 24972ns | 24972ns | -5.79% |
| mw_jumptable_c_mw3_skew | 24885ns | 24885ns | -6.12% |
| mw_predicate_all_c_mw3_skew | 52577ns | 52577ns | +98.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 23711ns | base | --- | [20528, 27640] | --- | --- | --- | --- |
| mw_chain_c_mw3_skew | 23607ns | no significant difference | [-3949, +761]ns | [20639, 24305] | no | 0.9167 | 0.6875 | 0 |
| mw_chain_rev_c_mw3_skew | 22166ns | no significant difference | [-5479, +1214]ns | [21066, 24509] | no | 0.9167 | 0.6875 | 0 |
| mw_jumptable_c_mw3_skew | 22972ns | no significant difference | [-4668, +363]ns | [20699, 23892] | no | 1.0000 | 1.0000 | 0 |
| mw_predicate_all_c_mw3_skew | 49678ns | +27222.3ns (+114.8%) | [+21274, +30087]ns | [46987, 53798] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw3_skew | mw_chain_c_mw3_skew | mw_chain_rev_c_mw3_skew | mw_jumptable_c_mw3_skew | mw_predicate_all_c_mw3_skew |
|---|---|---|---|---|---|
| 1 | 30685ns | -24.4% | -25.0% | -25.2% | +52.3% |
| 2 | 24428ns | -1.7% | +0.8% | +1.5% | +119.4% |
| 3 | 24595ns | -0.7% | -13.3% | -6.5% | +107.9% |
| 4 | 20740ns | -0.5% | +0.3% | -0.2% | +127.8% |
| 5 | 20316ns | +1.6% | +5.0% | +1.8% | +137.3% |
| 6 | 22995ns | +5.2% | +6.1% | +0.0% | +134.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw3_skew | 0.236 | moderate+ |
| mw_chain_c_mw3_skew | 0.044 | ok |
| mw_chain_rev_c_mw3_skew | 0.033 | ok |
| mw_jumptable_c_mw3_skew | 0.291 | moderate+ |
| mw_predicate_all_c_mw3_skew | -0.253 | moderate- |

**Consistency summary:**

- **mw_chain_c_mw3_skew**: won 4/6, lost 2/6
- **mw_chain_rev_c_mw3_skew**: won 2/6, lost 4/6
- **mw_jumptable_c_mw3_skew**: won 3/6, lost 2/6
- **mw_predicate_all_c_mw3_skew**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 5.9ns | 23959.7ns | 0.0% |  |
| mw_chain_c_mw3_skew | 5.5ns | 22850.2ns | 0.0% |  |
| mw_chain_rev_c_mw3_skew | 4.4ns | 22580.1ns | 0.0% |  |
| mw_jumptable_c_mw3_skew | 3.8ns | 22520.6ns | 0.0% |  |
| mw_predicate_all_c_mw3_skew | 4.5ns | 50154.2ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw3_skew (n=6, range 20316.2-27639.6 ns)
  20316.2 |####################
  20682.4 |####################
  21048.5 |
  21414.7 |
  21780.9 |
  22147.0 |
  22513.2 |
  22879.4 |####################
  23245.6 |
  23611.7 |
  23977.9 |
  24344.1 |########################################
  24710.2 |
  25076.4 |
  25442.6 |
  25808.8 |
  26174.9 |
  26541.1 |
  26907.3 |
  27273.4 |
  (0 below, 1 above range)

mw_chain_c_mw3_skew (n=6, range 20637.1-24304.8 ns)
  20637.1 |########################################
  20820.5 |
  21003.9 |
  21187.2 |
  21370.6 |
  21554.0 |
  21737.4 |
  21920.8 |
  22104.2 |
  22287.5 |
  22470.9 |
  22654.3 |
  22837.7 |
  23021.1 |####################
  23204.5 |
  23387.8 |
  23571.2 |
  23754.6 |
  23938.0 |####################
  24121.4 |####################
  (0 below, 1 above range)

mw_chain_rev_c_mw3_skew (n=6, range 20810.0-24508.5 ns)
  20810.0 |####################
  20994.9 |
  21179.9 |########################################
  21364.8 |
  21549.7 |
  21734.6 |
  21919.6 |
  22104.5 |
  22289.4 |
  22474.3 |
  22659.3 |
  22844.2 |####################
  23029.1 |
  23214.1 |
  23399.0 |
  23583.9 |
  23768.8 |
  23953.8 |
  24138.7 |
  24323.6 |####################
  (0 below, 1 above range)

mw_jumptable_c_mw3_skew (n=6, range 20687.9-23891.7 ns)
  20687.9 |##########################
  20848.1 |
  21008.3 |
  21168.5 |
  21328.7 |
  21488.8 |
  21649.0 |
  21809.2 |
  21969.4 |
  22129.6 |
  22289.8 |
  22450.0 |
  22610.2 |
  22770.3 |
  22930.5 |########################################
  23090.7 |
  23250.9 |
  23411.1 |
  23571.3 |
  23731.5 |
  (0 below, 1 above range)

mw_predicate_all_c_mw3_skew (n=6, range 46732.1-53798.1 ns)
  46732.1 |########################################
  47085.4 |########################################
  47438.7 |
  47792.0 |
  48145.3 |########################################
  48498.6 |
  48851.9 |
  49205.2 |
  49558.5 |
  49911.8 |
  50265.1 |
  50618.4 |
  50971.7 |########################################
  51325.0 |
  51678.3 |
  52031.6 |
  52384.9 |
  52738.2 |
  53091.5 |
  53444.8 |########################################
  (0 below, 1 above range)

```
