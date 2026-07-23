# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_tight_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_tight_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_tight_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_tight_interp has the worst median (2.17 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_tight_copypatch at 793 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_tight_copypatch beats baseline by 63% (significant)

carrier_nat_tight_copypatch is -1.37 us (63%) faster than baseline carrier_nat_tight_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_tight_interp is an outlier: 2.7x slower than the field

carrier_nat_tight_interp (2.17 us) is 2.7x the fastest (793 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_nat_tight_copypatch** at 792.9 ns median (-63.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.74x (fastest 792.9 ns, slowest 2171.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 3121ns | 3047ns | 2984ns | 3030ns | 3326ns | -30.07% |
| carrier_nat_tight_direct | 3187ns | 3194ns | 2968ns | 3127ns | 3387ns | -28.60% |
| carrier_nat_tight_interp | 4464ns | 4520ns | 4078ns | 4401ns | 4751ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 818ns | 783ns | 875ns | -61.92% | 0.078 |
| carrier_nat_tight_direct | 840ns | 767ns | 892ns | -60.93% | 0.076 |
| carrier_nat_tight_interp | 2149ns | 1961ns | 2292ns | base | 0.030 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_tight_copypatch | 243496 | 824210 | 0.295 | 0.91× |
| carrier_nat_tight_direct | 244804 | 820419 | 0.298 | 0.92× |
| carrier_nat_tight_interp | 266265 | 1517883 | 0.175 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.083 Gops/s** (carrier_nat_tight_direct; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_tight_copypatch | 0.081 | 96.7% |
| carrier_nat_tight_direct | 0.076 | 90.7% |
| carrier_nat_tight_interp | 0.029 | 35.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_tight_copypatch | 3121ns | 3121ns | -30.07% |
| carrier_nat_tight_direct | 3187ns | 3187ns | -28.60% |
| carrier_nat_tight_interp | 4464ns | 4464ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_tight_interp | 2171ns | base | --- | [1984, 2292] | --- | --- | --- | --- |
| carrier_nat_tight_copypatch | 793ns | -1366.1ns (-62.9%) | [-1477, -1149]ns | [786, 875] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_tight_direct | 845ns | -1325.6ns (-61.1%) | [-1399, -1203]ns | [781, 892] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_tight_interp | carrier_nat_tight_copypatch | carrier_nat_tight_direct |
|---|---|---|---|
| 1 | 2007ns | -60.5% | -60.4% |
| 2 | 1961ns | -55.2% | -60.9% |
| 3 | 2314ns | -62.3% | -61.1% |
| 4 | 2269ns | -65.0% | -61.1% |
| 5 | 2260ns | -65.4% | -61.3% |
| 6 | 2082ns | -62.1% | -60.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_tight_copypatch | 0.227 | moderate+ |
| carrier_nat_tight_direct | 0.148 | ok |
| carrier_nat_tight_interp | 0.188 | ok |

**Consistency summary:**

- **carrier_nat_tight_copypatch**: won 6/6, lost 0/6
- **carrier_nat_tight_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_tight_copypatch | 78217.2ns | 818.2ns | 9559.1% | HIGH |
| carrier_nat_tight_direct | 85436.6ns | 839.6ns | 10176.1% | HIGH |
| carrier_nat_tight_interp | 86289.7ns | 2148.8ns | 4015.7% | HIGH |

## Distribution (algo ns)

```
carrier_nat_tight_copypatch (n=6, range 782.9-875.4 ns)
    782.9 |####################
    787.5 |####################
    792.1 |########################################
    796.8 |
    801.4 |
    806.0 |
    810.6 |
    815.3 |
    819.9 |
    824.5 |
    829.1 |
    833.8 |
    838.4 |
    843.0 |
    847.6 |
    852.3 |
    856.9 |
    861.5 |
    866.1 |
    870.8 |####################
  (0 below, 1 above range)

carrier_nat_tight_direct (n=6, range 766.7-892.5 ns)
    766.7 |########################################
    773.0 |
    779.3 |
    785.6 |
    791.9 |########################################
    798.2 |
    804.4 |
    810.7 |########################################
    817.0 |
    823.3 |
    829.6 |
    835.9 |
    842.2 |
    848.5 |
    854.8 |
    861.0 |
    867.3 |
    873.6 |########################################
    879.9 |########################################
    886.2 |
  (0 below, 1 above range)

carrier_nat_tight_interp (n=6, range 1960.8-2291.5 ns)
   1960.8 |####################
   1977.3 |
   1993.9 |####################
   2010.4 |
   2026.9 |
   2043.5 |
   2060.0 |
   2076.5 |####################
   2093.1 |
   2109.6 |
   2126.2 |
   2142.7 |
   2159.2 |
   2175.8 |
   2192.3 |
   2208.8 |
   2225.4 |
   2241.9 |
   2258.4 |########################################
   2275.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_tight_copypatch**: bridge=9846.0% of algo (FFI overhead may distort results)
- **carrier_nat_tight_direct**: bridge=9516.7% of algo (FFI overhead may distort results)
- **carrier_nat_tight_interp**: bridge=3973.1% of algo (FFI overhead may distort results)
