# Branch lowering strategy map

Which strategy the vehje compiler should pick for a script-level branch, mapped by the branch's
PREDICTABILITY, its ARM COST, and (for multiway) its ARITY and key skew. Run through the mockspace
bench harness (`vehje-benches`), cross-validated (every strategy in a bench produces byte-identical
output). Arms are `#[inline(never)]` so `branch`/`chain` stay real branches (LLVM does not if-convert
a call), modelling the interpreter dispatching into a script subtree of cost W (W=1 cheap, W=24 heavy).

Rebuild/extend: `python3 gen_branch_variants.py && python3 gen_multiway_variants.py`, build the
`variants/*` cdylibs, `./target/release/vehje-benches`. Results promote to `results/<bench>/` on a
full successful run.

## Strategy legend (naming)

Two-way `if cond { A } else { B }`:

| name | what it does |
|---|---|
| `branch` | real conditional branch; evaluate only the selected arm. Mispredicts when the predictor is wrong. |
| `predicate` | evaluate BOTH arms, `csel`-select the result. Branchless; never mispredicts; always pays both arms. |
| `lut` | evaluate both arms, index a 2-entry array by the condition. Branchless. |
| `mask` | evaluate both arms, arithmetic mask-blend `(A & m) | (B & !m)`. Branchless. |
| `profiled_hot` | evaluate the HOT arm always; evaluate the cold arm only on the rare case (a predicted-not-taken branch). Needs a known hot arm. |

Multiway `if k==0 {A0} else if k==1 {A1} ... {A_{K-1}}`:

| name | what it does |
|---|---|
| `chain` | forward linear: test k==0, k==1, ... in order; evaluate the ONE selected arm. |
| `chain_rev` | backward linear: test k==K-1, ..., k==0; evaluate the ONE selected arm. |
| `jumptable` | `match k` lowered to a jump table: O(1) dispatch to the ONE selected arm. |
| `bintree` | binary search on k: log2(K) compares to the ONE selected arm. |
| `predicate_all` | evaluate ALL K arms, index-select the k-th. Branchless; pays K arms. |

Branch tier (orthogonal: interpret vs emit native):

| name | what it does |
|---|---|
| `interp_dispatch` | a bytecode interpreter dispatching the branch program per element via a switch. |
| `native_direct` | the kernel as compiled native code (what an optimizing backend / copy-and-patch steady-state emits). |
| `stencil_jit` | copy-and-patch: emit a native aarch64 branch loop into MAP_JIT memory once, then run it. |

## Two-way map (median ns, n=4096, Δ vs `branch`)

Cheap arms (W=1):

| situation | branch | predicate | lut | mask | profiled_hot |
|---|---|---|---|---|---|
| rand50 (unpred ~50%) | 14347 | **8051 (-44%)** | 8134 (-43%) | 9374 (-35%) | n/a |
| pred95 (~95% taken) | 6212 | 9267 (+49%) | 8557 (+38%) | 8526 (+37%) | **5824 (-6%)** |
| pred05 (~5% taken) | 6751 | 9033 (+34%) | 9687 (+43%) | 9340 (+38%) | **6120 (-9%)** |
| biased25 (~25%) | **8303** | 9059 (+9%) | 9035 (+9%) | 8822 (+6%) | 9520 (+15%) |
| runs (correlated) | **7387** | 9076 (+23%) | 9110 (+23%) | 9210 (+25%) | n/a |
| alt (alternating) | **4790** | 8272 (+73%) | 8691 (+81%) | 9899 (+107%) | n/a |

Heavy arms (W=24):

| situation | branch | predicate | profiled_hot |
|---|---|---|---|
| rand50 | **35425** | 67440 (+90%) | n/a |
| pred95 | **42725** | 71255 (+67%) | 44542 (+4%) |
| pred05 | 28778 | 73165 (+154%) | **28654 (-0.4%)** |
| biased25 | **33082** | 70388 (+113%) | 38753 (+17%) |
| runs | **37036** | 72618 (+96%) | n/a |
| alt | **34792** | 69679 (+100%) | n/a |

