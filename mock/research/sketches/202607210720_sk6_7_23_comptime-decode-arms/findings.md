# SK6 / SK7 / SK23 findings: byte-shift decode, panic-free path, differential gate

**Date:** 2026-07-21 | **Outcome:** WORKS (all three) | Zig 0.16.0 | artifact `arms.zig`

- **SK6 byte-shift decode (not pointer overlay):** reading a u32 LE from a deliberately UNALIGNED offset (1) via
  `b[0] | b[1]<<8 | ...` yields 0x12345678 correctly. This is the decode discipline for `@embedFile`-style bytes
  whose alignment is not guaranteed (complements SK20's finding that zero-copy overlay needs aligned regions:
  where alignment cannot be guaranteed, byte-shift decode is the fallback, and it works). Settles the 1845
  byte-shift-not-overlay constraint.
- **SK7 panic-free untrusted path:** `decodeChecked` returns an error union (`error.TruncatedInput`,
  `error.OutOfRange`) on bad input, never a panic. This is mandatory because a Zig panic in a C-ABI-exported
  function aborts the host process; the untrusted path must thread errors, not panic. Confirmed: truncated and
  out-of-range inputs both surface as errors, no panic. Settles the 1845 panic-free-untrusted-path constraint.
- **SK23 differential-testing gate:** the same kernel run at the COMPTIME locus and the RUNTIME locus produces
  identical output (both 0xEF693FF0). This is the grade-0 shadow / the 1845 differential gate: the kernel at both
  binding times agrees. It is the operational assurance that the two loci compute the same function (defence in
  depth over the SK1 by-construction certification).

## Design impact
- The untrusted decode is byte-shift (SK6) + error-union (SK7): no alignment assumption, no panic across the ABI.
- The differential gate (SK23) is a real, cheap check the kernel-at-both-loci agrees. Combined with the SK1
  by-construction certification and the return-to-canonical single engine, it is defence-in-depth, not the sole
  assurance.
