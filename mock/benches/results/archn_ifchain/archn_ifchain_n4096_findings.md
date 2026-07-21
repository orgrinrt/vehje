# Per-type strategy (NATIVE tier): all ifchain

5 variants, 6 samples per variant.
Baseline: **an_ifchain_table**

## Highlights

Baseline for all deltas below: **an_ifchain_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_ifchain_prof is fastest but the noisiest (CV 15.5%)

an_ifchain_prof wins on median (45.86 us) yet has the highest variance (CV 15.5%), while an_ifchain_tree is the steadiest (CV 7.9%, 56.55 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### an_ifchain_seq shows alternating (throttle bounce) (autocorr -0.84)

an_ifchain_seq's per-pass series has lag-1 autocorrelation -0.84, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {an_ifchain_prof, an_ifchain_seq, an_ifchain_table, an_ifchain_tree} vs {an_ifchain_pred} (52% apart)

The field splits into a fast tier {an_ifchain_prof, an_ifchain_seq, an_ifchain_table, an_ifchain_tree} and a slow tier {an_ifchain_pred} with a 52% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### an_ifchain_prof is inconsistent: worst-20% is 1.5x its best-20%

an_ifchain_prof's best 20% of batches run at 36.60 us but its worst 20% at 55.82 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: an_ifchain_prof** at 45859.3 ns median (-14.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.88x (fastest 45859.3 ns, slowest 86174.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_ifchain_pred | 89127ns | 88600ns | 76098ns | 88031ns | 97287ns | +61.03% |
| an_ifchain_prof | 49312ns | 48045ns | 38764ns | 46829ns | 58310ns | -10.91% |
| an_ifchain_seq | 50331ns | 49872ns | 43894ns | 48826ns | 55805ns | -9.07% |
| an_ifchain_table | 55348ns | 55719ns | 44979ns | 55388ns | 60473ns | base |
| an_ifchain_tree | 58149ns | 58873ns | 48582ns | 58608ns | 62245ns | +5.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_ifchain_pred | 86730ns | 73857ns | 94776ns | +63.64% | 0.047 |
| an_ifchain_prof | 46990ns | 36598ns | 55820ns | -11.34% | 0.087 |
| an_ifchain_seq | 47897ns | 41456ns | 53275ns | -9.63% | 0.086 |
| an_ifchain_table | 53002ns | 42794ns | 58099ns | base | 0.077 |
| an_ifchain_tree | 55736ns | 46428ns | 59685ns | +5.16% | 0.073 |

## Performance model

- Peak throughput: **0.112 Gops/s** (an_ifchain_prof; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_ifchain_pred | 0.048 | 42.5% |
| an_ifchain_prof | 0.089 | 79.8% |
| an_ifchain_seq | 0.086 | 76.8% |
| an_ifchain_table | 0.077 | 68.5% |
| an_ifchain_tree | 0.072 | 64.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_ifchain_pred | 89127ns | 89127ns | +61.03% |
| an_ifchain_prof | 49312ns | 49312ns | -10.91% |
| an_ifchain_seq | 50331ns | 50331ns | -9.07% |
| an_ifchain_table | 55348ns | 55348ns | base |
| an_ifchain_tree | 58149ns | 58149ns | +5.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_ifchain_table | 53463ns | base | --- | [47443, 58099] | --- | --- | --- | --- |
| an_ifchain_pred | 86175ns | +31878.5ns (+59.6%) | [+31320, +37987]ns | [79240, 94776] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_ifchain_prof | 45859ns | no significant difference | [-17745, +4416]ns | [39290, 55820] | no | 0.6875 | 0.6875 | 0 |
| an_ifchain_seq | 47660ns | no significant difference | [-10647, +1671]ns | [42755, 53275] | no | 0.2917 | 0.2188 | 0 |
| an_ifchain_tree | 56548ns | no significant difference | [-510, +6142]ns | [50975, 59685] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_ifchain_table | an_ifchain_pred | an_ifchain_prof | an_ifchain_seq | an_ifchain_tree |
|---|---|---|---|---|---|
| 1 | 54343ns | +58.4% | -15.1% | -10.6% | +2.8% |
| 2 | 42794ns | +72.6% | +6.5% | +9.2% | +8.5% |
| 3 | 52584ns | +60.9% | -30.4% | -1.2% | +8.9% |
| 4 | 52092ns | +77.7% | +11.6% | -20.4% | +14.6% |
| 5 | 61486ns | +57.7% | -31.7% | -11.2% | -3.0% |
| 6 | 54712ns | +57.7% | -2.2% | -19.5% | +1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_ifchain_pred | 0.254 | moderate+ |
| an_ifchain_prof | -0.623 | HIGH- (thermal bounce) |
| an_ifchain_seq | -0.842 | HIGH- (thermal bounce) |
| an_ifchain_table | -0.012 | ok |
| an_ifchain_tree | 0.047 | ok |

**Consistency summary:**

- **an_ifchain_pred**: won 0/6, lost 6/6
- **an_ifchain_prof**: won 4/6, lost 2/6
- **an_ifchain_seq**: won 5/6, lost 1/6
- **an_ifchain_tree**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_ifchain_pred | 6.1ns | 86730.3ns | 0.0% |  |
| an_ifchain_prof | 4.8ns | 46989.8ns | 0.0% |  |
| an_ifchain_seq | 5.5ns | 47896.9ns | 0.0% |  |
| an_ifchain_table | 5.7ns | 53001.7ns | 0.0% |  |
| an_ifchain_tree | 6.6ns | 55735.8ns | 0.0% |  |

## Distribution (algo ns)

```
an_ifchain_pred (n=6, range 73857.1-94776.1 ns)
  73857.1 |####################
  74903.0 |
  75949.0 |
  76994.9 |
  78040.9 |
  79086.8 |
  80132.8 |
  81178.7 |
  82224.7 |
  83270.6 |
  84316.6 |####################
  85362.5 |########################################
  86408.5 |
  87454.4 |
  88500.4 |
  89546.3 |
  90592.3 |
  91638.2 |####################
  92684.2 |
  93730.1 |
  (0 below, 1 above range)

an_ifchain_prof (n=6, range 36597.9-55819.6 ns)
  36597.9 |####################
  37559.0 |
  38520.1 |
  39481.2 |
  40442.2 |
  41403.3 |####################
  42364.4 |
  43325.5 |
  44286.6 |
  45247.7 |########################################
  46208.8 |
  47169.8 |
  48130.9 |
  49092.0 |
  50053.1 |
  51014.2 |
  51975.3 |
  52936.3 |####################
  53897.4 |
  54858.5 |
  (0 below, 1 above range)

an_ifchain_seq (n=6, range 41456.2-53275.2 ns)
  41456.2 |########################################
  42047.1 |
  42638.1 |
  43229.0 |
  43820.0 |########################################
  44410.9 |
  45001.9 |
  45592.8 |
  46183.8 |########################################
  46774.8 |
  47365.7 |
  47956.6 |
  48547.6 |########################################
  49138.5 |
  49729.5 |
  50320.4 |
  50911.4 |
  51502.3 |########################################
  52093.3 |
  52684.2 |
  (0 below, 1 above range)

an_ifchain_table (n=6, range 42793.7-58098.9 ns)
  42793.7 |####################
  43559.0 |
  44324.2 |
  45089.5 |
  45854.8 |
  46620.0 |
  47385.3 |
  48150.5 |
  48915.8 |
  49681.1 |
  50446.3 |
  51211.6 |
  51976.8 |########################################
  52742.1 |
  53507.4 |
  54272.6 |########################################
  55037.9 |
  55803.2 |
  56568.4 |
  57333.7 |
  (0 below, 1 above range)

an_ifchain_tree (n=6, range 46427.9-59685.0 ns)
  46427.9 |########################################
  47090.8 |
  47753.6 |
  48416.5 |
  49079.3 |
  49742.2 |
  50405.0 |
  51067.9 |
  51730.7 |
  52393.6 |
  53056.4 |
  53719.3 |
  54382.2 |
  55045.0 |########################################
  55707.9 |########################################
  56370.7 |
  57033.6 |########################################
  57696.4 |
  58359.3 |
  59022.1 |########################################
  (0 below, 1 above range)

```
