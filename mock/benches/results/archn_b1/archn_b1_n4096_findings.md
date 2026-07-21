# Per-branch strategy (NATIVE tier): archetype 1

5 variants, 6 samples per variant.
Baseline: **an_b1_table**

## Highlights

Baseline for all deltas below: **an_b1_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b1_tree is fastest but the noisiest (CV 15.4%)

an_b1_tree wins on median (49.05 us) yet has the highest variance (CV 15.4%), while an_b1_table is the steadiest (CV 6.7%, 49.81 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {an_b1_tree, an_b1_table, an_b1_seq, an_b1_prof} vs {an_b1_pred} (29% apart)

The field splits into a fast tier {an_b1_tree, an_b1_table, an_b1_seq, an_b1_prof} and a slow tier {an_b1_pred} with a 29% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader an_b1_tree vs stability leader an_b1_table (+2% speed for 2.3x steadier)

an_b1_tree is fastest (49.05 us, CV 15.4%); an_b1_table gives up 1.5% median for 2.3x lower variance (CV 6.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: an_b1_tree** at 49049.4 ns median (-1.5% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.42x (fastest 49049.4 ns, slowest 69617.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b1_pred | 73514ns | 72069ns | 65548ns | 70985ns | 81290ns | +39.04% |
| an_b1_prof | 55214ns | 56614ns | 48261ns | 53879ns | 60694ns | +4.43% |
| an_b1_seq | 54196ns | 53704ns | 49014ns | 52889ns | 58746ns | +2.50% |
| an_b1_table | 52874ns | 52106ns | 48193ns | 51825ns | 56787ns | base |
| an_b1_tree | 53062ns | 51561ns | 44384ns | 49895ns | 62151ns | +0.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b1_pred | 71071ns | 63302ns | 78672ns | +40.63% | 0.058 |
| an_b1_prof | 52817ns | 46073ns | 58236ns | +4.51% | 0.078 |
| an_b1_seq | 51880ns | 46835ns | 56449ns | +2.65% | 0.079 |
| an_b1_table | 50539ns | 45954ns | 54339ns | base | 0.081 |
| an_b1_tree | 50515ns | 41773ns | 59635ns | -0.05% | 0.081 |

## Performance model

- Peak throughput: **0.098 Gops/s** (an_b1_tree; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b1_pred | 0.059 | 60.0% |
| an_b1_prof | 0.076 | 77.2% |
| an_b1_seq | 0.080 | 81.5% |
| an_b1_table | 0.082 | 83.9% |
| an_b1_tree | 0.084 | 85.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b1_pred | 73514ns | 73514ns | +39.04% |
| an_b1_prof | 55214ns | 55214ns | +4.43% |
| an_b1_seq | 54196ns | 54196ns | +2.50% |
| an_b1_table | 52874ns | 52874ns | base |
| an_b1_tree | 53062ns | 53062ns | +0.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b1_table | 49809ns | base | --- | [47469, 54339] | --- | --- | --- | --- |
| an_b1_pred | 69618ns | +19247.5ns (+38.6%) | [+13488, +28863]ns | [64925, 78672] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b1_prof | 54083ns | no significant difference | [-1833, +7097]ns | [46131, 58236] | no | 1.0000 | 0.6875 | 0 |
| an_b1_seq | 51255ns | no significant difference | [-603, +4222]ns | [47935, 56449] | no | 1.0000 | 1.0000 | 0 |
| an_b1_tree | 49049ns | no significant difference | [-7907, +9448]ns | [42860, 59635] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b1_table | an_b1_pred | an_b1_prof | an_b1_seq | an_b1_tree |
|---|---|---|---|---|---|
| 1 | 49777ns | +56.2% | -7.2% | -1.5% | -16.1% |
| 2 | 45954ns | +37.8% | +0.3% | +1.9% | +17.3% |
| 3 | 51757ns | +35.6% | +6.9% | -0.1% | -15.1% |
| 4 | 56920ns | +16.9% | -0.1% | -0.8% | +2.8% |
| 5 | 49841ns | +59.7% | +6.1% | +2.0% | +21.9% |
| 6 | 48983ns | +41.0% | +21.7% | +15.2% | -9.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b1_pred | -0.476 | moderate- |
| an_b1_prof | 0.243 | moderate+ |
| an_b1_seq | 0.063 | ok |
| an_b1_table | 0.035 | ok |
| an_b1_tree | -0.258 | moderate- |

**Consistency summary:**

- **an_b1_pred**: won 0/6, lost 6/6
- **an_b1_prof**: won 2/6, lost 4/6
- **an_b1_seq**: won 3/6, lost 3/6
- **an_b1_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b1_pred | 5.2ns | 71071.2ns | 0.0% |  |
| an_b1_prof | 5.2ns | 52816.6ns | 0.0% |  |
| an_b1_seq | 6.2ns | 51879.6ns | 0.0% |  |
| an_b1_table | 4.7ns | 50538.6ns | 0.0% |  |
| an_b1_tree | 5.0ns | 50514.8ns | 0.0% |  |

## Distribution (algo ns)

```
an_b1_pred (n=6, range 63302.1-78671.6 ns)
  63302.1 |########################################
  64070.6 |
  64839.1 |
  65607.5 |
  66376.0 |########################################
  67144.5 |
  67913.0 |
  68681.4 |########################################
  69449.9 |########################################
  70218.4 |
  70986.9 |
  71755.4 |
  72523.8 |
  73292.3 |
  74060.8 |
  74829.3 |
  75597.7 |
  76366.2 |
  77134.7 |########################################
  77903.2 |
  (0 below, 1 above range)

an_b1_prof (n=6, range 46072.9-58236.1 ns)
  46072.9 |########################################
  46681.1 |
  47289.2 |
  47897.4 |
  48505.5 |
  49113.7 |
  49721.8 |
  50330.0 |
  50938.2 |
  51546.3 |
  52154.5 |
  52762.6 |####################
  53370.8 |
  53978.9 |
  54587.1 |
  55195.3 |####################
  55803.4 |
  56411.6 |####################
  57019.7 |
  57627.9 |
  (0 below, 1 above range)

an_b1_seq (n=6, range 46835.0-56449.2 ns)
  46835.0 |########################################
  47315.7 |
  47796.4 |
  48277.1 |
  48757.8 |########################################
  49238.5 |
  49719.2 |
  50200.0 |
  50680.7 |########################################
  51161.4 |
  51642.1 |########################################
  52122.8 |
  52603.5 |
  53084.2 |
  53564.9 |
  54045.6 |
  54526.3 |
  55007.0 |
  55487.7 |
  55968.4 |########################################
  (0 below, 1 above range)

an_b1_table (n=6, range 45954.2-54338.6 ns)
  45954.2 |####################
  46373.4 |
  46792.6 |
  47211.9 |
  47631.1 |
  48050.3 |
  48469.5 |
  48888.7 |####################
  49307.9 |
  49727.2 |########################################
  50146.4 |
  50565.6 |
  50984.8 |
  51404.0 |####################
  51823.2 |
  52242.5 |
  52661.7 |
  53080.9 |
  53500.1 |
  53919.3 |
  (0 below, 1 above range)

an_b1_tree (n=6, range 41773.3-59635.4 ns)
  41773.3 |####################
  42666.4 |
  43559.5 |########################################
  44452.6 |
  45345.7 |
  46238.8 |
  47131.9 |
  48025.0 |
  48918.1 |
  49811.2 |
  50704.4 |
  51597.5 |
  52490.6 |
  53383.7 |####################
  54276.8 |
  55169.9 |
  56063.0 |
  56956.1 |
  57849.2 |####################
  58742.3 |
  (0 below, 1 above range)

```
