# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_tight_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_tight_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_tight_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_tight_interp has the worst median (10.22 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_tight_direct at 4.71 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_tight_direct beats baseline by 54% (significant)

carrier_nat_tight_direct is -5.56 us (54%) faster than baseline carrier_nat_tight_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_tight_interp is an outlier: 2.2x slower than the field

carrier_nat_tight_interp (10.22 us) is 2.2x the fastest (4.71 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_nat_tight_direct** at 4708.4 ns median (-53.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.17x (fastest 4708.4 ns, slowest 10216.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 7273ns | 7513ns | 6631ns | 7257ns | 7617ns | -42.74% |
| carrier_nat_tight_direct | 7034ns | 6967ns | 6581ns | 6877ns | 7496ns | -44.62% |
| carrier_nat_tight_interp | 12701ns | 12437ns | 12059ns | 12314ns | 13603ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 4906ns | 4462ns | 5139ns | -52.92% | 0.052 |
| carrier_nat_tight_direct | 4744ns | 4438ns | 5044ns | -54.47% | 0.054 |
| carrier_nat_tight_interp | 10420ns | 9899ns | 11140ns | base | 0.025 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_tight_copypatch | 279381 | 704088 | 0.397 | 0.92× |
| carrier_nat_tight_direct | 275902 | 688126 | 0.401 | 0.90× |
| carrier_nat_tight_interp | 305125 | 1439502 | 0.212 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.058 Gops/s** (carrier_nat_tight_direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_tight_copypatch | 0.051 | 87.7% |
| carrier_nat_tight_direct | 0.054 | 94.2% |
| carrier_nat_tight_interp | 0.025 | 43.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_tight_copypatch | 7273ns | 7273ns | -42.74% |
| carrier_nat_tight_direct | 7034ns | 7034ns | -44.62% |
| carrier_nat_tight_interp | 12701ns | 12701ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_tight_interp | 10216ns | base | --- | [9902, 11140] | --- | --- | --- | --- |
| carrier_nat_tight_copypatch | 5060ns | -5424.6ns (-53.1%) | [-6023, -5093]ns | [4519, 5139] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_tight_direct | 4708ns | -5555.4ns (-54.4%) | [-6097, -5374]ns | [4481, 5044] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_tight_interp | carrier_nat_tight_copypatch | carrier_nat_tight_direct |
|---|---|---|---|
| 1 | 9966ns | -48.7% | -55.5% |
| 2 | 10467ns | -51.7% | -53.3% |
| 3 | 9905ns | -53.8% | -54.2% |
| 4 | 9899ns | -54.9% | -54.3% |
| 5 | 11255ns | -55.0% | -55.9% |
| 6 | 11025ns | -53.1% | -53.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_tight_copypatch | 0.213 | moderate+ |
| carrier_nat_tight_direct | 0.022 | ok |
| carrier_nat_tight_interp | 0.162 | ok |

**Consistency summary:**

- **carrier_nat_tight_copypatch**: won 6/6, lost 0/6
- **carrier_nat_tight_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_tight_copypatch | 89207.2ns | 4906.0ns | 1818.3% | HIGH |
| carrier_nat_tight_direct | 85341.6ns | 4744.2ns | 1798.9% | HIGH |
| carrier_nat_tight_interp | 89762.3ns | 10419.6ns | 861.5% | HIGH |

## Distribution (algo ns)

```
carrier_nat_tight_copypatch (n=6, range 4462.1-5138.8 ns)
   4462.1 |####################
   4495.9 |
   4529.8 |
   4563.6 |####################
   4597.4 |
   4631.3 |
   4665.1 |
   4698.9 |
   4732.8 |
   4766.6 |
   4800.4 |
   4834.3 |
   4868.1 |
   4901.9 |
   4935.8 |
   4969.6 |
   5003.4 |
   5037.3 |########################################
   5071.1 |
   5104.9 |####################
  (0 below, 1 above range)

carrier_nat_tight_direct (n=6, range 4437.5-5043.5 ns)
   4437.5 |########################################
   4467.8 |
   4498.1 |########################################
   4528.4 |########################################
   4558.7 |
   4589.0 |
   4619.3 |
   4649.6 |
   4679.9 |
   4710.2 |
   4740.5 |
   4770.8 |
   4801.1 |
   4831.4 |
   4861.7 |########################################
   4892.0 |
   4922.3 |
   4952.6 |########################################
   4982.9 |
   5013.2 |
  (0 below, 1 above range)

carrier_nat_tight_interp (n=6, range 9899.2-11140.2 ns)
   9899.2 |########################################
   9961.2 |####################
  10023.3 |
  10085.4 |
  10147.4 |
  10209.5 |
  10271.5 |
  10333.6 |
  10395.6 |
  10457.7 |####################
  10519.7 |
  10581.8 |
  10643.8 |
  10705.9 |
  10767.9 |
  10830.0 |
  10892.0 |
  10954.1 |
  11016.1 |####################
  11078.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_tight_copypatch**: bridge=1757.5% of algo (FFI overhead may distort results)
- **carrier_nat_tight_direct**: bridge=1808.4% of algo (FFI overhead may distort results)
- **carrier_nat_tight_interp**: bridge=876.8% of algo (FFI overhead may distort results)
