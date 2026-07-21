# Wire-format versioning + proof-compatibility of the residual

**Date:** 2026-07-21
**Outcome:** WORKS. Zig 0.16.0, aarch64. Artifact: `version.zig`.
**Settles:** how the compile-to-runtime residual (the two-artifact contract) versions safely as both artifacts
evolve independently, including the vehje-specific proof-compatibility axis that a plain wire-format version does
not capture.

## Why this probe

The residual is the wire format between the two artifacts: the Rust compiler emits it, the Zig runtime executes
it. Both evolve independently. A consumer ships a runtime built at some version; residuals arrive built by
compilers at various versions (a mod compiled by an older toolchain, a script compiled by a newer one). The
runtime must decide, per residual, whether it can safely execute it. The naive answer (a format version number)
misses the load-bearing case for a certified-generation system, so this sketch works out both axes.

## The two compatibility axes

Version compatibility splits into two genuinely distinct questions:

1. **Format compatibility: can this runtime parse this residual's byte layout?** Standard wire-format evolution: a
   `format_major` for breaking layout changes (record widths, section structure) that requires an exact match, and
   a `format_minor` for additive changes (new optional sections) where the runtime accepts and skips what it does
   not know.

2. **Proof / contract compatibility (the vehje-specific axis): was this residual's proof discharged against MY
   contract?** This is the one a plain format version misses. The residual carries an effect-inclusion proof (the
   certified-generation guarantee: every construct's effect is within the target's permits), and that proof was
   discharged against a specific target permit set, a specific effect-family vocabulary (the family-to-bit
   numbering), and a specific ABI. A runtime built for a different permit set or family numbering must REJECT the
   residual even if the bytes parse perfectly, because the proof is meaningless against a different contract: an
   effect that was "family 5, permitted" in the compiler's numbering might be "family 5, forbidden" or "a
   different family entirely" in the runtime's. The proof only certifies anything relative to the contract it was
   checked against.

The sketch captures the contract as a `contract_hash` in the header: a hash of (target permit set + family
vocabulary + ABI version). The runtime compares it to its own and rejects on mismatch.

## What was demonstrated

A versioned residual header (`magic`, `format_major`, `format_minor`, `contract_hash`, `abi_version`,
`section_count`) plus a compatibility check covering both axes, plus a forward-compatible TLV section walk. All
scenarios resolve correctly:

- **Same contract, newer format_minor:** accept (forward-compatible; the runtime processes what it knows).
- **Breaking format_major bump:** reject (exact major match required).
- **Different contract (permit set / family vocab / ABI differs):** reject_contract (the proof was discharged
  against a different contract, so it is not valid here, even though the format parses).
- **ABI mismatch:** reject_abi.
- **Bad magic (not a residual):** reject_magic.
- **Forward-compat section walk:** a residual from a newer compiler carries a section this runtime does not know
  (tag 99); the runtime skips it via its TLV length and processes the 3 known sections. 3 processed, 1 skipped,
  correct.

## Design impact

- The residual header carries both a format version (major = breaking, minor = additive) and a `contract_hash`
  (the proof's discharge contract). The runtime's compatibility check is: magic, then exact format_major, then
  ABI, then contract_hash; only then does it trust the proof and execute.
- **The contract_hash is the load-bearing versioning mechanism for certified generation.** It ties the residual's
  proof to the exact (permit set + family vocabulary + ABI) it was discharged against, so a runtime cannot
  accidentally execute a residual whose proof means something different in its own numbering. This is what makes
  the two-artifact split safe to evolve: the effect proof is only trusted when the contract matches, and the check
  is a single u64 compare.
- Sections are TLV (tag, length, bytes), so a newer compiler can add sections that older runtimes skip cleanly
  (forward compatibility), and format_minor signals their presence. Breaking layout changes bump format_major and
  are rejected outright by older runtimes (fail-closed, no silent misparse).
- This composes with the value-arena/transport benches (the sections carry the core IR, effect proof, string
  blob) and the effect-lattice bench (the contract_hash covers the family vocabulary that the effect bits are
  numbered against, so the inclusion proof's bit numbering is pinned to the contract).

## Boundary

The contract_hash covers permit set, family vocabulary, and ABI; if the design adds other proof-relevant contract
elements (a numeric-domain version for tnum, a capability set), they fold into the same hash. The sketch does not
implement proof RE-checking (a runtime that wanted to accept a different-contract residual could re-run the
inclusion check against its own permits, an optional adapter path); it rejects, which is the safe default. Version
negotiation (a runtime fetching a compatible residual variant) is a distribution concern above this check. The
magic/version scheme is standard; the vehje-specific contribution is the contract_hash axis, which a format
version alone does not provide.

## Artifacts
- `version.zig` (the versioned header, the two-axis compatibility check, and the forward-compat TLV section walk).