The tradeoff is one inequality: `branch` does one arm but pays a mispredict; branchless does both arms
but never mispredicts. Branchless wins only when `mispredict_rate x penalty > cost_of_second_arm`:

1. **Cheap arms + UNPREDICTABLE (rand50): branchless (predicate/lut), ~44% faster.** The only regime
   where the ~50% mispredict cost exceeds the second cheap arm. `predicate` and `lut` tie; `mask` is
   slightly worse (extra blend). Legal always because the language is pure-functional (both arms are
   side-effect-free).
2. **Cheap arms + PREDICTABLE (pred95/pred05/alt/runs/biased25): branch.** The predictor is free (alt
   is a perfectly-learned 2-cycle: branch 4790 vs branchless 8272, +73%; runs is a learned run-length),
   so computing the second arm is pure waste.
3. **Heavy arms + ANY pattern: branch, always.** Doing both heavy arms costs ~2x (predicate +67% to
   +154% everywhere, including rand50 where cheap branchless won), dwarfing any mispredict penalty.
   **Arm cost flips the rand50 verdict**: cheap+unpredictable is branchless, heavy+unpredictable is branch.
4. **profiled_hot for strongly one-sided branches (new strategy):** best or tied-best for pred95/pred05
   at both arm costs (does the hot arm always, the cold arm only on the rare predicted-not-taken case).
   Loses for biased25 (25% is not rare enough to keep the guard cheap). Pick it when a branch is
   heavily one-sided AND its hot arm is known (bias annotation or profile).

## Multiway map (median ns, n=4096, Δ vs the bench baseline)

Cheap arms:

| situation | chain | chain_rev | jumptable | bintree | predicate_all |
|---|---|---|---|---|---|
| mw3 uniform | 8061 (-4%) | **7713 (-8%)** | 7980 (-5%) | 8421 | 13739 (+63%) |
| mw3 skew→0 | 6105 (-3%) | 6046 (-4%) | 6062 (-3%) | 6274 | 13532 (+116%) |
| mw8 uniform | **5328 (-40%)** | 5369 (-40%) | 5407 (-40%) | 8944 | 32360 (+262%) |
| mw8 skew→0 | 9127 (-7%) | 7885 (-20%) | **6730 (-32%)** | 9843 | 31899 (+224%) |

Heavy arms (dispatch is noise vs the one heavy arm; `predicate_all` excluded, it would be K heavy arms):

| situation | chain | chain_rev | jumptable | bintree |
|---|---|---|---|---|
| mw3 uniform | **75323** | 76060 | 75987 | 75493 |
| mw8 uniform | 67462 | 73972 | 69668 | 79069 |
| mw8 skew→0 | **69934** | 78260 | 75786 | 79879 |

1. **`predicate_all` (evaluate all K arms) is catastrophic for K>2** (+63% at K=3 up to +262% at K=8):
   it pays every arm. It is only the right shape as the 2-way `predicate` (K=2). Never use it for
   multiway with more than 2 arms, and never for heavy arms.
2. **Few arms (K=3): all one-arm dispatch strategies tie** (~6-8k ns, within a few %). The branch count
   is too small to separate chain / jumptable / bintree. Use the simplest (`chain`).
3. **Many arms (K=8), uniform key: chain ≈ jumptable (~5350 ns), bintree worse (8944).** LLVM tends to
   lower a dense uniform `chain` to a jump table itself, so they converge; the explicit binary tree's
   log2 compares are not worth it here. Use `jumptable` (or let the chain lower to one).
4. **Many arms (K=8), skewed key: jumptable wins (6730, -32%).** O(1) dispatch beats the chain's
   variable compare count. Notably `chain_rev` (7885) BEATS `chain` (9127) here even though the skew is
   toward the FIRST arm: reverse order makes the K-1 leading compares all predictable-not-taken (cheap,
   the predictor nails them) before falling to the common arm, whereas forward `chain`'s first compare
   is the one unpredictable 70/30 branch. So for a chain, ordering the tests so they are predictable-not-
   taken can beat ordering the hot arm first; but a jump table beats both.
