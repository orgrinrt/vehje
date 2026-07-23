# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_tight_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_tight_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_tight_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_tight_interp has the worst median (44.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_tight_direct at 21.90 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_tight_copypatch beats baseline by 50% (significant)

carrier_nat_tight_copypatch is -22.28 us (50%) faster than baseline carrier_nat_tight_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_tight_interp is an outlier: 2.0x slower than the field

carrier_nat_tight_interp (44.29 us) is 2.0x the fastest (21.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_nat_tight_direct, carrier_nat_tight_copypatch) are a dead heat (<1%)

carrier_nat_tight_direct (21.90 us) and carrier_nat_tight_copypatch (21.95 us) differ by 0.25%, inside the noise, even though the wider field spreads 102.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: carrier_nat_tight_direct** at 21898.6 ns median (-50.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.02x (fastest 21898.6 ns, slowest 44290.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 24279ns | 24231ns | 23610ns | 24025ns | 24995ns | -47.66% |
| carrier_nat_tight_direct | 24247ns | 24150ns | 23625ns | 24005ns | 24920ns | -47.73% |
| carrier_nat_tight_interp | 46383ns | 46586ns | 44435ns | 45929ns | 48040ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 22017ns | 21430ns | 22665ns | -50.11% | 0.047 |
| carrier_nat_tight_direct | 21989ns | 21455ns | 22592ns | -50.17% | 0.047 |
| carrier_nat_tight_interp | 44129ns | 42270ns | 45732ns | base | 0.023 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_tight_copypatch | 414295 | 950722 | 0.436 | 0.92× |
| carrier_nat_tight_direct | 384563 | 847552 | 0.454 | 0.85× |
| carrier_nat_tight_interp | 452016 | 2013283 | 0.225 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.048 Gops/s** (carrier_nat_tight_copypatch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_tight_copypatch | 0.047 | 97.6% |
| carrier_nat_tight_direct | 0.047 | 97.9% |
| carrier_nat_tight_interp | 0.023 | 48.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_tight_copypatch | 24279ns | 24279ns | -47.66% |
| carrier_nat_tight_direct | 24247ns | 24247ns | -47.73% |
| carrier_nat_tight_interp | 46383ns | 46383ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_tight_interp | 44291ns | base | --- | [42364, 45732] | --- | --- | --- | --- |
| carrier_nat_tight_copypatch | 21954ns | -22277.1ns (-50.3%) | [-23127, -20933]ns | [21431, 22665] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_tight_direct | 21899ns | -22233.2ns (-50.2%) | [-23352, -20836]ns | [21475, 22592] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_tight_interp | carrier_nat_tight_copypatch | carrier_nat_tight_direct |
|---|---|---|---|
| 1 | 42457ns | -49.5% | -49.5% |
| 2 | 45425ns | -50.7% | -51.1% |
| 3 | 45575ns | -51.0% | -50.0% |
| 4 | 45890ns | -50.0% | -51.2% |
| 5 | 42270ns | -49.3% | -48.9% |
| 6 | 43157ns | -50.1% | -50.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_tight_copypatch | -0.027 | ok |
| carrier_nat_tight_direct | 0.271 | moderate+ |
| carrier_nat_tight_interp | 0.056 | ok |

**Consistency summary:**

- **carrier_nat_tight_copypatch**: won 6/6, lost 0/6
- **carrier_nat_tight_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_tight_copypatch | 112029.9ns | 22016.6ns | 508.8% | HIGH |
| carrier_nat_tight_direct | 102050.3ns | 21988.7ns | 464.1% | HIGH |
| carrier_nat_tight_interp | 103360.2ns | 44128.9ns | 234.2% | HIGH |

## Distribution (algo ns)

```
carrier_nat_tight_copypatch (n=6, range 21430.0-22665.4 ns)
  21430.0 |########################################
  21491.8 |
  21553.5 |####################
  21615.3 |
  21677.1 |
  21738.8 |
  21800.6 |
  21862.4 |
  21924.2 |
  21985.9 |
  22047.7 |
  22109.5 |
  22171.2 |
  22233.0 |
  22294.8 |####################
  22356.6 |####################
  22418.3 |
  22480.1 |
  22541.9 |
  22603.6 |
  (0 below, 1 above range)

carrier_nat_tight_direct (n=6, range 21455.0-22592.1 ns)
  21455.0 |########################################
  21511.9 |
  21568.7 |####################
  21625.6 |
  21682.4 |
  21739.3 |
  21796.1 |
  21853.0 |
  21909.8 |
  21966.7 |
  22023.5 |
  22080.4 |
  22137.3 |
  22194.1 |####################
  22251.0 |
  22307.8 |
  22364.7 |####################
  22421.5 |
  22478.4 |
  22535.2 |
  (0 below, 1 above range)

carrier_nat_tight_interp (n=6, range 42270.0-45732.3 ns)
  42270.0 |########################################
  42443.1 |########################################
  42616.2 |
  42789.3 |
  42962.5 |
  43135.6 |########################################
  43308.7 |
  43481.8 |
  43654.9 |
  43828.0 |
  44001.2 |
  44174.3 |
  44347.4 |
  44520.5 |
  44693.6 |
  44866.7 |
  45039.8 |
  45213.0 |
  45386.1 |########################################
  45559.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_tight_copypatch**: bridge=506.3% of algo (FFI overhead may distort results)
- **carrier_nat_tight_direct**: bridge=467.4% of algo (FFI overhead may distort results)
- **carrier_nat_tight_interp**: bridge=218.9% of algo (FFI overhead may distort results)
