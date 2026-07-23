# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_tight_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_tight_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_tight_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_tight_interp has the worst median (187.65 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_tight_direct at 86.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_tight_direct beats baseline by 54% (significant)

carrier_nat_tight_direct is -101.72 us (54%) faster than baseline carrier_nat_tight_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_tight_interp is an outlier: 2.2x slower than the field

carrier_nat_tight_interp (187.65 us) is 2.2x the fastest (86.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_nat_tight_direct, carrier_nat_tight_copypatch) are a dead heat (<1%)

carrier_nat_tight_direct (86.29 us) and carrier_nat_tight_copypatch (86.97 us) differ by 0.79%, inside the noise, even though the wider field spreads 117.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: carrier_nat_tight_direct** at 86292.3 ns median (-54.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.17x (fastest 86292.3 ns, slowest 187645.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 89917ns | 89307ns | 88504ns | 89057ns | 91913ns | -52.68% |
| carrier_nat_tight_direct | 88940ns | 88539ns | 88085ns | 88427ns | 90137ns | -53.19% |
| carrier_nat_tight_interp | 190018ns | 189916ns | 187037ns | 189217ns | 192710ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 87600ns | 86246ns | 89547ns | -53.32% | 0.047 |
| carrier_nat_tight_direct | 86684ns | 85866ns | 87832ns | -53.81% | 0.047 |
| carrier_nat_tight_interp | 187677ns | 184735ns | 190207ns | base | 0.022 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_tight_copypatch | 807606 | 1966554 | 0.411 | 0.69× |
| carrier_nat_tight_direct | 689836 | 1547849 | 0.446 | 0.59× |
| carrier_nat_tight_interp | 1172208 | 4800890 | 0.244 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.048 Gops/s** (carrier_nat_tight_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_tight_copypatch | 0.047 | 98.7% |
| carrier_nat_tight_direct | 0.047 | 99.5% |
| carrier_nat_tight_interp | 0.022 | 45.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_tight_copypatch | 89917ns | 89917ns | -52.68% |
| carrier_nat_tight_direct | 88940ns | 88940ns | -53.19% |
| carrier_nat_tight_interp | 190018ns | 190018ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_tight_interp | 187645ns | base | --- | [185178, 190207] | --- | --- | --- | --- |
| carrier_nat_tight_copypatch | 86973ns | -101110.2ns (-53.9%) | [-103490, -95631]ns | [86278, 89547] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_tight_direct | 86292ns | -101716.9ns (-54.2%) | [-103649, -97612]ns | [85928, 87832] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_tight_interp | carrier_nat_tight_copypatch | carrier_nat_tight_direct |
|---|---|---|---|
| 1 | 190850ns | -54.8% | -54.9% |
| 2 | 187814ns | -54.0% | -54.2% |
| 3 | 185621ns | -52.1% | -52.3% |
| 4 | 189564ns | -54.0% | -54.1% |
| 5 | 187477ns | -53.7% | -54.2% |
| 6 | 184735ns | -51.2% | -53.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_tight_copypatch | -0.176 | ok |
| carrier_nat_tight_direct | -0.067 | ok |
| carrier_nat_tight_interp | -0.132 | ok |

**Consistency summary:**

- **carrier_nat_tight_copypatch**: won 6/6, lost 0/6
- **carrier_nat_tight_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_tight_copypatch | 174346.2ns | 87599.6ns | 199.0% | HIGH |
| carrier_nat_tight_direct | 134623.1ns | 86684.1ns | 155.3% | HIGH |
| carrier_nat_tight_interp | 188161.9ns | 187676.8ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_nat_tight_copypatch (n=6, range 86245.8-89547.3 ns)
  86245.8 |########################################
  86410.9 |
  86575.9 |
  86741.0 |####################
  86906.1 |
  87071.2 |####################
  87236.2 |
  87401.3 |
  87566.4 |
  87731.5 |
  87896.5 |
  88061.6 |
  88226.7 |
  88391.8 |
  88556.8 |
  88721.9 |####################
  88887.0 |
  89052.1 |
  89217.1 |
  89382.2 |
  (0 below, 1 above range)

carrier_nat_tight_direct (n=6, range 85866.2-87831.6 ns)
  85866.2 |####################
  85964.5 |########################################
  86062.7 |
  86161.0 |
  86259.3 |
  86357.6 |
  86455.8 |####################
  86554.1 |
  86652.4 |
  86750.7 |
  86848.9 |
  86947.2 |
  87045.5 |####################
  87143.7 |
  87242.0 |
  87340.3 |
  87438.6 |
  87536.8 |
  87635.1 |
  87733.4 |
  (0 below, 1 above range)

carrier_nat_tight_interp (n=6, range 184735.4-190206.7 ns)
  184735.4 |########################################
  185009.0 |
  185282.5 |
  185556.1 |########################################
  185829.6 |
  186103.2 |
  186376.8 |
  186650.3 |
  186923.9 |
  187197.5 |
  187471.0 |########################################
  187744.6 |########################################
  188018.2 |
  188291.7 |
  188565.3 |
  188838.8 |
  189112.4 |
  189386.0 |########################################
  189659.5 |
  189933.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_tight_copypatch**: bridge=200.0% of algo (FFI overhead may distort results)
- **carrier_nat_tight_direct**: bridge=155.9% of algo (FFI overhead may distort results)
- **carrier_nat_tight_interp**: bridge=100.3% of algo (FFI overhead may distort results)
