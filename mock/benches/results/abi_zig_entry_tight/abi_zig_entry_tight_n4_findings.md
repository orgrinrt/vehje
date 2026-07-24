# abi_zig_entry (tight)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_tight_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_tight_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_tight_zig_null dominates: 48207% faster than the next best (abi_zig_entry_tight_zig_dispatch)

abi_zig_entry_tight_zig_null (4.11 us) leads abi_zig_entry_tight_zig_dispatch (1.99 ms) by 48207%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_tight_zig_null beats baseline by 100% (significant)

abi_zig_entry_tight_zig_null is -2.00 ms (100%) faster than baseline abi_zig_entry_tight_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_tight_zig_tail_runtime_w is an outlier: 752.5x slower than the field

abi_zig_entry_tight_zig_tail_runtime_w (3.09 ms) is 752.5x the fastest (4.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_tight_zig_dispatch shows alternating (throttle bounce) (autocorr -0.51)

abi_zig_entry_tight_zig_dispatch's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_tight_zig_null} vs {abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_tail_dispatch, abi_zig_entry_tight_zig_tail_runtime_w} (48207% apart)

The field splits into a fast tier {abi_zig_entry_tight_zig_null} and a slow tier {abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_tail_dispatch, abi_zig_entry_tight_zig_tail_runtime_w} with a 48207% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 752.5x the fastest

Fastest abi_zig_entry_tight_zig_null (4.11 us) to slowest abi_zig_entry_tight_zig_tail_runtime_w (3.09 ms): 752.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_tight_zig_null** at 4109.1 ns median (-99.8% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 752.51x (fastest 4109.1 ns, slowest 3092173.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2000828ns | 1995208ns | 1986857ns | 1994834ns | 2016806ns | -0.21% |
| abi_zig_entry_tight_zig_dispatch | 1987114ns | 1987572ns | 1980783ns | 1986025ns | 1991913ns | -0.90% |
| abi_zig_entry_tight_zig_null | 6434ns | 6422ns | 6350ns | 6401ns | 6526ns | -99.68% |
| abi_zig_entry_tight_zig_per_w_set | 1987761ns | 1988140ns | 1977870ns | 1987270ns | 1993443ns | -0.86% |
| abi_zig_entry_tight_zig_runtime_w | 2005097ns | 2004022ns | 1991258ns | 2000216ns | 2019340ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3085300ns | 3091864ns | 3035020ns | 3090887ns | 3102060ns | +53.87% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3121669ns | 3095002ns | 3081454ns | 3093506ns | 3184022ns | +55.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 1998122ns | 1984313ns | 2013920ns | -0.21% | 0.000 |
| abi_zig_entry_tight_zig_dispatch | 1984515ns | 1978305ns | 1989183ns | -0.89% | 0.000 |
| abi_zig_entry_tight_zig_null | 4125ns | 4065ns | 4200ns | -99.79% | 0.001 |
| abi_zig_entry_tight_zig_per_w_set | 1985158ns | 1975359ns | 1990824ns | -0.86% | 0.000 |
| abi_zig_entry_tight_zig_runtime_w | 2002396ns | 1988593ns | 2016549ns | base | 0.000 |
| abi_zig_entry_tight_zig_tail_dispatch | 3082534ns | 3032412ns | 3099143ns | +53.94% | 0.000 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3118872ns | 3078826ns | 3181111ns | +55.76% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 181231.9 | 2061872.9 | 1998122.0 | n/a |
| abi_zig_entry_tight_zig_dispatch | 176242.5 | 1984302.8 | 1984514.9 | n/a |
| abi_zig_entry_tight_zig_null | 155352.5 | 4167.2 | 4125.2 | n/a |
| abi_zig_entry_tight_zig_per_w_set | 175631.4 | 1984900.8 | 1985157.9 | n/a |
| abi_zig_entry_tight_zig_runtime_w | 188264.7 | 2001342.8 | 2002396.4 | n/a |
| abi_zig_entry_tight_zig_tail_dispatch | 186794.1 | 3081014.5 | 3082534.4 | 0 |
| abi_zig_entry_tight_zig_tail_runtime_w | 192472.6 | 3098766.7 | 3118872.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_zig_entry_tight_zig_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_null | 0.001 | 98.9% |
| abi_zig_entry_tight_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2000828ns | 2000828ns | -0.21% |
| abi_zig_entry_tight_zig_dispatch | 1987114ns | 1987114ns | -0.90% |
| abi_zig_entry_tight_zig_null | 6434ns | 6434ns | -99.68% |
| abi_zig_entry_tight_zig_per_w_set | 1987761ns | 1987761ns | -0.86% |
| abi_zig_entry_tight_zig_runtime_w | 2005097ns | 2005097ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3085300ns | 3085300ns | +53.87% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3121669ns | 3121669ns | +55.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_runtime_w | 2001336ns | base | --- | [1989304, 2016549] | --- | --- | --- | --- |
| abi_zig_entry_tight_zig_anchor | 1992581ns | no significant difference | [-21394, +7555]ns | [1987865, 2013920] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_tight_zig_dispatch | 1985011ns | -16656.2ns (-0.8%) | [-31538, -5450]ns | [1979351, 1989183] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_null | 4109ns | -1997188.8ns (-99.8%) | [-2012397, -1985228]ns | [4067, 4200] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_per_w_set | 1985497ns | -12598.9ns (-0.6%) | [-36583, -2534]ns | [1979153, 1990824] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_tail_dispatch | 3089182ns | +1090725.2ns (+54.5%) | [+1044168, +1105520]ns | [3059279, 3099143] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3092173ns | +1099518.2ns (+54.9%) | [+1083875, +1166034]ns | [3083333, 3181111] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_tight_zig_runtime_w | abi_zig_entry_tight_zig_anchor | abi_zig_entry_tight_zig_dispatch | abi_zig_entry_tight_zig_null | abi_zig_entry_tight_zig_per_w_set | abi_zig_entry_tight_zig_tail_dispatch | abi_zig_entry_tight_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1988593ns | +0.1% | -0.1% | -99.8% | -0.1% | +55.4% | +55.3% |
| 2 | 1992260ns | -0.0% | -0.7% | -99.8% | -0.5% | +55.7% | +55.4% |
| 3 | 2011673ns | -1.4% | -1.3% | -99.8% | -1.3% | +54.0% | +54.7% |
| 4 | 2021425ns | +0.6% | -1.8% | -99.8% | -2.3% | +52.8% | +60.8% |
| 5 | 2010412ns | -0.8% | -1.0% | -99.8% | -0.8% | +50.8% | +53.7% |
| 6 | 1990015ns | +0.2% | -0.5% | -99.8% | -0.2% | +55.1% | +54.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | -0.289 | moderate- |
| abi_zig_entry_tight_zig_dispatch | -0.512 | HIGH- (thermal bounce) |
| abi_zig_entry_tight_zig_null | -0.446 | moderate- |
| abi_zig_entry_tight_zig_per_w_set | -0.397 | moderate- |
| abi_zig_entry_tight_zig_runtime_w | 0.287 | moderate+ |
| abi_zig_entry_tight_zig_tail_dispatch | 0.017 | ok |
| abi_zig_entry_tight_zig_tail_runtime_w | -0.129 | ok |

**Consistency summary:**

- **abi_zig_entry_tight_zig_anchor**: won 2/6, lost 3/6
- **abi_zig_entry_tight_zig_dispatch**: won 5/6, lost 0/6
- **abi_zig_entry_tight_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_tight_zig_per_w_set**: won 5/6, lost 0/6
- **abi_zig_entry_tight_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_tight_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 6317031.9ns | 1998122.0ns | 316.1% | HIGH |
| abi_zig_entry_tight_zig_dispatch | 6198492.2ns | 1984514.9ns | 312.3% | HIGH |
| abi_zig_entry_tight_zig_null | 307800.8ns | 4125.2ns | 7461.4% | HIGH |
| abi_zig_entry_tight_zig_per_w_set | 6198025.3ns | 1985157.9ns | 312.2% | HIGH |
| abi_zig_entry_tight_zig_runtime_w | 6271112.3ns | 2002396.4ns | 313.2% | HIGH |
| abi_zig_entry_tight_zig_tail_dispatch | 9501904.7ns | 3082534.4ns | 308.2% | HIGH |
| abi_zig_entry_tight_zig_tail_runtime_w | 9595000.1ns | 3118872.3ns | 307.6% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_tight_zig_anchor (n=6, range 1984313.3-2013920.4 ns)
  1984313.3 |####################
  1985793.7 |
  1987274.0 |
  1988754.4 |
  1990234.7 |########################################
  1991715.1 |
  1993195.4 |####################
  1994675.8 |####################
  1996156.1 |
  1997636.5 |
  1999116.9 |
  2000597.2 |
  2002077.6 |
  2003557.9 |
  2005038.3 |
  2006518.6 |
  2007999.0 |
  2009479.3 |
  2010959.7 |
  2012440.0 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_dispatch (n=6, range 1978305.0-1989182.9 ns)
  1978305.0 |########################################
  1978848.9 |
  1979392.8 |
  1979936.7 |########################################
  1980480.6 |
  1981024.5 |
  1981568.4 |
  1982112.3 |
  1982656.2 |
  1983200.1 |
  1983743.9 |########################################
  1984287.8 |
  1984831.7 |
  1985375.6 |########################################
  1985919.5 |
  1986463.4 |
  1987007.3 |########################################
  1987551.2 |
  1988095.1 |
  1988639.0 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_null (n=6, range 4064.6-4199.8 ns)
   4064.6 |########################################
   4071.4 |
   4078.1 |
   4084.9 |####################
   4091.6 |
   4098.4 |
   4105.2 |
   4111.9 |
   4118.7 |
   4125.4 |####################
   4132.2 |
   4139.0 |
   4145.7 |
   4152.5 |
   4159.2 |####################
   4166.0 |
   4172.8 |
   4179.5 |
   4186.3 |
   4193.0 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_per_w_set (n=6, range 1975359.2-1990823.9 ns)
  1975359.2 |########################################
  1976132.4 |
  1976905.7 |
  1977678.9 |
  1978452.1 |
  1979225.4 |
  1979998.6 |
  1980771.9 |
  1981545.1 |
  1982318.3 |########################################
  1983091.6 |
  1983864.8 |########################################
  1984638.1 |
  1985411.3 |
  1986184.5 |########################################
  1986957.8 |########################################
  1987731.0 |
  1988504.2 |
  1989277.5 |
  1990050.7 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_runtime_w (n=6, range 1988592.9-2016548.9 ns)
  1988592.9 |########################################
  1989990.7 |########################################
  1991388.5 |########################################
  1992786.3 |
  1994184.1 |
  1995581.9 |
  1996979.7 |
  1998377.5 |
  1999775.3 |
  2001173.1 |
  2002570.9 |
  2003968.7 |
  2005366.5 |
  2006764.3 |
  2008162.1 |
  2009559.9 |########################################
  2010957.7 |########################################
  2012355.5 |
  2013753.3 |
  2015151.1 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_dispatch (n=6, range 3032412.5-3099142.8 ns)
  3032412.5 |####################
  3035749.0 |
  3039085.5 |
  3042422.0 |
  3045758.5 |
  3049095.1 |
  3052431.6 |
  3055768.1 |
  3059104.6 |
  3062441.1 |
  3065777.6 |
  3069114.1 |
  3072450.6 |
  3075787.2 |
  3079123.7 |
  3082460.2 |
  3085796.7 |########################################
  3089133.2 |####################
  3092469.7 |
  3095806.2 |####################
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_runtime_w (n=6, range 3078826.2-3181110.9 ns)
  3078826.2 |########################################
  3083940.4 |########################################
  3089054.7 |########################################
  3094168.9 |########################################
  3099283.1 |
  3104397.4 |
  3109511.6 |########################################
  3114625.8 |
  3119740.1 |
  3124854.3 |
  3129968.5 |
  3135082.8 |
  3140197.0 |
  3145311.2 |
  3150425.5 |
  3155539.7 |
  3160653.9 |
  3165768.2 |
  3170882.4 |
  3175996.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_tight_zig_anchor**: bridge=312.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_dispatch**: bridge=312.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_null**: bridge=7477.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_per_w_set**: bridge=312.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_runtime_w**: bridge=312.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_dispatch**: bridge=308.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_runtime_w**: bridge=308.0% of algo (FFI overhead may distort results)
