# Multiway branch strategies, cheap-arm, mw3_uni: 3-way, uniform key

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw3_uni**

## Key findings

- **Baseline (mw_bintree_c_mw3_uni) is the fastest** at 176.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.64x (fastest 176.6 ns, slowest 289.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 2926ns | 2998ns | 2301ns | 2911ns | 3262ns | base |
| mw_chain_c_mw3_uni | 3013ns | 3085ns | 2423ns | 3048ns | 3255ns | +2.95% |
| mw_chain_rev_c_mw3_uni | 3049ns | 3095ns | 2342ns | 2986ns | 3498ns | +4.20% |
| mw_jumptable_c_mw3_uni | 2931ns | 3003ns | 2308ns | 2916ns | 3265ns | +0.17% |
| mw_predicate_all_c_mw3_uni | 3029ns | 3083ns | 2390ns | 3006ns | 3382ns | +3.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 173ns | 138ns | 194ns | base | 0.370 |
| mw_chain_c_mw3_uni | 177ns | 136ns | 193ns | +2.37% | 0.361 |
| mw_chain_rev_c_mw3_uni | 177ns | 140ns | 203ns | +2.20% | 0.362 |
| mw_jumptable_c_mw3_uni | 174ns | 138ns | 193ns | +0.60% | 0.368 |
| mw_predicate_all_c_mw3_uni | 285ns | 223ns | 319ns | +64.64% | 0.225 |

## Performance model

- Peak throughput: **0.471 Gops/s** (mw_chain_c_mw3_uni; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw3_uni | 0.362 | 76.9% |
| mw_chain_c_mw3_uni | 0.350 | 74.3% |
| mw_chain_rev_c_mw3_uni | 0.359 | 76.2% |
| mw_jumptable_c_mw3_uni | 0.358 | 76.1% |
| mw_predicate_all_c_mw3_uni | 0.221 | 46.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw3_uni | 2926ns | 2926ns | base |
| mw_chain_c_mw3_uni | 3013ns | 3013ns | +2.95% |
| mw_chain_rev_c_mw3_uni | 3049ns | 3049ns | +4.20% |
| mw_jumptable_c_mw3_uni | 2931ns | 2931ns | +0.17% |
| mw_predicate_all_c_mw3_uni | 3029ns | 3029ns | +3.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 177ns | base | --- | [149, 194] | --- | --- | --- | --- |
| mw_chain_c_mw3_uni | 183ns | no significant difference | [-3, +16]ns | [156, 193] | no | 1.0000 | 1.0000 | 0 |
| mw_chain_rev_c_mw3_uni | 178ns | no significant difference | [-9, +20]ns | [149, 203] | no | 1.0000 | 1.0000 | 0 |
| mw_jumptable_c_mw3_uni | 179ns | no significant difference | [-9, +12]ns | [151, 193] | no | 1.0000 | 0.6875 | 0 |
| mw_predicate_all_c_mw3_uni | 290ns | +117.3ns (+66.4%) | [+91, +127]ns | [246, 319] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw3_uni | mw_chain_c_mw3_uni | mw_chain_rev_c_mw3_uni | mw_jumptable_c_mw3_uni | mw_predicate_all_c_mw3_uni |
|---|---|---|---|---|---|
| 1 | 138ns | -1.5% | +1.5% | +0.3% | +61.9% |
| 2 | 161ns | +9.6% | +10.6% | +11.7% | +79.4% |
| 3 | 194ns | -1.1% | -2.2% | -1.5% | +49.8% |
| 4 | 159ns | +10.2% | -0.3% | +2.9% | +68.5% |
| 5 | 192ns | -1.7% | -7.2% | -7.8% | +66.0% |
| 6 | 194ns | +0.2% | +11.3% | +0.2% | +64.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw3_uni | 0.008 | ok |
| mw_chain_c_mw3_uni | 0.072 | ok |
| mw_chain_rev_c_mw3_uni | -0.069 | ok |
| mw_jumptable_c_mw3_uni | -0.119 | ok |
| mw_predicate_all_c_mw3_uni | 0.043 | ok |

**Consistency summary:**

- **mw_chain_c_mw3_uni**: won 3/6, lost 3/6
- **mw_chain_rev_c_mw3_uni**: won 3/6, lost 3/6
- **mw_jumptable_c_mw3_uni**: won 2/6, lost 4/6
- **mw_predicate_all_c_mw3_uni**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 3.4ns | 173.1ns | 2.0% |  |
| mw_chain_c_mw3_uni | 5.0ns | 177.2ns | 2.9% |  |
| mw_chain_rev_c_mw3_uni | 3.9ns | 176.9ns | 2.2% |  |
| mw_jumptable_c_mw3_uni | 3.4ns | 174.1ns | 2.0% |  |
| mw_predicate_all_c_mw3_uni | 3.6ns | 284.9ns | 1.3% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw3_uni (n=6, range 137.9-194.0 ns)
    137.9 |####################
    140.7 |
    143.5 |
    146.3 |
    149.1 |
    151.9 |
    154.7 |
    157.5 |####################
    160.3 |####################
    163.1 |
    165.9 |
    168.8 |
    171.6 |
    174.4 |
    177.2 |
    180.0 |
    182.8 |
    185.6 |
    188.4 |
    191.2 |########################################
  (0 below, 1 above range)

mw_chain_c_mw3_uni (n=6, range 135.8-193.1 ns)
    135.8 |########################################
    138.7 |
    141.5 |
    144.4 |
    147.3 |
    150.1 |
    153.0 |
    155.9 |
    158.7 |
    161.6 |
    164.5 |
    167.3 |
    170.2 |
    173.1 |########################################
    175.9 |########################################
    178.8 |
    181.7 |
    184.5 |
    187.4 |########################################
    190.3 |########################################
  (0 below, 1 above range)

mw_chain_rev_c_mw3_uni (n=6, range 140.0-202.9 ns)
    140.0 |####################
    143.1 |
    146.3 |
    149.4 |
    152.6 |
    155.7 |####################
    158.9 |
    162.0 |
    165.2 |
    168.3 |
    171.4 |
    174.6 |
    177.7 |########################################
    180.9 |
    184.0 |
    187.2 |####################
    190.3 |
    193.5 |
    196.6 |
    199.8 |
  (0 below, 1 above range)

mw_jumptable_c_mw3_uni (n=6, range 138.3-192.7 ns)
    138.3 |########################################
    141.0 |
    143.7 |
    146.5 |
    149.2 |
    151.9 |
    154.6 |
    157.3 |
    160.1 |
    162.8 |########################################
    165.5 |
    168.2 |
    170.9 |
    173.7 |
    176.4 |########################################
    179.1 |########################################
    181.8 |
    184.5 |
    187.3 |
    190.0 |########################################
  (0 below, 1 above range)

mw_predicate_all_c_mw3_uni (n=6, range 223.3-319.2 ns)
    223.3 |####################
    228.1 |
    232.9 |
    237.7 |
    242.5 |
    247.3 |
    252.1 |
    256.9 |
    261.7 |
    266.5 |####################
    271.2 |
    276.0 |
    280.8 |
    285.6 |########################################
    290.4 |
    295.2 |
    300.0 |
    304.8 |
    309.6 |
    314.4 |####################
  (0 below, 1 above range)

```
