# Branch strategies, heavy-arm, biased25: ~25% taken (b<64)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_biased25**

## Key findings

- **Baseline (br_branch_h_biased25) is the fastest** at 2035.2 ns median
- 2 variants significantly slower than baseline
- Spread: 2.29x (fastest 2035.2 ns, slowest 4661.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_biased25 | 4447ns | 4378ns | 4073ns | 4324ns | 4818ns | base |
| br_predicate_h_biased25 | 6977ns | 7086ns | 6318ns | 6845ns | 7506ns | +56.91% |
| br_profiled_hot_h_biased25 | 4762ns | 4754ns | 4348ns | 4693ns | 5072ns | +7.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_biased25 | 2043ns | 1883ns | 2202ns | base | 0.125 |
| br_predicate_h_biased25 | 4565ns | 4130ns | 4904ns | +123.48% | 0.056 |
| br_profiled_hot_h_biased25 | 2403ns | 2159ns | 2567ns | +17.64% | 0.107 |

## Performance model

- Peak throughput: **0.136 Gops/s** (br_branch_h_biased25; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_biased25 | 0.126 | 92.5% |
| br_predicate_h_biased25 | 0.055 | 40.4% |
| br_profiled_hot_h_biased25 | 0.106 | 78.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_biased25 | 4447ns | 4447ns | base |
| br_predicate_h_biased25 | 6977ns | 6977ns | +56.91% |
| br_profiled_hot_h_biased25 | 4762ns | 4762ns | +7.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_biased25 | 2035ns | base | --- | [1891, 2202] | --- | --- | --- | --- |
| br_predicate_h_biased25 | 4661ns | +2609.2ns (+128.2%) | [+2191, +2767]ns | [4132, 4904] | YES | 0.0313 | 0.0313 | 0 |
| br_profiled_hot_h_biased25 | 2405ns | +368.3ns (+18.1%) | [+254, +459]ns | [2238, 2567] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_biased25 | br_predicate_h_biased25 | br_profiled_hot_h_biased25 |
|---|---|---|---|
| 1 | 1883ns | +119.3% | +25.5% |
| 2 | 1899ns | +132.8% | +13.7% |
| 3 | 1997ns | +107.0% | +16.0% |
| 4 | 2073ns | +136.6% | +21.1% |
| 5 | 2199ns | +122.9% | +11.3% |
| 6 | 2206ns | +122.3% | +18.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_biased25 | 0.583 | HIGH+ (drift/warm-up) |
| br_predicate_h_biased25 | 0.279 | moderate+ |
| br_profiled_hot_h_biased25 | 0.273 | moderate+ |

**Consistency summary:**

- **br_predicate_h_biased25**: won 0/6, lost 6/6
- **br_profiled_hot_h_biased25**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_biased25 | 4.3ns | 2042.9ns | 0.2% |  |
| br_predicate_h_biased25 | 4.7ns | 4565.5ns | 0.1% |  |
| br_profiled_hot_h_biased25 | 4.4ns | 2403.3ns | 0.2% |  |

## Distribution (algo ns)

```
br_branch_h_biased25 (n=6, range 1883.3-2202.3 ns)
   1883.3 |########################################
   1899.2 |
   1915.2 |
   1931.2 |
   1947.1 |
   1963.0 |
   1979.0 |
   1995.0 |####################
   2010.9 |
   2026.9 |
   2042.8 |
   2058.8 |####################
   2074.7 |
   2090.7 |
   2106.6 |
   2122.6 |
   2138.5 |
   2154.5 |
   2170.4 |
   2186.4 |####################
  (0 below, 1 above range)

br_predicate_h_biased25 (n=6, range 4129.6-4903.6 ns)
   4129.6 |########################################
   4168.3 |
   4207.0 |
   4245.7 |
   4284.4 |
   4323.1 |
   4361.8 |
   4400.5 |####################
   4439.2 |
   4477.9 |
   4516.6 |
   4555.3 |
   4594.0 |
   4632.7 |
   4671.4 |
   4710.1 |
   4748.8 |
   4787.5 |
   4826.2 |
   4864.9 |########################################
  (0 below, 1 above range)

br_profiled_hot_h_biased25 (n=6, range 2159.2-2567.3 ns)
   2159.2 |########################################
   2179.6 |
   2200.0 |
   2220.4 |
   2240.8 |
   2261.2 |
   2281.6 |
   2302.0 |########################################
   2322.4 |
   2342.8 |
   2363.2 |########################################
   2383.7 |
   2404.1 |
   2424.5 |
   2444.9 |########################################
   2465.3 |
   2485.7 |
   2506.1 |########################################
   2526.5 |
   2546.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **br_branch_h_biased25**: autocorrelation=0.58 (measurement drift or warm-up artifact)
