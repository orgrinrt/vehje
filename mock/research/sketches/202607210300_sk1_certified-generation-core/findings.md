# SK1 findings: the certified-generation core holds (the architecture's crux)

**Date:** 2026-07-21
**Outcome:** WORKS. The two-artifact certified-generation model is real on the pinned toolchain.
**Settles:** the crux of the whole architecture (topics 1627, 1845): Rust emits validated data, Zig comptime
specialises a hand-authored engine to it, illegal programs fail to lower by construction, both certifications at
our build, neither at the consumer.
**Toolchain:** Rust nightly-2026-05-28 (emitter), Zig 0.16.0 (engine).

## What was built and confirmed

- **Certification 1 (rustc-typed data emitter), `emit_data.rs`.** A Rust program holds a language definition
  (families with ids, a target's supported-family mask), validates it (unique ids, id < 64), and emits pure const
  DATA to `gen/families_data.zig`. It emits DATA, not Zig logic, which is the canonical shape (1627 rejected
  "Rust metaprogram emitting Zig source"). A validation failure aborts before emitting, so a well-typed run only
  emits consistent data.
- **Certification 2 (Zig-typed specialisation), `engine.zig`.** A hand-authored comptime engine `@import`s the
  emitted data and specialises to it, generating from `data.FAMILIES`, nothing language-specific by hand:
  - **Illegal-states-unrepresentable:** the family tag enum is generated from the data. Referencing a family the
    language does not define (`FamilyTag.Witcher`) is a compile error ("enum has no member named 'Witcher'"), so
    a program using an out-of-language family cannot even be written into the IR.
  - **Total dispatch:** an exhaustive `switch` over the generated enum; a missing arm on an exhaustive enum is a
    Zig compile error, so total-over-declared-families is compile-checked with no runtime test.
  - **Comptime-folded inclusion check:** `script_mask & ~target_mask == 0`. For a bundled (comptime-known)
    script it folds; an unsupported family makes it `@compileError` ("script uses a family the target does not
    support"), confirmed by `demo_bad_inclusion.zig` failing to compile. For an arriving script only the target
    mask is comptime, leaving one runtime bitop (`checkIncludedRuntime`), confirmed returning true/false
    correctly (Doc-only true, Clausewitz false). One source, stratified by the compiler, exactly 1845's
    strongest single idea.
- **Control:** the valid engine and a valid bundled program compile and run correctly.

So: bundled scripts are proven at our build (an illegal one fails to compile); arriving scripts get a
by-construction structural rejection at load (an out-of-set family has nowhere to go) plus one runtime bitop for
the inclusion mask; the consumer carries no rustc, no typestate, only the doubly-certified binary. The guarantee
holds by the same logic a compiled Rust binary is trusted: the thing that produced it was proven correct.

## Load-bearing toolchain finding: `@Type` is gone in 0.16, replaced by specialised builtins

Topics 1627 and 1845 name comptime `@Type` reification as the mechanism that generates the IR type from data
("Zig's comptime `@Type` reification, the thing only comptime can do"). **Zig 0.16 removed the monolithic `@Type`
builtin.** The full 0.16 builtin set has no `@Type` and no `Reify`; reification is now a family of specialised
builtins: `@Enum`, `@Union`, `@Struct`, `@Int`, `@Fn`, `@Pointer`, `@Frame`, `@Tuple`, `@Vector`, `@EnumLiteral`.
The capability survives (this sketch uses `@Enum(u8, .exhaustive, names, values)` and it works), so cert-gen is
not blocked, but the design record's "`@Type`" references are stale and should be updated to the 0.16
specialised-builtin API. Signatures observed in std 0.16: `@Enum(tag_type, mode, names, values)`,
`@Union(layout, tag_type, names, types, attrs)`, `@Struct(layout, backing, names, types, attrs)`.

## Design impact

- The certified-generation architecture is validated end to end on the pinned toolchain. The two-artifact model
  (Rust emits data, Zig comptime specialises, both certified at our build) is buildable.
- Combined with SK5, the comptime engine is written iteratively (the family-table loop here is shallow and safe,
  but any deep IR fold inside the engine must be the explicit work-stack from SK5).
- Update the design docs' `@Type` references to the `@Enum`/`@Union`/`@Struct` family (a stale-API fix, tracked
  for the design-proper pass).

## Artifacts
- `emit_data.rs` (cert-1), `gen/families_data.zig` (the emitted data)
- `engine.zig` (cert-2: the specialising engine)
- `demo_bad_inclusion.zig` (bundled unsupported family -> @compileError), `demo_bad_family.zig` (undefined family
  unrepresentable)
