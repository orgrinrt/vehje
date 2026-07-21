# Strategy showdown (skew2): all strategies x tiers (native/interp) same footing

12 variants, 6 samples per variant.
Baseline: **sd_bintree_int_skew2**

## Key findings

- **Fastest: sd_bintree_nat_skew2** at 123.1 ns median (-80.6% vs baseline)
- 10 variants significantly faster than baseline
- Spread: 5.32x (fastest 123.1 ns, slowest 654.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 3178ns | 3205ns | 2786ns | 3189ns | 3357ns | base |
| sd_bintree_nat_skew2 | 2718ns | 2712ns | 2533ns | 2693ns | 2848ns | -14.47% |
| sd_chain_int_skew2 | 3157ns | 3251ns | 2725ns | 3078ns | 3490ns | -0.67% |
| sd_chain_nat_skew2 | 2668ns | 2703ns | 2265ns | 2701ns | 2820ns | -16.05% |
| sd_chain_rev_int_skew2 | 3025ns | 3059ns | 2587ns | 2950ns | 3356ns | -4.82% |
| sd_chain_rev_nat_skew2 | 2623ns | 2716ns | 2276ns | 2570ns | 2877ns | -17.45% |
| sd_evalall_int_skew2 | 2751ns | 2749ns | 2564ns | 2690ns | 2936ns | -13.42% |
| sd_evalall_nat_skew2 | 2784ns | 2928ns | 2464ns | 2780ns | 2950ns | -12.40% |
| sd_jumptable_int_skew2 | 2697ns | 2709ns | 2281ns | 2659ns | 2962ns | -15.13% |
| sd_jumptable_nat_skew2 | 2751ns | 2821ns | 2277ns | 2781ns | 2945ns | -13.42% |
| sd_profiled_int_skew2 | 3074ns | 3103ns | 2712ns | 3045ns | 3298ns | -3.29% |
| sd_profiled_nat_skew2 | 2788ns | 2702ns | 2259ns | 2690ns | 3200ns | -12.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 641ns | 550ns | 709ns | base | 0.100 |
| sd_bintree_nat_skew2 | 127ns | 114ns | 138ns | -80.26% | 0.506 |
| sd_chain_int_skew2 | 629ns | 527ns | 691ns | -1.82% | 0.102 |
| sd_chain_nat_skew2 | 122ns | 105ns | 129ns | -80.95% | 0.524 |
| sd_chain_rev_int_skew2 | 556ns | 458ns | 615ns | -13.28% | 0.115 |
| sd_chain_rev_nat_skew2 | 121ns | 104ns | 132ns | -81.12% | 0.529 |
| sd_evalall_int_skew2 | 332ns | 297ns | 356ns | -48.20% | 0.193 |
| sd_evalall_nat_skew2 | 335ns | 283ns | 366ns | -47.74% | 0.191 |
| sd_jumptable_int_skew2 | 124ns | 110ns | 135ns | -80.59% | 0.514 |
| sd_jumptable_nat_skew2 | 127ns | 108ns | 137ns | -80.21% | 0.504 |
| sd_profiled_int_skew2 | 509ns | 458ns | 548ns | -20.55% | 0.126 |
| sd_profiled_nat_skew2 | 145ns | 105ns | 202ns | -77.37% | 0.441 |

## Performance model

- Peak throughput: **0.614 Gops/s** (sd_chain_rev_nat_skew2; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sd_bintree_int_skew2 | 0.101 | 16.5% |
| sd_bintree_nat_skew2 | 0.520 | 84.6% |
| sd_chain_int_skew2 | 0.098 | 15.9% |
| sd_chain_nat_skew2 | 0.516 | 84.0% |
| sd_chain_rev_int_skew2 | 0.113 | 18.5% |
| sd_chain_rev_nat_skew2 | 0.516 | 84.1% |
| sd_evalall_int_skew2 | 0.191 | 31.1% |
| sd_evalall_nat_skew2 | 0.188 | 30.6% |
| sd_jumptable_int_skew2 | 0.514 | 83.7% |
| sd_jumptable_nat_skew2 | 0.492 | 80.2% |
| sd_profiled_int_skew2 | 0.126 | 20.6% |
| sd_profiled_nat_skew2 | 0.519 | 84.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sd_bintree_int_skew2 | 3178ns | 3178ns | base |
| sd_bintree_nat_skew2 | 2718ns | 2718ns | -14.47% |
| sd_chain_int_skew2 | 3157ns | 3157ns | -0.67% |
| sd_chain_nat_skew2 | 2668ns | 2668ns | -16.05% |
| sd_chain_rev_int_skew2 | 3025ns | 3025ns | -4.82% |
| sd_chain_rev_nat_skew2 | 2623ns | 2623ns | -17.45% |
| sd_evalall_int_skew2 | 2751ns | 2751ns | -13.42% |
| sd_evalall_nat_skew2 | 2784ns | 2784ns | -12.40% |
| sd_jumptable_int_skew2 | 2697ns | 2697ns | -15.13% |
| sd_jumptable_nat_skew2 | 2751ns | 2751ns | -13.42% |
| sd_profiled_int_skew2 | 3074ns | 3074ns | -3.29% |
| sd_profiled_nat_skew2 | 2788ns | 2788ns | -12.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 633ns | base | --- | [582, 709] | --- | --- | --- | --- |
| sd_bintree_nat_skew2 | 123ns | -510.4ns (-80.6%) | [-570, -463]ns | [118, 138] | YES | 0.0382 | 0.0313 | 0 |
| sd_chain_int_skew2 | 655ns | no significant difference | [-70, +47]ns | [542, 691] | no | 0.6875 | 0.6875 | 0 |
| sd_chain_nat_skew2 | 124ns | -509.6ns (-80.5%) | [-580, -468]ns | [114, 129] | YES | 0.0382 | 0.0313 | 0 |
| sd_chain_rev_int_skew2 | 564ns | -86.1ns (-13.6%) | [-159, -11]ns | [488, 615] | YES (adj: no) | 0.2406 | 0.2188 | 0 |
| sd_chain_rev_nat_skew2 | 124ns | -510.4ns (-80.6%) | [-577, -473]ns | [107, 132] | YES | 0.0382 | 0.0313 | 0 |
| sd_evalall_int_skew2 | 335ns | -300.2ns (-47.4%) | [-373, -254]ns | [305, 356] | YES | 0.0382 | 0.0313 | 0 |
| sd_evalall_nat_skew2 | 340ns | -301.1ns (-47.5%) | [-367, -250]ns | [299, 366] | YES | 0.0382 | 0.0313 | 0 |
| sd_jumptable_int_skew2 | 125ns | -505.2ns (-79.8%) | [-577, -468]ns | [114, 135] | YES | 0.0382 | 0.0313 | 0 |
| sd_jumptable_nat_skew2 | 130ns | -497.8ns (-78.6%) | [-577, -468]ns | [114, 137] | YES | 0.0382 | 0.0313 | 0 |
| sd_profiled_int_skew2 | 506ns | -103.1ns (-16.3%) | [-211, -81]ns | [474, 548] | YES | 0.0382 | 0.0313 | 0 |
| sd_profiled_nat_skew2 | 123ns | -513.5ns (-81.1%) | [-584, -390]ns | [110, 202] | YES | 0.0382 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | sd_bintree_int_skew2 | sd_bintree_nat_skew2 | sd_chain_int_skew2 | sd_chain_nat_skew2 | sd_chain_rev_int_skew2 | sd_chain_rev_nat_skew2 | sd_evalall_int_skew2 | sd_evalall_nat_skew2 | sd_jumptable_int_skew2 | sd_jumptable_nat_skew2 | sd_profiled_int_skew2 | sd_profiled_nat_skew2 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 742ns | -83.2% | -11.3% | -83.3% | -21.8% | -83.1% | -53.5% | -54.3% | -83.1% | -83.2% | -34.1% | -83.5% |
| 2 | 675ns | -77.5% | -0.3% | -80.2% | -12.9% | -79.6% | -51.8% | -46.1% | -79.6% | -79.5% | -25.1% | -81.4% |
| 3 | 614ns | -81.4% | -9.0% | -79.8% | -25.3% | -83.0% | -51.6% | -53.8% | -80.9% | -80.4% | -17.3% | -54.7% |
| 4 | 632ns | -80.6% | +2.8% | -80.6% | -13.4% | -79.9% | -42.8% | -46.1% | -80.5% | -78.6% | -15.8% | -80.4% |
| 5 | 634ns | -80.7% | +12.0% | -80.4% | +1.5% | -80.7% | -44.8% | -42.0% | -79.1% | -78.6% | -11.1% | -81.9% |
| 6 | 550ns | -77.6% | -4.2% | -81.0% | -5.6% | -80.1% | -43.1% | -42.7% | -80.0% | -80.4% | -16.6% | -80.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sd_bintree_int_skew2 | 0.167 | ok |
| sd_bintree_nat_skew2 | -0.346 | moderate- |
| sd_chain_int_skew2 | -0.391 | moderate- |
| sd_chain_nat_skew2 | 0.018 | ok |
| sd_chain_rev_int_skew2 | -0.270 | moderate- |
| sd_chain_rev_nat_skew2 | -0.424 | moderate- |
| sd_evalall_int_skew2 | -0.234 | moderate- |
| sd_evalall_nat_skew2 | -0.424 | moderate- |
| sd_jumptable_int_skew2 | -0.384 | moderate- |
| sd_jumptable_nat_skew2 | -0.363 | moderate- |
| sd_profiled_int_skew2 | -0.225 | moderate- |
| sd_profiled_nat_skew2 | -0.143 | ok |

**Consistency summary:**

- **sd_bintree_nat_skew2**: won 6/6, lost 0/6
- **sd_chain_int_skew2**: won 4/6, lost 2/6
- **sd_chain_nat_skew2**: won 6/6, lost 0/6
- **sd_chain_rev_int_skew2**: won 5/6, lost 1/6
- **sd_chain_rev_nat_skew2**: won 6/6, lost 0/6
- **sd_evalall_int_skew2**: won 6/6, lost 0/6
- **sd_evalall_nat_skew2**: won 6/6, lost 0/6
- **sd_jumptable_int_skew2**: won 6/6, lost 0/6
- **sd_jumptable_nat_skew2**: won 6/6, lost 0/6
- **sd_profiled_int_skew2**: won 6/6, lost 0/6
- **sd_profiled_nat_skew2**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sd_bintree_int_skew2 | 3.5ns | 641.1ns | 0.6% |  |
| sd_bintree_nat_skew2 | 4.3ns | 126.5ns | 3.4% |  |
| sd_chain_int_skew2 | 4.5ns | 629.4ns | 0.7% |  |
| sd_chain_nat_skew2 | 4.3ns | 122.2ns | 3.5% |  |
| sd_chain_rev_int_skew2 | 4.5ns | 556.0ns | 0.8% |  |
| sd_chain_rev_nat_skew2 | 3.1ns | 121.1ns | 2.6% |  |
| sd_evalall_int_skew2 | 4.5ns | 332.1ns | 1.3% |  |
| sd_evalall_nat_skew2 | 3.3ns | 335.1ns | 1.0% |  |
| sd_jumptable_int_skew2 | 3.2ns | 124.4ns | 2.6% |  |
| sd_jumptable_nat_skew2 | 3.9ns | 126.9ns | 3.1% |  |
| sd_profiled_int_skew2 | 5.1ns | 509.4ns | 1.0% |  |
| sd_profiled_nat_skew2 | 4.9ns | 145.1ns | 3.4% |  |

## Distribution (algo ns)

```
sd_bintree_int_skew2 (n=6, range 549.6-708.5 ns)
    549.6 |####################
    557.5 |
    565.5 |
    573.4 |
    581.4 |
    589.3 |
    597.3 |
    605.2 |
    613.2 |####################
    621.1 |
    629.1 |########################################
    637.0 |
    645.0 |
    652.9 |
    660.9 |
    668.8 |####################
    676.8 |
    684.7 |
    692.7 |
    700.6 |
  (0 below, 1 above range)

sd_bintree_nat_skew2 (n=6, range 114.2-138.1 ns)
    114.2 |####################
    115.4 |
    116.6 |
    117.8 |
    119.0 |
    120.2 |
    121.4 |####################
    122.6 |########################################
    123.8 |####################
    125.0 |
    126.2 |
    127.4 |
    128.6 |
    129.8 |
    131.0 |
    132.2 |
    133.4 |
    134.6 |
    135.8 |
    137.0 |
  (0 below, 1 above range)

sd_chain_int_skew2 (n=6, range 526.7-691.2 ns)
    526.7 |########################################
    534.9 |
    543.2 |
    551.4 |########################################
    559.6 |
    567.8 |
    576.1 |
    584.3 |
    592.5 |
    600.7 |
    609.0 |
    617.2 |
    625.4 |
    633.7 |
    641.9 |
    650.1 |########################################
    658.3 |########################################
    666.6 |########################################
    674.8 |
    683.0 |
  (0 below, 1 above range)

sd_chain_nat_skew2 (n=6, range 104.6-128.8 ns)
    104.6 |####################
    105.8 |
    107.0 |
    108.2 |
    109.4 |
    110.6 |
    111.8 |
    113.1 |
    114.3 |
    115.5 |
    116.7 |
    117.9 |
    119.1 |
    120.3 |
    121.5 |
    122.7 |########################################
    123.9 |########################################
    125.1 |
    126.3 |
    127.5 |
  (0 below, 1 above range)

sd_chain_rev_int_skew2 (n=6, range 458.3-615.4 ns)
    458.3 |########################################
    466.2 |
    474.0 |
    481.9 |
    489.7 |
    497.6 |
    505.4 |
    513.3 |########################################
    521.1 |
    529.0 |
    536.9 |
    544.7 |########################################
    552.6 |
    560.4 |
    568.3 |
    576.1 |########################################
    584.0 |########################################
    591.8 |
    599.7 |
    607.5 |
  (0 below, 1 above range)

sd_chain_rev_nat_skew2 (n=6, range 104.2-132.3 ns)
    104.2 |########################################
    105.6 |
    107.0 |
    108.4 |########################################
    109.8 |
    111.2 |
    112.6 |
    114.0 |
    115.4 |
    116.8 |
    118.2 |
    119.7 |
    121.1 |
    122.5 |########################################
    123.9 |
    125.3 |########################################
    126.7 |########################################
    128.1 |
    129.5 |
    130.9 |
  (0 below, 1 above range)

sd_evalall_int_skew2 (n=6, range 297.1-355.9 ns)
    297.1 |########################################
    300.0 |
    303.0 |
    305.9 |
    308.9 |
    311.8 |########################################
    314.7 |
    317.7 |
    320.6 |
    323.5 |########################################
    326.5 |
    329.4 |
    332.4 |
    335.3 |
    338.2 |
    341.2 |
    344.1 |########################################
    347.0 |
    350.0 |########################################
    352.9 |
  (0 below, 1 above range)

sd_evalall_nat_skew2 (n=6, range 283.3-365.6 ns)
    283.3 |########################################
    287.4 |
    291.5 |
    295.7 |
    299.8 |
    303.9 |
    308.0 |
    312.1 |########################################
    316.2 |
    320.4 |
    324.5 |
    328.6 |
    332.7 |
    336.8 |########################################
    340.9 |########################################
    345.1 |
    349.2 |
    353.3 |
    357.4 |
    361.5 |########################################
  (0 below, 1 above range)

sd_jumptable_int_skew2 (n=6, range 110.0-135.0 ns)
    110.0 |########################################
    111.2 |
    112.5 |
    113.8 |
    115.0 |
    116.2 |
    117.5 |########################################
    118.8 |
    120.0 |
    121.2 |
    122.5 |########################################
    123.8 |
    125.0 |########################################
    126.2 |
    127.5 |
    128.8 |
    130.0 |
    131.2 |
    132.5 |########################################
    133.8 |
  (0 below, 1 above range)

sd_jumptable_nat_skew2 (n=6, range 107.5-136.9 ns)
    107.5 |####################
    109.0 |
    110.4 |
    111.9 |
    113.4 |
    114.8 |
    116.3 |
    117.8 |
    119.2 |####################
    120.7 |
    122.2 |
    123.6 |####################
    125.1 |
    126.6 |
    128.0 |
    129.5 |
    131.0 |
    132.4 |
    133.9 |
    135.4 |########################################
  (0 below, 1 above range)

sd_profiled_int_skew2 (n=6, range 458.3-547.9 ns)
    458.3 |####################
    462.8 |
    467.3 |
    471.7 |
    476.2 |
    480.7 |
    485.2 |####################
    489.7 |
    494.1 |
    498.6 |
    503.1 |########################################
    507.6 |
    512.1 |
    516.5 |
    521.0 |
    525.5 |
    530.0 |####################
    534.5 |
    538.9 |
    543.4 |
  (0 below, 1 above range)

sd_profiled_nat_skew2 (n=6, range 105.0-201.8 ns)
    105.0 |####################
    109.8 |
    114.7 |####################
    119.5 |########################################
    124.4 |####################
    129.2 |
    134.1 |
    138.9 |
    143.7 |
    148.6 |
    153.4 |
    158.3 |
    163.1 |
    168.0 |
    172.8 |
    177.6 |
    182.5 |
    187.3 |
    192.2 |
    197.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **sd_profiled_nat_skew2**: CV=41.2% (high variance, measurements may be unstable)
