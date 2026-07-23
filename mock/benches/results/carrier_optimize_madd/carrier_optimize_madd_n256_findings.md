# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, madd profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_madd_none**

## Highlights

Baseline for all deltas below: **carrier_opt_madd_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_madd_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_madd_none has the worst median (9.40 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_madd_all at 613 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_madd_all dominates: 1018% faster than the next best (carrier_opt_madd_fold)

carrier_opt_madd_all (613 ns) leads carrier_opt_madd_fold (6.86 us) by 1018%, a clear separation rather than a photo finish. CV 5.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_madd_all beats baseline by 94% (significant)

carrier_opt_madd_all is -8.81 us (94%) faster than baseline carrier_opt_madd_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_madd_none is an outlier: 15.3x slower than the field

carrier_opt_madd_none (9.40 us) is 15.3x the fastest (613 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_opt_madd_all} vs {carrier_opt_madd_fold, carrier_opt_madd_canon, carrier_opt_madd_cse, carrier_opt_madd_dce, carrier_opt_madd_none} (1018% apart)

The field splits into a fast tier {carrier_opt_madd_all} and a slow tier {carrier_opt_madd_fold, carrier_opt_madd_canon, carrier_opt_madd_cse, carrier_opt_madd_dce, carrier_opt_madd_none} with a 1018% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 15.3x the fastest

Fastest carrier_opt_madd_all (613 ns) to slowest carrier_opt_madd_none (9.40 us): 15.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_madd_all** at 613.1 ns median (-93.5% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 15.33x (fastest 613.1 ns, slowest 9402.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_madd_all | 2963ns | 2994ns | 2765ns | 2919ns | 3128ns | -74.81% |
| carrier_opt_madd_canon | 11351ns | 10983ns | 10858ns | 10954ns | 12193ns | -3.48% |
| carrier_opt_madd_cse | 11460ns | 11425ns | 10704ns | 11198ns | 12231ns | -2.55% |
| carrier_opt_madd_dce | 11523ns | 11505ns | 11004ns | 11363ns | 12022ns | -2.02% |
| carrier_opt_madd_fold | 9146ns | 9232ns | 8434ns | 9013ns | 9703ns | -22.23% |
| carrier_opt_madd_none | 11760ns | 11635ns | 11056ns | 11462ns | 12559ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_madd_all | 598ns | 547ns | 626ns | -93.66% | 0.428 |
| carrier_opt_madd_canon | 9050ns | 8697ns | 9663ns | -4.05% | 0.028 |
| carrier_opt_madd_cse | 9039ns | 8384ns | 9660ns | -4.17% | 0.028 |
| carrier_opt_madd_dce | 9238ns | 8771ns | 9645ns | -2.06% | 0.028 |
| carrier_opt_madd_fold | 6731ns | 6266ns | 7063ns | -28.64% | 0.038 |
| carrier_opt_madd_none | 9432ns | 8897ns | 9967ns | base | 0.027 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_madd_all | 254200 | 1613304 | 0.158 | 0.83× |
| carrier_opt_madd_canon | 301289 | 1328939 | 0.227 | 0.98× |
| carrier_opt_madd_cse | 301907 | 1414602 | 0.213 | 0.98× |
| carrier_opt_madd_dce | 307072 | 1462661 | 0.210 | 1.00× |
| carrier_opt_madd_fold | 290727 | 1782625 | 0.163 | 0.95× |
| carrier_opt_madd_none | 306899 | 1440341 | 0.213 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.468 Gops/s** (carrier_opt_madd_all; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_madd_all | 0.418 | 89.2% |
| carrier_opt_madd_canon | 0.029 | 6.2% |
| carrier_opt_madd_cse | 0.028 | 6.0% |
| carrier_opt_madd_dce | 0.028 | 5.9% |
| carrier_opt_madd_fold | 0.037 | 8.0% |
| carrier_opt_madd_none | 0.027 | 5.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_madd_all | 2963ns | 2963ns | -74.81% |
| carrier_opt_madd_canon | 11351ns | 11351ns | -3.48% |
| carrier_opt_madd_cse | 11460ns | 11460ns | -2.55% |
| carrier_opt_madd_dce | 11523ns | 11523ns | -2.02% |
| carrier_opt_madd_fold | 9146ns | 9146ns | -22.23% |
| carrier_opt_madd_none | 11760ns | 11760ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_madd_none | 9402ns | base | --- | [8927, 9967] | --- | --- | --- | --- |
| carrier_opt_madd_all | 613ns | -8812.9ns (-93.7%) | [-9354, -8335]ns | [555, 626] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_madd_canon | 8776ns | -165.0ns (-1.8%) | [-943, -38]ns | [8711, 9663] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| carrier_opt_madd_cse | 9037ns | -445.6ns (-4.7%) | [-637, -96]ns | [8420, 9660] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_madd_dce | 9267ns | no significant difference | [-389, +41]ns | [8801, 9645] | no | 0.2188 | 0.2188 | 0 |
| carrier_opt_madd_fold | 6855ns | -2777.5ns (-29.5%) | [-3134, -2193]ns | [6274, 7063] | YES (adj: no) | 0.0521 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_madd_none | carrier_opt_madd_all | carrier_opt_madd_canon | carrier_opt_madd_cse | carrier_opt_madd_dce | carrier_opt_madd_fold |
|---|---|---|---|---|---|---|
| 1 | 9190ns | -94.1% | -5.1% | -8.0% | -4.6% | -31.8% |
| 2 | 8897ns | -93.0% | -2.2% | -5.8% | +2.3% | -19.3% |
| 3 | 9788ns | -93.8% | +0.1% | -0.8% | -3.7% | -29.4% |
| 4 | 8958ns | -93.7% | -1.5% | -4.2% | -1.4% | -29.9% |
| 5 | 10145ns | -93.9% | -14.0% | -5.3% | -3.0% | -33.0% |
| 6 | 9615ns | -93.4% | -0.9% | -1.2% | -1.7% | -27.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_madd_all | -0.209 | moderate- |
| carrier_opt_madd_canon | -0.340 | moderate- |
| carrier_opt_madd_cse | -0.190 | ok |
| carrier_opt_madd_dce | -0.184 | ok |
| carrier_opt_madd_fold | -0.320 | moderate- |
| carrier_opt_madd_none | -0.354 | moderate- |

**Consistency summary:**

- **carrier_opt_madd_all**: won 6/6, lost 0/6
- **carrier_opt_madd_canon**: won 5/6, lost 0/6
- **carrier_opt_madd_cse**: won 6/6, lost 0/6
- **carrier_opt_madd_dce**: won 5/6, lost 1/6
- **carrier_opt_madd_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_madd_all | 83847.9ns | 598.0ns | 14021.4% | HIGH |
| carrier_opt_madd_canon | 88112.0ns | 9049.8ns | 973.6% | HIGH |
| carrier_opt_madd_cse | 90690.6ns | 9039.1ns | 1003.3% | HIGH |
| carrier_opt_madd_dce | 90128.2ns | 9237.6ns | 975.7% | HIGH |
| carrier_opt_madd_fold | 90297.0ns | 6730.5ns | 1341.6% | HIGH |
| carrier_opt_madd_none | 90401.8ns | 9432.0ns | 958.5% | HIGH |

## Distribution (algo ns)

```
carrier_opt_madd_all (n=6, range 546.7-625.7 ns)
    546.7 |####################
    550.6 |
    554.6 |
    558.5 |
    562.5 |####################
    566.4 |
    570.4 |
    574.3 |
    578.3 |
    582.2 |
    586.2 |
    590.1 |
    594.1 |
    598.0 |
    602.0 |
    605.9 |####################
    609.9 |
    613.8 |
    617.8 |########################################
    621.7 |
  (0 below, 1 above range)

carrier_opt_madd_canon (n=6, range 8697.1-9663.0 ns)
   8697.1 |########################################
   8745.4 |
   8793.7 |#############
   8842.0 |
   8890.3 |
   8938.6 |
   8986.9 |
   9035.1 |
   9083.4 |
   9131.7 |
   9180.0 |
   9228.3 |
   9276.6 |
   9324.9 |
   9373.2 |
   9421.5 |
   9469.8 |
   9518.1 |#############
   9566.4 |
   9614.7 |
  (0 below, 1 above range)

carrier_opt_madd_cse (n=6, range 8383.8-9659.8 ns)
   8383.8 |########################################
   8447.6 |########################################
   8511.4 |
   8575.2 |########################################
   8639.0 |
   8702.8 |
   8766.6 |
   8830.4 |
   8894.2 |
   8958.0 |
   9021.8 |
   9085.6 |
   9149.4 |
   9213.2 |
   9277.0 |
   9340.8 |
   9404.6 |
   9468.4 |########################################
   9532.2 |
   9596.0 |########################################
  (0 below, 1 above range)

carrier_opt_madd_dce (n=6, range 8770.8-9645.5 ns)
   8770.8 |####################
   8814.5 |####################
   8858.3 |
   8902.0 |
   8945.7 |
   8989.5 |
   9033.2 |
   9076.9 |####################
   9120.7 |
   9164.4 |
   9208.1 |
   9251.9 |
   9295.6 |
   9339.3 |
   9383.1 |
   9426.8 |########################################
   9470.5 |
   9514.3 |
   9558.0 |
   9601.7 |
  (0 below, 1 above range)

carrier_opt_madd_fold (n=6, range 6265.8-7062.9 ns)
   6265.8 |########################################
   6305.7 |
   6345.5 |
   6385.4 |
   6425.2 |
   6465.1 |
   6504.9 |
   6544.8 |
   6584.6 |
   6624.5 |
   6664.4 |
   6704.2 |
   6744.1 |
   6783.9 |####################
   6823.8 |
   6863.6 |
   6903.5 |####################
   6943.3 |####################
   6983.2 |
   7023.0 |
  (0 below, 1 above range)

carrier_opt_madd_none (n=6, range 8896.7-9966.8 ns)
   8896.7 |########################################
   8950.2 |########################################
   9003.7 |
   9057.2 |
   9110.7 |
   9164.2 |########################################
   9217.7 |
   9271.3 |
   9324.8 |
   9378.3 |
   9431.8 |
   9485.3 |
   9538.8 |
   9592.3 |########################################
   9645.8 |
   9699.3 |
   9752.8 |########################################
   9806.3 |
   9859.8 |
   9913.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_madd_all**: bridge=13679.6% of algo (FFI overhead may distort results)
- **carrier_opt_madd_canon**: bridge=1000.3% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cse**: bridge=1008.9% of algo (FFI overhead may distort results)
- **carrier_opt_madd_dce**: bridge=980.5% of algo (FFI overhead may distort results)
- **carrier_opt_madd_fold**: bridge=1328.2% of algo (FFI overhead may distort results)
- **carrier_opt_madd_none**: bridge=953.9% of algo (FFI overhead may distort results)
