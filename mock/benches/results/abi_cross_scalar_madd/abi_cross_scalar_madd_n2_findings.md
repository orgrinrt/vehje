# abi_cross_scalar (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_madd_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_madd_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_madd_null_entry dominates: 80773% faster than the next best (abi_cross_scalar_madd_inproc_direct)

abi_cross_scalar_madd_null_entry (3.41 us) leads abi_cross_scalar_madd_inproc_direct (2.76 ms) by 80773%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_madd_null_entry beats baseline by 100% (significant)

abi_cross_scalar_madd_null_entry is -2.76 ms (100%) faster than baseline abi_cross_scalar_madd_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_madd_ffi_batched_scalar is an outlier: 818.7x slower than the field

abi_cross_scalar_madd_ffi_batched_scalar (2.80 ms) is 818.7x the fastest (3.41 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_madd_null_entry shows alternating (throttle bounce) (autocorr -0.54)

abi_cross_scalar_madd_null_entry's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_madd_null_entry} vs {abi_cross_scalar_madd_inproc_direct, abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar} (80773% apart)

The field splits into a fast tier {abi_cross_scalar_madd_null_entry} and a slow tier {abi_cross_scalar_madd_inproc_direct, abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar} with a 80773% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 818.7x the fastest

Fastest abi_cross_scalar_madd_null_entry (3.41 us) to slowest abi_cross_scalar_madd_ffi_batched_scalar (2.80 ms): 818.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_madd_null_entry** at 3414.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 818.68x (fastest 3414.8 ns, slowest 2795636.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2801140ns | 2799438ns | 2790782ns | 2796654ns | 2813048ns | +0.83% |
| abi_cross_scalar_madd_inproc_direct | 2778214ns | 2765173ns | 2737990ns | 2760910ns | 2824282ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 2778255ns | 2779005ns | 2771063ns | 2778353ns | 2781704ns | +0.00% |
| abi_cross_scalar_madd_null_entry | 5758ns | 5767ns | 5581ns | 5714ns | 5913ns | -99.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2797281ns | 2786923ns | 2809076ns | +0.82% | 0.000 |
| abi_cross_scalar_madd_inproc_direct | 2774459ns | 2734696ns | 2820008ns | base | 0.000 |
| abi_cross_scalar_madd_inproc_fnptr | 2774646ns | 2767276ns | 2778247ns | +0.01% | 0.000 |
| abi_cross_scalar_madd_null_entry | 3438ns | 3361ns | 3524ns | -99.88% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 81342.7 | 2797230.8 | 2797281.2 | n/a |
| abi_cross_scalar_madd_inproc_direct | 8727.7 | 2848515.8 | 2774459.4 | n/a |
| abi_cross_scalar_madd_inproc_fnptr | 8654.2 | 2780560.4 | 2774646.2 | 0 |
| abi_cross_scalar_madd_null_entry | 28260.9 | 3543.2 | 3438.0 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_scalar_madd_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_madd_null_entry | 0.001 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2801140ns | 2801140ns | +0.83% |
| abi_cross_scalar_madd_inproc_direct | 2778214ns | 2778214ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 2778255ns | 2778255ns | +0.00% |
| abi_cross_scalar_madd_null_entry | 5758ns | 5758ns | -99.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_inproc_direct | 2761658ns | base | --- | [2741712, 2820008] | --- | --- | --- | --- |
| abi_cross_scalar_madd_ffi_batched_scalar | 2795636ns | no significant difference | [-25572, +53769]ns | [2787131, 2809076] | no | 0.3281 | 0.2188 | 0 |
| abi_cross_scalar_madd_inproc_fnptr | 2775295ns | no significant difference | [-48360, +34481]ns | [2770397, 2778247] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_madd_null_entry | 3415ns | -2758208.0ns (-99.9%) | [-2816602, -2738254]ns | [3375, 3524] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_madd_inproc_direct | abi_cross_scalar_madd_ffi_batched_scalar | abi_cross_scalar_madd_inproc_fnptr | abi_cross_scalar_madd_null_entry |
|---|---|---|---|---|
| 1 | 2774117ns | +0.5% | -0.0% | -99.9% |
| 2 | 2749198ns | +2.0% | +0.9% | -99.9% |
| 3 | 2748729ns | +1.6% | +1.0% | -99.9% |
| 4 | 2734696ns | +1.9% | +1.5% | -99.9% |
| 5 | 2776618ns | +1.3% | +0.1% | -99.9% |
| 6 | 2863398ns | -2.2% | -3.4% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | -0.323 | moderate- |
| abi_cross_scalar_madd_inproc_direct | 0.166 | ok |
| abi_cross_scalar_madd_inproc_fnptr | -0.295 | moderate- |
| abi_cross_scalar_madd_null_entry | -0.541 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_cross_scalar_madd_ffi_batched_scalar**: won 1/6, lost 5/6
- **abi_cross_scalar_madd_inproc_fnptr**: won 1/6, lost 4/6
- **abi_cross_scalar_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 8468023.0ns | 2797281.2ns | 302.7% | HIGH |
| abi_cross_scalar_madd_inproc_direct | 8435043.2ns | 2774459.4ns | 304.0% | HIGH |
| abi_cross_scalar_madd_inproc_fnptr | 8347479.3ns | 2774646.2ns | 300.8% | HIGH |
| abi_cross_scalar_madd_null_entry | 121519.5ns | 3438.0ns | 3534.5% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_madd_ffi_batched_scalar (n=6, range 2786922.9-2809076.2 ns)
  2786922.9 |########################################
  2788030.6 |
  2789138.2 |
  2790245.9 |
  2791353.6 |####################
  2792461.2 |
  2793568.9 |
  2794676.6 |
  2795784.2 |
  2796891.9 |
  2797999.6 |
  2799107.2 |####################
  2800214.9 |
  2801322.6 |
  2802430.2 |
  2803537.9 |####################
  2804645.6 |
  2805753.2 |
  2806860.9 |
  2807968.6 |
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_direct (n=6, range 2734695.8-2820008.3 ns)
  2734695.8 |####################
  2738961.4 |
  2743227.0 |
  2747492.7 |########################################
  2751758.3 |
  2756023.9 |
  2760289.5 |
  2764555.2 |
  2768820.8 |
  2773086.4 |########################################
  2777352.0 |
  2781617.7 |
  2785883.3 |
  2790148.9 |
  2794414.5 |
  2798680.2 |
  2802945.8 |
  2807211.4 |
  2811477.0 |
  2815742.7 |
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_fnptr (n=6, range 2767275.8-2778246.6 ns)
  2767275.8 |########################################
  2767824.3 |
  2768372.9 |
  2768921.4 |
  2769470.0 |
  2770018.5 |
  2770567.1 |
  2771115.6 |
  2771664.1 |
  2772212.7 |
  2772761.2 |
  2773309.8 |########################################
  2773858.3 |
  2774406.9 |########################################
  2774955.4 |
  2775503.9 |########################################
  2776052.5 |
  2776601.0 |########################################
  2777149.6 |
  2777698.1 |
  (0 below, 1 above range)

abi_cross_scalar_madd_null_entry (n=6, range 3360.8-3523.9 ns)
   3360.8 |########################################
   3369.0 |
   3377.1 |
   3385.3 |########################################
   3393.4 |
   3401.6 |########################################
   3409.7 |
   3417.9 |########################################
   3426.1 |
   3434.2 |
   3442.4 |
   3450.5 |
   3458.7 |
   3466.8 |
   3475.0 |
   3483.2 |
   3491.3 |
   3499.5 |
   3507.6 |########################################
   3515.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_madd_ffi_batched_scalar**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_direct**: bridge=299.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_fnptr**: bridge=300.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_null_entry**: bridge=3554.0% of algo (FFI overhead may distort results)
