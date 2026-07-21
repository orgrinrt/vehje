# Interner intern hot path (harness): FNV vs FxHash x load factor 25% vs 75%


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

## Measured results

Ratio to baseline (intern_fnv_lf25), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | intern_fnv_lf25 (base) | intern_fnv_lf75 (ratio) | intern_fx_lf25 (ratio) | intern_fx_lf75 (ratio) |
|---|---|---|---|---|
| 64 | 381 ns | 1.05x | 1.05x | 1.08x |
| 256 | 1848 ns | 1.01x | 1.07x | 1.15x |
| 1024 | 10656 ns | 0.79x | 1.05x | 0.86x |
| 4096 | 67137 ns | 0.88x | 1.06x | 0.86x |
| 16384 | 284778 ns | 1.00x | 1.04x | 1.02x |

## Cost-model sanity line

At n=16384, the baseline (intern_fnv_lf25) median is 284778 ns for ~16384 interns per call, a 3..11-byte hash + probe + byte-compare each. Treating n as the work-item count, that is 17.38 ns/item, about 55.6 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass (id is first-appearance order, independent of hash and load factor).

## Verdict

A statistical dead heat at scale (4% spread at n=16384, all four within noise). Hash choice (FNV vs FxHash) and load factor (25% vs 75%) are both second-order: the cost is the unavoidable string hash on insert plus a byte compare on a hit, not the table mechanics. Pick a simple byte-walking hash and a high, memory-cheap load factor. Confirms the expected shape.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).
