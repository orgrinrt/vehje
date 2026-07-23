# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_leaf_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_leaf_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_leaf_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_leaf_interp has the worst median (9.62 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_leaf_direct at 3.36 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_leaf_direct beats baseline by 65% (significant)

carrier_nat_leaf_direct is -6.25 us (65%) faster than baseline carrier_nat_leaf_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_leaf_interp is an outlier: 2.9x slower than the field

carrier_nat_leaf_interp (9.62 us) is 2.9x the fastest (3.36 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Speed leader carrier_nat_leaf_direct vs stability leader carrier_nat_leaf_copypatch (+3% speed for 1.3x steadier)

carrier_nat_leaf_direct is fastest (3.36 us, CV 6.2%); carrier_nat_leaf_copypatch gives up 2.8% median for 1.3x lower variance (CV 4.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_nat_leaf_direct** at 3356.8 ns median (-65.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.87x (fastest 3356.8 ns, slowest 9623.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 5739ns | 5864ns | 5347ns | 5701ns | 5991ns | -53.06% |
| carrier_nat_leaf_direct | 5761ns | 5731ns | 5322ns | 5617ns | 6197ns | -52.88% |
| carrier_nat_leaf_interp | 12226ns | 12175ns | 10620ns | 11734ns | 13767ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 3375ns | 3152ns | 3522ns | -65.55% | 0.076 |
| carrier_nat_leaf_direct | 3363ns | 3128ns | 3590ns | -65.67% | 0.076 |
| carrier_nat_leaf_interp | 9798ns | 8463ns | 11226ns | base | 0.026 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 273025 | 795896 | 0.343 | 0.91× |
| carrier_nat_leaf_direct | 268938 | 776057 | 0.347 | 0.90× |
| carrier_nat_leaf_interp | 299678 | 1464829 | 0.205 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.082 Gops/s** (carrier_nat_leaf_direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_leaf_copypatch | 0.074 | 90.7% |
| carrier_nat_leaf_direct | 0.076 | 93.2% |
| carrier_nat_leaf_interp | 0.027 | 32.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_leaf_copypatch | 5739ns | 5739ns | -53.06% |
| carrier_nat_leaf_direct | 5761ns | 5761ns | -52.88% |
| carrier_nat_leaf_interp | 12226ns | 12226ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_leaf_interp | 9624ns | base | --- | [8543, 11226] | --- | --- | --- | --- |
| carrier_nat_leaf_copypatch | 3449ns | -6158.8ns (-64.0%) | [-7720, -5389]ns | [3154, 3522] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_leaf_direct | 3357ns | -6249.0ns (-64.9%) | [-7684, -5370]ns | [3143, 3590] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_leaf_interp | carrier_nat_leaf_copypatch | carrier_nat_leaf_direct |
|---|---|---|---|
| 1 | 8624ns | -63.4% | -63.4% |
| 2 | 9470ns | -64.2% | -67.0% |
| 3 | 11992ns | -70.8% | -70.3% |
| 4 | 10460ns | -66.5% | -66.3% |
| 5 | 9778ns | -63.8% | -63.0% |
| 6 | 8463ns | -62.7% | -62.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_leaf_copypatch | 0.007 | ok |
| carrier_nat_leaf_direct | 0.118 | ok |
| carrier_nat_leaf_interp | 0.133 | ok |

**Consistency summary:**

- **carrier_nat_leaf_copypatch**: won 6/6, lost 0/6
- **carrier_nat_leaf_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 87504.3ns | 3375.2ns | 2592.6% | HIGH |
| carrier_nat_leaf_direct | 85278.1ns | 3363.4ns | 2535.5% | HIGH |
| carrier_nat_leaf_interp | 91614.9ns | 9797.7ns | 935.1% | HIGH |

## Distribution (algo ns)

```
carrier_nat_leaf_copypatch (n=6, range 3152.5-3521.9 ns)
   3152.5 |########################################
   3171.0 |
   3189.4 |
   3207.9 |
   3226.4 |
   3244.8 |
   3263.3 |
   3281.8 |
   3300.2 |
   3318.7 |
   3337.2 |
   3355.6 |
   3374.1 |
   3392.6 |####################
   3411.0 |
   3429.5 |
   3448.0 |
   3466.4 |
   3484.9 |
   3503.4 |########################################
  (0 below, 1 above range)

carrier_nat_leaf_direct (n=6, range 3128.3-3589.9 ns)
   3128.3 |########################################
   3151.4 |########################################
   3174.5 |########################################
   3197.5 |
   3220.6 |
   3243.7 |
   3266.8 |
   3289.9 |
   3313.0 |
   3336.0 |
   3359.1 |
   3382.2 |
   3405.3 |
   3428.4 |
   3451.5 |
   3474.5 |
   3497.6 |
   3520.7 |########################################
   3543.8 |########################################
   3566.9 |
  (0 below, 1 above range)

carrier_nat_leaf_interp (n=6, range 8462.9-11226.0 ns)
   8462.9 |########################################
   8601.1 |########################################
   8739.2 |
   8877.4 |
   9015.5 |
   9153.7 |
   9291.8 |
   9430.0 |########################################
   9568.2 |
   9706.3 |########################################
   9844.5 |
   9982.6 |
  10120.8 |
  10258.9 |
  10397.1 |########################################
  10535.3 |
  10673.4 |
  10811.6 |
  10949.7 |
  11087.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_leaf_copypatch**: bridge=2533.1% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_direct**: bridge=2536.0% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_interp**: bridge=958.3% of algo (FFI overhead may distort results)
