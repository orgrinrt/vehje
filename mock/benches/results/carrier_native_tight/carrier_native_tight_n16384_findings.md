# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_tight_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_tight_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_tight_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_tight_interp has the worst median (843.60 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_tight_copypatch at 439.37 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_tight_copypatch beats baseline by 48% (significant)

carrier_nat_tight_copypatch is -403.89 us (48%) faster than baseline carrier_nat_tight_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (carrier_nat_tight_copypatch, carrier_nat_tight_direct) are a dead heat (<1%)

carrier_nat_tight_copypatch (439.37 us) and carrier_nat_tight_direct (443.47 us) differ by 0.93%, inside the noise, even though the wider field spreads 92.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: carrier_nat_tight_copypatch** at 439367.1 ns median (-47.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.92x (fastest 439367.1 ns, slowest 843599.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 445550ns | 441666ns | 441358ns | 441580ns | 453600ns | -47.39% |
| carrier_nat_tight_direct | 446635ns | 445810ns | 441974ns | 444701ns | 451867ns | -47.26% |
| carrier_nat_tight_interp | 846813ns | 846903ns | 836233ns | 843366ns | 857274ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 443145ns | 438968ns | 451062ns | -47.48% | 0.037 |
| carrier_nat_tight_direct | 444215ns | 439670ns | 449288ns | -47.35% | 0.037 |
| carrier_nat_tight_interp | 843736ns | 832537ns | 854500ns | base | 0.019 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_tight_copypatch | 3989711 | 9500016 | 0.420 | 0.76× |
| carrier_nat_tight_direct | 3536109 | 7798742 | 0.453 | 0.67× |
| carrier_nat_tight_interp | 5258934 | 19144186 | 0.275 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.037 Gops/s** (carrier_nat_tight_copypatch; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_tight_copypatch | 0.037 | 99.9% |
| carrier_nat_tight_direct | 0.037 | 99.0% |
| carrier_nat_tight_interp | 0.019 | 52.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_tight_copypatch | 445550ns | 445550ns | -47.39% |
| carrier_nat_tight_direct | 446635ns | 446635ns | -47.26% |
| carrier_nat_tight_interp | 846813ns | 846813ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_tight_interp | 843600ns | base | --- | [833108, 854500] | --- | --- | --- | --- |
| carrier_nat_tight_copypatch | 439367ns | -403888.3ns (-47.9%) | [-415133, -382752]ns | [439005, 451062] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_tight_direct | 443469ns | -396277.5ns (-47.0%) | [-409894, -392393]ns | [439887, 449288] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_tight_interp | carrier_nat_tight_copypatch | carrier_nat_tight_direct |
|---|---|---|---|
| 1 | 856198ns | -48.7% | -47.5% |
| 2 | 842638ns | -47.7% | -46.7% |
| 3 | 852803ns | -48.5% | -48.4% |
| 4 | 833678ns | -47.3% | -47.2% |
| 5 | 832537ns | -44.5% | -47.0% |
| 6 | 844561ns | -48.0% | -47.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_tight_copypatch | -0.279 | moderate- |
| carrier_nat_tight_direct | 0.305 | moderate+ |
| carrier_nat_tight_interp | -0.025 | ok |

**Consistency summary:**

- **carrier_nat_tight_copypatch**: won 6/6, lost 0/6
- **carrier_nat_tight_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_tight_copypatch | 835092.4ns | 443144.6ns | 188.4% | HIGH |
| carrier_nat_tight_direct | 691588.3ns | 444214.5ns | 155.7% | HIGH |
| carrier_nat_tight_interp | 844221.5ns | 843736.0ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_nat_tight_copypatch (n=6, range 438967.5-451061.7 ns)
  438967.5 |########################################
  439572.2 |#############
  440176.9 |#############
  440781.6 |
  441386.3 |
  441991.0 |
  442595.7 |
  443200.5 |
  443805.2 |
  444409.9 |
  445014.6 |
  445619.3 |
  446224.0 |
  446828.7 |
  447433.4 |
  448038.1 |
  448642.8 |
  449247.5 |
  449852.2 |
  450456.9 |
  (0 below, 1 above range)

carrier_nat_tight_direct (n=6, range 439670.0-449287.5 ns)
  439670.0 |########################################
  440150.9 |
  440631.8 |
  441112.6 |####################
  441593.5 |
  442074.4 |
  442555.2 |
  443036.1 |
  443517.0 |
  443997.9 |
  444478.8 |
  444959.6 |
  445440.5 |####################
  445921.4 |
  446402.2 |
  446883.1 |
  447364.0 |
  447844.9 |
  448325.8 |
  448806.6 |####################
  (0 below, 1 above range)

carrier_nat_tight_interp (n=6, range 832537.1-854500.4 ns)
  832537.1 |########################################
  833635.3 |########################################
  834733.4 |
  835831.6 |
  836929.8 |
  838027.9 |
  839126.1 |
  840224.3 |
  841322.4 |
  842420.6 |########################################
  843518.8 |########################################
  844616.9 |
  845715.1 |
  846813.2 |
  847911.4 |
  849009.6 |
  850107.7 |
  851205.9 |
  852304.1 |########################################
  853402.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_tight_copypatch**: bridge=187.6% of algo (FFI overhead may distort results)
- **carrier_nat_tight_direct**: bridge=155.0% of algo (FFI overhead may distort results)
- **carrier_nat_tight_interp**: bridge=99.9% of algo (FFI overhead may distort results)
