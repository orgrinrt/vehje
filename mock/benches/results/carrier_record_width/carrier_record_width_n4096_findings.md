# Record width: wire-format stride and inline-vs-pool decode cost (carrier)

5 variants, 6 samples per variant.
Baseline: **carrier_rec24**

## Highlights

Baseline for all deltas below: **carrier_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_rec24) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_rec24 has the worst median (591.46 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_rec16 at 537.88 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_rec12 shows alternating (throttle bounce) (autocorr -0.60)

carrier_rec12's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader carrier_rec16 vs stability leader carrier_rec12 (+4% speed for 1.1x steadier)

carrier_rec16 is fastest (537.88 us, CV 5.4%); carrier_rec12 gives up 3.7% median for 1.1x lower variance (CV 4.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_rec16** at 537883.8 ns median (-9.1% vs baseline)
- Spread: 1.10x (fastest 537883.8 ns, slowest 591459.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_rec12 | 564256ns | 561125ns | 521308ns | 557239ns | 596257ns | -2.00% |
| carrier_rec16 | 550038ns | 540929ns | 511580ns | 539850ns | 584549ns | -4.47% |
| carrier_rec20 | 561866ns | 554948ns | 526442ns | 546328ns | 602885ns | -2.41% |
| carrier_rec24 | 575760ns | 594335ns | 532903ns | 574852ns | 598549ns | base |
| carrier_rec32 | 564752ns | 563735ns | 523627ns | 554149ns | 601221ns | -1.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_rec12 | 561265ns | 518187ns | 593368ns | -2.04% | 0.007 |
| carrier_rec16 | 547074ns | 509218ns | 581416ns | -4.51% | 0.007 |
| carrier_rec20 | 558833ns | 523371ns | 600004ns | -2.46% | 0.007 |
| carrier_rec24 | 572935ns | 529672ns | 595935ns | base | 0.007 |
| carrier_rec32 | 561908ns | 520741ns | 598591ns | -1.92% | 0.007 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_rec16; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_rec12 | 0.007 | 91.2% |
| carrier_rec16 | 0.008 | 94.7% |
| carrier_rec20 | 0.007 | 92.3% |
| carrier_rec24 | 0.007 | 86.1% |
| carrier_rec32 | 0.007 | 90.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_rec12 | 564256ns | 564256ns | -2.00% |
| carrier_rec16 | 550038ns | 550038ns | -4.47% |
| carrier_rec20 | 561866ns | 561866ns | -2.41% |
| carrier_rec24 | 575760ns | 575760ns | base |
| carrier_rec32 | 564752ns | 564752ns | -1.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_rec24 | 591460ns | base | --- | [531410, 595935] | --- | --- | --- | --- |
| carrier_rec12 | 558052ns | no significant difference | [-58995, +37943]ns | [532374, 593368] | no | 0.9167 | 0.6875 | 0 |
| carrier_rec16 | 537884ns | no significant difference | [-56491, +18169]ns | [521923, 581416] | no | 0.9167 | 0.6875 | 0 |
| carrier_rec20 | 551837ns | no significant difference | [-63879, +21887]ns | [524658, 600004] | no | 1.0000 | 1.0000 | 0 |
| carrier_rec32 | 560732ns | no significant difference | [-67141, +62605]ns | [526402, 598591] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_rec24 | carrier_rec12 | carrier_rec16 | carrier_rec20 | carrier_rec32 |
|---|---|---|---|---|---|
| 1 | 533148ns | +2.5% | -4.5% | +7.0% | +13.4% |
| 2 | 592655ns | -6.9% | -9.2% | +1.0% | -9.2% |
| 3 | 590265ns | -4.4% | -9.4% | -9.6% | -9.9% |
| 4 | 596822ns | -0.4% | +1.0% | -11.9% | -12.7% |
| 5 | 595048ns | -12.9% | -9.6% | +1.1% | -0.4% |
| 6 | 529672ns | +11.8% | +5.7% | -1.2% | +10.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_rec12 | -0.600 | HIGH- (thermal bounce) |
| carrier_rec16 | -0.176 | ok |
| carrier_rec20 | -0.406 | moderate- |
| carrier_rec24 | -0.088 | ok |
| carrier_rec32 | 0.051 | ok |

**Consistency summary:**

- **carrier_rec12**: won 4/6, lost 2/6
- **carrier_rec16**: won 4/6, lost 2/6
- **carrier_rec20**: won 3/6, lost 3/6
- **carrier_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_rec12 | 436.0ns | 561264.8ns | 0.1% |  |
| carrier_rec16 | 466.9ns | 547074.1ns | 0.1% |  |
| carrier_rec20 | 405.7ns | 558832.9ns | 0.1% |  |
| carrier_rec24 | 423.2ns | 572934.8ns | 0.1% |  |
| carrier_rec32 | 375.3ns | 561908.3ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_rec12 (n=6, range 518186.7-593368.4 ns)
  518186.7 |########################################
  521945.8 |
  525704.9 |
  529463.9 |
  533223.0 |
  536982.1 |
  540741.2 |
  544500.3 |########################################
  548259.4 |########################################
  552018.4 |
  555777.5 |
  559536.6 |
  563295.7 |########################################
  567054.8 |
  570813.9 |
  574572.9 |
  578332.0 |
  582091.1 |
  585850.2 |
  589609.3 |########################################
  (0 below, 1 above range)

carrier_rec16 (n=6, range 509217.9-581415.6 ns)
  509217.9 |#############
  512827.8 |
  516437.7 |
  520047.6 |
  523657.4 |
  527267.3 |
  530877.2 |
  534487.1 |########################################
  538097.0 |
  541706.9 |
  545316.8 |
  548926.6 |
  552536.5 |
  556146.4 |
  559756.3 |#############
  563366.2 |
  566976.1 |
  570585.9 |
  574195.8 |
  577805.7 |
  (0 below, 1 above range)

carrier_rec20 (n=6, range 523371.2-600003.9 ns)
  523371.2 |########################################
  527202.8 |
  531034.5 |####################
  534866.1 |
  538697.8 |
  542529.4 |
  546361.0 |
  550192.7 |
  554024.3 |
  557855.9 |
  561687.6 |
  565519.2 |
  569350.8 |####################
  573182.5 |
  577014.1 |
  580845.8 |
  584677.4 |
  588509.0 |
  592340.7 |
  596172.3 |####################
  (0 below, 1 above range)

carrier_rec24 (n=6, range 529671.7-595935.0 ns)
  529671.7 |####################
  532984.9 |####################
  536298.0 |
  539611.2 |
  542924.4 |
  546237.5 |
  549550.7 |
  552863.9 |
  556177.0 |
  559490.2 |
  562803.3 |
  566116.5 |
  569429.7 |
  572742.8 |
  576056.0 |
  579369.2 |
  582682.3 |
  585995.5 |
  589308.7 |####################
  592621.8 |########################################
  (0 below, 1 above range)

carrier_rec32 (n=6, range 520741.2-598591.1 ns)
  520741.2 |########################################
  524633.7 |
  528526.2 |########################################
  532418.7 |
  536311.2 |########################################
  540203.7 |
  544096.2 |
  547988.6 |
  551881.1 |
  555773.6 |
  559666.1 |
  563558.6 |
  567451.1 |
  571343.6 |
  575236.1 |
  579128.6 |
  583021.1 |########################################
  586913.6 |
  590806.1 |########################################
  594698.6 |
  (0 below, 1 above range)

```
