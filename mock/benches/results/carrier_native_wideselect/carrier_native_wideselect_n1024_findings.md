# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_wideselect_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_wideselect_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_wideselect_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_wideselect_interp has the worst median (43.01 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_wideselect_copypatch at 15.74 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_wideselect_copypatch beats baseline by 63% (significant)

carrier_nat_wideselect_copypatch is -27.18 us (63%) faster than baseline carrier_nat_wideselect_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_wideselect_interp is an outlier: 2.7x slower than the field

carrier_nat_wideselect_interp (43.01 us) is 2.7x the fastest (15.74 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_nat_wideselect_copypatch** at 15742.0 ns median (-63.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.73x (fastest 15742.0 ns, slowest 43008.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 18027ns | 18059ns | 17336ns | 17861ns | 18620ns | -59.70% |
| carrier_nat_wideselect_direct | 18143ns | 18283ns | 17332ns | 18031ns | 18718ns | -59.44% |
| carrier_nat_wideselect_interp | 44731ns | 45546ns | 42175ns | 44572ns | 46249ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 15725ns | 15145ns | 16256ns | -62.83% | 0.065 |
| carrier_nat_wideselect_direct | 15798ns | 15121ns | 16304ns | -62.65% | 0.065 |
| carrier_nat_wideselect_interp | 42303ns | 39974ns | 43719ns | base | 0.024 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 388550 | 1183503 | 0.328 | 0.85× |
| carrier_nat_wideselect_direct | 341162 | 1039881 | 0.328 | 0.75× |
| carrier_nat_wideselect_interp | 457176 | 2170585 | 0.211 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.068 Gops/s** (carrier_nat_wideselect_direct; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_wideselect_copypatch | 0.065 | 96.1% |
| carrier_nat_wideselect_direct | 0.064 | 95.1% |
| carrier_nat_wideselect_interp | 0.024 | 35.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_wideselect_copypatch | 18027ns | 18027ns | -59.70% |
| carrier_nat_wideselect_direct | 18143ns | 18143ns | -59.44% |
| carrier_nat_wideselect_interp | 44731ns | 44731ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_wideselect_interp | 43009ns | base | --- | [40180, 43719] | --- | --- | --- | --- |
| carrier_nat_wideselect_copypatch | 15742ns | -27175.4ns (-63.2%) | [-27555, -25004]ns | [15176, 16256] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_wideselect_direct | 15901ns | -26943.3ns (-62.6%) | [-27579, -24992]ns | [15189, 16304] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_wideselect_interp | carrier_nat_wideselect_copypatch | carrier_nat_wideselect_direct |
|---|---|---|---|
| 1 | 40387ns | -62.5% | -62.2% |
| 2 | 39974ns | -62.0% | -62.2% |
| 3 | 44055ns | -62.9% | -63.0% |
| 4 | 43163ns | -62.9% | -63.5% |
| 5 | 43383ns | -62.7% | -62.4% |
| 6 | 42855ns | -63.9% | -62.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_wideselect_copypatch | 0.135 | ok |
| carrier_nat_wideselect_direct | 0.077 | ok |
| carrier_nat_wideselect_interp | 0.238 | moderate+ |

**Consistency summary:**

- **carrier_nat_wideselect_copypatch**: won 6/6, lost 0/6
- **carrier_nat_wideselect_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 111312.1ns | 15724.6ns | 707.9% | HIGH |
| carrier_nat_wideselect_direct | 95596.2ns | 15798.1ns | 605.1% | HIGH |
| carrier_nat_wideselect_interp | 109319.5ns | 42302.7ns | 258.4% | HIGH |

## Distribution (algo ns)

```
carrier_nat_wideselect_copypatch (n=6, range 15145.4-16255.6 ns)
  15145.4 |########################################
  15200.9 |########################################
  15256.4 |
  15311.9 |
  15367.4 |
  15423.0 |########################################
  15478.5 |
  15534.0 |
  15589.5 |
  15645.0 |
  15700.5 |
  15756.0 |
  15811.5 |
  15867.1 |
  15922.6 |
  15978.1 |########################################
  16033.6 |
  16089.1 |
  16144.6 |########################################
  16200.1 |
  (0 below, 1 above range)

carrier_nat_wideselect_direct (n=6, range 15120.8-16304.4 ns)
  15120.8 |########################################
  15180.0 |
  15239.2 |########################################
  15298.3 |
  15357.5 |
  15416.7 |
  15475.9 |
  15535.1 |
  15594.2 |
  15653.4 |
  15712.6 |########################################
  15771.8 |
  15831.0 |
  15890.1 |
  15949.3 |
  16008.5 |########################################
  16067.7 |
  16126.9 |
  16186.0 |
  16245.2 |########################################
  (0 below, 1 above range)

carrier_nat_wideselect_interp (n=6, range 39973.8-43718.8 ns)
  39973.8 |########################################
  40161.0 |
  40348.3 |########################################
  40535.5 |
  40722.8 |
  40910.0 |
  41097.3 |
  41284.5 |
  41471.8 |
  41659.0 |
  41846.3 |
  42033.5 |
  42220.8 |
  42408.0 |
  42595.3 |
  42782.5 |########################################
  42969.8 |
  43157.0 |########################################
  43344.3 |########################################
  43531.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_wideselect_copypatch**: bridge=707.5% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_direct**: bridge=603.3% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_interp**: bridge=248.8% of algo (FFI overhead may distort results)
