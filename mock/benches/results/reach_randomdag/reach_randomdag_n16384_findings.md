# Reachability fixpoint: whole-column vs real semi-naive (randomdag)

2 variants, 6 samples per variant.
Baseline: **r_randomdag_whole**

## Highlights

Baseline for all deltas below: **r_randomdag_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_randomdag_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_randomdag_whole has the worst median (21.08 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_randomdag_semi at 8.27 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_randomdag_semi dominates: 155% faster than the next best (r_randomdag_whole)

r_randomdag_semi (8.27 ms) leads r_randomdag_whole (21.08 ms) by 155%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_randomdag_semi beats baseline by 61% (significant)

r_randomdag_semi is -12.84 ms (61%) faster than baseline r_randomdag_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: r_randomdag_semi** at 8273364.8 ns median (-60.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.55x (fastest 8273364.8 ns, slowest 21082128.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_randomdag_semi | 8428376ns | 8276488ns | 8176092ns | 8259936ns | 8807180ns | -60.88% |
| r_randomdag_whole | 21547192ns | 21086319ns | 21013500ns | 21067224ns | 22533990ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_randomdag_semi | 8425041ns | 8173512ns | 8803090ns | -60.89% | 0.002 |
| r_randomdag_whole | 21543071ns | 21009619ns | 22529537ns | base | 0.001 |

## Performance model

- Peak throughput: **0.002 Gops/s** (r_randomdag_semi; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_randomdag_semi | 0.002 | 98.8% |
| r_randomdag_whole | 0.001 | 38.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_randomdag_semi | 8428376ns | 8428376ns | -60.88% |
| r_randomdag_whole | 21547192ns | 21547192ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_randomdag_whole | 21082128ns | base | --- | [21017546, 22529537] | --- | --- | --- | --- |
| r_randomdag_semi | 8273365ns | -12840802.1ns (-60.9%) | [-13761588, -12751700]ns | [8198667, 8803090] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_randomdag_whole | r_randomdag_semi |
|---|---|---|
| 1 | 21009619ns | -61.1% |
| 2 | 21094937ns | -60.6% |
| 3 | 21103348ns | -61.0% |
| 4 | 21069319ns | -61.0% |
| 5 | 23955727ns | -61.2% |
| 6 | 21025474ns | -60.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_randomdag_semi | -0.203 | moderate- |
| r_randomdag_whole | -0.250 | moderate- |

**Consistency summary:**

- **r_randomdag_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_randomdag_semi | 5786.1ns | 8425040.6ns | 0.1% |  |
| r_randomdag_whole | 6029.0ns | 21543070.6ns | 0.0% |  |

## Distribution (algo ns)

```
r_randomdag_semi (n=6, range 8173511.7-8803090.0 ns)
  8173511.7 |####################
  8204990.6 |####################
  8236469.5 |####################
  8267948.4 |
  8299427.4 |########################################
  8330906.3 |
  8362385.2 |
  8393864.1 |
  8425343.0 |
  8456821.9 |
  8488300.8 |
  8519779.8 |
  8551258.7 |
  8582737.6 |
  8614216.5 |
  8645695.4 |
  8677174.3 |
  8708653.3 |
  8740132.2 |
  8771611.1 |
  (0 below, 1 above range)

r_randomdag_whole (n=6, range 21009619.2-22529537.3 ns)
  21009619.2 |########################################
  21085615.1 |##########################
  21161611.0 |
  21237606.9 |
  21313602.8 |
  21389598.7 |
  21465594.6 |
  21541590.5 |
  21617586.4 |
  21693582.3 |
  21769578.2 |
  21845574.2 |
  21921570.1 |
  21997566.0 |
  22073561.9 |
  22149557.8 |
  22225553.7 |
  22301549.6 |
  22377545.5 |
  22453541.4 |
  (0 below, 1 above range)

```
