# abi_payload_cost (tight)

3 variants, 6 samples per variant.
Baseline: **abi_payload_cost_tight_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_payload_cost_tight_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_payload_cost_tight_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_payload_cost_tight_scalar_payload has the worst median (2.00 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_payload_cost_tight_null_entry at 2.59 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_payload_cost_tight_null_entry dominates: 36815% faster than the next best (abi_payload_cost_tight_soa_payload)

abi_payload_cost_tight_null_entry (2.59 us) leads abi_payload_cost_tight_soa_payload (956.55 us) by 36815%, a clear separation rather than a photo finish. CV 53.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_payload_cost_tight_null_entry beats baseline by 100% (significant)

abi_payload_cost_tight_null_entry is -1.99 ms (100%) faster than baseline abi_payload_cost_tight_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_payload_cost_tight_scalar_payload is an outlier: 770.6x slower than the field

abi_payload_cost_tight_scalar_payload (2.00 ms) is 770.6x the fastest (2.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_payload_cost_tight_null_entry is fastest but the noisiest (CV 53.7%)

abi_payload_cost_tight_null_entry wins on median (2.59 us) yet has the highest variance (CV 53.7%), while abi_payload_cost_tight_scalar_payload is the steadiest (CV 0.7%, 2.00 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 770.6x the fastest

Fastest abi_payload_cost_tight_null_entry (2.59 us) to slowest abi_payload_cost_tight_scalar_payload (2.00 ms): 770.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_payload_cost_tight_null_entry is inconsistent: worst-20% is 1.8x its best-20%

abi_payload_cost_tight_null_entry's best 20% of batches run at 2.46 us but its worst 20% at 4.51 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: abi_payload_cost_tight_null_entry** at 2591.2 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 770.58x (fastest 2591.2 ns, slowest 1996774.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 5769ns | 4902ns | 4686ns | 4873ns | 7654ns | -99.71% |
| abi_payload_cost_tight_scalar_payload | 2005876ns | 1999293ns | 1997773ns | 1999102ns | 2020088ns | base |
| abi_payload_cost_tight_soa_payload | 965559ns | 959045ns | 949417ns | 957151ns | 986242ns | -51.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 3198ns | 2458ns | 4506ns | -99.84% | 0.080 |
| abi_payload_cost_tight_scalar_payload | 2003279ns | 1995318ns | 2017324ns | base | 0.000 |
| abi_payload_cost_tight_soa_payload | 963056ns | 946924ns | 983632ns | -51.93% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 38600.2 | 3090.6 | 3198.1 | n/a |
| abi_payload_cost_tight_scalar_payload | 41500.1 | 2008542.1 | 2003279.3 | n/a |
| abi_payload_cost_tight_soa_payload | 36519.9 | 962625.3 | 963056.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.104 Gops/s** (abi_payload_cost_tight_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_payload_cost_tight_null_entry | 0.099 | 94.8% |
| abi_payload_cost_tight_scalar_payload | 0.000 | 0.1% |
| abi_payload_cost_tight_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_payload_cost_tight_null_entry | 5769ns | 5769ns | -99.71% |
| abi_payload_cost_tight_scalar_payload | 2005876ns | 2005876ns | base |
| abi_payload_cost_tight_soa_payload | 965559ns | 965559ns | -51.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_payload_cost_tight_scalar_payload | 1996774ns | base | --- | [1995739, 2017324] | --- | --- | --- | --- |
| abi_payload_cost_tight_null_entry | 2591ns | -1994193.8ns (-99.9%) | [-2012818, -1993231]ns | [2497, 4506] | YES | 0.0313 | 0.0313 | 0 |
| abi_payload_cost_tight_soa_payload | 956554ns | -1040220.9ns (-52.1%) | [-1046756, -1033693]ns | [948983, 983632] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_payload_cost_tight_scalar_payload | abi_payload_cost_tight_null_entry | abi_payload_cost_tight_soa_payload |
|---|---|---|---|
| 1 | 2035761ns | -99.7% | -50.9% |
| 2 | 1998888ns | -99.9% | -51.5% |
| 3 | 1996500ns | -99.9% | -52.1% |
| 4 | 1995318ns | -99.9% | -52.3% |
| 5 | 1997049ns | -99.9% | -52.1% |
| 6 | 1996160ns | -99.9% | -52.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_payload_cost_tight_null_entry | 0.001 | ok |
| abi_payload_cost_tight_scalar_payload | 0.028 | ok |
| abi_payload_cost_tight_soa_payload | 0.223 | moderate+ |

**Consistency summary:**

- **abi_payload_cost_tight_null_entry**: won 6/6, lost 0/6
- **abi_payload_cost_tight_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_payload_cost_tight_null_entry | 146613.1ns | 3198.1ns | 4584.3% | HIGH |
| abi_payload_cost_tight_scalar_payload | 6066308.6ns | 2003279.3ns | 302.8% | HIGH |
| abi_payload_cost_tight_soa_payload | 2927286.2ns | 963056.1ns | 304.0% | HIGH |

## Distribution (algo ns)

```
abi_payload_cost_tight_null_entry (n=6, range 2457.5-4506.1 ns)
   2457.5 |########################################
   2559.9 |#############
   2662.4 |#############
   2764.8 |
   2867.2 |
   2969.6 |
   3072.1 |
   3174.5 |
   3276.9 |
   3379.3 |
   3481.8 |
   3584.2 |
   3686.6 |
   3789.1 |
   3891.5 |
   3993.9 |
   4096.3 |
   4198.8 |
   4301.2 |
   4403.6 |
  (0 below, 1 above range)

abi_payload_cost_tight_scalar_payload (n=6, range 1995317.9-2017324.4 ns)
  1995317.9 |########################################
  1996418.2 |########################################
  1997518.5 |
  1998618.9 |####################
  1999719.2 |
  2000819.5 |
  2001919.8 |
  2003020.2 |
  2004120.5 |
  2005220.8 |
  2006321.1 |
  2007421.4 |
  2008521.8 |
  2009622.1 |
  2010722.4 |
  2011822.7 |
  2012923.1 |
  2014023.4 |
  2015123.7 |
  2016224.0 |
  (0 below, 1 above range)

abi_payload_cost_tight_soa_payload (n=6, range 946923.7-983631.7 ns)
  946923.7 |########################################
  948759.1 |
  950594.5 |########################################
  952429.9 |
  954265.3 |########################################
  956100.7 |########################################
  957936.1 |
  959771.5 |
  961606.9 |
  963442.3 |
  965277.7 |
  967113.1 |########################################
  968948.5 |
  970783.9 |
  972619.3 |
  974454.7 |
  976290.1 |
  978125.5 |
  979960.9 |
  981796.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_payload_cost_tight_null_entry**: CV=43.5% (high variance, measurements may be unstable)
- **abi_payload_cost_tight_null_entry**: bridge=4407.4% of algo (FFI overhead may distort results)
- **abi_payload_cost_tight_scalar_payload**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_payload_cost_tight_soa_payload**: bridge=303.7% of algo (FFI overhead may distort results)
