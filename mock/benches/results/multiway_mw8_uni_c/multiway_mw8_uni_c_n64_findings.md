# Multiway branch strategies, cheap-arm, mw8_uni: 8-way, uniform key

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw8_uni**

## Key findings

- **Fastest: mw_jumptable_c_mw8_uni** at 131.1 ns median (-35.9% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.68x (fastest 131.1 ns, slowest 614.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 3175ns | 3263ns | 2302ns | 3093ns | 3735ns | base |
| mw_chain_c_mw8_uni | 3397ns | 3393ns | 2342ns | 3046ns | 4450ns | +6.97% |
| mw_chain_rev_c_mw8_uni | 3277ns | 3385ns | 2271ns | 3042ns | 4131ns | +3.19% |
| mw_jumptable_c_mw8_uni | 3160ns | 3026ns | 2264ns | 2774ns | 4187ns | -0.48% |
| mw_predicate_all_c_mw8_uni | 3688ns | 3413ns | 2642ns | 3255ns | 4861ns | +16.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 197ns | 145ns | 229ns | base | 0.324 |
| mw_chain_c_mw8_uni | 148ns | 101ns | 191ns | -25.28% | 0.434 |
| mw_chain_rev_c_mw8_uni | 144ns | 101ns | 182ns | -26.99% | 0.444 |
| mw_jumptable_c_mw8_uni | 137ns | 97ns | 182ns | -30.59% | 0.467 |
| mw_predicate_all_c_mw8_uni | 661ns | 480ns | 864ns | +234.85% | 0.097 |

## Performance model

- Peak throughput: **0.662 Gops/s** (mw_jumptable_c_mw8_uni; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw8_uni | 0.313 | 47.3% |
| mw_chain_c_mw8_uni | 0.429 | 64.8% |
| mw_chain_rev_c_mw8_uni | 0.428 | 64.6% |
| mw_jumptable_c_mw8_uni | 0.488 | 73.8% |
| mw_predicate_all_c_mw8_uni | 0.104 | 15.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw8_uni | 3175ns | 3175ns | base |
| mw_chain_c_mw8_uni | 3397ns | 3397ns | +6.97% |
| mw_chain_rev_c_mw8_uni | 3277ns | 3277ns | +3.19% |
| mw_jumptable_c_mw8_uni | 3160ns | 3160ns | -0.48% |
| mw_predicate_all_c_mw8_uni | 3688ns | 3688ns | +16.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 205ns | base | --- | [159, 229] | --- | --- | --- | --- |
| mw_chain_c_mw8_uni | 149ns | -46.3ns (-22.6%) | [-68, -35]ns | [102, 191] | YES | 0.0313 | 0.0313 | 0 |
| mw_chain_rev_c_mw8_uni | 150ns | -53.7ns (-26.3%) | [-72, -34]ns | [101, 182] | YES | 0.0313 | 0.0313 | 0 |
| mw_jumptable_c_mw8_uni | 131ns | -55.0ns (-26.9%) | [-92, -34]ns | [98, 182] | YES | 0.0313 | 0.0313 | 0 |
| mw_predicate_all_c_mw8_uni | 614ns | +424.1ns (+207.3%) | [+319, +648]ns | [505, 864] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw8_uni | mw_chain_c_mw8_uni | mw_chain_rev_c_mw8_uni | mw_jumptable_c_mw8_uni | mw_predicate_all_c_mw8_uni |
|---|---|---|---|---|---|
| 1 | 228ns | -21.3% | -31.9% | -47.4% | +132.3% |
| 2 | 230ns | -11.8% | -10.4% | -10.0% | +325.7% |
| 3 | 202ns | -21.2% | -22.0% | -22.4% | +271.0% |
| 4 | 145ns | -30.3% | -30.3% | -31.1% | +231.7% |
| 5 | 173ns | -40.0% | -41.5% | -44.1% | +228.2% |
| 6 | 207ns | -32.8% | -30.4% | -31.3% | +219.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw8_uni | 0.356 | moderate+ |
| mw_chain_c_mw8_uni | 0.512 | HIGH+ (drift/warm-up) |
| mw_chain_rev_c_mw8_uni | 0.360 | moderate+ |
| mw_jumptable_c_mw8_uni | 0.089 | ok |
| mw_predicate_all_c_mw8_uni | -0.073 | ok |

**Consistency summary:**

- **mw_chain_c_mw8_uni**: won 6/6, lost 0/6
- **mw_chain_rev_c_mw8_uni**: won 6/6, lost 0/6
- **mw_jumptable_c_mw8_uni**: won 6/6, lost 0/6
- **mw_predicate_all_c_mw8_uni**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 4.6ns | 197.4ns | 2.3% |  |
| mw_chain_c_mw8_uni | 5.0ns | 147.5ns | 3.4% |  |
| mw_chain_rev_c_mw8_uni | 4.8ns | 144.2ns | 3.3% |  |
| mw_jumptable_c_mw8_uni | 4.3ns | 137.0ns | 3.2% |  |
| mw_predicate_all_c_mw8_uni | 4.8ns | 661.1ns | 0.7% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw8_uni (n=6, range 144.6-228.9 ns)
    144.6 |########################################
    148.8 |
    153.0 |
    157.3 |
    161.5 |
    165.7 |
    169.9 |########################################
    174.1 |
    178.3 |
    182.6 |
    186.8 |
    191.0 |
    195.2 |
    199.4 |########################################
    203.6 |########################################
    207.9 |
    212.1 |
    216.3 |
    220.5 |
    224.7 |########################################
  (0 below, 1 above range)

mw_chain_c_mw8_uni (n=6, range 100.8-191.1 ns)
    100.8 |########################################
    105.3 |
    109.8 |
    114.3 |
    118.8 |
    123.4 |
    127.9 |
    132.4 |
    136.9 |####################
    141.4 |
    145.9 |
    150.4 |
    155.0 |
    159.5 |####################
    164.0 |
    168.5 |
    173.0 |
    177.5 |####################
    182.0 |
    186.5 |
  (0 below, 1 above range)

mw_chain_rev_c_mw8_uni (n=6, range 100.8-181.9 ns)
    100.8 |########################################
    104.9 |
    108.9 |
    113.0 |
    117.0 |
    121.1 |
    125.1 |
    129.2 |
    133.2 |
    137.3 |
    141.3 |####################
    145.4 |
    149.4 |
    153.5 |####################
    157.5 |####################
    161.6 |
    165.6 |
    169.7 |
    173.7 |
    177.8 |
  (0 below, 1 above range)

mw_jumptable_c_mw8_uni (n=6, range 96.7-181.9 ns)
     96.7 |########################################
    101.0 |
    105.2 |
    109.5 |
    113.7 |
    118.0 |####################
    122.3 |
    126.5 |
    130.8 |
    135.0 |
    139.3 |####################
    143.6 |
    147.8 |
    152.1 |
    156.3 |####################
    160.6 |
    164.9 |
    169.1 |
    173.4 |
    177.6 |
  (0 below, 1 above range)

mw_predicate_all_c_mw8_uni (n=6, range 479.6-864.4 ns)
    479.6 |########################################
    498.8 |
    518.1 |########################################
    537.3 |
    556.6 |########################################
    575.8 |
    595.0 |
    614.3 |
    633.5 |
    652.7 |########################################
    672.0 |
    691.2 |
    710.5 |
    729.7 |
    748.9 |########################################
    768.2 |
    787.4 |
    806.6 |
    825.9 |
    845.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **mw_chain_c_mw8_uni**: CV=25.3% (high variance, measurements may be unstable)
- **mw_chain_c_mw8_uni**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **mw_chain_rev_c_mw8_uni**: CV=25.1% (high variance, measurements may be unstable)
- **mw_jumptable_c_mw8_uni**: CV=27.6% (high variance, measurements may be unstable)
- **mw_predicate_all_c_mw8_uni**: CV=25.2% (high variance, measurements may be unstable)
