# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, leaf profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_leaf_none**

## Highlights

Baseline for all deltas below: **carrier_opt_leaf_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_leaf_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_leaf_none has the worst median (8.79 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_leaf_all at 3.58 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_leaf_all dominates: 11% faster than the next best (carrier_opt_leaf_cse)

carrier_opt_leaf_all (3.58 us) leads carrier_opt_leaf_cse (3.98 us) by 11%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_leaf_all beats baseline by 59% (significant)

carrier_opt_leaf_all is -5.16 us (59%) faster than baseline carrier_opt_leaf_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_leaf_none is an outlier: 2.5x slower than the field

carrier_opt_leaf_none (8.79 us) is 2.5x the fastest (3.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_opt_leaf_all, carrier_opt_leaf_cse, carrier_opt_leaf_eqsat, carrier_opt_leaf_cseeqsat} vs {carrier_opt_leaf_fold, carrier_opt_leaf_dce, carrier_opt_leaf_none} (90% apart)

The field splits into a fast tier {carrier_opt_leaf_all, carrier_opt_leaf_cse, carrier_opt_leaf_eqsat, carrier_opt_leaf_cseeqsat} and a slow tier {carrier_opt_leaf_fold, carrier_opt_leaf_dce, carrier_opt_leaf_none} with a 90% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_opt_leaf_all** at 3584.1 ns median (-59.2% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.45x (fastest 3584.1 ns, slowest 8785.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_leaf_all | 6155ns | 6175ns | 6033ns | 6138ns | 6240ns | -47.51% |
| carrier_opt_leaf_cse | 6371ns | 6451ns | 5684ns | 6441ns | 6609ns | -45.67% |
| carrier_opt_leaf_cseeqsat | 7143ns | 7130ns | 7063ns | 7122ns | 7215ns | -39.08% |
| carrier_opt_leaf_dce | 11576ns | 11288ns | 11143ns | 11266ns | 12258ns | -1.28% |
| carrier_opt_leaf_eqsat | 7019ns | 6994ns | 6932ns | 6985ns | 7114ns | -40.14% |
| carrier_opt_leaf_fold | 11227ns | 11214ns | 11099ns | 11208ns | 11318ns | -4.26% |
| carrier_opt_leaf_none | 11726ns | 11436ns | 11067ns | 11399ns | 12547ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_leaf_all | 3591ns | 3555ns | 3628ns | -60.62% | 0.071 |
| carrier_opt_leaf_cse | 3901ns | 3463ns | 4011ns | -57.22% | 0.066 |
| carrier_opt_leaf_cseeqsat | 4565ns | 4550ns | 4584ns | -49.94% | 0.056 |
| carrier_opt_leaf_dce | 8990ns | 8607ns | 9644ns | -1.42% | 0.028 |
| carrier_opt_leaf_eqsat | 4522ns | 4490ns | 4561ns | -50.41% | 0.057 |
| carrier_opt_leaf_fold | 8670ns | 8608ns | 8744ns | -4.92% | 0.030 |
| carrier_opt_leaf_none | 9119ns | 8611ns | 9930ns | base | 0.028 |

## Performance model

- Peak throughput: **0.074 Gops/s** (carrier_opt_leaf_cse; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_leaf_all | 0.071 | 96.6% |
| carrier_opt_leaf_cse | 0.064 | 87.1% |
| carrier_opt_leaf_cseeqsat | 0.056 | 76.0% |
| carrier_opt_leaf_dce | 0.029 | 39.8% |
| carrier_opt_leaf_eqsat | 0.057 | 76.7% |
| carrier_opt_leaf_fold | 0.030 | 40.0% |
| carrier_opt_leaf_none | 0.029 | 39.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_leaf_all | 6155ns | 6155ns | -47.51% |
| carrier_opt_leaf_cse | 6371ns | 6371ns | -45.67% |
| carrier_opt_leaf_cseeqsat | 7143ns | 7143ns | -39.08% |
| carrier_opt_leaf_dce | 11576ns | 11576ns | -1.28% |
| carrier_opt_leaf_eqsat | 7019ns | 7019ns | -40.14% |
| carrier_opt_leaf_fold | 11227ns | 11227ns | -4.26% |
| carrier_opt_leaf_none | 11726ns | 11726ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_leaf_none | 8786ns | base | --- | [8641, 9930] | --- | --- | --- | --- |
| carrier_opt_leaf_all | 3584ns | -5161.2ns (-58.7%) | [-6369, -5054]ns | [3561, 3628] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_leaf_cse | 3977ns | -4790.8ns (-54.5%) | [-6196, -4668]ns | [3714, 4011] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_leaf_cseeqsat | 4558ns | -4215.4ns (-48.0%) | [-5363, -4084]ns | [4553, 4584] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_leaf_dce | 8709ns | no significant difference | [-1220, +927]ns | [8616, 9644] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_leaf_eqsat | 4513ns | -4286.2ns (-48.8%) | [-5393, -4111]ns | [4493, 4561] | YES | 0.0469 | 0.0313 | 0 |
| carrier_opt_leaf_fold | 8655ns | no significant difference | [-1260, +52]ns | [8612, 8744] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_leaf_none | carrier_opt_leaf_all | carrier_opt_leaf_cse | carrier_opt_leaf_cseeqsat | carrier_opt_leaf_dce | carrier_opt_leaf_eqsat | carrier_opt_leaf_fold |
|---|---|---|---|---|---|---|---|
| 1 | 9495ns | -62.6% | -63.5% | -51.8% | -7.8% | -52.0% | -8.0% |
| 2 | 8611ns | -58.3% | -54.0% | -47.1% | -0.0% | -47.8% | +0.3% |
| 3 | 8810ns | -58.4% | -54.9% | -47.9% | -2.1% | -48.8% | -1.5% |
| 4 | 8672ns | -58.6% | -54.1% | -47.4% | +13.7% | -47.4% | +1.0% |
| 5 | 8761ns | -59.1% | -54.1% | -48.1% | +7.6% | -48.7% | -1.7% |
| 6 | 10365ns | -65.6% | -61.4% | -56.0% | -16.4% | -56.4% | -17.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_leaf_all | -0.017 | ok |
| carrier_opt_leaf_cse | 0.017 | ok |
| carrier_opt_leaf_cseeqsat | -0.234 | moderate- |
| carrier_opt_leaf_dce | 0.111 | ok |
| carrier_opt_leaf_eqsat | -0.479 | moderate- |
| carrier_opt_leaf_fold | -0.165 | ok |
| carrier_opt_leaf_none | -0.077 | ok |

**Consistency summary:**

- **carrier_opt_leaf_all**: won 6/6, lost 0/6
- **carrier_opt_leaf_cse**: won 6/6, lost 0/6
- **carrier_opt_leaf_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_leaf_dce**: won 3/6, lost 2/6
- **carrier_opt_leaf_eqsat**: won 6/6, lost 0/6
- **carrier_opt_leaf_fold**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_leaf_all | 87612.4ns | 3591.0ns | 2439.8% | HIGH |
| carrier_opt_leaf_cse | 87448.8ns | 3900.6ns | 2241.9% | HIGH |
| carrier_opt_leaf_cseeqsat | 86864.5ns | 4564.8ns | 1902.9% | HIGH |
| carrier_opt_leaf_dce | 88488.8ns | 8989.6ns | 984.3% | HIGH |
| carrier_opt_leaf_eqsat | 86976.3ns | 4522.4ns | 1923.2% | HIGH |
| carrier_opt_leaf_fold | 86991.4ns | 8670.3ns | 1003.3% | HIGH |
| carrier_opt_leaf_none | 88236.6ns | 9118.9ns | 967.6% | HIGH |

## Distribution (algo ns)

```
carrier_opt_leaf_all (n=6, range 3554.6-3627.7 ns)
   3554.6 |########################################
   3558.3 |
   3561.9 |
   3565.6 |########################################
   3569.2 |
   3572.9 |
   3576.5 |
   3580.2 |########################################
   3583.8 |########################################
   3587.5 |########################################
   3591.1 |
   3594.8 |
   3598.5 |
   3602.1 |
   3605.8 |
   3609.4 |
   3613.1 |
   3616.7 |
   3620.4 |
   3624.0 |
  (0 below, 1 above range)

carrier_opt_leaf_cse (n=6, range 3462.9-4010.8 ns)
   3462.9 |#############
   3490.3 |
   3517.7 |
   3545.1 |
   3572.5 |
   3599.9 |
   3627.3 |
   3654.7 |
   3682.1 |
   3709.5 |
   3736.9 |
   3764.3 |
   3791.7 |
   3819.1 |
   3846.5 |
   3873.9 |
   3901.3 |
   3928.7 |
   3956.1 |########################################
   3983.5 |#############
  (0 below, 1 above range)

carrier_opt_leaf_cseeqsat (n=6, range 4550.0-4583.5 ns)
   4550.0 |####################
   4551.7 |
   4553.4 |
   4555.0 |####################
   4556.7 |########################################
   4558.4 |
   4560.1 |
   4561.7 |
   4563.4 |
   4565.1 |
   4566.8 |
   4568.5 |
   4570.1 |
   4571.8 |
   4573.5 |
   4575.2 |####################
   4576.8 |
   4578.5 |
   4580.2 |
   4581.9 |
  (0 below, 1 above range)

carrier_opt_leaf_dce (n=6, range 8607.1-9643.5 ns)
   8607.1 |########################################
   8658.9 |####################
   8710.7 |####################
   8762.6 |
   8814.4 |
   8866.2 |
   8918.0 |
   8969.9 |
   9021.7 |
   9073.5 |
   9125.3 |
   9177.1 |
   9229.0 |
   9280.8 |
   9332.6 |
   9384.4 |####################
   9436.3 |
   9488.1 |
   9539.9 |
   9591.7 |
  (0 below, 1 above range)

carrier_opt_leaf_eqsat (n=6, range 4490.0-4560.8 ns)
   4490.0 |########################################
   4493.5 |########################################
   4497.1 |
   4500.6 |
   4504.2 |
   4507.7 |########################################
   4511.2 |
   4514.8 |########################################
   4518.3 |
   4521.9 |
   4525.4 |
   4528.9 |
   4532.5 |
   4536.0 |
   4539.6 |
   4543.1 |
   4546.6 |
   4550.2 |
   4553.7 |########################################
   4557.3 |
  (0 below, 1 above range)

carrier_opt_leaf_fold (n=6, range 8607.5-8744.0 ns)
   8607.5 |########################################
   8614.3 |########################################
   8621.1 |
   8628.0 |########################################
   8634.8 |
   8641.6 |
   8648.4 |
   8655.3 |
   8662.1 |
   8668.9 |
   8675.7 |########################################
   8682.5 |
   8689.4 |
   8696.2 |
   8703.0 |
   8709.8 |
   8716.7 |
   8723.5 |
   8730.3 |########################################
   8737.1 |
  (0 below, 1 above range)

carrier_opt_leaf_none (n=6, range 8610.8-9929.8 ns)
   8610.8 |########################################
   8676.8 |
   8742.7 |####################
   8808.6 |####################
   8874.6 |
   8940.5 |
   9006.5 |
   9072.4 |
   9138.4 |
   9204.3 |
   9270.3 |
   9336.2 |
   9402.2 |
   9468.1 |####################
   9534.1 |
   9600.0 |
   9666.0 |
   9731.9 |
   9797.9 |
   9863.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_leaf_all**: bridge=2447.1% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cse**: bridge=2198.7% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cseeqsat**: bridge=1904.8% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_dce**: bridge=1005.4% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_eqsat**: bridge=1927.2% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_fold**: bridge=1004.5% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_none**: bridge=1000.3% of algo (FFI overhead may distort results)
