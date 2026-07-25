# Runtime string interning: should a constructed string be interned by default?

**Date:** 2026-07-25
**Type:** harness cell. Three Rust cdylib variants per regime, three regimes, per-variant subprocess isolation,
cross-validated byte-identical output, 6 passes x 100 runs, 5 sizes. Generator `gen_string_interning.py`.
Artifacts: `results/str_compare_{light,heavy,equal}/`, `.bench_history/str_compare_*.tsv`.
**Settles:** the second mechanism the optimisation mandate named, automatic interning of strings, for the
runtime population as distinct from the compile-side one.

## Why this cell, given the compile-side number already exists

`interner-intern-hotpath` measured the compile-side interner at about 19 ns per token and found it
string-hash-bound and not a bottleneck. That population is small, heavily repeated, and fully known before
anything runs, so the number says interning identifiers during lexing is affordable. It does not say interning
a running program's strings is a good idea, because those are a different population: built by concatenation
and interpolation, frequently unique, frequently short-lived. `dedup-vs-reuse` refused to let its record
verdict transfer here for the same reason in the other direction, since a short string is much cheaper to hash
than a sixteen-field record and is compared more often. So the question needed its own cell.

## Strategies and regimes

All three strategies use the identical string handle (a span in a byte arena plus a memoised id), so the
comparison is between policies rather than between representations. The harness cross-validated that all
three produce byte-identical output, which is what proves the id comparison and the byte comparison agree.

- **plain**: copy the bytes, compare bytes. Never interned.
- **eager**: copy the bytes, then hash and intern immediately. Every construction pays the hash whether or
  not a comparison ever follows.
- **lazy**: copy the bytes and leave the string un-interned; intern on first comparison and memoise the id.
  Construction stays free and only compared strings pay. This is the option a design that jumped straight to
  "intern everything" would never have considered, which is the reason it is in the field.

Strings are 21 bytes, word-like, from a duplicate-rich population. Three regimes:

- `str_compare_light`: 1 comparison per construction. The templating norm.
- `str_compare_heavy`: 8 comparisons per construction, strings usually differ.
- `str_compare_equal`: 8 comparisons per construction, compared strings are usually **equal**, so the byte
  comparison cannot exit on the first differing byte. This regime exists because the first two flatter plain
  unfairly, and it is interning's genuine best case.

## Results (ns, lower is better)

| regime | n | plain | eager | lazy |
|---|---|---|---|---|
| light (1 compare) | 4096 | **38.78 us** (1.00x) | 94.04 us (2.42x) | 93.48 us (2.41x) |
| light (1 compare) | 16384 | **158.57 us** (1.00x) | 353.23 us (2.23x) | 356.16 us (2.25x) |
| heavy (8 compares) | 4096 | **33.97 us** (1.00x) | 89.97 us (2.65x) | 114.83 us (3.38x) |
| heavy (8 compares) | 16384 | **138.46 us** (1.00x) | 354.04 us (2.56x) | 458.84 us (3.31x) |
| equal (8 equal compares) | 4096 | **28.59 us** (1.00x) | 86.05 us (3.01x) | 109.95 us (3.85x) |
| equal (8 equal compares) | 16384 | **110.37 us** (1.00x) | 337.29 us (3.06x) | 423.64 us (3.84x) |

## The finding: do not intern constructed strings by default, in any of the three regimes

Plain wins at every size in every regime, by 2.2x to 3.1x over eager interning and 2.2x to 3.9x over lazy.
The verdict does not depend on the comparison count and does not depend on whether the compared strings are
equal.

The mechanism is that hashing is intrinsically more expensive per byte than comparing. A 21-byte comparison
is a couple of wide loads and a compare; a 21-byte hash is a chain of dependent multiplies, one per byte, and
it is followed by a table probe. So interning does not trade a cheap operation for a cheaper one, it trades a
cheap operation for an expensive one and hopes repetition amortises it. Within the equal regime the interning
overhead is about 13.9 ns per construction, and eight full 21-byte comparisons cost less than that, so the
break-even is somewhere above eight comparisons per construction.

**The bias correction is the load-bearing part of this cell and is worth reading as method rather than
result.** The first two regimes let the byte comparison exit on the first differing byte, which flatters
plain enormously and would have produced a confident verdict from a workload that was not testing the thing
it claimed to test. The third regime was added specifically to remove that advantage, and it is the regime
where interning should look best. It narrowed the gap slightly and left the verdict intact. A conclusion that
survives the correction designed to overturn it is worth more than the same conclusion asserted twice.

**Lazy is worse than eager wherever comparisons are frequent**, which is not obvious and is worth recording.
Deferring the hash does not avoid it once a comparison arrives; it only adds a check on every comparison and
splits the work across a colder path. Lazy matches eager in the light regime, where most strings are never
compared and the deferral genuinely pays, and loses by 20% to 30% in both heavy regimes. So lazy is not a
strictly better eager, and the compare-light case is the only place it has an argument.

## Design impact

Strings are values the runtime hands around, compares, and drops; they are not identifiers. Leave a
constructed string as bytes in the arena. A string literal is already a zero-copy slice into the residual and
costs nothing to produce, which remains the right shape and is untouched by this result.

Together with `dedup-vs-reuse`, both of the mandate's interning candidates are now refused for the runtime
value domain, while the reclaim-on-scope-exit candidate is confirmed by two cells. That is a clean split and
it was not the expected one: the intuition that interning is broadly beneficial held for the compile side, and
did not survive contact with the runtime population.

Interning stays the right answer where it is already used: compile-side identifiers, field-name keys, and
family ids, where the population is bounded, the values are compared constantly, and the hash is paid once
during lexing rather than once per construction in a hot loop.

## Boundary

String length is fixed at 21 bytes. Both hashing and comparing are linear in length, so the ratio between
them is roughly length-invariant and the verdict should hold across lengths, but that is an argument rather
than a measurement and a length sweep would settle it.

The comparison-count crossover is bracketed, not located. Interning loses at 8 comparisons per construction
and must win eventually, since its comparison is constant-time; where exactly is unmeasured and would need a
sweep over the comparison count. A workload that compares one string many tens of times per construction is
the regime this cell does not cover.

Memory is not measured. Interning deduplicates a repeated string population and this cell times only, so the
memory argument for interning stands untested here.

The hash is FNV walking one byte at a time, which is what the compile-side interner cell found adequate and
found second-order there. A wide or vectorised hash would narrow the gap; whether it narrows it by the 2x to
3x needed to change the verdict is unmeasured, and is the single most likely way this result could move.
