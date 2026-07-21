# Interner intern hot path (harness): FNV vs FxHash x load factor 25% vs 75%

Scaffold. The main agent fills the medians and the cost-model line after the serialized bench run.

## What this measures

The intern hot path: insert-or-get a string, return its stable u32 id. Runs on every identifier/literal token
during lex/parse, and interned ids are the prerequisite for resolve, Project field access, and effect family
ids. Four variants over an identical token stream (4096 distinct 3..11-byte identifiers, access 70% weighted to
the top-64 common tokens, the parse reality), crossing two axes:

- hash: FNV-1a (`intern_fnv_*`) vs an FxHash-style multiply-rotate (`intern_fx_*`). Both byte-walking.
- load factor: ~25% (CAP 16384, `*_lf25`) vs ~75% (CAP 5462, `*_lf75`).

Baseline = `intern_fnv_lf25` (mode = subtract). The id a token receives is its first-appearance order in the
stream, which is independent of hash and load factor, so all four variants produce byte-identical output; the
harness cross-validates that (offline check confirmed identical accumulators across all four).

## The audit defect this fixes

The standalone `interner-intern-hotpath/intern.zig` ran outside the harness with a hand-timed 20M loop and a
header-only CSV, and its "~75%" load-factor label was actually ~100% (CAP == DISTINCT). Fixed here: the corpus
and access stream are built once via `OnceLock` (not const-foldable) outside the timed region; the accessed
token index is folded with the FFI input byte so the intern cannot be hoisted; the load factors are asserted
in-code to their real bands (25% and 75%); and the hit path does a real byte compare against the corpus bytes
(insert path does a real byte hash), so the measured cost is the genuine string-op-bound intern, not an
integer-keyed table. The table scratch arrays are allocated and reset outside the timed region (matching the
original, which memset before its timer), so only the intern loop is measured.

## Result (fill after run)

Medians, normalised against `intern_fnv_lf25` (mode = subtract):

| n | fnv_lf25 (base) | fx_lf25 | fnv_lf75 | fx_lf75 |
|---|---|---|---|---|
| 64 | | | | |
| 256 | | | | |
| 1024 | | | | |
| 4096 | | | | |
| 16384 | | | | |

Expected shape (to confirm or refute): all four within a few percent of one another (string-hash-bound; hash
choice and load factor second-order). The prior standalone measured ~18-20 ns/intern across the board.

## The finding (fill after run)

Cross-validation: [pass/fail]. The interner is [not] a bottleneck; hash choice (FNV vs FxHash) is [second-order]
and load factor (25% vs 75%) is [second-order], because the cost is the unavoidable string hash on insert plus
the byte compare on a hit, not the table mechanics. Pick a simple byte-walking hash and a high (memory-cheap)
load factor.

## Cost-model sanity line (fill after run)

At n=16384, [median] for 16384 interns is [ns/intern], about [N] cycles at ~3.2 GHz for a [len]-byte hash plus a
probe plus a byte compare on a hit (70% of accesses are first-probe hits on the common tokens). A 500K-token
program interns in ~[X] ms, a fraction of the compile budget.

## Boundary

3..11-byte tokens (typical identifiers); longer string literals hash and compare proportionally slower but are
re-interned less often. The 70%-common-token access is the parse reality; a pathological all-distinct workload
would expose the load factor more. The high-load cell uses a non-power-of-two CAP (5462) with `% CONST`, which
LLVM strength-reduces to a reciprocal multiply, so both load cells share the probe arithmetic and the
comparison isolates the load factor.
