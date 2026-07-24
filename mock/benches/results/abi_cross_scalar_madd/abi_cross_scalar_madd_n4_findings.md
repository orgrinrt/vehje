# abi_cross_scalar (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_madd_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_madd_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_madd_null_entry dominates: 67758% faster than the next best (abi_cross_scalar_madd_inproc_direct)

abi_cross_scalar_madd_null_entry (4.02 us) leads abi_cross_scalar_madd_inproc_direct (2.73 ms) by 67758%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_madd_null_entry beats baseline by 100% (significant)

abi_cross_scalar_madd_null_entry is -2.72 ms (100%) faster than baseline abi_cross_scalar_madd_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_madd_ffi_batched_scalar is an outlier: 687.9x slower than the field

abi_cross_scalar_madd_ffi_batched_scalar (2.76 ms) is 687.9x the fastest (4.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_madd_inproc_fnptr shows alternating (throttle bounce) (autocorr -0.70)

abi_cross_scalar_madd_inproc_fnptr's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_madd_null_entry} vs {abi_cross_scalar_madd_inproc_direct, abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar} (67758% apart)

The field splits into a fast tier {abi_cross_scalar_madd_null_entry} and a slow tier {abi_cross_scalar_madd_inproc_direct, abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar} with a 67758% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 687.9x the fastest

Fastest abi_cross_scalar_madd_null_entry (4.02 us) to slowest abi_cross_scalar_madd_ffi_batched_scalar (2.76 ms): 687.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_madd_null_entry** at 4018.3 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 687.88x (fastest 4018.3 ns, slowest 2764101.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2770002ns | 2767934ns | 2758069ns | 2766049ns | 2781897ns | +1.42% |
| abi_cross_scalar_madd_inproc_direct | 2731324ns | 2730159ns | 2725688ns | 2729213ns | 2737310ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 2752126ns | 2746805ns | 2742099ns | 2745596ns | 2766935ns | +0.76% |
| abi_cross_scalar_madd_null_entry | 6302ns | 6318ns | 6119ns | 6267ns | 6445ns | -99.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2766148ns | 2754348ns | 2777784ns | +1.40% | 0.000 |
| abi_cross_scalar_madd_inproc_direct | 2727828ns | 2722176ns | 2733674ns | base | 0.000 |
| abi_cross_scalar_madd_inproc_fnptr | 2748634ns | 2738695ns | 2763384ns | +0.76% | 0.000 |
| abi_cross_scalar_madd_null_entry | 4019ns | 3922ns | 4104ns | -99.85% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 82553.1 | 2764789.0 | 2766148.0 | n/a |
| abi_cross_scalar_madd_inproc_direct | 8599.5 | 2727402.0 | 2727828.1 | n/a |
| abi_cross_scalar_madd_inproc_fnptr | 8658.8 | 2746872.4 | 2748633.7 | n/a |
| abi_cross_scalar_madd_null_entry | 27647.4 | 4173.5 | 4019.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_scalar_madd_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_madd_null_entry | 0.001 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2770002ns | 2770002ns | +1.42% |
| abi_cross_scalar_madd_inproc_direct | 2731324ns | 2731324ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 2752126ns | 2752126ns | +0.76% |
| abi_cross_scalar_madd_null_entry | 6302ns | 6302ns | -99.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_inproc_direct | 2726740ns | base | --- | [2723071, 2733674] | --- | --- | --- | --- |
| abi_cross_scalar_madd_ffi_batched_scalar | 2764101ns | +37360.8ns (+1.4%) | [+28198, +49401]ns | [2756559, 2777784] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_madd_inproc_fnptr | 2743044ns | +18432.5ns (+0.7%) | [+11112, +32872]ns | [2739473, 2763384] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_madd_null_entry | 4018ns | -2722674.6ns (-99.9%) | [-2729738, -2719014]ns | [3936, 4104] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_madd_inproc_direct | abi_cross_scalar_madd_ffi_batched_scalar | abi_cross_scalar_madd_inproc_fnptr | abi_cross_scalar_madd_null_entry |
|---|---|---|---|---|
| 1 | 2727046ns | +1.4% | +0.6% | -99.9% |
| 2 | 2722176ns | +2.2% | +0.7% | -99.9% |
| 3 | 2726434ns | +1.4% | +1.3% | -99.8% |
| 4 | 2723965ns | +1.3% | +0.6% | -99.9% |
| 5 | 2734590ns | +1.4% | +1.1% | -99.9% |
| 6 | 2732757ns | +0.8% | +0.2% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | -0.358 | moderate- |
| abi_cross_scalar_madd_inproc_direct | 0.208 | moderate+ |
| abi_cross_scalar_madd_inproc_fnptr | -0.696 | HIGH- (thermal bounce) |
| abi_cross_scalar_madd_null_entry | 0.344 | moderate+ |

**Consistency summary:**

- **abi_cross_scalar_madd_ffi_batched_scalar**: won 0/6, lost 6/6
- **abi_cross_scalar_madd_inproc_fnptr**: won 0/6, lost 6/6
- **abi_cross_scalar_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 8371231.2ns | 2766148.0ns | 302.6% | HIGH |
| abi_cross_scalar_madd_inproc_direct | 8196689.5ns | 2727828.1ns | 300.5% | HIGH |
| abi_cross_scalar_madd_inproc_fnptr | 8259715.3ns | 2748633.7ns | 300.5% | HIGH |
| abi_cross_scalar_madd_null_entry | 121947.0ns | 4019.3ns | 3034.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_madd_ffi_batched_scalar (n=6, range 2754347.9-2777783.8 ns)
  2754347.9 |########################################
  2755519.7 |
  2756691.5 |
  2757863.3 |########################################
  2759035.1 |
  2760206.9 |
  2761378.7 |
  2762550.4 |########################################
  2763722.2 |########################################
  2764894.0 |
  2766065.8 |
  2767237.6 |
  2768409.4 |
  2769581.2 |
  2770753.0 |
  2771924.8 |
  2773096.6 |########################################
  2774268.4 |
  2775440.2 |
  2776612.0 |
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_direct (n=6, range 2722176.2-2733673.5 ns)
  2722176.2 |########################################
  2722751.1 |
  2723325.9 |
  2723900.8 |########################################
  2724475.7 |
  2725050.5 |
  2725625.4 |
  2726200.3 |########################################
  2726775.1 |########################################
  2727350.0 |
  2727924.9 |
  2728499.7 |
  2729074.6 |
  2729649.5 |
  2730224.3 |
  2730799.2 |
  2731374.1 |
  2731948.9 |
  2732523.8 |########################################
  2733098.7 |
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_fnptr (n=6, range 2738694.6-2763384.0 ns)
  2738694.6 |########################################
  2739929.1 |########################################
  2741163.5 |########################################
  2742398.0 |
  2743632.5 |########################################
  2744866.9 |
  2746101.4 |
  2747335.9 |
  2748570.3 |
  2749804.8 |
  2751039.3 |
  2752273.7 |
  2753508.2 |
  2754742.7 |
  2755977.1 |
  2757211.6 |
  2758446.1 |
  2759680.5 |
  2760915.0 |
  2762149.5 |########################################
  (0 below, 1 above range)

abi_cross_scalar_madd_null_entry (n=6, range 3922.5-4104.1 ns)
   3922.5 |########################################
   3931.6 |
   3940.7 |########################################
   3949.7 |
   3958.8 |
   3967.9 |
   3977.0 |
   3986.1 |
   3995.2 |########################################
   4004.2 |
   4013.3 |
   4022.4 |
   4031.5 |########################################
   4040.6 |
   4049.7 |
   4058.7 |
   4067.8 |
   4076.9 |########################################
   4086.0 |
   4095.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_madd_ffi_batched_scalar**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_direct**: bridge=300.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_fnptr**: bridge=300.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_null_entry**: bridge=3043.8% of algo (FFI overhead may distort results)
