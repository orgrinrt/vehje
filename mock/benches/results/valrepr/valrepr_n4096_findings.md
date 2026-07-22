# Value representation: static/raw vs runtime-tagged vs NaN-boxed (dynamic-typing cost)

3 variants, 6 samples per variant.
Baseline: **valrepr_static**

## Highlights

Baseline for all deltas below: **valrepr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (valrepr_static) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline valrepr_static has the worst median (162.55 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest valrepr_nanbox at 138.75 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### valrepr_nanbox shows alternating (throttle bounce) (autocorr -0.55)

valrepr_nanbox's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### valrepr_tagged is inconsistent: worst-20% is 1.5x its best-20%

valrepr_tagged's best 20% of batches run at 135.39 us but its worst 20% at 203.64 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: valrepr_nanbox** at 138751.9 ns median (-14.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.17x (fastest 138751.9 ns, slowest 162551.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| valrepr_nanbox | 141184ns | 141244ns | 134014ns | 139651ns | 147069ns | -15.69% |
| valrepr_static | 167463ns | 164999ns | 143409ns | 161374ns | 188622ns | base |
| valrepr_tagged | 165030ns | 150317ns | 137932ns | 146616ns | 206199ns | -1.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| valrepr_nanbox | 138809ns | 131805ns | 144718ns | -15.89% | 0.030 |
| valrepr_static | 165035ns | 141011ns | 186268ns | base | 0.025 |
| valrepr_tagged | 162578ns | 135388ns | 203640ns | -1.49% | 0.025 |

## Performance model

- Peak throughput: **0.031 Gops/s** (valrepr_nanbox; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| valrepr_nanbox | 0.030 | 95.0% |
| valrepr_static | 0.025 | 81.1% |
| valrepr_tagged | 0.028 | 89.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| valrepr_nanbox | 141184ns | 141184ns | -15.69% |
| valrepr_static | 167463ns | 167463ns | base |
| valrepr_tagged | 165030ns | 165030ns | -1.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| valrepr_static | 162552ns | base | --- | [146286, 186268] | --- | --- | --- | --- |
| valrepr_nanbox | 138752ns | -22762.4ns (-14.0%) | [-49848, -6066]ns | [132958, 144718] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| valrepr_tagged | 147891ns | no significant difference | [-50064, +46844]ns | [136203, 203640] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | valrepr_static | valrepr_nanbox | valrepr_tagged |
|---|---|---|---|
| 1 | 169360ns | -22.2% | -19.1% |
| 2 | 151561ns | -3.5% | +28.0% |
| 3 | 162033ns | -15.8% | +31.7% |
| 4 | 203175ns | -30.6% | -33.4% |
| 5 | 163071ns | -12.2% | -10.3% |
| 6 | 141011ns | -4.9% | +6.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| valrepr_nanbox | -0.550 | HIGH- (thermal bounce) |
| valrepr_static | -0.071 | ok |
| valrepr_tagged | 0.012 | ok |

**Consistency summary:**

- **valrepr_nanbox**: won 6/6, lost 0/6
- **valrepr_tagged**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| valrepr_nanbox | 440.0ns | 138809.5ns | 0.3% |  |
| valrepr_static | 441.3ns | 165035.1ns | 0.3% |  |
| valrepr_tagged | 513.4ns | 162578.2ns | 0.3% |  |

## Distribution (algo ns)

```
valrepr_nanbox (n=6, range 131805.0-144718.1 ns)
  131805.0 |########################################
  132450.7 |
  133096.3 |
  133742.0 |########################################
  134387.6 |
  135033.3 |
  135678.9 |
  136324.6 |########################################
  136970.3 |
  137615.9 |
  138261.6 |
  138907.2 |
  139552.9 |
  140198.5 |
  140844.2 |########################################
  141489.9 |
  142135.5 |
  142781.2 |########################################
  143426.8 |
  144072.5 |
  (0 below, 1 above range)

valrepr_static (n=6, range 141011.2-186267.5 ns)
  141011.2 |####################
  143274.0 |
  145536.8 |
  147799.6 |
  150062.5 |####################
  152325.3 |
  154588.1 |
  156850.9 |
  159113.7 |
  161376.5 |########################################
  163639.4 |
  165902.2 |
  168165.0 |####################
  170427.8 |
  172690.6 |
  174953.4 |
  177216.2 |
  179479.1 |
  181741.9 |
  184004.7 |
  (0 below, 1 above range)

valrepr_tagged (n=6, range 135388.3-203640.5 ns)
  135388.3 |########################################
  138800.9 |
  142213.5 |
  145626.1 |####################
  149038.7 |####################
  152451.3 |
  155863.9 |
  159276.6 |
  162689.2 |
  166101.8 |
  169514.4 |
  172927.0 |
  176339.6 |
  179752.2 |
  183164.8 |
  186577.4 |
  189990.0 |
  193402.6 |####################
  196815.2 |
  200227.8 |
  (0 below, 1 above range)

```
