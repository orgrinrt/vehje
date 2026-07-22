# Predecoded dispatch shape, madd profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_madd_regcache dominates: 17% faster than the next best (carrier_pre_madd_null)

carrier_pre_madd_regcache (146.57 us) leads carrier_pre_madd_null (171.42 us) by 17%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_madd_regcache beats baseline by 23% (significant)

carrier_pre_madd_regcache is -43.69 us (23%) faster than baseline carrier_pre_madd_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_pre_madd_regcache** at 146574.4 ns median (-22.7% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.29x (fastest 146574.4 ns, slowest 189795.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_madd_direct | 198752ns | 192025ns | 189019ns | 191794ns | 214057ns | +1.64% |
| carrier_pre_madd_fntable | 188975ns | 188816ns | 188272ns | 188685ns | 189761ns | -3.36% |
| carrier_pre_madd_null | 172678ns | 173631ns | 169380ns | 172667ns | 174345ns | -11.69% |
| carrier_pre_madd_regcache | 150980ns | 148807ns | 147885ns | 148500ns | 156247ns | -22.79% |
| carrier_pre_madd_switch | 195541ns | 191791ns | 189638ns | 191631ns | 204358ns | base |
| carrier_pre_madd_threaded | 193341ns | 191988ns | 187573ns | 190700ns | 200186ns | -1.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_madd_direct | 196396ns | 186907ns | 211359ns | +1.62% | 0.021 |
| carrier_pre_madd_fntable | 186758ns | 186068ns | 187571ns | -3.37% | 0.022 |
| carrier_pre_madd_null | 170479ns | 167255ns | 172102ns | -11.79% | 0.024 |
| carrier_pre_madd_regcache | 148705ns | 145640ns | 153868ns | -23.06% | 0.028 |
| carrier_pre_madd_switch | 193262ns | 187520ns | 201888ns | base | 0.021 |
| carrier_pre_madd_threaded | 191053ns | 185404ns | 197759ns | -1.14% | 0.021 |

## Performance model

- Peak throughput: **0.028 Gops/s** (carrier_pre_madd_regcache; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_madd_direct | 0.022 | 76.7% |
| carrier_pre_madd_fntable | 0.022 | 78.1% |
| carrier_pre_madd_null | 0.024 | 85.0% |
| carrier_pre_madd_regcache | 0.028 | 99.4% |
| carrier_pre_madd_switch | 0.022 | 76.8% |
| carrier_pre_madd_threaded | 0.022 | 76.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_madd_direct | 198752ns | 198752ns | +1.64% |
| carrier_pre_madd_fntable | 188975ns | 188975ns | -3.36% |
| carrier_pre_madd_null | 172678ns | 172678ns | -11.69% |
| carrier_pre_madd_regcache | 150980ns | 150980ns | -22.79% |
| carrier_pre_madd_switch | 195541ns | 195541ns | base |
| carrier_pre_madd_threaded | 193341ns | 193341ns | -1.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_madd_switch | 189585ns | base | --- | [188312, 201888] | --- | --- | --- | --- |
| carrier_pre_madd_direct | 189796ns | no significant difference | [-1765, +10315]ns | [188032, 211359] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_madd_fntable | 186569ns | -3028.1ns (-1.6%) | [-15513, -969]ns | [186135, 187571] | YES (adj: no) | 0.3646 | 0.2188 | 0 |
| carrier_pre_madd_null | 171420ns | -19209.6ns (-10.1%) | [-30973, -18165]ns | [167916, 172102] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_pre_madd_regcache | 146574ns | -43685.3ns (-23.0%) | [-48529, -41456]ns | [145672, 153868] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_pre_madd_threaded | 189711ns | no significant difference | [-7488, +3451]ns | [185688, 197759] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_madd_switch | carrier_pre_madd_direct | carrier_pre_madd_fntable | carrier_pre_madd_null | carrier_pre_madd_regcache | carrier_pre_madd_threaded |
|---|---|---|---|---|---|---|
| 1 | 189609ns | -1.4% | -1.7% | -9.5% | -23.2% | -1.9% |
| 2 | 191121ns | -0.4% | -2.3% | -10.2% | -23.5% | -3.0% |
| 3 | 189105ns | +0.1% | -1.5% | -11.6% | -23.0% | +1.6% |
| 4 | 187520ns | +0.9% | +0.1% | -10.1% | -21.7% | +2.1% |
| 5 | 189560ns | +0.8% | -1.1% | -9.7% | -22.3% | -0.8% |
| 6 | 212654ns | +8.9% | -12.5% | -18.9% | -24.6% | -4.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_madd_direct | 0.002 | ok |
| carrier_pre_madd_fntable | -0.128 | ok |
| carrier_pre_madd_null | 0.178 | ok |
| carrier_pre_madd_regcache | 0.037 | ok |
| carrier_pre_madd_switch | -0.022 | ok |
| carrier_pre_madd_threaded | -0.070 | ok |

**Consistency summary:**

- **carrier_pre_madd_direct**: won 2/6, lost 4/6
- **carrier_pre_madd_fntable**: won 5/6, lost 0/6
- **carrier_pre_madd_null**: won 6/6, lost 0/6
- **carrier_pre_madd_regcache**: won 6/6, lost 0/6
- **carrier_pre_madd_threaded**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_madd_direct | 209659.9ns | 196395.7ns | 106.8% | HIGH |
| carrier_pre_madd_fntable | 196044.9ns | 186758.2ns | 105.0% | HIGH |
| carrier_pre_madd_null | 180004.3ns | 170479.3ns | 105.6% | HIGH |
| carrier_pre_madd_regcache | 158841.0ns | 148704.8ns | 106.8% | HIGH |
| carrier_pre_madd_switch | 202655.3ns | 193261.6ns | 104.9% | HIGH |
| carrier_pre_madd_threaded | 200662.8ns | 191052.9ns | 105.0% | HIGH |

## Distribution (algo ns)

```
carrier_pre_madd_direct (n=6, range 186907.1-211359.0 ns)
  186907.1 |####################
  188129.7 |########################################
  189352.3 |####################
  190574.9 |####################
  191797.5 |
  193020.1 |
  194242.7 |
  195465.2 |
  196687.8 |
  197910.4 |
  199133.0 |
  200355.6 |
  201578.2 |
  202800.8 |
  204023.4 |
  205246.0 |
  206468.6 |
  207691.2 |
  208913.8 |
  210136.4 |
  (0 below, 1 above range)

carrier_pre_madd_fntable (n=6, range 186067.9-187570.6 ns)
  186067.9 |########################################
  186143.0 |########################################
  186218.2 |
  186293.3 |
  186368.4 |
  186443.6 |########################################
  186518.7 |
  186593.8 |
  186669.0 |########################################
  186744.1 |
  186819.2 |
  186894.4 |
  186969.5 |
  187044.7 |
  187119.8 |
  187194.9 |
  187270.1 |
  187345.2 |
  187420.3 |########################################
  187495.5 |
  (0 below, 1 above range)

carrier_pre_madd_null (n=6, range 167255.4-172102.0 ns)
  167255.4 |####################
  167497.7 |
  167740.1 |
  167982.4 |
  168224.7 |
  168467.1 |####################
  168709.4 |
  168951.7 |
  169194.1 |
  169436.4 |
  169678.7 |
  169921.1 |
  170163.4 |
  170405.7 |
  170648.1 |
  170890.4 |
  171132.7 |####################
  171375.1 |
  171617.4 |########################################
  171859.7 |
  (0 below, 1 above range)

carrier_pre_madd_regcache (n=6, range 145640.4-153868.4 ns)
  145640.4 |########################################
  146051.8 |####################
  146463.2 |####################
  146874.6 |
  147286.0 |####################
  147697.4 |
  148108.8 |
  148520.2 |
  148931.6 |
  149343.0 |
  149754.4 |
  150165.8 |
  150577.2 |
  150988.6 |
  151400.0 |
  151811.4 |
  152222.8 |
  152634.2 |
  153045.6 |
  153457.0 |
  (0 below, 1 above range)

carrier_pre_madd_switch (n=6, range 187520.0-201887.7 ns)
  187520.0 |#############
  188238.4 |
  188956.8 |########################################
  189675.2 |
  190393.5 |
  191111.9 |#############
  191830.3 |
  192548.7 |
  193267.1 |
  193985.5 |
  194703.9 |
  195422.2 |
  196140.6 |
  196859.0 |
  197577.4 |
  198295.8 |
  199014.2 |
  199732.5 |
  200450.9 |
  201169.3 |
  (0 below, 1 above range)

carrier_pre_madd_threaded (n=6, range 185404.2-197759.3 ns)
  185404.2 |########################################
  186022.0 |
  186639.7 |
  187257.5 |
  187875.2 |####################
  188493.0 |
  189110.7 |
  189728.5 |
  190346.3 |
  190964.0 |####################
  191581.8 |####################
  192199.5 |
  192817.3 |
  193435.0 |
  194052.8 |
  194670.6 |
  195288.3 |
  195906.1 |
  196523.8 |
  197141.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_madd_direct**: bridge=106.9% of algo (FFI overhead may distort results)
- **carrier_pre_madd_fntable**: bridge=105.0% of algo (FFI overhead may distort results)
- **carrier_pre_madd_null**: bridge=105.5% of algo (FFI overhead may distort results)
- **carrier_pre_madd_regcache**: bridge=106.1% of algo (FFI overhead may distort results)
- **carrier_pre_madd_switch**: bridge=105.1% of algo (FFI overhead may distort results)
- **carrier_pre_madd_threaded**: bridge=105.1% of algo (FFI overhead may distort results)
