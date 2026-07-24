# abi_zig_entry (tight)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_tight_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_tight_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_tight_zig_null dominates: 57841% faster than the next best (abi_zig_entry_tight_zig_per_w_set)

abi_zig_entry_tight_zig_null (3.43 us) leads abi_zig_entry_tight_zig_per_w_set (1.99 ms) by 57841%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_tight_zig_null beats baseline by 100% (significant)

abi_zig_entry_tight_zig_null is -2.00 ms (100%) faster than baseline abi_zig_entry_tight_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_tight_zig_tail_dispatch is an outlier: 903.6x slower than the field

abi_zig_entry_tight_zig_tail_dispatch (3.10 ms) is 903.6x the fastest (3.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_tight_zig_null} vs {abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_tail_runtime_w, abi_zig_entry_tight_zig_tail_dispatch} (57841% apart)

The field splits into a fast tier {abi_zig_entry_tight_zig_null} and a slow tier {abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_tail_runtime_w, abi_zig_entry_tight_zig_tail_dispatch} with a 57841% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 903.6x the fastest

Fastest abi_zig_entry_tight_zig_null (3.43 us) to slowest abi_zig_entry_tight_zig_tail_dispatch (3.10 ms): 903.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_tight_zig_null** at 3426.6 ns median (-99.8% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 903.63x (fastest 3426.6 ns, slowest 3096422.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2022401ns | 1997031ns | 1985251ns | 1996318ns | 2080102ns | +0.73% |
| abi_zig_entry_tight_zig_dispatch | 1993374ns | 1993174ns | 1989768ns | 1992798ns | 1996041ns | -0.71% |
| abi_zig_entry_tight_zig_null | 5786ns | 5805ns | 5638ns | 5756ns | 5903ns | -99.71% |
| abi_zig_entry_tight_zig_per_w_set | 1990664ns | 1988255ns | 1986568ns | 1988073ns | 1996598ns | -0.85% |
| abi_zig_entry_tight_zig_runtime_w | 2007651ns | 2003831ns | 2000125ns | 2003170ns | 2018136ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3100341ns | 3099188ns | 3097309ns | 3098774ns | 3104208ns | +54.43% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3077817ns | 3090509ns | 3045368ns | 3076819ns | 3095538ns | +53.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2019590ns | 1982637ns | 2076913ns | +0.73% | 0.000 |
| abi_zig_entry_tight_zig_dispatch | 1990693ns | 1987064ns | 1993294ns | -0.71% | 0.000 |
| abi_zig_entry_tight_zig_null | 3427ns | 3370ns | 3480ns | -99.83% | 0.001 |
| abi_zig_entry_tight_zig_per_w_set | 1988020ns | 1984100ns | 1994014ns | -0.84% | 0.000 |
| abi_zig_entry_tight_zig_runtime_w | 2004961ns | 1997574ns | 2015240ns | base | 0.000 |
| abi_zig_entry_tight_zig_tail_dispatch | 3097591ns | 3094689ns | 3101341ns | +54.50% | 0.000 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3075091ns | 3042779ns | 3092652ns | +53.37% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 192907.7 | 2041965.6 | 2019590.0 | n/a |
| abi_zig_entry_tight_zig_dispatch | 179278.1 | 1988238.3 | 1990693.3 | n/a |
| abi_zig_entry_tight_zig_null | 154064.6 | 3579.5 | 3427.4 | n/a |
| abi_zig_entry_tight_zig_per_w_set | 180020.0 | 1990368.8 | 1988019.8 | n/a |
| abi_zig_entry_tight_zig_runtime_w | 181761.0 | 2003330.9 | 2004961.2 | n/a |
| abi_zig_entry_tight_zig_tail_dispatch | 185400.6 | 3089685.5 | 3097590.7 | n/a |
| abi_zig_entry_tight_zig_tail_runtime_w | 183156.4 | 3073589.3 | 3075090.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_zig_entry_tight_zig_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_null | 0.001 | 98.4% |
| abi_zig_entry_tight_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2022401ns | 2022401ns | +0.73% |
| abi_zig_entry_tight_zig_dispatch | 1993374ns | 1993374ns | -0.71% |
| abi_zig_entry_tight_zig_null | 5786ns | 5786ns | -99.71% |
| abi_zig_entry_tight_zig_per_w_set | 1990664ns | 1990664ns | -0.85% |
| abi_zig_entry_tight_zig_runtime_w | 2007651ns | 2007651ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3100341ns | 3100341ns | +54.43% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3077817ns | 3077817ns | +53.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_runtime_w | 2001263ns | base | --- | [1998381, 2015240] | --- | --- | --- | --- |
| abi_zig_entry_tight_zig_anchor | 1994437ns | no significant difference | [-11014, +61674]ns | [1987420, 2076913] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_tight_zig_dispatch | 1990510ns | -10839.4ns (-0.5%) | [-25250, -6714]ns | [1988276, 1993294] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_null | 3427ns | -1997869.8ns (-99.8%) | [-2011781, -1994950]ns | [3375, 3480] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_per_w_set | 1985450ns | -14558.1ns (-0.7%) | [-25601, -10665]ns | [1984595, 1994014] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_tail_dispatch | 3096423ns | +1096394.6ns (+54.8%) | [+1084037, +1097457]ns | [3095008, 3101341] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3087892ns | +1074635.6ns (+53.7%) | [+1045091, +1090661]ns | [3044727, 3092652] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_tight_zig_runtime_w | abi_zig_entry_tight_zig_anchor | abi_zig_entry_tight_zig_dispatch | abi_zig_entry_tight_zig_null | abi_zig_entry_tight_zig_per_w_set | abi_zig_entry_tight_zig_tail_dispatch | abi_zig_entry_tight_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2001973ns | -0.3% | -0.6% | -99.8% | -0.4% | +54.8% | +54.2% |
| 2 | 1999187ns | -0.3% | -0.2% | -99.8% | -0.7% | +54.9% | +54.7% |
| 3 | 2028506ns | +6.4% | -1.9% | -99.8% | -1.7% | +53.0% | +52.5% |
| 4 | 2001698ns | -0.4% | -0.6% | -99.8% | -0.9% | +54.6% | +52.2% |
| 5 | 1997574ns | -0.7% | -0.5% | -99.8% | -0.6% | +55.0% | +52.3% |
| 6 | 2000829ns | -0.3% | -0.4% | -99.8% | -0.8% | +54.8% | +54.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | -0.205 | moderate- |
| abi_zig_entry_tight_zig_dispatch | -0.194 | ok |
| abi_zig_entry_tight_zig_null | 0.474 | moderate+ |
| abi_zig_entry_tight_zig_per_w_set | -0.322 | moderate- |
| abi_zig_entry_tight_zig_runtime_w | -0.208 | moderate- |
| abi_zig_entry_tight_zig_tail_dispatch | -0.324 | moderate- |
| abi_zig_entry_tight_zig_tail_runtime_w | 0.150 | ok |

**Consistency summary:**

- **abi_zig_entry_tight_zig_anchor**: won 5/6, lost 1/6
- **abi_zig_entry_tight_zig_dispatch**: won 6/6, lost 0/6
- **abi_zig_entry_tight_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_tight_zig_per_w_set**: won 6/6, lost 0/6
- **abi_zig_entry_tight_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_tight_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 6333300.8ns | 2019590.0ns | 313.6% | HIGH |
| abi_zig_entry_tight_zig_dispatch | 6217164.8ns | 1990693.3ns | 312.3% | HIGH |
| abi_zig_entry_tight_zig_null | 303861.8ns | 3427.4ns | 8865.6% | HIGH |
| abi_zig_entry_tight_zig_per_w_set | 6217142.6ns | 1988019.8ns | 312.7% | HIGH |
| abi_zig_entry_tight_zig_runtime_w | 6263541.8ns | 2004961.2ns | 312.4% | HIGH |
| abi_zig_entry_tight_zig_tail_dispatch | 9525013.8ns | 3097590.7ns | 307.5% | HIGH |
| abi_zig_entry_tight_zig_tail_runtime_w | 9474682.5ns | 3075090.6ns | 308.1% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_tight_zig_anchor (n=6, range 1982637.1-2076913.4 ns)
  1982637.1 |##########
  1987350.9 |
  1992064.7 |########################################
  1996778.5 |
  2001492.4 |
  2006206.2 |
  2010920.0 |
  2015633.8 |
  2020347.6 |
  2025061.4 |
  2029775.2 |
  2034489.0 |
  2039202.9 |
  2043916.7 |
  2048630.5 |
  2053344.3 |
  2058058.1 |
  2062771.9 |
  2067485.7 |
  2072199.5 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_dispatch (n=6, range 1987063.7-1993293.8 ns)
  1987063.7 |########################################
  1987375.2 |
  1987686.7 |
  1987998.2 |
  1988309.7 |
  1988621.2 |
  1988932.7 |
  1989244.2 |########################################
  1989555.7 |
  1989867.2 |
  1990178.7 |########################################
  1990490.2 |
  1990801.7 |########################################
  1991113.2 |
  1991424.7 |
  1991736.2 |########################################
  1992047.7 |
  1992359.2 |
  1992670.7 |
  1992982.2 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_null (n=6, range 3370.4-3480.4 ns)
   3370.4 |########################################
   3375.9 |########################################
   3381.4 |
   3386.9 |
   3392.4 |
   3397.9 |
   3403.4 |########################################
   3408.9 |
   3414.4 |
   3419.9 |
   3425.4 |
   3430.9 |
   3436.4 |
   3441.9 |########################################
   3447.4 |
   3452.9 |
   3458.4 |
   3463.9 |
   3469.4 |########################################
   3474.9 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_per_w_set (n=6, range 1984099.6-1994014.4 ns)
  1984099.6 |########################################
  1984595.3 |########################################
  1985091.1 |########################################
  1985586.8 |########################################
  1986082.6 |
  1986578.3 |
  1987074.0 |
  1987569.8 |
  1988065.5 |
  1988561.2 |
  1989057.0 |
  1989552.7 |
  1990048.5 |
  1990544.2 |
  1991039.9 |
  1991535.7 |
  1992031.4 |
  1992527.1 |
  1993022.9 |########################################
  1993518.6 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_runtime_w (n=6, range 1997574.2-2015239.6 ns)
  1997574.2 |####################
  1998457.5 |####################
  1999340.7 |
  2000224.0 |####################
  2001107.3 |########################################
  2001990.5 |
  2002873.8 |
  2003757.1 |
  2004640.3 |
  2005523.6 |
  2006406.9 |
  2007290.1 |
  2008173.4 |
  2009056.7 |
  2009939.9 |
  2010823.2 |
  2011706.5 |
  2012589.7 |
  2013473.0 |
  2014356.3 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_dispatch (n=6, range 3094688.8-3101341.2 ns)
  3094688.8 |########################################
  3095021.4 |########################################
  3095354.0 |
  3095686.7 |
  3096019.3 |########################################
  3096351.9 |########################################
  3096684.5 |
  3097017.2 |
  3097349.8 |
  3097682.4 |
  3098015.0 |
  3098347.6 |
  3098680.3 |
  3099012.9 |########################################
  3099345.5 |
  3099678.1 |
  3100010.8 |
  3100343.4 |
  3100676.0 |
  3101008.6 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_runtime_w (n=6, range 3042778.8-3092652.0 ns)
  3042778.8 |########################################
  3045272.5 |########################################
  3047766.1 |
  3050259.8 |
  3052753.4 |
  3055247.1 |
  3057740.8 |
  3060234.4 |
  3062728.1 |
  3065221.8 |
  3067715.4 |
  3070209.1 |
  3072702.8 |
  3075196.4 |
  3077690.1 |
  3080183.7 |
  3082677.4 |
  3085171.1 |########################################
  3087664.7 |########################################
  3090158.4 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_tight_zig_anchor**: bridge=312.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_dispatch**: bridge=312.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_null**: bridge=8866.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_per_w_set**: bridge=313.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_runtime_w**: bridge=312.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_dispatch**: bridge=307.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_runtime_w**: bridge=307.9% of algo (FFI overhead may distort results)
