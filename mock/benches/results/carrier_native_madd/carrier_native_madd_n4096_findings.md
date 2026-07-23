# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_madd_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_madd_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_madd_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_madd_interp has the worst median (194.14 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_madd_copypatch at 133.01 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_madd_copypatch beats baseline by 31% (significant)

carrier_nat_madd_copypatch is -61.15 us (31%) faster than baseline carrier_nat_madd_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (carrier_nat_madd_copypatch, carrier_nat_madd_direct) are a dead heat (<1%)

carrier_nat_madd_copypatch (133.01 us) and carrier_nat_madd_direct (134.09 us) differ by 0.81%, inside the noise, even though the wider field spreads 46.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_nat_madd_interp shows alternating (throttle bounce) (autocorr -0.60)

carrier_nat_madd_interp's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_nat_madd_copypatch** at 133011.5 ns median (-31.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.46x (fastest 133011.5 ns, slowest 194140.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 135179ns | 135266ns | 134302ns | 135132ns | 135689ns | -30.96% |
| carrier_nat_madd_direct | 136229ns | 136334ns | 133767ns | 135916ns | 137929ns | -30.42% |
| carrier_nat_madd_interp | 195786ns | 196763ns | 193214ns | 195593ns | 197362ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 132919ns | 131985ns | 133426ns | -31.25% | 0.031 |
| carrier_nat_madd_direct | 133962ns | 131570ns | 135589ns | -30.71% | 0.031 |
| carrier_nat_madd_interp | 193337ns | 190526ns | 195074ns | base | 0.021 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_madd_copypatch | 1087437 | 1968276 | 0.552 | 0.90× |
| carrier_nat_madd_direct | 973311 | 1549109 | 0.628 | 0.81× |
| carrier_nat_madd_interp | 1206878 | 4801960 | 0.251 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_nat_madd_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_madd_copypatch | 0.031 | 98.9% |
| carrier_nat_madd_direct | 0.031 | 98.1% |
| carrier_nat_madd_interp | 0.021 | 67.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_madd_copypatch | 135179ns | 135179ns | -30.96% |
| carrier_nat_madd_direct | 136229ns | 136229ns | -30.42% |
| carrier_nat_madd_interp | 195786ns | 195786ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_madd_interp | 194140ns | base | --- | [190799, 195074] | --- | --- | --- | --- |
| carrier_nat_madd_copypatch | 133011ns | -61151.9ns (-31.5%) | [-62675, -57428]ns | [132320, 133426] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_madd_direct | 134092ns | -59512.9ns (-30.7%) | [-60982, -57631]ns | [132207, 135589] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_madd_interp | carrier_nat_madd_copypatch | carrier_nat_madd_direct |
|---|---|---|---|
| 1 | 195278ns | -32.4% | -31.4% |
| 2 | 190526ns | -30.1% | -30.9% |
| 3 | 194744ns | -31.5% | -30.8% |
| 4 | 191072ns | -30.1% | -30.5% |
| 5 | 193536ns | -31.5% | -29.5% |
| 6 | 194869ns | -31.8% | -31.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_madd_copypatch | -0.027 | ok |
| carrier_nat_madd_direct | -0.356 | moderate- |
| carrier_nat_madd_interp | -0.602 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_nat_madd_copypatch**: won 6/6, lost 0/6
- **carrier_nat_madd_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_madd_copypatch | 214993.3ns | 132919.1ns | 161.7% | HIGH |
| carrier_nat_madd_direct | 179520.3ns | 133962.4ns | 134.0% | HIGH |
| carrier_nat_madd_interp | 193474.2ns | 193337.5ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_nat_madd_copypatch (n=6, range 131985.0-133426.0 ns)
  131985.0 |########################################
  132057.0 |
  132129.1 |
  132201.1 |
  132273.2 |
  132345.2 |
  132417.3 |
  132489.4 |
  132561.4 |
  132633.5 |########################################
  132705.5 |
  132777.5 |########################################
  132849.6 |
  132921.6 |
  132993.7 |
  133065.8 |
  133137.8 |
  133209.9 |########################################
  133281.9 |########################################
  133354.0 |
  (0 below, 1 above range)

carrier_nat_madd_direct (n=6, range 131569.6-135588.5 ns)
  131569.6 |####################
  131770.5 |
  131971.5 |
  132172.4 |
  132373.4 |
  132574.3 |
  132775.3 |####################
  132976.2 |
  133177.2 |
  133378.1 |
  133579.1 |
  133780.0 |
  133981.0 |########################################
  134181.9 |
  134382.9 |
  134583.8 |####################
  134784.8 |
  134985.7 |
  135186.7 |
  135387.6 |
  (0 below, 1 above range)

carrier_nat_madd_interp (n=6, range 190525.8-195073.5 ns)
  190525.8 |########################################
  190753.2 |
  190980.6 |########################################
  191208.0 |
  191435.3 |
  191662.7 |
  191890.1 |
  192117.5 |
  192344.9 |
  192572.3 |
  192799.7 |
  193027.1 |
  193254.4 |
  193481.8 |########################################
  193709.2 |
  193936.6 |
  194164.0 |
  194391.4 |
  194618.8 |########################################
  194846.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_madd_copypatch**: bridge=161.1% of algo (FFI overhead may distort results)
- **carrier_nat_madd_direct**: bridge=133.4% of algo (FFI overhead may distort results)
- **carrier_nat_madd_interp**: bridge=99.9% of algo (FFI overhead may distort results)
