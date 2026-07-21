# Multiway branch strategies, heavy-arm, mw3_skew: 3-way, skewed to arm 0 (~80%)

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw3_skew**

## Key findings

- **Fastest: mw_jumptable_h_mw3_skew** at 72564.8 ns median (-2.3% vs baseline)
- Spread: 1.02x (fastest 72564.8 ns, slowest 74286.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 75943ns | 76779ns | 69080ns | 74600ns | 81388ns | base |
| mw_chain_h_mw3_skew | 75221ns | 75366ns | 68914ns | 73279ns | 81288ns | -0.95% |
| mw_chain_rev_h_mw3_skew | 76085ns | 76384ns | 69686ns | 74376ns | 81846ns | +0.19% |
| mw_jumptable_h_mw3_skew | 75011ns | 75021ns | 69688ns | 73311ns | 80224ns | -1.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 73481ns | 66835ns | 78729ns | base | 0.056 |
| mw_chain_h_mw3_skew | 72815ns | 66770ns | 78678ns | -0.91% | 0.056 |
| mw_chain_rev_h_mw3_skew | 73611ns | 67460ns | 79112ns | +0.18% | 0.056 |
| mw_jumptable_h_mw3_skew | 72614ns | 67516ns | 77691ns | -1.18% | 0.056 |

## Performance model

- Peak throughput: **0.061 Gops/s** (mw_chain_h_mw3_skew; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw3_skew | 0.055 | 89.9% |
| mw_chain_h_mw3_skew | 0.056 | 91.5% |
| mw_chain_rev_h_mw3_skew | 0.055 | 90.2% |
| mw_jumptable_h_mw3_skew | 0.056 | 92.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw3_skew | 75943ns | 75943ns | base |
| mw_chain_h_mw3_skew | 75221ns | 75221ns | -0.95% |
| mw_chain_rev_h_mw3_skew | 76085ns | 76085ns | +0.19% |
| mw_jumptable_h_mw3_skew | 75011ns | 75011ns | -1.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 74286ns | base | --- | [67429, 78729] | --- | --- | --- | --- |
| mw_chain_h_mw3_skew | 72946ns | no significant difference | [-1950, +559]ns | [66821, 78678] | no | 1.0000 | 0.6875 | 0 |
| mw_chain_rev_h_mw3_skew | 73987ns | no significant difference | [-910, +994]ns | [67734, 79112] | no | 1.0000 | 1.0000 | 0 |
| mw_jumptable_h_mw3_skew | 72565ns | no significant difference | [-2332, +549]ns | [67588, 77691] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw3_skew | mw_chain_h_mw3_skew | mw_chain_rev_h_mw3_skew | mw_jumptable_h_mw3_skew |
|---|---|---|---|---|
| 1 | 70660ns | -3.1% | -0.9% | -3.3% |
| 2 | 68022ns | -1.7% | -0.0% | -0.5% |
| 3 | 66835ns | -0.1% | +0.9% | +1.0% |
| 4 | 77913ns | +0.1% | +0.9% | +0.5% |
| 5 | 78324ns | +1.3% | +1.7% | -1.6% |
| 6 | 79134ns | -2.1% | -1.5% | -3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw3_skew | 0.453 | moderate+ |
| mw_chain_h_mw3_skew | 0.521 | HIGH+ (drift/warm-up) |
| mw_chain_rev_h_mw3_skew | 0.494 | moderate+ |
| mw_jumptable_h_mw3_skew | 0.441 | moderate+ |

**Consistency summary:**

- **mw_chain_h_mw3_skew**: won 3/6, lost 1/6
- **mw_chain_rev_h_mw3_skew**: won 2/6, lost 3/6
- **mw_jumptable_h_mw3_skew**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw3_skew | 4.2ns | 73481.4ns | 0.0% |  |
| mw_chain_h_mw3_skew | 3.3ns | 72815.1ns | 0.0% |  |
| mw_chain_rev_h_mw3_skew | 3.8ns | 73611.0ns | 0.0% |  |
| mw_jumptable_h_mw3_skew | 3.2ns | 72614.3ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw3_skew (n=6, range 66835.4-78729.0 ns)
  66835.4 |########################################
  67430.1 |########################################
  68024.8 |
  68619.4 |
  69214.1 |
  69808.8 |
  70403.5 |########################################
  70998.2 |
  71592.8 |
  72187.5 |
  72782.2 |
  73376.9 |
  73971.6 |
  74566.2 |
  75160.9 |
  75755.6 |
  76350.3 |
  76945.0 |
  77539.6 |########################################
  78134.3 |########################################
  (0 below, 1 above range)

mw_chain_h_mw3_skew (n=6, range 66770.4-78678.1 ns)
  66770.4 |########################################
  67365.8 |
  67961.2 |####################
  68556.6 |
  69151.9 |
  69747.3 |
  70342.7 |
  70938.1 |
  71533.5 |
  72128.9 |
  72724.2 |
  73319.6 |
  73915.0 |
  74510.4 |
  75105.8 |
  75701.2 |
  76296.6 |
  76891.9 |####################
  77487.3 |####################
  78082.7 |
  (0 below, 1 above range)

mw_chain_rev_h_mw3_skew (n=6, range 67459.6-79112.2 ns)
  67459.6 |########################################
  68042.2 |
  68624.9 |
  69207.5 |
  69790.1 |####################
  70372.8 |
  70955.4 |
  71538.0 |
  72120.7 |
  72703.3 |
  73285.9 |
  73868.6 |
  74451.2 |
  75033.8 |
  75616.5 |
  76199.1 |
  76781.7 |
  77364.4 |####################
  77947.0 |
  78529.6 |####################
  (0 below, 1 above range)

mw_jumptable_h_mw3_skew (n=6, range 67515.8-77690.6 ns)
  67515.8 |########################################
  68024.5 |####################
  68533.3 |
  69042.0 |
  69550.8 |
  70059.5 |
  70568.2 |
  71077.0 |
  71585.7 |
  72094.5 |
  72603.2 |
  73111.9 |
  73620.7 |
  74129.4 |
  74638.2 |
  75146.9 |
  75655.6 |
  76164.4 |
  76673.1 |########################################
  77181.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **mw_chain_h_mw3_skew**: autocorrelation=0.52 (measurement drift or warm-up artifact)
