# Predecoded dispatch shape, tight profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_pre_tight_switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_pre_tight_switch has the worst median (41.02 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_pre_tight_null at 32.44 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

## Key findings

- **Fastest: carrier_pre_tight_null** at 32441.7 ns median (-20.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.26x (fastest 32441.7 ns, slowest 41025.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_tight_direct | 40684ns | 39600ns | 38662ns | 39565ns | 43374ns | -5.83% |
| carrier_pre_tight_fntable | 41937ns | 40977ns | 40413ns | 40803ns | 44401ns | -2.93% |
| carrier_pre_tight_null | 35228ns | 34696ns | 33723ns | 34597ns | 36926ns | -18.46% |
| carrier_pre_tight_regcache | 36223ns | 35633ns | 35424ns | 35568ns | 37606ns | -16.15% |
| carrier_pre_tight_switch | 43202ns | 43245ns | 41180ns | 42589ns | 45131ns | base |
| carrier_pre_tight_threaded | 40124ns | 39020ns | 38676ns | 38907ns | 42672ns | -7.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_tight_direct | 38454ns | 36548ns | 40996ns | -6.15% | 0.027 |
| carrier_pre_tight_fntable | 39740ns | 38292ns | 42066ns | -3.01% | 0.026 |
| carrier_pre_tight_null | 32998ns | 31609ns | 34616ns | -19.47% | 0.031 |
| carrier_pre_tight_regcache | 34058ns | 33308ns | 35363ns | -16.88% | 0.030 |
| carrier_pre_tight_switch | 40974ns | 39061ns | 42789ns | base | 0.025 |
| carrier_pre_tight_threaded | 37894ns | 36545ns | 40284ns | -7.52% | 0.027 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_tight_direct | 475426 | 1060552 | 0.448 | 0.99× |
| carrier_pre_tight_fntable | 501705 | 1630511 | 0.308 | 1.05× |
| carrier_pre_tight_null | 418656 | 1503473 | 0.278 | 0.87× |
| carrier_pre_tight_regcache | 436246 | 1945708 | 0.224 | 0.91× |
| carrier_pre_tight_switch | 479122 | 1316394 | 0.364 | 1.00× |
| carrier_pre_tight_threaded | 469214 | 1433680 | 0.327 | 0.98× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_pre_tight_null; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_tight_direct | 0.027 | 84.5% |
| carrier_pre_tight_fntable | 0.026 | 81.4% |
| carrier_pre_tight_null | 0.032 | 97.4% |
| carrier_pre_tight_regcache | 0.031 | 94.4% |
| carrier_pre_tight_switch | 0.025 | 77.0% |
| carrier_pre_tight_threaded | 0.028 | 85.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_tight_direct | 40684ns | 40684ns | -5.83% |
| carrier_pre_tight_fntable | 41937ns | 41937ns | -2.93% |
| carrier_pre_tight_null | 35228ns | 35228ns | -18.46% |
| carrier_pre_tight_regcache | 36223ns | 36223ns | -16.15% |
| carrier_pre_tight_switch | 43202ns | 43202ns | base |
| carrier_pre_tight_threaded | 40124ns | 40124ns | -7.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_tight_switch | 41025ns | base | --- | [39107, 42789] | --- | --- | --- | --- |
| carrier_pre_tight_direct | 37428ns | no significant difference | [-5407, +1510]ns | [36937, 40996] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_tight_fntable | 38842ns | no significant difference | [-4341, +1240]ns | [38313, 42066] | no | 0.2734 | 0.2188 | 0 |
| carrier_pre_tight_null | 32442ns | -7393.3ns (-18.0%) | [-10718, -5816]ns | [31935, 34616] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_pre_tight_regcache | 33500ns | -7265.9ns (-17.7%) | [-9404, -4078]ns | [33311, 35363] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_pre_tight_threaded | 36852ns | -2406.9ns (-5.9%) | [-6109, -724]ns | [36545, 40284] | YES (adj: no) | 0.2734 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_tight_switch | carrier_pre_tight_direct | carrier_pre_tight_fntable | carrier_pre_tight_null | carrier_pre_tight_regcache | carrier_pre_tight_threaded |
|---|---|---|---|---|---|---|
| 1 | 43078ns | -13.4% | -11.0% | -25.1% | -22.3% | -15.2% |
| 2 | 42500ns | -11.9% | +6.3% | -18.8% | -21.6% | -5.6% |
| 3 | 39061ns | -6.4% | -0.8% | -16.7% | -13.2% | -5.3% |
| 4 | 39152ns | +5.9% | -0.5% | -17.3% | -14.3% | -6.3% |
| 5 | 42229ns | -11.4% | -9.3% | -25.1% | -21.1% | -13.5% |
| 6 | 39821ns | +1.7% | -2.2% | -12.8% | -7.5% | +1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_tight_direct | -0.390 | moderate- |
| carrier_pre_tight_fntable | -0.281 | moderate- |
| carrier_pre_tight_null | -0.363 | moderate- |
| carrier_pre_tight_regcache | -0.110 | ok |
| carrier_pre_tight_switch | 0.003 | ok |
| carrier_pre_tight_threaded | -0.334 | moderate- |

**Consistency summary:**

- **carrier_pre_tight_direct**: won 4/6, lost 2/6
- **carrier_pre_tight_fntable**: won 5/6, lost 1/6
- **carrier_pre_tight_null**: won 6/6, lost 0/6
- **carrier_pre_tight_regcache**: won 6/6, lost 0/6
- **carrier_pre_tight_threaded**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_tight_direct | 116967.9ns | 38453.7ns | 304.2% | HIGH |
| carrier_pre_tight_fntable | 118952.2ns | 39740.5ns | 299.3% | HIGH |
| carrier_pre_tight_null | 101785.1ns | 32997.8ns | 308.5% | HIGH |
| carrier_pre_tight_regcache | 104576.7ns | 34057.8ns | 307.1% | HIGH |
| carrier_pre_tight_switch | 114737.8ns | 40973.7ns | 280.0% | HIGH |
| carrier_pre_tight_threaded | 114940.4ns | 37893.8ns | 303.3% | HIGH |

## Distribution (algo ns)

```
carrier_pre_tight_direct (n=6, range 36548.3-40996.2 ns)
  36548.3 |####################
  36770.7 |
  36993.1 |
  37215.5 |########################################
  37437.9 |####################
  37660.3 |
  37882.7 |
  38105.1 |
  38327.5 |
  38549.9 |
  38772.2 |
  38994.6 |
  39217.0 |
  39439.4 |
  39661.8 |
  39884.2 |
  40106.6 |
  40329.0 |####################
  40551.4 |
  40773.8 |
  (0 below, 1 above range)

carrier_pre_tight_fntable (n=6, range 38292.5-42066.1 ns)
  38292.5 |########################################
  38481.2 |
  38669.9 |####################
  38858.5 |########################################
  39047.2 |
  39235.9 |
  39424.6 |
  39613.2 |
  39801.9 |
  39990.6 |
  40179.3 |
  40368.0 |
  40556.6 |
  40745.3 |
  40934.0 |
  41122.7 |
  41311.3 |
  41500.0 |
  41688.7 |
  41877.4 |
  (0 below, 1 above range)

carrier_pre_tight_null (n=6, range 31609.2-34616.4 ns)
  31609.2 |####################
  31759.6 |
  31909.9 |
  32060.3 |
  32210.7 |########################################
  32361.0 |
  32511.4 |####################
  32661.7 |
  32812.1 |
  32962.5 |
  33112.8 |
  33263.2 |
  33413.5 |
  33563.9 |
  33714.3 |
  33864.6 |
  34015.0 |
  34165.4 |
  34315.7 |
  34466.1 |####################
  (0 below, 1 above range)

carrier_pre_tight_regcache (n=6, range 33308.3-35362.9 ns)
  33308.3 |########################################
  33411.0 |####################
  33513.8 |####################
  33616.5 |
  33719.2 |
  33822.0 |####################
  33924.7 |
  34027.4 |
  34130.1 |
  34232.9 |
  34335.6 |
  34438.3 |
  34541.1 |
  34643.8 |
  34746.5 |
  34849.2 |
  34952.0 |
  35054.7 |
  35157.4 |
  35260.2 |
  (0 below, 1 above range)

carrier_pre_tight_switch (n=6, range 39061.2-42789.4 ns)
  39061.2 |########################################
  39247.6 |
  39434.0 |
  39620.4 |
  39806.8 |####################
  39993.2 |
  40179.6 |
  40366.1 |
  40552.5 |
  40738.9 |
  40925.3 |
  41111.7 |
  41298.1 |
  41484.5 |
  41670.9 |
  41857.3 |
  42043.7 |####################
  42230.1 |
  42416.5 |####################
  42602.9 |
  (0 below, 1 above range)

carrier_pre_tight_threaded (n=6, range 36544.6-40283.9 ns)
  36544.6 |########################################
  36731.6 |
  36918.5 |#############
  37105.5 |
  37292.5 |
  37479.4 |
  37666.4 |
  37853.4 |
  38040.3 |
  38227.3 |
  38414.3 |
  38601.2 |
  38788.2 |
  38975.2 |
  39162.1 |
  39349.1 |
  39536.1 |
  39723.0 |
  39910.0 |
  40097.0 |#############
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_tight_direct**: bridge=308.6% of algo (FFI overhead may distort results)
- **carrier_pre_tight_fntable**: bridge=305.4% of algo (FFI overhead may distort results)
- **carrier_pre_tight_null**: bridge=308.8% of algo (FFI overhead may distort results)
- **carrier_pre_tight_regcache**: bridge=307.2% of algo (FFI overhead may distort results)
- **carrier_pre_tight_switch**: bridge=282.2% of algo (FFI overhead may distort results)
- **carrier_pre_tight_threaded**: bridge=306.8% of algo (FFI overhead may distort results)
