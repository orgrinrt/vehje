# Reachability fixpoint: whole-column vs real semi-naive (chain128)

2 variants, 6 samples per variant.
Baseline: **r_chain128_whole**

## Highlights

Baseline for all deltas below: **r_chain128_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain128_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain128_whole has the worst median (881.17 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain128_semi at 293.13 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain128_semi dominates: 201% faster than the next best (r_chain128_whole)

r_chain128_semi (293.13 us) leads r_chain128_whole (881.17 us) by 201%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain128_semi beats baseline by 67% (significant)

r_chain128_semi is -588.04 us (67%) faster than baseline r_chain128_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### r_chain128_semi shows alternating (throttle bounce) (autocorr -0.72)

r_chain128_semi's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.0x the fastest

Fastest r_chain128_semi (293.13 us) to slowest r_chain128_whole (881.17 us): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: r_chain128_semi** at 293129.8 ns median (-66.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.01x (fastest 293129.8 ns, slowest 881170.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain128_semi | 292789ns | 296007ns | 277691ns | 289937ns | 304618ns | -66.73% |
| r_chain128_whole | 880154ns | 884331ns | 841236ns | 871346ns | 912825ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain128_semi | 290100ns | 275205ns | 301858ns | -66.92% | 0.001 |
| r_chain128_whole | 877055ns | 838435ns | 909446ns | base | 0.000 |

## Performance model

- Peak throughput: **0.001 Gops/s** (r_chain128_semi; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain128_semi | 0.001 | 93.9% |
| r_chain128_whole | 0.000 | 31.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain128_semi | 292789ns | 292789ns | -66.73% |
| r_chain128_whole | 880154ns | 880154ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain128_whole | 881170ns | base | --- | [840550, 909446] | --- | --- | --- | --- |
| r_chain128_semi | 293130ns | -588040.4ns (-66.7%) | [-607588, -565238]ns | [275312, 301858] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain128_whole | r_chain128_semi |
|---|---|---|
| 1 | 869862ns | -66.2% |
| 2 | 892478ns | -67.3% |
| 3 | 912827ns | -67.3% |
| 4 | 842665ns | -67.3% |
| 5 | 906065ns | -66.4% |
| 6 | 838435ns | -67.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain128_semi | -0.722 | HIGH- (thermal bounce) |
| r_chain128_whole | -0.572 | HIGH- (thermal bounce) |

**Consistency summary:**

- **r_chain128_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain128_semi | 488.3ns | 290099.9ns | 0.2% |  |
| r_chain128_whole | 466.6ns | 877055.3ns | 0.1% |  |

## Distribution (algo ns)

```
r_chain128_semi (n=6, range 275205.4-301857.9 ns)
  275205.4 |########################################
  276538.0 |
  277870.7 |
  279203.3 |
  280535.9 |
  281868.5 |
  283201.2 |
  284533.8 |
  285866.4 |
  287199.0 |
  288531.7 |
  289864.3 |
  291196.9 |####################
  292529.6 |
  293862.2 |####################
  295194.8 |
  296527.4 |
  297860.1 |####################
  299192.7 |
  300525.3 |
  (0 below, 1 above range)

r_chain128_whole (n=6, range 838435.0-909445.8 ns)
  838435.0 |########################################
  841985.5 |########################################
  845536.1 |
  849086.6 |
  852637.2 |
  856187.7 |
  859738.3 |
  863288.8 |
  866839.3 |########################################
  870389.9 |
  873940.4 |
  877491.0 |
  881041.5 |
  884592.1 |
  888142.6 |
  891693.1 |########################################
  895243.7 |
  898794.2 |
  902344.8 |
  905895.3 |########################################
  (0 below, 1 above range)

```
