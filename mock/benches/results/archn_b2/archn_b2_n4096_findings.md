# Per-branch strategy (NATIVE tier): archetype 2

5 variants, 6 samples per variant.
Baseline: **an_b2_table**

## Highlights

Baseline for all deltas below: **an_b2_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (an_b2_table) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline an_b2_table has the worst median (57.58 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest an_b2_pred at 43.35 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### an_b2_pred dominates: 25% faster than the next best (an_b2_prof)

an_b2_pred (43.35 us) leads an_b2_prof (54.18 us) by 25%, a clear separation rather than a photo finish. CV 7.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### an_b2_pred beats baseline by 22% (significant)

an_b2_pred is -12.68 us (22%) faster than baseline an_b2_table, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### an_b2_table shows alternating (throttle bounce) (autocorr -0.65)

an_b2_table's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: an_b2_pred** at 43346.8 ns median (-24.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.33x (fastest 43346.8 ns, slowest 57579.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b2_pred | 46054ns | 45681ns | 40426ns | 45444ns | 49782ns | -22.57% |
| an_b2_prof | 57939ns | 56468ns | 53983ns | 55743ns | 63211ns | -2.59% |
| an_b2_seq | 59340ns | 59349ns | 54047ns | 58581ns | 63125ns | -0.23% |
| an_b2_table | 59479ns | 60122ns | 50044ns | 58593ns | 65526ns | base |
| an_b2_tree | 56382ns | 57596ns | 46877ns | 57220ns | 59879ns | -5.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b2_pred | 43690ns | 38270ns | 47355ns | -23.39% | 0.094 |
| an_b2_prof | 55504ns | 51490ns | 60709ns | -2.67% | 0.074 |
| an_b2_seq | 56999ns | 51850ns | 60696ns | -0.05% | 0.072 |
| an_b2_table | 57027ns | 47820ns | 62919ns | base | 0.072 |
| an_b2_tree | 54043ns | 44675ns | 57443ns | -5.23% | 0.076 |

## Performance model

- Peak throughput: **0.107 Gops/s** (an_b2_pred; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b2_pred | 0.094 | 88.3% |
| an_b2_prof | 0.076 | 70.6% |
| an_b2_seq | 0.072 | 67.1% |
| an_b2_table | 0.071 | 66.5% |
| an_b2_tree | 0.074 | 69.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b2_pred | 46054ns | 46054ns | -22.57% |
| an_b2_prof | 57939ns | 57939ns | -2.59% |
| an_b2_seq | 59340ns | 59340ns | -0.23% |
| an_b2_table | 59479ns | 59479ns | base |
| an_b2_tree | 56382ns | 56382ns | -5.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b2_table | 57579ns | base | --- | [50582, 62919] | --- | --- | --- | --- |
| an_b2_pred | 43347ns | -12675.4ns (-22.0%) | [-17778, -9556]ns | [40369, 47355] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b2_prof | 54175ns | no significant difference | [-8401, +6987]ns | [51629, 60709] | no | 0.9167 | 0.6875 | 0 |
| an_b2_seq | 57005ns | no significant difference | [-5895, +6080]ns | [53295, 60696] | no | 1.0000 | 1.0000 | 0 |
| an_b2_tree | 55263ns | no significant difference | [-8118, +2934]ns | [49424, 57443] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b2_table | an_b2_pred | an_b2_prof | an_b2_seq | an_b2_tree |
|---|---|---|---|---|---|
| 1 | 53345ns | -17.9% | -2.7% | -2.8% | -16.3% |
| 2 | 57915ns | -18.6% | -10.6% | +1.7% | -1.2% |
| 3 | 57244ns | -25.8% | +9.4% | +9.2% | -5.4% |
| 4 | 62145ns | -23.5% | -17.1% | -5.2% | -7.2% |
| 5 | 47820ns | -20.0% | +18.0% | +14.5% | +13.8% |
| 6 | 63693ns | -32.6% | -7.7% | -13.4% | -11.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b2_pred | -0.433 | moderate- |
| an_b2_prof | -0.402 | moderate- |
| an_b2_seq | 0.152 | ok |
| an_b2_table | -0.651 | HIGH- (thermal bounce) |
| an_b2_tree | -0.231 | moderate- |

**Consistency summary:**

- **an_b2_pred**: won 6/6, lost 0/6
- **an_b2_prof**: won 4/6, lost 2/6
- **an_b2_seq**: won 3/6, lost 3/6
- **an_b2_tree**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b2_pred | 5.1ns | 43690.3ns | 0.0% |  |
| an_b2_prof | 5.0ns | 55504.4ns | 0.0% |  |
| an_b2_seq | 5.6ns | 56998.7ns | 0.0% |  |
| an_b2_table | 5.8ns | 57026.8ns | 0.0% |  |
| an_b2_tree | 5.5ns | 54043.2ns | 0.0% |  |

## Distribution (algo ns)

```
an_b2_pred (n=6, range 38269.6-47354.6 ns)
  38269.6 |########################################
  38723.8 |
  39178.1 |
  39632.3 |
  40086.6 |
  40540.8 |
  40995.1 |
  41449.3 |
  41903.6 |
  42357.8 |########################################
  42812.1 |########################################
  43266.3 |
  43720.6 |########################################
  44174.8 |
  44629.1 |
  45083.3 |
  45537.6 |
  45991.8 |
  46446.1 |
  46900.3 |########################################
  (0 below, 1 above range)

an_b2_prof (n=6, range 51490.0-60708.9 ns)
  51490.0 |########################################
  51950.9 |
  52411.9 |
  52872.8 |
  53333.8 |
  53794.7 |
  54255.7 |
  54716.6 |
  55177.6 |
  55638.5 |
  56099.5 |#############
  56560.4 |
  57021.4 |
  57482.3 |
  57943.3 |
  58404.2 |#############
  58865.2 |
  59326.1 |
  59787.1 |
  60248.0 |
  (0 below, 1 above range)

an_b2_seq (n=6, range 51849.6-60696.2 ns)
  51849.6 |####################
  52291.9 |
  52734.3 |
  53176.6 |
  53618.9 |
  54061.2 |
  54503.6 |####################
  54945.9 |####################
  55388.2 |
  55830.6 |
  56272.9 |
  56715.2 |
  57157.6 |
  57599.9 |
  58042.2 |
  58484.5 |########################################
  58926.9 |
  59369.2 |
  59811.5 |
  60253.9 |
  (0 below, 1 above range)

an_b2_table (n=6, range 47819.6-62918.9 ns)
  47819.6 |########################################
  48574.6 |
  49329.5 |
  50084.5 |
  50839.5 |
  51594.4 |
  52349.4 |
  53104.4 |########################################
  53859.3 |
  54614.3 |
  55369.3 |
  56124.2 |
  56879.2 |########################################
  57634.2 |########################################
  58389.1 |
  59144.1 |
  59899.1 |
  60654.0 |
  61409.0 |########################################
  62164.0 |
  (0 below, 1 above range)

an_b2_tree (n=6, range 44675.0-57442.7 ns)
  44675.0 |########################################
  45313.4 |
  45951.8 |
  46590.2 |
  47228.5 |
  47866.9 |
  48505.3 |
  49143.7 |
  49782.1 |
  50420.5 |
  51058.8 |
  51697.2 |
  52335.6 |
  52974.0 |
  53612.4 |########################################
  54250.8 |########################################
  54889.2 |
  55527.5 |########################################
  56165.9 |
  56804.3 |########################################
  (0 below, 1 above range)

```
