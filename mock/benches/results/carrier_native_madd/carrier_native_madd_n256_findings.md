# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_madd_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_madd_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_madd_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_madd_interp has the worst median (11.33 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_madd_direct at 6.85 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_madd_direct beats baseline by 40% (significant)

carrier_nat_madd_direct is -4.49 us (40%) faster than baseline carrier_nat_madd_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_nat_madd_direct** at 6850.0 ns median (-39.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.65x (fastest 6850.0 ns, slowest 11330.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 9401ns | 9696ns | 8757ns | 9390ns | 9738ns | -31.76% |
| carrier_nat_madd_direct | 9208ns | 9163ns | 8761ns | 9039ns | 9686ns | -33.15% |
| carrier_nat_madd_interp | 13775ns | 13663ns | 13118ns | 13481ns | 14545ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 7038ns | 6530ns | 7307ns | -38.54% | 0.036 |
| carrier_nat_madd_direct | 6901ns | 6572ns | 7272ns | -39.74% | 0.037 |
| carrier_nat_madd_interp | 11451ns | 10942ns | 12079ns | base | 0.022 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_madd_copypatch | 302400 | 557184 | 0.543 | 0.97× |
| carrier_nat_madd_direct | 296817 | 529264 | 0.561 | 0.96× |
| carrier_nat_madd_interp | 310489 | 1325712 | 0.234 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.039 Gops/s** (carrier_nat_madd_copypatch; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_madd_copypatch | 0.035 | 89.8% |
| carrier_nat_madd_direct | 0.037 | 95.3% |
| carrier_nat_madd_interp | 0.023 | 57.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_madd_copypatch | 9401ns | 9401ns | -31.76% |
| carrier_nat_madd_direct | 9208ns | 9208ns | -33.15% |
| carrier_nat_madd_interp | 13775ns | 13775ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_madd_interp | 11331ns | base | --- | [10944, 12079] | --- | --- | --- | --- |
| carrier_nat_madd_copypatch | 7272ns | -4420.6ns (-39.0%) | [-4775, -4045]ns | [6535, 7307] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_madd_direct | 6850ns | -4490.7ns (-39.6%) | [-4807, -4354]ns | [6580, 7272] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_madd_interp | carrier_nat_madd_copypatch | carrier_nat_madd_direct |
|---|---|---|---|
| 1 | 10952ns | -40.3% | -39.7% |
| 2 | 10946ns | -40.3% | -39.8% |
| 3 | 11710ns | -37.8% | -39.4% |
| 4 | 12105ns | -39.9% | -40.5% |
| 5 | 12054ns | -39.2% | -39.1% |
| 6 | 10942ns | -33.6% | -39.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_madd_copypatch | 0.425 | moderate+ |
| carrier_nat_madd_direct | 0.134 | ok |
| carrier_nat_madd_interp | 0.233 | moderate+ |

**Consistency summary:**

- **carrier_nat_madd_copypatch**: won 6/6, lost 0/6
- **carrier_nat_madd_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_madd_copypatch | 94393.8ns | 7037.8ns | 1341.2% | HIGH |
| carrier_nat_madd_direct | 89777.8ns | 6900.8ns | 1301.0% | HIGH |
| carrier_nat_madd_interp | 89855.1ns | 11451.4ns | 784.7% | HIGH |

## Distribution (algo ns)

```
carrier_nat_madd_copypatch (n=6, range 6530.4-7306.6 ns)
   6530.4 |########################################
   6569.2 |
   6608.0 |
   6646.8 |
   6685.6 |
   6724.5 |
   6763.3 |
   6802.1 |
   6840.9 |
   6879.7 |
   6918.5 |
   6957.3 |
   6996.1 |
   7035.0 |
   7073.8 |
   7112.6 |
   7151.4 |
   7190.2 |
   7229.0 |####################
   7267.8 |########################################
  (0 below, 1 above range)

carrier_nat_madd_direct (n=6, range 6572.5-7272.1 ns)
   6572.5 |########################################
   6607.5 |
   6642.5 |
   6677.4 |
   6712.4 |
   6747.4 |
   6782.4 |
   6817.4 |
   6852.3 |
   6887.3 |
   6922.3 |
   6957.3 |
   6992.3 |
   7027.2 |
   7062.2 |
   7097.2 |#############
   7132.2 |
   7167.2 |
   7202.1 |#############
   7237.1 |
  (0 below, 1 above range)

carrier_nat_madd_interp (n=6, range 10941.7-12079.4 ns)
  10941.7 |########################################
  10998.6 |
  11055.5 |
  11112.4 |
  11169.2 |
  11226.1 |
  11283.0 |
  11339.9 |
  11396.8 |
  11453.7 |
  11510.5 |
  11567.4 |
  11624.3 |
  11681.2 |#############
  11738.1 |
  11795.0 |
  11851.9 |
  11908.7 |
  11965.6 |
  12022.5 |#############
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_madd_copypatch**: bridge=1293.4% of algo (FFI overhead may distort results)
- **carrier_nat_madd_direct**: bridge=1312.2% of algo (FFI overhead may distort results)
- **carrier_nat_madd_interp**: bridge=779.8% of algo (FFI overhead may distort results)
