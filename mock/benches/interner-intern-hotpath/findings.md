# Interner intern hot path: is the single-string insert-or-get a bottleneck?

**Date:** 2026-07-21
**Type:** Zig-native bench, ReleaseFast, 20M interns, 4096 distinct ids with a 70%-common-token (Zipf-ish) access
pattern, load-factor and hash sweep, 5-run best. Zig 0.16.0, aarch64. Artifacts: `intern.zig`, `intern.csv`.
**Settles:** the cost of the incremental single-string intern (insert-or-get a string, return its stable u32 id),
which runs on every identifier and literal during lex/parse. The interner-merge bench sized the cross-mod dedup;
this sizes the per-token intern and settles the second-order knobs (hash, load factor).

## Why this probe

Interning runs on every identifier/literal token during lexing, and the resulting u32 ids are the prerequisite
for the resolve pass (name to binder), Project field access (field names), and the effect system (family ids). So
the intern hot path underlies most of the compile side. The question is whether it is a bottleneck and how much
its tuning (hash function, load factor) matters. The access pattern is parse-realistic: 70% of interned tokens hit
the ~64 most common identifiers/keywords (a heavily repeated small set), the rest spread over 4096 distinct names.

## Results

| load factor | FxHash | FNV |
|---|---|---|
| ~25% | 19.55 ns | 18.79 ns |
| ~50% | 19.67 ns | 18.20 ns |
| ~75% | 20.25 ns | 18.10 ns |

51 to 55 M interns/s across the board.

## The finding: intern is string-hash-bound (~19 ns), not a bottleneck, and its knobs are second-order

Interning costs ~19 ns per token, and it is dominated by the string operations (hashing the token bytes on the
way in, comparing bytes on a hit), not by the hash-table mechanics. Two consequences:

- **Load factor barely matters (25% to 75%: 18 to 20 ns).** Because 70% of interns hit the common tokens, which
  are interned early into low-collision slots and found on the first probe, the probe-length increase at higher
  load factor is hidden: the common case is a first-probe hit regardless. So the interner can run at a high load
  factor (75%) without penalty, saving memory, for the common-token-heavy parse workload.
- **Hash choice is second-order (FNV about equals FxHash, within noise, FNV marginally ahead for these short
  strings).** Both are byte-walking hashes over 3-to-11-byte tokens, so they cost about the same; the table
  mechanics do not dominate, so the hash quality difference does not show. Either is fine.

The practical conclusion: the interner is not a bottleneck and does not warrant tuning effort. At 51 to 55 M
interns/s, a program with 500K identifier tokens interns in ~10 ms, a fraction of the compile budget. The design
should pick a reasonable hash (FNV or FxHash) and a high load factor (memory-cheap, no speed penalty here) and
move on; the intern cost is the unavoidable string-hash-plus-compare, and no table cleverness reduces it because
the table mechanics are already not the cost.

## Design impact

- The interner uses open-addressing with a simple byte-walking hash (FNV or FxHash) at a high load factor (~75%);
  the choice does not matter, so pick for simplicity. Intern is ~19 ns/token, ~10 ms for a 500K-token program, not
  a bottleneck.
- The one real lever (not a table knob) is avoiding redundant interns: each distinct token occurrence is interned
  once and its id reused, and during incremental re-parse a source-position-to-id cache avoids re-hashing
  unchanged tokens (the mod-stack-load bench's warm path). The table itself is already as cheap as string-hashing
  allows.
- The lexer has the token bytes in hand (a slice into the source buffer), so it hashes in place with no extra
  indirection; the interned id is then the currency for resolve, field access, and effects, all of which compare
  u32 ids rather than strings (the reason those benches are integer-comparison-fast).

## Boundary

3-to-11-byte tokens (typical identifiers); longer string literals cost proportionally more to hash and compare
(the string-op-bound cost scales with length), but literals are less frequently re-interned. The 70%-common-token
access is the parse reality; a pathological all-distinct workload would expose the load factor more, but real
source has heavy token repetition (keywords, common identifiers). The bench includes a double indirection
(access index then name slice) that the real lexer does not have (it hashes the in-hand token bytes), so the true
lexer intern is slightly faster than the ~19 ns here; the conclusion (string-hash-bound, knobs second-order) holds
either way.

## Artifacts
- `intern.zig` (open-addressing interner, FxHash vs FNV, load-factor sweep, common-token-heavy access),
  `intern.csv`.