5. **Heavy arms: dispatch strategy is noise** (the single heavy arm dominates the ~70us). Use anything
   cheap; do not evaluate all arms.

This enriches the earlier match-lowering result (jump-table wins for many arms) with the branch-
prediction and arm-cost nuance, and adds the chain-ordering finding.

## Branch tier map (median ns; interpret vs native vs copy-and-patch)

One cross-validated branch kernel, three tiers:

| n | interp_dispatch | native_direct | stencil_jit |
|---|---|---|---|
| 64 | 755 (8.6x) | 90 (1.03x) | **88 (1.00x)** |
| 256 | 3270 (10.4x) | 390 (1.25x) | **313 (1.00x)** |
| 1024 | 14070 (12.2x) | 1620 (1.40x) | **1160 (1.00x)** |
| 4096 | 56200 (9.2x) | **6130 (1.00x)** | 9010 (1.47x) |
| 16384 | 202870 (9.0x) | **22600 (1.00x)** | 58390 (2.58x) |

Two load-bearing results:

1. **The interpreter dispatch overhead is large (~9x to 12x) for a naive bytecode interpreter, so the
   native tier (native_direct or stencil) is a big win over it.** This reconciles the earlier
   interp-vs-native-ceiling finding (~1.2x): that compared an OPTIMIZED register-VM to native; this
   compares a NAIVE per-op bytecode interpreter to native. **The JIT's value depends entirely on the
   interpreter's quality**: against a good register-VM the native win is ~1.2x, against a naive
   bytecode/tree-walker it is ~10x. So the design should first build a good interpreter (the ~1.2x
   floor); a JIT is only a ~10x win if the interpreter is naive.
2. **The scalar copy-and-patch stencil ties (or beats) compiled native at small n but LOSES to it at
   large n (1.47x at 4096, 2.58x at 16384).** My stencil emits a REAL scalar branch on the unpredictable
   condition, so it mispredicts at scale; `native_direct` (LLVM) if-converts the cheap branch to
   branchless and vectorizes, so it pulls ahead as n grows. This confirms and sharpens the earlier
   result: **copy-and-patch (scalar straight-line stencils) is beaten by an optimizing backend at scale,
   because it does not if-convert or vectorize.** The fix is the map itself: **copy-and-patch must apply
   the branch-strategy map when it emits.** For an unpredictable branch the stencil should emit the
   branchless (predicate/csel) shape, not a real branch; then it would match native. So the branch map
   feeds the stencil emitter, not just the interpreter.

## The synthesis for the compiler

For any script branch, the compiler picks along three orthogonal axes:

- **Tier** (interpret vs native): interpret is ~10x slower than native for a naive interpreter but only
  ~1.2x for a good register-VM; build the good interpreter first, reserve the native tier for hot code.
- **Strategy** (branch vs branchless vs profiled vs jump-table): from the maps above, driven by
  predictability, arm cost, arity, and key skew. This choice applies in BOTH tiers: the interpreter
  dispatches it, and the copy-and-patch stencil emits it.
- **Arm ordering** (for chains): order tests to be predictable-not-taken where possible; a jump table
  removes the ordering question for dense keys.

The inputs the compiler needs (predictability, arm cost, arity, skew) are all available at compile time:
arm cost is the evaluated subtree size, predictability/skew is a numeric range/bias fact (the tnum
bench), arity is structural.

## Next variants to add (extensible)

- An `adaptive` strategy: a per-branch runtime counter switching branch<->branchless as observed
  predictability drifts.
- A `simd_batch` strategy for correlated/streaming branches (partition a window by condition, apply each
  arm to its bucket), valid because the reduction is order-independent.
- Stencil variants that emit the branchless shape (to confirm they match native at scale, closing the
  large-n gap), and a stencil for the multiway jump-table.
- Larger arity (16-way, 32-way) and a mixed-skew key distribution.
