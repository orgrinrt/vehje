# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, real profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_real_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_real_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_real_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_real_interp has the worst median (10.74 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_real_stencil at 3.73 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_real_stencil beats baseline by 66% (significant)

carrier_nat_real_stencil is -7.11 us (66%) faster than baseline carrier_nat_real_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_real_interp is an outlier: 2.9x slower than the field

carrier_nat_real_interp (10.74 us) is 2.9x the fastest (3.73 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_nat_real_stencil** at 3733.6 ns median (-65.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.88x (fastest 3733.6 ns, slowest 10744.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 6491ns | 6358ns | 5855ns | 6207ns | 7236ns | -50.87% |
| carrier_nat_real_interp | 13214ns | 13238ns | 12318ns | 12998ns | 13986ns | base |
| carrier_nat_real_stencil | 6222ns | 6191ns | 5656ns | 6072ns | 6731ns | -52.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 3934ns | 3530ns | 4440ns | -63.39% | 0.065 |
| carrier_nat_real_interp | 10748ns | 10088ns | 11380ns | base | 0.024 |
| carrier_nat_real_stencil | 3731ns | 3432ns | 3982ns | -65.29% | 0.069 |

## Performance model

- Peak throughput: **0.075 Gops/s** (carrier_nat_real_stencil; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_real_copypatch | 0.067 | 90.3% |
| carrier_nat_real_interp | 0.024 | 31.9% |
| carrier_nat_real_stencil | 0.069 | 91.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_real_copypatch | 6491ns | 6491ns | -50.87% |
| carrier_nat_real_interp | 13214ns | 13214ns | base |
| carrier_nat_real_stencil | 6222ns | 6222ns | -52.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_real_interp | 10745ns | base | --- | [10120, 11380] | --- | --- | --- | --- |
| carrier_nat_real_copypatch | 3800ns | -6994.2ns (-65.1%) | [-7568, -5879]ns | [3564, 4440] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_real_stencil | 3734ns | -7113.1ns (-66.2%) | [-7617, -6321]ns | [3478, 3982] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_real_interp | carrier_nat_real_copypatch | carrier_nat_real_stencil |
|---|---|---|---|
| 1 | 10088ns | -65.0% | -60.1% |
| 2 | 10345ns | -51.6% | -65.9% |
| 3 | 10152ns | -63.3% | -64.8% |
| 4 | 11145ns | -67.7% | -69.2% |
| 5 | 11460ns | -66.2% | -65.6% |
| 6 | 11299ns | -65.8% | -65.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_real_copypatch | -0.378 | moderate- |
| carrier_nat_real_interp | 0.492 | moderate+ |
| carrier_nat_real_stencil | -0.029 | ok |

**Consistency summary:**

- **carrier_nat_real_copypatch**: won 6/6, lost 0/6
- **carrier_nat_real_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_real_copypatch | 82425.7ns | 3934.4ns | 2095.0% | HIGH |
| carrier_nat_real_interp | 91352.6ns | 10748.1ns | 849.9% | HIGH |
| carrier_nat_real_stencil | 82649.4ns | 3731.0ns | 2215.2% | HIGH |

## Distribution (algo ns)

```
carrier_nat_real_copypatch (n=6, range 3529.6-4439.6 ns)
   3529.6 |####################
   3575.1 |####################
   3620.6 |
   3666.1 |
   3711.6 |####################
   3757.1 |
   3802.6 |
   3848.1 |########################################
   3893.6 |
   3939.1 |
   3984.6 |
   4030.1 |
   4075.6 |
   4121.1 |
   4166.6 |
   4212.1 |
   4257.6 |
   4303.1 |
   4348.6 |
   4394.1 |
  (0 below, 1 above range)

carrier_nat_real_interp (n=6, range 10087.9-11379.6 ns)
  10087.9 |########################################
  10152.5 |
  10217.1 |
  10281.7 |####################
  10346.2 |
  10410.8 |
  10475.4 |
  10540.0 |
  10604.6 |
  10669.2 |
  10733.8 |
  10798.3 |
  10862.9 |
  10927.5 |
  10992.1 |
  11056.7 |
  11121.3 |####################
  11185.8 |
  11250.4 |####################
  11315.0 |
  (0 below, 1 above range)

carrier_nat_real_stencil (n=6, range 3432.5-3981.5 ns)
   3432.5 |########################################
   3459.9 |
   3487.4 |
   3514.8 |########################################
   3542.3 |
   3569.8 |########################################
   3597.2 |
   3624.7 |
   3652.1 |
   3679.6 |
   3707.0 |
   3734.4 |
   3761.9 |
   3789.3 |
   3816.8 |
   3844.2 |
   3871.7 |########################################
   3899.2 |
   3926.6 |########################################
   3954.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_real_copypatch**: bridge=2135.9% of algo (FFI overhead may distort results)
- **carrier_nat_real_interp**: bridge=851.7% of algo (FFI overhead may distort results)
- **carrier_nat_real_stencil**: bridge=2187.7% of algo (FFI overhead may distort results)
