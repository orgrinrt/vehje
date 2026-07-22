# Predecode: zero-copy wire decode vs flat predecoded form (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_predec_wire**

## Highlights

Baseline for all deltas below: **carrier_predec_wire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_predec_wire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_predec_wire has the worst median (552.03 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_predec_flatthread at 419.20 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_predec_flatthread beats baseline by 26% (significant)

carrier_predec_flatthread is -141.82 us (26%) faster than baseline carrier_predec_wire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_predec_flatthread is fastest but the noisiest (CV 7.0%)

carrier_predec_flatthread wins on median (419.20 us) yet has the highest variance (CV 7.0%), while carrier_predec_wire is the steadiest (CV 3.2%, 552.03 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_predec_wire shows alternating (throttle bounce) (autocorr -0.55)

carrier_predec_wire's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_predec_flatthread** at 419195.4 ns median (-24.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.32x (fastest 419195.4 ns, slowest 552034.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_predec_flat | 436030ns | 433791ns | 419686ns | 429495ns | 454004ns | -21.99% |
| carrier_predec_flatthread | 422347ns | 421446ns | 386568ns | 411703ns | 456202ns | -24.44% |
| carrier_predec_wire | 558938ns | 554952ns | 536771ns | 551457ns | 581242ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_predec_flat | 433763ns | 417486ns | 451751ns | -22.00% | 0.009 |
| carrier_predec_flatthread | 420117ns | 384340ns | 453978ns | -24.45% | 0.010 |
| carrier_predec_wire | 556100ns | 533821ns | 578678ns | base | 0.007 |

## Performance model

- Peak throughput: **0.011 Gops/s** (carrier_predec_flatthread; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_predec_flat | 0.009 | 89.1% |
| carrier_predec_flatthread | 0.010 | 91.7% |
| carrier_predec_wire | 0.007 | 69.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_predec_flat | 436030ns | 436030ns | -21.99% |
| carrier_predec_flatthread | 422347ns | 422347ns | -24.44% |
| carrier_predec_wire | 558938ns | 558938ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_predec_wire | 552034ns | base | --- | [537588, 578678] | --- | --- | --- | --- |
| carrier_predec_flat | 431527ns | -124647.5ns (-22.6%) | [-142265, -100099]ns | [418011, 451751] | YES | 0.0313 | 0.0313 | 0 |
| carrier_predec_flatthread | 419195ns | -141818.8ns (-25.7%) | [-182522, -83611]ns | [387176, 453978] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_predec_wire | carrier_predec_flat | carrier_predec_flatthread |
|---|---|---|---|
| 1 | 560107ns | -17.7% | -31.4% |
| 2 | 533821ns | -19.0% | -17.3% |
| 3 | 579289ns | -23.6% | -32.7% |
| 4 | 541356ns | -22.7% | -13.8% |
| 5 | 543962ns | -23.3% | -20.5% |
| 6 | 578067ns | -25.5% | -29.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_predec_flat | 0.096 | ok |
| carrier_predec_flatthread | -0.468 | moderate- |
| carrier_predec_wire | -0.546 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_predec_flat**: won 6/6, lost 0/6
- **carrier_predec_flatthread**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_predec_flat | 9857.0ns | 433763.1ns | 2.3% |  |
| carrier_predec_flatthread | 9475.5ns | 420116.5ns | 2.3% |  |
| carrier_predec_wire | 430.6ns | 556100.2ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_predec_flat (n=6, range 417485.8-451750.8 ns)
  417485.8 |########################################
  419199.0 |
  420912.3 |
  422625.5 |
  424338.8 |
  426052.0 |
  427765.3 |
  429478.5 |####################
  431191.8 |####################
  432905.0 |
  434618.3 |
  436331.5 |
  438044.8 |
  439758.0 |
  441471.3 |####################
  443184.5 |
  444897.8 |
  446611.0 |
  448324.3 |
  450037.5 |
  (0 below, 1 above range)

carrier_predec_flatthread (n=6, range 384340.0-453977.9 ns)
  384340.0 |########################################
  387821.9 |########################################
  391303.8 |
  394785.7 |
  398267.6 |
  401749.5 |
  405231.4 |########################################
  408713.3 |
  412195.2 |
  415677.1 |
  419159.0 |
  422640.8 |
  426122.7 |
  429604.6 |########################################
  433086.5 |
  436568.4 |
  440050.3 |########################################
  443532.2 |
  447014.1 |
  450496.0 |
  (0 below, 1 above range)

carrier_predec_wire (n=6, range 533821.2-578677.9 ns)
  533821.2 |########################################
  536064.0 |
  538306.9 |
  540549.7 |########################################
  542792.5 |########################################
  545035.4 |
  547278.2 |
  549521.1 |
  551763.9 |
  554006.7 |
  556249.6 |
  558492.4 |########################################
  560735.2 |
  562978.1 |
  565220.9 |
  567463.8 |
  569706.6 |
  571949.4 |
  574192.3 |
  576435.1 |########################################
  (0 below, 1 above range)

```
