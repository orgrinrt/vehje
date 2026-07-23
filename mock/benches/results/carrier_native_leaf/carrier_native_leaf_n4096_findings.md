# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_leaf_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_leaf_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_leaf_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_leaf_interp has the worst median (287.05 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_leaf_copypatch at 55.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_leaf_direct beats baseline by 81% (significant)

carrier_nat_leaf_direct is -231.92 us (81%) faster than baseline carrier_nat_leaf_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_leaf_interp is an outlier: 5.2x slower than the field

carrier_nat_leaf_interp (287.05 us) is 5.2x the fastest (55.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_nat_leaf_copypatch is fastest but the noisiest (CV 8.7%)

carrier_nat_leaf_copypatch wins on median (55.29 us) yet has the highest variance (CV 8.7%), while carrier_nat_leaf_interp is the steadiest (CV 1.9%, 287.05 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (carrier_nat_leaf_copypatch, carrier_nat_leaf_direct) are a dead heat (<1%)

carrier_nat_leaf_copypatch (55.29 us) and carrier_nat_leaf_direct (55.47 us) differ by 0.33%, inside the noise, even though the wider field spreads 419.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 5.2x the fastest

Fastest carrier_nat_leaf_copypatch (55.29 us) to slowest carrier_nat_leaf_interp (287.05 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_nat_leaf_copypatch** at 55291.4 ns median (-80.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.19x (fastest 55291.4 ns, slowest 287048.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 59912ns | 57572ns | 56673ns | 57317ns | 65425ns | -79.46% |
| carrier_nat_leaf_direct | 57802ns | 57805ns | 55845ns | 57532ns | 59185ns | -80.19% |
| carrier_nat_leaf_interp | 291727ns | 289795ns | 286569ns | 289159ns | 298159ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 57258ns | 54475ns | 61946ns | -80.18% | 0.072 |
| carrier_nat_leaf_direct | 55480ns | 53538ns | 56840ns | -80.80% | 0.074 |
| carrier_nat_leaf_interp | 288931ns | 283756ns | 295360ns | base | 0.014 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 709743 | 1973560 | 0.360 | 0.40× |
| carrier_nat_leaf_direct | 636201 | 1699385 | 0.374 | 0.36× |
| carrier_nat_leaf_interp | 1790374 | 4497800 | 0.398 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.077 Gops/s** (carrier_nat_leaf_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_leaf_copypatch | 0.074 | 96.8% |
| carrier_nat_leaf_direct | 0.074 | 96.5% |
| carrier_nat_leaf_interp | 0.014 | 18.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_leaf_copypatch | 59912ns | 59912ns | -79.46% |
| carrier_nat_leaf_direct | 57802ns | 57802ns | -80.19% |
| carrier_nat_leaf_interp | 291727ns | 291727ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_leaf_interp | 287049ns | base | --- | [284385, 295360] | --- | --- | --- | --- |
| carrier_nat_leaf_copypatch | 55291ns | -231248.4ns (-80.6%) | [-234322, -229449]ns | [54536, 61946] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_leaf_direct | 55472ns | -231917.0ns (-80.8%) | [-238520, -229916]ns | [54129, 56840] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_leaf_interp | carrier_nat_leaf_copypatch | carrier_nat_leaf_direct |
|---|---|---|---|
| 1 | 288579ns | -81.1% | -80.5% |
| 2 | 290769ns | -80.7% | -80.4% |
| 3 | 299950ns | -77.4% | -81.1% |
| 4 | 285518ns | -80.4% | -80.8% |
| 5 | 283756ns | -80.8% | -80.7% |
| 6 | 285014ns | -80.8% | -81.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_leaf_copypatch | -0.096 | ok |
| carrier_nat_leaf_direct | 0.445 | moderate+ |
| carrier_nat_leaf_interp | 0.112 | ok |

**Consistency summary:**

- **carrier_nat_leaf_copypatch**: won 6/6, lost 0/6
- **carrier_nat_leaf_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 180495.3ns | 57257.9ns | 315.2% | HIGH |
| carrier_nat_leaf_direct | 149745.3ns | 55480.2ns | 269.9% | HIGH |
| carrier_nat_leaf_interp | 290397.4ns | 288931.0ns | 100.5% | HIGH |

## Distribution (algo ns)

```
carrier_nat_leaf_copypatch (n=6, range 54475.4-61945.8 ns)
  54475.4 |########################################
  54848.9 |
  55222.4 |
  55596.0 |#############
  55969.5 |#############
  56343.0 |
  56716.5 |
  57090.1 |
  57463.6 |
  57837.1 |
  58210.6 |
  58584.1 |
  58957.7 |
  59331.2 |
  59704.7 |
  60078.2 |
  60451.8 |
  60825.3 |
  61198.8 |
  61572.3 |
  (0 below, 1 above range)

carrier_nat_leaf_direct (n=6, range 53538.3-56839.6 ns)
  53538.3 |####################
  53703.4 |
  53868.4 |
  54033.5 |
  54198.6 |
  54363.6 |
  54528.7 |
  54693.8 |########################################
  54858.8 |
  55023.9 |
  55188.9 |
  55354.0 |
  55519.1 |
  55684.1 |
  55849.2 |
  56014.3 |
  56179.3 |####################
  56344.4 |
  56509.5 |####################
  56674.5 |
  (0 below, 1 above range)

carrier_nat_leaf_interp (n=6, range 283755.8-295359.6 ns)
  283755.8 |########################################
  284336.0 |
  284916.2 |########################################
  285496.4 |########################################
  286076.6 |
  286656.8 |
  287236.9 |
  287817.1 |
  288397.3 |########################################
  288977.5 |
  289557.7 |
  290137.9 |
  290718.1 |########################################
  291298.3 |
  291878.5 |
  292458.6 |
  293038.8 |
  293619.0 |
  294199.2 |
  294779.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_leaf_copypatch**: bridge=313.6% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_direct**: bridge=270.8% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_interp**: bridge=100.6% of algo (FFI overhead may distort results)
