# Runtime string interning, 8 compares of EQUAL strings: byte compare cannot exit early

3 variants, 6 samples per variant.
Baseline: **str_e_plain**

## Highlights

Baseline for all deltas below: **str_e_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### str_e_plain dominates: 206% faster than the next best (str_e_eager)

str_e_plain (110.37 us) leads str_e_eager (337.29 us) by 206%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### str_e_lazy is an outlier: 3.8x slower than the field

str_e_lazy (423.64 us) is 3.8x the fastest (110.37 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### str_e_plain shows alternating (throttle bounce) (autocorr -0.68)

str_e_plain's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (str_e_plain)

The baseline str_e_plain is the fastest (110.37 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.8x the fastest

Fastest str_e_plain (110.37 us) to slowest str_e_lazy (423.64 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (str_e_plain) is the fastest** at 110366.1 ns median
- 2 variants significantly slower than baseline
- Spread: 3.84x (fastest 110366.1 ns, slowest 423641.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| str_e_eager | 339595ns | 339893ns | 334757ns | 339289ns | 342473ns | +201.47% |
| str_e_lazy | 426601ns | 426245ns | 421874ns | 425909ns | 430002ns | +278.71% |
| str_e_plain | 112646ns | 112730ns | 110701ns | 112188ns | 114306ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| str_e_eager | 336905ns | 331964ns | 339982ns | +205.80% | 0.049 |
| str_e_lazy | 423872ns | 418680ns | 427505ns | +284.73% | 0.039 |
| str_e_plain | 110173ns | 108327ns | 111771ns | base | 0.149 |

## Performance model

- Peak throughput: **0.151 Gops/s** (str_e_plain; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| str_e_eager | 0.049 | 32.1% |
| str_e_lazy | 0.039 | 25.6% |
| str_e_plain | 0.148 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| str_e_eager | 339595ns | 339595ns | +201.47% |
| str_e_lazy | 426601ns | 426601ns | +278.71% |
| str_e_plain | 112646ns | 112646ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| str_e_plain | 110366ns | base | --- | [108382, 111771] | --- | --- | --- | --- |
| str_e_eager | 337295ns | +227215.4ns (+205.9%) | [+223231, +229750]ns | [333440, 339982] | YES | 0.0313 | 0.0313 | 0 |
| str_e_lazy | 423642ns | +313111.5ns (+283.7%) | [+311233, +316752]ns | [420469, 427505] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | str_e_plain | str_e_eager | str_e_lazy |
|---|---|---|---|
| 1 | 108327ns | +206.4% | +289.8% |
| 2 | 111454ns | +203.8% | +284.8% |
| 3 | 108437ns | +210.4% | +286.1% |
| 4 | 112089ns | +198.8% | +278.6% |
| 5 | 110052ns | +210.2% | +287.2% |
| 6 | 110680ns | +205.4% | +282.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| str_e_eager | -0.236 | moderate- |
| str_e_lazy | -0.620 | HIGH- (thermal bounce) |
| str_e_plain | -0.684 | HIGH- (thermal bounce) |

**Consistency summary:**

- **str_e_eager**: won 0/6, lost 6/6
- **str_e_lazy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| str_e_eager | 3919.4ns | 336905.3ns | 1.2% |  |
| str_e_lazy | 3953.2ns | 423871.7ns | 0.9% |  |
| str_e_plain | 4182.9ns | 110173.1ns | 3.8% |  |

## Distribution (algo ns)

```
str_e_eager (n=6, range 331963.8-339981.7 ns)
  331963.8 |########################################
  332364.7 |
  332765.6 |
  333166.5 |
  333567.4 |
  333968.3 |
  334369.2 |
  334770.0 |########################################
  335170.9 |
  335571.8 |
  335972.7 |
  336373.6 |########################################
  336774.5 |
  337175.4 |
  337576.3 |########################################
  337977.2 |
  338378.1 |########################################
  338779.0 |
  339179.9 |
  339580.8 |
  (0 below, 1 above range)

str_e_lazy (n=6, range 418680.4-427504.5 ns)
  418680.4 |########################################
  419121.6 |
  419562.8 |
  420004.0 |
  420445.2 |
  420886.4 |
  421327.6 |
  421768.9 |
  422210.1 |########################################
  422651.3 |########################################
  423092.5 |
  423533.7 |
  423974.9 |########################################
  424416.1 |
  424857.3 |
  425298.5 |
  425739.7 |########################################
  426180.9 |
  426622.1 |
  427063.3 |
  (0 below, 1 above range)

str_e_plain (n=6, range 108327.1-111771.4 ns)
  108327.1 |########################################
  108499.3 |
  108671.5 |
  108843.8 |
  109016.0 |
  109188.2 |
  109360.4 |
  109532.6 |
  109704.8 |
  109877.1 |
  110049.3 |####################
  110221.5 |
  110393.7 |
  110565.9 |####################
  110738.1 |
  110910.4 |
  111082.6 |
  111254.8 |
  111427.0 |####################
  111599.2 |
  (0 below, 1 above range)

```
