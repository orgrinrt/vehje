# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_madd_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_madd_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_madd_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_madd_interp has the worst median (2.39 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_madd_direct at 976 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_madd_direct beats baseline by 60% (significant)

carrier_nat_madd_direct is -1.42 us (60%) faster than baseline carrier_nat_madd_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_madd_interp is an outlier: 2.5x slower than the field

carrier_nat_madd_interp (2.39 us) is 2.5x the fastest (976 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_nat_madd_direct** at 976.0 ns median (-59.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.45x (fastest 976.0 ns, slowest 2392.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 3335ns | 3249ns | 3149ns | 3237ns | 3575ns | -29.57% |
| carrier_nat_madd_direct | 3282ns | 3202ns | 3132ns | 3180ns | 3510ns | -30.69% |
| carrier_nat_madd_interp | 4735ns | 4620ns | 4536ns | 4605ns | 5031ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 1022ns | 964ns | 1101ns | -58.27% | 0.063 |
| carrier_nat_madd_direct | 1001ns | 962ns | 1066ns | -59.12% | 0.064 |
| carrier_nat_madd_interp | 2450ns | 2377ns | 2579ns | base | 0.026 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_madd_copypatch | 257656 | 706504 | 0.365 | 0.93× |
| carrier_nat_madd_direct | 255021 | 708668 | 0.360 | 0.92× |
| carrier_nat_madd_interp | 277466 | 1343016 | 0.207 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.067 Gops/s** (carrier_nat_madd_direct; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_madd_copypatch | 0.064 | 96.7% |
| carrier_nat_madd_direct | 0.066 | 98.5% |
| carrier_nat_madd_interp | 0.027 | 40.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_madd_copypatch | 3335ns | 3335ns | -29.57% |
| carrier_nat_madd_direct | 3282ns | 3282ns | -30.69% |
| carrier_nat_madd_interp | 4735ns | 4735ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_madd_interp | 2392ns | base | --- | [2377, 2579] | --- | --- | --- | --- |
| carrier_nat_madd_copypatch | 995ns | -1411.2ns (-59.0%) | [-1534, -1336]ns | [971, 1101] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_madd_direct | 976ns | -1424.6ns (-59.5%) | [-1537, -1383]ns | [962, 1066] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_madd_interp | carrier_nat_madd_copypatch | carrier_nat_madd_direct |
|---|---|---|---|
| 1 | 2377ns | -54.3% | -59.5% |
| 2 | 2380ns | -58.9% | -56.8% |
| 3 | 2415ns | -58.8% | -59.4% |
| 4 | 2378ns | -58.1% | -59.6% |
| 5 | 2405ns | -59.9% | -59.6% |
| 6 | 2743ns | -59.3% | -59.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_madd_copypatch | -0.242 | moderate- |
| carrier_nat_madd_direct | -0.166 | ok |
| carrier_nat_madd_interp | 0.001 | ok |

**Consistency summary:**

- **carrier_nat_madd_copypatch**: won 6/6, lost 0/6
- **carrier_nat_madd_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_madd_copypatch | 84743.2ns | 1022.3ns | 8289.7% | HIGH |
| carrier_nat_madd_direct | 80790.1ns | 1001.5ns | 8067.2% | HIGH |
| carrier_nat_madd_interp | 86371.9ns | 2449.7ns | 3525.9% | HIGH |

## Distribution (algo ns)

```
carrier_nat_madd_copypatch (n=6, range 963.7-1100.8 ns)
    963.7 |####################
    970.6 |
    977.4 |####################
    984.3 |
    991.1 |########################################
    998.0 |
   1004.8 |
   1011.7 |
   1018.5 |
   1025.4 |
   1032.2 |
   1039.1 |
   1046.0 |
   1052.8 |
   1059.7 |
   1066.5 |
   1073.4 |
   1080.2 |####################
   1087.1 |
   1093.9 |
  (0 below, 1 above range)

carrier_nat_madd_direct (n=6, range 961.7-1066.5 ns)
    961.7 |########################################
    966.9 |####################
    972.2 |
    977.4 |####################
    982.7 |
    987.9 |
    993.1 |
    998.4 |
   1003.6 |
   1008.8 |
   1014.1 |
   1019.3 |
   1024.5 |####################
   1029.8 |
   1035.0 |
   1040.3 |
   1045.5 |
   1050.7 |
   1056.0 |
   1061.2 |
  (0 below, 1 above range)

carrier_nat_madd_interp (n=6, range 2376.7-2579.4 ns)
   2376.7 |########################################
   2386.8 |
   2397.0 |#############
   2407.1 |#############
   2417.2 |
   2427.4 |
   2437.5 |
   2447.6 |
   2457.8 |
   2467.9 |
   2478.0 |
   2488.2 |
   2498.3 |
   2508.4 |
   2518.6 |
   2528.7 |
   2538.8 |
   2549.0 |
   2559.1 |
   2569.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_madd_copypatch**: bridge=8565.9% of algo (FFI overhead may distort results)
- **carrier_nat_madd_direct**: bridge=8391.9% of algo (FFI overhead may distort results)
- **carrier_nat_madd_interp**: bridge=3614.3% of algo (FFI overhead may distort results)
