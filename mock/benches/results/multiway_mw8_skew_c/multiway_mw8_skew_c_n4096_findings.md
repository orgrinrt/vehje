# Multiway branch strategies, cheap-arm, mw8_skew: 8-way, skewed to arm 0 (~70%)

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw8_skew**

## Key findings

- **Fastest: mw_jumptable_c_mw8_skew** at 6934.4 ns median (-29.7% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.64x (fastest 6934.4 ns, slowest 32173.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 12383ns | 12457ns | 10512ns | 11876ns | 14078ns | base |
| mw_chain_c_mw8_skew | 11714ns | 12023ns | 10013ns | 11659ns | 12649ns | -5.40% |
| mw_chain_rev_c_mw8_skew | 10436ns | 10613ns | 8909ns | 10610ns | 10940ns | -15.72% |
| mw_jumptable_c_mw8_skew | 9198ns | 9444ns | 8085ns | 9292ns | 9614ns | -25.72% |
| mw_predicate_all_c_mw8_skew | 34281ns | 34565ns | 30939ns | 33815ns | 36651ns | +176.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 9843ns | 8322ns | 11258ns | base | 0.416 |
| mw_chain_c_mw8_skew | 9127ns | 7828ns | 9829ns | -7.28% | 0.449 |
| mw_chain_rev_c_mw8_skew | 7885ns | 6729ns | 8262ns | -19.89% | 0.519 |
| mw_jumptable_c_mw8_skew | 6730ns | 5902ns | 7022ns | -31.63% | 0.609 |
| mw_predicate_all_c_mw8_skew | 31899ns | 28795ns | 34098ns | +224.07% | 0.128 |

## Performance model

- Peak throughput: **0.694 Gops/s** (mw_jumptable_c_mw8_skew; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw8_skew | 0.415 | 59.8% |
| mw_chain_c_mw8_skew | 0.436 | 62.8% |
| mw_chain_rev_c_mw8_skew | 0.511 | 73.6% |
| mw_jumptable_c_mw8_skew | 0.591 | 85.1% |
| mw_predicate_all_c_mw8_skew | 0.127 | 18.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw8_skew | 12383ns | 12383ns | base |
| mw_chain_c_mw8_skew | 11714ns | 11714ns | -5.40% |
| mw_chain_rev_c_mw8_skew | 10436ns | 10436ns | -15.72% |
| mw_jumptable_c_mw8_skew | 9198ns | 9198ns | -25.72% |
| mw_predicate_all_c_mw8_skew | 34281ns | 34281ns | +176.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 9867ns | base | --- | [8404, 11258] | --- | --- | --- | --- |
| mw_chain_c_mw8_skew | 9392ns | no significant difference | [-2072, +599]ns | [8160, 9829] | no | 0.2188 | 0.2188 | 0 |
| mw_chain_rev_c_mw8_skew | 8019ns | -1806.2ns (-18.3%) | [-3240, -829]ns | [7373, 8262] | YES | 0.0417 | 0.0313 | 0 |
| mw_jumptable_c_mw8_skew | 6934ns | -2846.7ns (-28.8%) | [-4322, -2171]ns | [6233, 7022] | YES | 0.0417 | 0.0313 | 0 |
| mw_predicate_all_c_mw8_skew | 32174ns | +22477.7ns (+227.8%) | [+19280, +24409]ns | [29425, 34098] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw8_skew | mw_chain_c_mw8_skew | mw_chain_rev_c_mw8_skew | mw_jumptable_c_mw8_skew | mw_predicate_all_c_mw8_skew |
|---|---|---|---|---|---|
| 1 | 11805ns | -28.1% | -32.1% | -42.0% | +143.9% |
| 2 | 9875ns | -1.0% | -18.8% | -28.9% | +258.3% |
| 3 | 10711ns | -7.8% | -25.2% | -34.4% | +206.3% |
| 4 | 9859ns | -7.0% | -13.8% | -28.8% | +231.8% |
| 5 | 8486ns | -7.8% | -20.7% | -30.5% | +254.2% |
| 6 | 8322ns | +15.6% | -3.6% | -21.1% | +280.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw8_skew | 0.245 | moderate+ |
| mw_chain_c_mw8_skew | -0.176 | ok |
| mw_chain_rev_c_mw8_skew | -0.423 | moderate- |
| mw_jumptable_c_mw8_skew | 0.102 | ok |
| mw_predicate_all_c_mw8_skew | -0.295 | moderate- |

**Consistency summary:**

- **mw_chain_c_mw8_skew**: won 5/6, lost 1/6
- **mw_chain_rev_c_mw8_skew**: won 6/6, lost 0/6
- **mw_jumptable_c_mw8_skew**: won 6/6, lost 0/6
- **mw_predicate_all_c_mw8_skew**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 5.4ns | 9843.1ns | 0.1% |  |
| mw_chain_c_mw8_skew | 3.1ns | 9126.8ns | 0.0% |  |
| mw_chain_rev_c_mw8_skew | 3.3ns | 7884.9ns | 0.0% |  |
| mw_jumptable_c_mw8_skew | 4.0ns | 6729.9ns | 0.1% |  |
| mw_predicate_all_c_mw8_skew | 3.3ns | 31898.8ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw8_skew (n=6, range 8322.5-11257.9 ns)
   8322.5 |####################
   8469.3 |####################
   8616.0 |
   8762.8 |
   8909.6 |
   9056.4 |
   9203.1 |
   9349.9 |
   9496.7 |
   9643.4 |
   9790.2 |########################################
   9937.0 |
  10083.7 |
  10230.5 |
  10377.3 |
  10524.1 |
  10670.8 |####################
  10817.6 |
  10964.4 |
  11111.1 |
  (0 below, 1 above range)

mw_chain_c_mw8_skew (n=6, range 7827.5-9828.5 ns)
   7827.5 |########################################
   7927.6 |
   8027.6 |
   8127.7 |
   8227.7 |
   8327.8 |
   8427.8 |########################################
   8527.9 |
   8627.9 |
   8728.0 |
   8828.0 |
   8928.1 |
   9028.1 |
   9128.2 |########################################
   9228.2 |
   9328.3 |
   9428.3 |
   9528.4 |########################################
   9628.4 |
   9728.5 |########################################
  (0 below, 1 above range)

mw_chain_rev_c_mw8_skew (n=6, range 6729.2-8262.3 ns)
   6729.2 |##########
   6805.9 |
   6882.5 |
   6959.2 |
   7035.8 |
   7112.5 |
   7189.1 |
   7265.8 |
   7342.4 |
   7419.1 |
   7495.8 |
   7572.4 |
   7649.1 |
   7725.7 |
   7802.4 |
   7879.0 |
   7955.7 |########################################
   8032.3 |
   8109.0 |
   8185.6 |
  (0 below, 1 above range)

mw_jumptable_c_mw8_skew (n=6, range 5901.7-7021.9 ns)
   5901.7 |####################
   5957.7 |
   6013.7 |
   6069.7 |
   6125.7 |
   6181.7 |
   6237.7 |
   6293.8 |
   6349.8 |
   6405.8 |
   6461.8 |
   6517.8 |####################
   6573.8 |
   6629.8 |
   6685.8 |
   6741.8 |
   6797.8 |####################
   6853.8 |
   6909.8 |
   6965.8 |########################################
  (0 below, 1 above range)

mw_predicate_all_c_mw8_skew (n=6, range 28795.4-34097.5 ns)
  28795.4 |########################################
  29060.5 |
  29325.6 |
  29590.7 |
  29855.8 |########################################
  30120.9 |
  30386.0 |
  30651.1 |
  30916.2 |
  31181.3 |
  31446.5 |########################################
  31711.6 |
  31976.7 |
  32241.8 |
  32506.9 |########################################
  32772.0 |########################################
  33037.1 |
  33302.2 |
  33567.3 |
  33832.4 |
  (0 below, 1 above range)

```
