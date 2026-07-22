# Record layout (REC12..REC32) with fixed switch dispatch, tight profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_tight_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_tight_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_tight_rec20 shows alternating (throttle bounce) (autocorr -0.75)

carrier_lay_tight_rec20's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.41 us) is smaller than the fastest variant's own run-to-run std-dev (1.97 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (carrier_lay_tight_rec24)

The baseline carrier_lay_tight_rec24 is the fastest (187.95 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.8% of the fastest

All 5 variants sit between 187.95 us and 189.36 us - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_lay_tight_rec24) is the fastest** at 187949.0 ns median
- Spread: 1.01x (fastest 187949.0 ns, slowest 189362.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 191129ns | 190767ns | 187942ns | 190181ns | 194144ns | +0.17% |
| carrier_lay_tight_rec16 | 190982ns | 191091ns | 188072ns | 190337ns | 193403ns | +0.10% |
| carrier_lay_tight_rec20 | 191294ns | 191936ns | 188173ns | 191247ns | 192924ns | +0.26% |
| carrier_lay_tight_rec24 | 190798ns | 190648ns | 188067ns | 190188ns | 193080ns | base |
| carrier_lay_tight_rec32 | 190620ns | 190816ns | 188607ns | 190517ns | 191780ns | -0.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 188660ns | 185697ns | 191464ns | +0.28% | 0.022 |
| carrier_lay_tight_rec16 | 188156ns | 185359ns | 190620ns | +0.01% | 0.022 |
| carrier_lay_tight_rec20 | 188604ns | 185652ns | 190110ns | +0.25% | 0.022 |
| carrier_lay_tight_rec24 | 188141ns | 185376ns | 190546ns | base | 0.022 |
| carrier_lay_tight_rec32 | 188027ns | 186365ns | 189036ns | -0.06% | 0.022 |

## Performance model

- Peak throughput: **0.022 Gops/s** (carrier_lay_tight_rec16; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_tight_rec12 | 0.022 | 98.5% |
| carrier_lay_tight_rec16 | 0.022 | 98.5% |
| carrier_lay_tight_rec20 | 0.022 | 97.9% |
| carrier_lay_tight_rec24 | 0.022 | 98.6% |
| carrier_lay_tight_rec32 | 0.022 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_tight_rec12 | 191129ns | 191129ns | +0.17% |
| carrier_lay_tight_rec16 | 190982ns | 190982ns | +0.10% |
| carrier_lay_tight_rec20 | 191294ns | 191294ns | +0.26% |
| carrier_lay_tight_rec24 | 190798ns | 190798ns | base |
| carrier_lay_tight_rec32 | 190620ns | 190620ns | -0.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_tight_rec24 | 187949ns | base | --- | [185927, 190546] | --- | --- | --- | --- |
| carrier_lay_tight_rec12 | 188259ns | no significant difference | [-2436, +3225]ns | [186258, 191464] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_tight_rec16 | 188156ns | no significant difference | [-4854, +4693]ns | [185692, 190620] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_tight_rec20 | 189362ns | no significant difference | [-2861, +3693]ns | [186340, 190110] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_tight_rec32 | 188243ns | no significant difference | [-3744, +2460]ns | [186801, 189036] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_tight_rec24 | carrier_lay_tight_rec12 | carrier_lay_tight_rec16 | carrier_lay_tight_rec20 | carrier_lay_tight_rec32 |
|---|---|---|---|---|---|
| 1 | 190026ns | -1.6% | -2.5% | -0.1% | -1.9% |
| 2 | 191065ns | +0.6% | -2.6% | -2.1% | -2.0% |
| 3 | 186478ns | +1.7% | +1.8% | +2.1% | +1.3% |
| 4 | 187336ns | +1.8% | -0.3% | -0.9% | +0.7% |
| 5 | 188562ns | -0.9% | +0.5% | +0.7% | +0.3% |
| 6 | 185376ns | +0.2% | +3.3% | +1.9% | +1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_tight_rec12 | 0.024 | ok |
| carrier_lay_tight_rec16 | 0.104 | ok |
| carrier_lay_tight_rec20 | -0.753 | HIGH- (thermal bounce) |
| carrier_lay_tight_rec24 | 0.021 | ok |
| carrier_lay_tight_rec32 | 0.279 | moderate+ |

**Consistency summary:**

- **carrier_lay_tight_rec12**: won 2/6, lost 4/6
- **carrier_lay_tight_rec16**: won 3/6, lost 3/6
- **carrier_lay_tight_rec20**: won 2/6, lost 3/6
- **carrier_lay_tight_rec32**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 189335.8ns | 188660.3ns | 100.4% | HIGH |
| carrier_lay_tight_rec16 | 188558.2ns | 188156.0ns | 100.2% | HIGH |
| carrier_lay_tight_rec20 | 189293.4ns | 188604.0ns | 100.4% | HIGH |
| carrier_lay_tight_rec24 | 188968.9ns | 188140.5ns | 100.4% | HIGH |
| carrier_lay_tight_rec32 | 188587.6ns | 188026.7ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_lay_tight_rec12 (n=6, range 185697.1-191464.4 ns)
  185697.1 |########################################
  185985.5 |
  186273.8 |
  186562.2 |########################################
  186850.6 |########################################
  187138.9 |
  187427.3 |
  187715.6 |
  188004.0 |
  188292.4 |
  188580.7 |
  188869.1 |
  189157.5 |
  189445.8 |########################################
  189734.2 |
  190022.5 |
  190310.9 |
  190599.3 |########################################
  190887.6 |
  191176.0 |
  (0 below, 1 above range)

carrier_lay_tight_rec16 (n=6, range 185359.2-190620.0 ns)
  185359.2 |########################################
  185622.2 |
  185885.3 |########################################
  186148.3 |
  186411.4 |
  186674.4 |########################################
  186937.4 |
  187200.5 |
  187463.5 |
  187726.6 |
  187989.6 |
  188252.6 |
  188515.7 |
  188778.7 |
  189041.8 |
  189304.8 |########################################
  189567.8 |########################################
  189830.9 |
  190093.9 |
  190357.0 |
  (0 below, 1 above range)

carrier_lay_tight_rec20 (n=6, range 185651.7-190110.2 ns)
  185651.7 |####################
  185874.6 |
  186097.6 |
  186320.5 |
  186543.4 |
  186766.3 |
  186989.2 |####################
  187212.2 |
  187435.1 |
  187658.0 |
  187881.0 |
  188103.9 |
  188326.8 |
  188549.7 |
  188772.7 |####################
  188995.6 |
  189218.5 |
  189441.4 |
  189664.4 |########################################
  189887.3 |
  (0 below, 1 above range)

carrier_lay_tight_rec24 (n=6, range 185376.2-190545.8 ns)
  185376.2 |########################################
  185634.7 |
  185893.2 |
  186151.6 |
  186410.1 |########################################
  186668.6 |
  186927.1 |
  187185.6 |########################################
  187444.0 |
  187702.5 |
  187961.0 |
  188219.5 |
  188478.0 |########################################
  188736.4 |
  188994.9 |
  189253.4 |
  189511.9 |
  189770.4 |########################################
  190028.8 |
  190287.3 |
  (0 below, 1 above range)

carrier_lay_tight_rec32 (n=6, range 186365.0-189035.9 ns)
  186365.0 |########################################
  186498.5 |
  186632.1 |
  186765.6 |
  186899.2 |
  187032.7 |
  187166.3 |########################################
  187299.8 |
  187433.3 |
  187566.9 |
  187700.4 |
  187834.0 |########################################
  187967.5 |
  188101.1 |
  188234.6 |
  188368.1 |
  188501.7 |########################################
  188635.2 |
  188768.8 |
  188902.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_tight_rec12**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec16**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec20**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec24**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec32**: bridge=100.3% of algo (FFI overhead may distort results)
