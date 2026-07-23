# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_madd_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_madd_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_madd_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_madd_interp has the worst median (48.03 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_madd_copypatch at 32.47 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_madd_copypatch beats baseline by 31% (significant)

carrier_nat_madd_copypatch is -14.96 us (31%) faster than baseline carrier_nat_madd_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_nat_madd_copypatch** at 32465.8 ns median (-32.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.48x (fastest 32465.8 ns, slowest 48034.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 34872ns | 34702ns | 34406ns | 34611ns | 35495ns | -30.32% |
| carrier_nat_madd_direct | 35190ns | 35090ns | 34474ns | 34953ns | 35905ns | -29.68% |
| carrier_nat_madd_interp | 50045ns | 50452ns | 48411ns | 50073ns | 50819ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 32615ns | 32199ns | 33165ns | -31.62% | 0.031 |
| carrier_nat_madd_direct | 32945ns | 32242ns | 33619ns | -30.93% | 0.031 |
| carrier_nat_madd_interp | 47700ns | 46141ns | 48423ns | base | 0.021 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_madd_copypatch | 482798 | 806330 | 0.599 | 1.08× |
| carrier_nat_madd_direct | 452279 | 703129 | 0.643 | 1.01× |
| carrier_nat_madd_interp | 447303 | 1805554 | 0.248 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_nat_madd_copypatch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_madd_copypatch | 0.032 | 99.2% |
| carrier_nat_madd_direct | 0.031 | 98.0% |
| carrier_nat_madd_interp | 0.021 | 67.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_madd_copypatch | 34872ns | 34872ns | -30.32% |
| carrier_nat_madd_direct | 35190ns | 35190ns | -29.68% |
| carrier_nat_madd_interp | 50045ns | 50045ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_madd_interp | 48034ns | base | --- | [46641, 48423] | --- | --- | --- | --- |
| carrier_nat_madd_copypatch | 32466ns | -14964.0ns (-31.2%) | [-15984, -14307]ns | [32213, 33165] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_madd_direct | 32860ns | -14525.2ns (-30.2%) | [-15973, -13766]ns | [32356, 33619] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_madd_interp | carrier_nat_madd_copypatch | carrier_nat_madd_direct |
|---|---|---|---|
| 1 | 48713ns | -33.8% | -33.8% |
| 2 | 48124ns | -31.2% | -31.1% |
| 3 | 47141ns | -31.1% | -29.6% |
| 4 | 47945ns | -32.3% | -32.3% |
| 5 | 46141ns | -30.2% | -29.4% |
| 6 | 48133ns | -31.0% | -29.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_madd_copypatch | -0.436 | moderate- |
| carrier_nat_madd_direct | -0.210 | moderate- |
| carrier_nat_madd_interp | -0.239 | moderate- |

**Consistency summary:**

- **carrier_nat_madd_copypatch**: won 6/6, lost 0/6
- **carrier_nat_madd_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_madd_copypatch | 120521.2ns | 32614.6ns | 369.5% | HIGH |
| carrier_nat_madd_direct | 112102.3ns | 32944.9ns | 340.3% | HIGH |
| carrier_nat_madd_interp | 95435.0ns | 47699.6ns | 200.1% | HIGH |

## Distribution (algo ns)

```
carrier_nat_madd_copypatch (n=6, range 32198.7-33164.6 ns)
  32198.7 |########################################
  32247.0 |
  32295.3 |
  32343.6 |
  32391.9 |
  32440.2 |########################################
  32488.5 |
  32536.8 |
  32585.1 |
  32633.4 |
  32681.7 |
  32729.9 |
  32778.2 |
  32826.5 |
  32874.8 |
  32923.1 |
  32971.4 |
  33019.7 |
  33068.0 |####################
  33116.3 |
  (0 below, 1 above range)

carrier_nat_madd_direct (n=6, range 32242.5-33618.6 ns)
  32242.5 |####################
  32311.3 |
  32380.1 |
  32448.9 |####################
  32517.7 |####################
  32586.5 |
  32655.3 |
  32724.1 |
  32792.9 |
  32861.7 |
  32930.5 |
  32999.3 |
  33068.1 |
  33136.9 |########################################
  33205.7 |
  33274.5 |
  33343.3 |
  33412.1 |
  33480.9 |
  33549.7 |
  (0 below, 1 above range)

carrier_nat_madd_interp (n=6, range 46140.8-48423.3 ns)
  46140.8 |####################
  46254.9 |
  46369.1 |
  46483.2 |
  46597.3 |
  46711.4 |
  46825.6 |
  46939.7 |
  47053.8 |####################
  47167.9 |
  47282.1 |
  47396.2 |
  47510.3 |
  47624.4 |
  47738.6 |
  47852.7 |####################
  47966.8 |
  48080.9 |########################################
  48195.1 |
  48309.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_madd_copypatch**: bridge=370.6% of algo (FFI overhead may distort results)
- **carrier_nat_madd_direct**: bridge=339.6% of algo (FFI overhead may distort results)
- **carrier_nat_madd_interp**: bridge=199.7% of algo (FFI overhead may distort results)
