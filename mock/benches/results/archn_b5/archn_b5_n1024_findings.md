# Per-branch strategy (NATIVE tier): archetype 5

5 variants, 6 samples per variant.
Baseline: **an_b5_table**

## Highlights

Baseline for all deltas below: **an_b5_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b5_prof shows warm-up / thermal drift (autocorr +0.55)

an_b5_prof's per-pass series has lag-1 autocorrelation +0.55, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (an_b5_table)

The baseline an_b5_table is the fastest (8.40 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader an_b5_table vs stability leader an_b5_tree (+3% speed for 1.2x steadier)

an_b5_table is fastest (8.40 us, CV 6.9%); an_b5_tree gives up 2.8% median for 1.2x lower variance (CV 5.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (an_b5_table) is the fastest** at 8404.8 ns median
- 2 variants significantly slower than baseline
- Spread: 1.28x (fastest 8404.8 ns, slowest 10788.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b5_pred | 13065ns | 13373ns | 11272ns | 13341ns | 13549ns | +22.43% |
| an_b5_prof | 11160ns | 11504ns | 9938ns | 10999ns | 12014ns | +4.58% |
| an_b5_seq | 11337ns | 11832ns | 10123ns | 11298ns | 12004ns | +6.24% |
| an_b5_table | 10671ns | 10890ns | 9382ns | 10653ns | 11344ns | base |
| an_b5_tree | 11002ns | 11222ns | 9718ns | 11042ns | 11583ns | +3.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b5_pred | 10542ns | 9098ns | 10943ns | +28.06% | 0.097 |
| an_b5_prof | 8759ns | 7788ns | 9434ns | +6.41% | 0.117 |
| an_b5_seq | 8873ns | 7976ns | 9349ns | +7.80% | 0.115 |
| an_b5_table | 8232ns | 7229ns | 8760ns | base | 0.124 |
| an_b5_tree | 8503ns | 7530ns | 8972ns | +3.29% | 0.120 |

## Performance model

- Peak throughput: **0.142 Gops/s** (an_b5_table; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b5_pred | 0.095 | 67.0% |
| an_b5_prof | 0.114 | 80.2% |
| an_b5_seq | 0.111 | 78.1% |
| an_b5_table | 0.122 | 86.0% |
| an_b5_tree | 0.118 | 83.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b5_pred | 13065ns | 13065ns | +22.43% |
| an_b5_prof | 11160ns | 11160ns | +4.58% |
| an_b5_seq | 11337ns | 11337ns | +6.24% |
| an_b5_table | 10671ns | 10671ns | base |
| an_b5_tree | 11002ns | 11002ns | +3.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b5_table | 8405ns | base | --- | [7530, 8760] | --- | --- | --- | --- |
| an_b5_pred | 10789ns | +2088.2ns (+24.8%) | [+1592, +3249]ns | [9893, 10943] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b5_prof | 9017ns | no significant difference | [-47, +1029]ns | [7828, 9434] | no | 0.6875 | 0.6875 | 0 |
| an_b5_seq | 9261ns | +606.4ns (+7.2%) | [+356, +963]ns | [8010, 9349] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b5_tree | 8641ns | no significant difference | [-227, +827]ns | [7894, 8972] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b5_table | an_b5_pred | an_b5_prof | an_b5_seq | an_b5_tree |
|---|---|---|---|---|---|
| 1 | 7832ns | +16.2% | -0.6% | +2.7% | -3.9% |
| 2 | 7229ns | +50.5% | +8.8% | +10.3% | +14.2% |
| 3 | 8769ns | +21.9% | -0.6% | +5.7% | +3.8% |
| 4 | 8075ns | +35.3% | +17.4% | +14.6% | +7.7% |
| 5 | 8751ns | +22.2% | +6.4% | +7.1% | +1.0% |
| 6 | 8735ns | +25.5% | +7.5% | +6.8% | -1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b5_pred | -0.101 | ok |
| an_b5_prof | 0.547 | HIGH+ (drift/warm-up) |
| an_b5_seq | 0.425 | moderate+ |
| an_b5_table | -0.021 | ok |
| an_b5_tree | 0.199 | ok |

**Consistency summary:**

- **an_b5_pred**: won 0/6, lost 6/6
- **an_b5_prof**: won 2/6, lost 4/6
- **an_b5_seq**: won 0/6, lost 6/6
- **an_b5_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b5_pred | 5.9ns | 10541.6ns | 0.1% |  |
| an_b5_prof | 5.1ns | 8759.2ns | 0.1% |  |
| an_b5_seq | 5.7ns | 8873.4ns | 0.1% |  |
| an_b5_table | 6.0ns | 8231.8ns | 0.1% |  |
| an_b5_tree | 6.4ns | 8502.6ns | 0.1% |  |

## Distribution (algo ns)

```
an_b5_pred (n=6, range 9098.3-10943.4 ns)
   9098.3 |####################
   9190.6 |
   9282.8 |
   9375.1 |
   9467.3 |
   9559.6 |
   9651.8 |
   9744.1 |
   9836.3 |
   9928.6 |
  10020.8 |
  10113.1 |
  10205.3 |
  10297.6 |
  10389.8 |
  10482.1 |
  10574.3 |
  10666.6 |########################################
  10758.8 |
  10851.1 |########################################
  (0 below, 1 above range)

an_b5_prof (n=6, range 7788.3-9433.5 ns)
   7788.3 |########################################
   7870.6 |
   7952.8 |
   8035.1 |
   8117.4 |
   8199.6 |
   8281.9 |
   8364.1 |
   8446.4 |
   8528.7 |
   8610.9 |
   8693.2 |####################
   8775.4 |
   8857.7 |
   8940.0 |
   9022.2 |
   9104.5 |
   9186.8 |
   9269.0 |####################
   9351.3 |####################
  (0 below, 1 above range)

an_b5_seq (n=6, range 7975.8-9349.1 ns)
   7975.8 |####################
   8044.5 |####################
   8113.1 |
   8181.8 |
   8250.5 |
   8319.1 |
   8387.8 |
   8456.5 |
   8525.1 |
   8593.8 |
   8662.5 |
   8731.1 |
   8799.8 |
   8868.5 |
   8937.1 |
   9005.8 |
   9074.5 |
   9143.1 |
   9211.8 |########################################
   9280.5 |####################
  (0 below, 1 above range)

an_b5_table (n=6, range 7228.8-8760.0 ns)
   7228.8 |####################
   7305.4 |
   7381.9 |
   7458.5 |
   7535.0 |
   7611.6 |
   7688.2 |
   7764.7 |####################
   7841.3 |
   7917.8 |
   7994.4 |
   8071.0 |####################
   8147.5 |
   8224.1 |
   8300.6 |
   8377.2 |
   8453.8 |
   8530.3 |
   8606.9 |
   8683.4 |########################################
  (0 below, 1 above range)

an_b5_tree (n=6, range 7529.6-8972.5 ns)
   7529.6 |########################################
   7601.7 |
   7673.9 |
   7746.0 |
   7818.2 |
   7890.3 |
   7962.5 |
   8034.6 |
   8106.8 |
   8178.9 |
   8251.0 |########################################
   8323.2 |
   8395.3 |
   8467.5 |
   8539.6 |########################################
   8611.8 |
   8683.9 |########################################
   8756.1 |
   8828.2 |########################################
   8900.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **an_b5_prof**: autocorrelation=0.55 (measurement drift or warm-up artifact)
