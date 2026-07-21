# Per-branch strategy: archetype 2 (match8_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b2_table**

## Highlights

Baseline for all deltas below: **ab_b2_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_b2_prof is fastest but the noisiest (CV 8.1%)

ab_b2_prof wins on median (21.53 us) yet has the highest variance (CV 8.1%), while ab_b2_tree is the steadiest (CV 6.1%, 23.99 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ab_b2_pred shows warm-up / thermal drift (autocorr +0.53)

ab_b2_pred's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {ab_b2_prof, ab_b2_table, ab_b2_seq, ab_b2_tree} vs {ab_b2_pred} (34% apart)

The field splits into a fast tier {ab_b2_prof, ab_b2_table, ab_b2_seq, ab_b2_tree} and a slow tier {ab_b2_pred} with a 34% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### ab_b2_prof's edge over baseline is significant but tiny (36 ns, 0.16%)

ab_b2_prof differs from baseline ab_b2_table by 36 ns (0.16%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ab_b2_prof** at 21534.0 ns median (-1.1% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.50x (fastest 21534.0 ns, slowest 32264.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b2_pred | 34475ns | 34754ns | 31202ns | 33782ns | 37152ns | +42.58% |
| ab_b2_prof | 24286ns | 23839ns | 22105ns | 23430ns | 26659ns | +0.44% |
| ab_b2_seq | 24853ns | 24699ns | 22601ns | 24368ns | 26706ns | +2.79% |
| ab_b2_table | 24179ns | 24071ns | 21938ns | 23677ns | 26053ns | base |
| ab_b2_tree | 25715ns | 26544ns | 23175ns | 25603ns | 27153ns | +6.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b2_pred | 32027ns | 28928ns | 34551ns | +46.57% | 0.002 |
| ab_b2_prof | 21942ns | 19955ns | 24092ns | +0.41% | 0.003 |
| ab_b2_seq | 22448ns | 20431ns | 24088ns | +2.73% | 0.003 |
| ab_b2_table | 21851ns | 19821ns | 23529ns | base | 0.003 |
| ab_b2_tree | 23275ns | 21005ns | 24582ns | +6.51% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b2_table; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b2_pred | 0.002 | 61.4% |
| ab_b2_prof | 0.003 | 92.0% |
| ab_b2_seq | 0.003 | 88.7% |
| ab_b2_table | 0.003 | 91.1% |
| ab_b2_tree | 0.003 | 82.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b2_pred | 34475ns | 34475ns | +42.58% |
| ab_b2_prof | 24286ns | 24286ns | +0.44% |
| ab_b2_seq | 24853ns | 24853ns | +2.79% |
| ab_b2_table | 24179ns | 24179ns | base |
| ab_b2_tree | 25715ns | 25715ns | +6.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b2_table | 21765ns | base | --- | [20260, 23529] | --- | --- | --- | --- |
| ab_b2_pred | 32265ns | +9499.2ns (+43.6%) | [+8413, +12616]ns | [29267, 34551] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b2_prof | 21534ns | no significant difference | [-411, +646]ns | [20199, 24092] | no | 1.0000 | 1.0000 | 0 |
| ab_b2_seq | 22342ns | no significant difference | [-180, +1392]ns | [20912, 24088] | no | 0.9167 | 0.6875 | 0 |
| ab_b2_tree | 23994ns | +1020.6ns (+4.7%) | [+167, +3083]ns | [21248, 24582] | YES (adj: no) | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b2_table | ab_b2_pred | ab_b2_prof | ab_b2_seq | ab_b2_tree |
|---|---|---|---|---|---|
| 1 | 19821ns | +76.1% | +0.7% | +7.9% | +22.2% |
| 2 | 24048ns | +42.2% | -0.1% | -0.4% | +1.4% |
| 3 | 22521ns | +44.8% | +0.5% | +1.8% | +5.5% |
| 4 | 23010ns | +38.7% | +5.0% | +5.3% | +7.7% |
| 5 | 20699ns | +43.0% | -1.2% | -1.3% | +3.8% |
| 6 | 21008ns | +37.7% | -2.7% | +3.6% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b2_pred | 0.528 | HIGH+ (drift/warm-up) |
| ab_b2_prof | -0.126 | ok |
| ab_b2_seq | -0.197 | ok |
| ab_b2_table | -0.202 | moderate- |
| ab_b2_tree | 0.286 | moderate+ |

**Consistency summary:**

- **ab_b2_pred**: won 0/6, lost 6/6
- **ab_b2_prof**: won 3/6, lost 3/6
- **ab_b2_seq**: won 2/6, lost 4/6
- **ab_b2_tree**: won 0/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b2_pred | 3.5ns | 32027.3ns | 0.0% |  |
| ab_b2_prof | 4.0ns | 21941.7ns | 0.0% |  |
| ab_b2_seq | 4.1ns | 22447.5ns | 0.0% |  |
| ab_b2_table | 3.7ns | 21851.2ns | 0.0% |  |
| ab_b2_tree | 3.5ns | 23274.7ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b2_pred (n=6, range 28927.5-34550.6 ns)
  28927.5 |########################################
  29208.7 |
  29489.8 |########################################
  29771.0 |
  30052.1 |
  30333.3 |
  30614.4 |
  30895.6 |
  31176.8 |
  31457.9 |
  31739.1 |########################################
  32020.2 |
  32301.4 |
  32582.5 |########################################
  32863.7 |
  33144.9 |
  33426.0 |
  33707.2 |
  33988.3 |########################################
  34269.5 |
  (0 below, 1 above range)

ab_b2_prof (n=6, range 19955.0-24092.5 ns)
  19955.0 |####################
  20161.9 |
  20368.8 |########################################
  20575.6 |
  20782.5 |
  20989.4 |
  21196.2 |
  21403.1 |
  21610.0 |
  21816.9 |
  22023.8 |
  22230.6 |
  22437.5 |####################
  22644.4 |
  22851.2 |
  23058.1 |
  23265.0 |
  23471.9 |
  23678.8 |
  23885.6 |####################
  (0 below, 1 above range)

ab_b2_seq (n=6, range 20431.2-24087.9 ns)
  20431.2 |########################################
  20614.0 |
  20796.9 |
  20979.7 |
  21162.5 |
  21345.4 |########################################
  21528.2 |
  21711.0 |########################################
  21893.9 |
  22076.7 |
  22259.6 |
  22442.4 |
  22625.2 |
  22808.1 |########################################
  22990.9 |
  23173.7 |
  23356.6 |
  23539.4 |
  23722.2 |
  23905.1 |########################################
  (0 below, 1 above range)

ab_b2_table (n=6, range 19820.8-23529.0 ns)
  19820.8 |########################################
  20006.2 |
  20191.6 |
  20377.0 |
  20562.4 |########################################
  20747.8 |
  20933.2 |########################################
  21118.7 |
  21304.1 |
  21489.5 |
  21674.9 |
  21860.3 |
  22045.7 |
  22231.1 |
  22416.5 |########################################
  22601.9 |
  22787.3 |
  22972.7 |########################################
  23158.1 |
  23343.5 |
  (0 below, 1 above range)

ab_b2_tree (n=6, range 21005.4-24582.3 ns)
  21005.4 |########################################
  21184.2 |
  21363.1 |########################################
  21541.9 |
  21720.8 |
  21899.6 |
  22078.5 |
  22257.3 |
  22436.2 |
  22615.0 |
  22793.9 |
  22972.7 |
  23151.5 |
  23330.4 |
  23509.2 |
  23688.1 |########################################
  23866.9 |
  24045.8 |########################################
  24224.6 |########################################
  24403.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **ab_b2_pred**: autocorrelation=0.53 (measurement drift or warm-up artifact)
