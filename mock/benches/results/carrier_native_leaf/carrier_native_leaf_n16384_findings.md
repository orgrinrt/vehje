# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_leaf_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_leaf_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_leaf_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_leaf_interp has the worst median (1.16 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_leaf_copypatch at 296.19 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_leaf_copypatch beats baseline by 74% (significant)

carrier_nat_leaf_copypatch is -858.34 us (74%) faster than baseline carrier_nat_leaf_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_leaf_interp is an outlier: 3.9x slower than the field

carrier_nat_leaf_interp (1.16 ms) is 3.9x the fastest (296.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_nat_leaf_copypatch, carrier_nat_leaf_direct) are a dead heat (<1%)

carrier_nat_leaf_copypatch (296.19 us) and carrier_nat_leaf_direct (297.86 us) differ by 0.56%, inside the noise, even though the wider field spreads 290.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_nat_leaf_interp shows warm-up / thermal drift (autocorr +0.50)

carrier_nat_leaf_interp's per-pass series has lag-1 autocorrelation +0.50, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.9x the fastest

Fastest carrier_nat_leaf_copypatch (296.19 us) to slowest carrier_nat_leaf_interp (1.16 ms): 3.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_nat_leaf_copypatch** at 296195.0 ns median (-74.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.90x (fastest 296195.0 ns, slowest 1155482.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 299802ns | 298450ns | 296373ns | 297798ns | 304525ns | -74.06% |
| carrier_nat_leaf_direct | 300082ns | 300197ns | 294703ns | 299741ns | 303282ns | -74.04% |
| carrier_nat_leaf_interp | 1155795ns | 1158498ns | 1138909ns | 1155012ns | 1165412ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 297449ns | 294078ns | 302055ns | -74.20% | 0.055 |
| carrier_nat_leaf_direct | 297767ns | 292458ns | 300945ns | -74.17% | 0.055 |
| carrier_nat_leaf_interp | 1152927ns | 1136096ns | 1162765ns | base | 0.014 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 2608255 | 6666627 | 0.391 | 0.36× |
| carrier_nat_leaf_direct | 2370951 | 5577829 | 0.425 | 0.33× |
| carrier_nat_leaf_interp | 7146171 | 17911287 | 0.399 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.056 Gops/s** (carrier_nat_leaf_direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_leaf_copypatch | 0.055 | 98.7% |
| carrier_nat_leaf_direct | 0.055 | 98.2% |
| carrier_nat_leaf_interp | 0.014 | 25.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_leaf_copypatch | 299802ns | 299802ns | -74.06% |
| carrier_nat_leaf_direct | 300082ns | 300082ns | -74.04% |
| carrier_nat_leaf_interp | 1155795ns | 1155795ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_leaf_interp | 1155482ns | base | --- | [1140532, 1162765] | --- | --- | --- | --- |
| carrier_nat_leaf_copypatch | 296195ns | -858338.9ns (-74.3%) | [-867086, -841009]ns | [294096, 302055] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_leaf_direct | 297862ns | -856742.9ns (-74.1%) | [-864903, -843832]ns | [294495, 300945] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_leaf_interp | carrier_nat_leaf_copypatch | carrier_nat_leaf_direct |
|---|---|---|---|
| 1 | 1158650ns | -74.1% | -74.4% |
| 2 | 1163099ns | -74.4% | -74.5% |
| 3 | 1162431ns | -74.7% | -74.3% |
| 4 | 1152315ns | -74.5% | -73.9% |
| 5 | 1144969ns | -74.2% | -73.7% |
| 6 | 1136096ns | -73.3% | -74.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_leaf_copypatch | 0.055 | ok |
| carrier_nat_leaf_direct | -0.077 | ok |
| carrier_nat_leaf_interp | 0.502 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **carrier_nat_leaf_copypatch**: won 6/6, lost 0/6
- **carrier_nat_leaf_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 538195.8ns | 297448.8ns | 180.9% | HIGH |
| carrier_nat_leaf_direct | 462679.8ns | 297767.4ns | 155.4% | HIGH |
| carrier_nat_leaf_interp | 1153828.6ns | 1152926.7ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_nat_leaf_copypatch (n=6, range 294078.3-302054.8 ns)
  294078.3 |########################################
  294477.1 |
  294876.0 |####################
  295274.8 |
  295673.6 |
  296072.4 |
  296471.2 |
  296870.1 |####################
  297268.9 |
  297667.7 |
  298066.5 |
  298465.4 |
  298864.2 |
  299263.0 |
  299661.8 |
  300060.7 |####################
  300459.5 |
  300858.3 |
  301257.1 |
  301656.0 |
  (0 below, 1 above range)

carrier_nat_leaf_direct (n=6, range 292457.5-300945.2 ns)
  292457.5 |########################################
  292881.9 |
  293306.3 |
  293730.7 |
  294155.0 |
  294579.4 |
  295003.8 |
  295428.2 |
  295852.6 |
  296277.0 |########################################
  296701.3 |
  297125.7 |########################################
  297550.1 |
  297974.5 |
  298398.9 |########################################
  298823.3 |
  299247.7 |
  299672.0 |
  300096.4 |
  300520.8 |########################################
  (0 below, 1 above range)

carrier_nat_leaf_interp (n=6, range 1136095.8-1162765.0 ns)
  1136095.8 |########################################
  1137429.3 |
  1138762.7 |
  1140096.2 |
  1141429.6 |
  1142763.1 |
  1144096.6 |########################################
  1145430.0 |
  1146763.5 |
  1148096.9 |
  1149430.4 |
  1150763.9 |
  1152097.3 |########################################
  1153430.8 |
  1154764.2 |
  1156097.7 |
  1157431.2 |########################################
  1158764.6 |
  1160098.1 |
  1161431.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_leaf_copypatch**: bridge=180.3% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_direct**: bridge=155.7% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_interp**: autocorrelation=0.50 (measurement drift or warm-up artifact)
- **carrier_nat_leaf_interp**: bridge=100.1% of algo (FFI overhead may distort results)
