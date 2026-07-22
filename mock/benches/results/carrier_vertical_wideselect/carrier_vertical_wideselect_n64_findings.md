# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_wideselect_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_wideselect_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_wideselect_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_wideselect_scalar has the worst median (17.87 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_wideselect_vert8 at 7.78 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_wideselect_vert8 beats baseline by 56% (significant)

carrier_vert_wideselect_vert8 is -10.05 us (56%) faster than baseline carrier_vert_wideselect_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_wideselect_scalar is an outlier: 2.3x slower than the field

carrier_vert_wideselect_scalar (17.87 us) is 2.3x the fastest (7.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_wideselect_vert8 is fastest but the noisiest (CV 15.7%)

carrier_vert_wideselect_vert8 wins on median (7.78 us) yet has the highest variance (CV 15.7%), while carrier_vert_wideselect_vert4 is the steadiest (CV 6.9%, 7.93 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Speed leader carrier_vert_wideselect_vert8 vs stability leader carrier_vert_wideselect_vert4 (+2% speed for 2.3x steadier)

carrier_vert_wideselect_vert8 is fastest (7.78 us, CV 15.7%); carrier_vert_wideselect_vert4 gives up 1.9% median for 2.3x lower variance (CV 6.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_vert_wideselect_vert8** at 7779.8 ns median (-56.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.30x (fastest 7779.8 ns, slowest 17865.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 21072ns | 20202ns | 19186ns | 19962ns | 23681ns | base |
| carrier_vert_wideselect_vert4 | 10585ns | 10485ns | 9591ns | 10217ns | 11635ns | -49.77% |
| carrier_vert_wideselect_vert8 | 10672ns | 10397ns | 9305ns | 10060ns | 12275ns | -49.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 18634ns | 16925ns | 20978ns | base | 0.003 |
| carrier_vert_wideselect_vert4 | 7921ns | 7304ns | 8530ns | -57.49% | 0.008 |
| carrier_vert_wideselect_vert8 | 8063ns | 6941ns | 9399ns | -56.73% | 0.008 |

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_wideselect_vert8; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_wideselect_scalar | 0.004 | 38.8% |
| carrier_vert_wideselect_vert4 | 0.008 | 87.6% |
| carrier_vert_wideselect_vert8 | 0.008 | 89.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_wideselect_scalar | 21072ns | 21072ns | base |
| carrier_vert_wideselect_vert4 | 10585ns | 10585ns | -49.77% |
| carrier_vert_wideselect_vert8 | 10672ns | 10672ns | -49.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 17866ns | base | --- | [17058, 20978] | --- | --- | --- | --- |
| carrier_vert_wideselect_vert4 | 7926ns | -9974.1ns (-55.8%) | [-12723, -9441]ns | [7307, 8530] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_wideselect_vert8 | 7780ns | -10047.3ns (-56.2%) | [-13198, -8467]ns | [7011, 9399] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_wideselect_scalar | carrier_vert_wideselect_vert4 | carrier_vert_wideselect_vert8 |
|---|---|---|---|
| 1 | 16925ns | -56.8% | -58.2% |
| 2 | 17191ns | -57.5% | -59.6% |
| 3 | 19018ns | -56.8% | -59.2% |
| 4 | 17692ns | -56.9% | -53.8% |
| 5 | 18040ns | -51.4% | -41.1% |
| 6 | 22938ns | -63.9% | -66.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_wideselect_scalar | -0.018 | ok |
| carrier_vert_wideselect_vert4 | 0.093 | ok |
| carrier_vert_wideselect_vert8 | 0.116 | ok |

**Consistency summary:**

- **carrier_vert_wideselect_vert4**: won 6/6, lost 0/6
- **carrier_vert_wideselect_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 95088.7ns | 18633.8ns | 510.3% | HIGH |
| carrier_vert_wideselect_vert4 | 91446.8ns | 7921.1ns | 1154.5% | HIGH |
| carrier_vert_wideselect_vert8 | 90869.2ns | 8063.0ns | 1127.0% | HIGH |

## Distribution (algo ns)

```
carrier_vert_wideselect_scalar (n=6, range 16925.0-20977.9 ns)
  16925.0 |########################################
  17127.6 |########################################
  17330.3 |
  17532.9 |########################################
  17735.6 |
  17938.2 |########################################
  18140.9 |
  18343.5 |
  18546.2 |
  18748.8 |
  18951.5 |########################################
  19154.1 |
  19356.7 |
  19559.4 |
  19762.0 |
  19964.7 |
  20167.3 |
  20370.0 |
  20572.6 |
  20775.3 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert4 (n=6, range 7304.2-8530.0 ns)
   7304.2 |########################################
   7365.5 |
   7426.8 |
   7488.1 |
   7549.4 |
   7610.6 |####################
   7671.9 |
   7733.2 |
   7794.5 |
   7855.8 |
   7917.1 |
   7978.4 |
   8039.7 |
   8101.0 |
   8162.3 |####################
   8223.5 |
   8284.8 |####################
   8346.1 |
   8407.4 |
   8468.7 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert8 (n=6, range 6940.8-9398.5 ns)
   6940.8 |########################################
   7063.7 |########################################
   7186.6 |
   7309.5 |
   7432.4 |
   7555.2 |
   7678.1 |########################################
   7801.0 |########################################
   7923.9 |
   8046.8 |
   8169.7 |########################################
   8292.6 |
   8415.5 |
   8538.3 |
   8661.2 |
   8784.1 |
   8907.0 |
   9029.9 |
   9152.8 |
   9275.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_wideselect_scalar**: bridge=533.2% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert4**: bridge=1153.6% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert8**: bridge=1169.2% of algo (FFI overhead may distort results)
