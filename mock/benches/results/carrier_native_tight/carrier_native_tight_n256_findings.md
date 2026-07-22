# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, tight profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_tight_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_tight_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_tight_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_tight_interp has the worst median (11.06 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_tight_stencil at 5.06 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_tight_stencil beats baseline by 54% (significant)

carrier_nat_tight_stencil is -5.99 us (54%) faster than baseline carrier_nat_tight_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_tight_interp is an outlier: 2.2x slower than the field

carrier_nat_tight_interp (11.06 us) is 2.2x the fastest (5.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_nat_tight_interp shows alternating (throttle bounce) (autocorr -0.64)

carrier_nat_tight_interp's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_nat_tight_stencil** at 5060.6 ns median (-54.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.19x (fastest 5060.6 ns, slowest 11058.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 7575ns | 7625ns | 6897ns | 7412ns | 8157ns | -44.32% |
| carrier_nat_tight_interp | 13605ns | 13516ns | 12115ns | 13112ns | 15089ns | base |
| carrier_nat_tight_stencil | 7528ns | 7521ns | 6866ns | 7395ns | 8059ns | -44.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 5075ns | 4598ns | 5456ns | -54.42% | 0.050 |
| carrier_nat_tight_interp | 11134ns | 9893ns | 12369ns | base | 0.023 |
| carrier_nat_tight_stencil | 5055ns | 4563ns | 5418ns | -54.60% | 0.051 |

## Performance model

- Peak throughput: **0.056 Gops/s** (carrier_nat_tight_stencil; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_tight_copypatch | 0.050 | 88.9% |
| carrier_nat_tight_interp | 0.023 | 41.3% |
| carrier_nat_tight_stencil | 0.051 | 90.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_tight_copypatch | 7575ns | 7575ns | -44.32% |
| carrier_nat_tight_interp | 13605ns | 13605ns | base |
| carrier_nat_tight_stencil | 7528ns | 7528ns | -44.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_tight_interp | 11058ns | base | --- | [9974, 12369] | --- | --- | --- | --- |
| carrier_nat_tight_copypatch | 5133ns | -5947.6ns (-53.8%) | [-7104, -5123]ns | [4637, 5456] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_tight_stencil | 5061ns | -5993.4ns (-54.2%) | [-7520, -4723]ns | [4686, 5418] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_tight_interp | carrier_nat_tight_copypatch | carrier_nat_tight_stencil |
|---|---|---|---|
| 1 | 9893ns | -53.5% | -42.5% |
| 2 | 13602ns | -57.7% | -66.5% |
| 3 | 10054ns | -49.2% | -52.2% |
| 4 | 11032ns | -57.6% | -54.4% |
| 5 | 11084ns | -53.4% | -54.1% |
| 6 | 11137ns | -53.6% | -53.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_tight_copypatch | -0.399 | moderate- |
| carrier_nat_tight_interp | -0.637 | HIGH- (thermal bounce) |
| carrier_nat_tight_stencil | -0.257 | moderate- |

**Consistency summary:**

- **carrier_nat_tight_copypatch**: won 6/6, lost 0/6
- **carrier_nat_tight_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_tight_copypatch | 86430.1ns | 5075.3ns | 1703.0% | HIGH |
| carrier_nat_tight_interp | 90991.5ns | 11133.6ns | 817.3% | HIGH |
| carrier_nat_tight_stencil | 86225.1ns | 5054.9ns | 1705.8% | HIGH |

## Distribution (algo ns)

```
carrier_nat_tight_copypatch (n=6, range 4597.5-5455.8 ns)
   4597.5 |####################
   4640.4 |####################
   4683.3 |
   4726.2 |
   4769.2 |
   4812.1 |
   4855.0 |
   4897.9 |
   4940.8 |
   4983.7 |
   5026.6 |
   5069.6 |####################
   5112.5 |
   5155.4 |########################################
   5198.3 |
   5241.2 |
   5284.1 |
   5327.1 |
   5370.0 |
   5412.9 |
  (0 below, 1 above range)

carrier_nat_tight_interp (n=6, range 9893.3-12369.4 ns)
   9893.3 |####################
  10017.1 |####################
  10140.9 |
  10264.7 |
  10388.5 |
  10512.3 |
  10636.1 |
  10759.9 |
  10883.7 |
  11007.5 |########################################
  11131.4 |####################
  11255.2 |
  11379.0 |
  11502.8 |
  11626.6 |
  11750.4 |
  11874.2 |
  11998.0 |
  12121.8 |
  12245.6 |
  (0 below, 1 above range)

carrier_nat_tight_stencil (n=6, range 4562.9-5418.1 ns)
   4562.9 |########################################
   4605.7 |
   4648.4 |
   4691.2 |
   4733.9 |
   4776.7 |########################################
   4819.5 |
   4862.2 |
   4905.0 |
   4947.8 |
   4990.5 |########################################
   5033.3 |
   5076.0 |########################################
   5118.8 |########################################
   5161.6 |
   5204.3 |
   5247.1 |
   5289.9 |
   5332.6 |
   5375.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_tight_copypatch**: bridge=1662.0% of algo (FFI overhead may distort results)
- **carrier_nat_tight_interp**: bridge=806.6% of algo (FFI overhead may distort results)
- **carrier_nat_tight_stencil**: bridge=1687.0% of algo (FFI overhead may distort results)
