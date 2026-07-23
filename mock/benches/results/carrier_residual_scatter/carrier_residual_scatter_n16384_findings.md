# Residual encoding: predecoded register/SSA vs stack bytecode, scatter profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_scatter_register**

## Highlights

Baseline for all deltas below: **carrier_res_scatter_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_scatter_register dominates: 42% faster than the next best (carrier_res_scatter_stack)

carrier_res_scatter_register (2.00 ms) leads carrier_res_scatter_stack (2.84 ms) by 42%, a clear separation rather than a photo finish. CV 0.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_scatter_register)

The baseline carrier_res_scatter_register is the fastest (2.00 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_scatter_register) is the fastest** at 2002934.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.42x (fastest 2002934.1 ns, slowest 2838813.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 2006091ns | 2006545ns | 1995891ns | 2005276ns | 2012414ns | base |
| carrier_res_scatter_stack | 2853356ns | 2841536ns | 2813931ns | 2836994ns | 2897611ns | +42.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_scatter_register | 2002453ns | 1992278ns | 2008675ns | base | 0.008 |
| carrier_res_scatter_stack | 2850487ns | 2811271ns | 2894354ns | +42.35% | 0.006 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_scatter_register | 12805150 | 9737762 | 1.315 | 1.00× |
| carrier_res_scatter_stack | 19271696 | 36188811 | 0.533 | 1.50× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_res_scatter_register; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_scatter_register | 0.008 | 99.5% |
| carrier_res_scatter_stack | 0.006 | 70.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_scatter_register | 2006091ns | 2006091ns | base |
| carrier_res_scatter_stack | 2853356ns | 2853356ns | +42.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 2002934ns | base | --- | [1995751, 2008675] | --- | --- | --- | --- |
| carrier_res_scatter_stack | 2838814ns | +833608.9ns (+41.6%) | [+815637, +894854]ns | [2818294, 2894354] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_scatter_register | carrier_res_scatter_stack |
|---|---|---|
| 1 | 2006720ns | +42.0% |
| 2 | 2010630ns | +41.6% |
| 3 | 1992278ns | +47.6% |
| 4 | 1999780ns | +41.5% |
| 5 | 1999225ns | +41.3% |
| 6 | 2006089ns | +40.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_scatter_register | -0.110 | ok |
| carrier_res_scatter_stack | -0.055 | ok |

**Consistency summary:**

- **carrier_res_scatter_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_scatter_register | 2086907.8ns | 2002453.5ns | 104.2% | HIGH |
| carrier_res_scatter_stack | 3334315.0ns | 2850486.8ns | 117.0% | HIGH |

## Distribution (algo ns)

```
carrier_res_scatter_register (n=6, range 1992278.3-2008674.8 ns)
  1992278.3 |########################################
  1993098.1 |
  1993917.9 |
  1994737.8 |
  1995557.6 |
  1996377.4 |
  1997197.2 |
  1998017.1 |
  1998836.9 |########################################
  1999656.7 |########################################
  2000476.6 |
  2001296.4 |
  2002116.2 |
  2002936.0 |
  2003755.9 |
  2004575.7 |
  2005395.5 |########################################
  2006215.3 |########################################
  2007035.2 |
  2007855.0 |
  (0 below, 1 above range)

carrier_res_scatter_stack (n=6, range 2811271.2-2894353.5 ns)
  2811271.2 |####################
  2815425.3 |
  2819579.4 |
  2823733.5 |####################
  2827887.7 |####################
  2832041.8 |
  2836195.9 |
  2840350.0 |
  2844504.1 |########################################
  2848658.2 |
  2852812.4 |
  2856966.5 |
  2861120.6 |
  2865274.7 |
  2869428.8 |
  2873582.9 |
  2877737.0 |
  2881891.2 |
  2886045.3 |
  2890199.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_scatter_register**: bridge=104.4% of algo (FFI overhead may distort results)
- **carrier_res_scatter_stack**: bridge=117.0% of algo (FFI overhead may distort results)
