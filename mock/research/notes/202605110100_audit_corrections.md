# Round 202605110100 — post-lock audit corrections

Recorded 2026-05-10 after the senior reviewer (pr-reviewer-senior agent)
flagged two discipline issues on PR #32. The src CL is locked, so this
note carries the corrections for the audit trail per
`cl-claim-sketch-discipline.md`.

## Correction 1: bump_byte count arithmetic in src CL

**The src CL `## CHANGE: explicitly discard Maybe returns from
`Cursor::bump*` in vehje-lex` block claims**:

> totals 18 matches across the two files (3 lexer + 15 tokenizers)

**Actual**: 3 lexer + 12 tokenizers = **15 matches**, not 18.

Source counts at lock commit (`84aaf2c`):

```
$ grep -c "let _ = .*bump_byte()" mock/crates/vehje-lex/src/lexer.rs
3
$ grep -c "let _ = .*bump_byte()" mock/crates/vehje-lex/src/tokenizers.rs
12
```

The semantic claim (every bare `c.bump_byte();` discard became
`let _ = c.bump_byte();`) is correct; only the arithmetic in the
verification grep summary is off.

The topic file states the right number (12 in tokenizers); the locked
src CL drifted at that one line. Mockspace #318 (the structured-CL
verifier lint) would catch this automatically once it ships; until
then, the discipline rule expects the audit trail to log the
discrepancy explicitly.

## Correction 2: forward-promised task ID in topic file

**The topic file claims**:

> Test rehab tracked separately as task #412 (to be created on the
> hilavitkutin-side cron iteration, sibling to #399).

**Actual at lock time**: task #412 did not exist; it was a
forward-promise that risked dropping. The reviewer correctly noted
that workspace discipline expects task IDs cited in CLs to exist at
lock time.

**Resolution**: task #412 has now been created with the test-rehab
scope. The topic file's claim is now honest in retrospect; future
references to "#412" point to the real task. Not retroactively
edited (locked artefact); logged here.

## Why not deprecate-and-relock the round

The two issues are low-stakes audit-trail noise. Deprecating and
re-locking the round to fix the arithmetic and forward-task reference
would introduce more audit-trail noise than the corrections themselves
solve. The mockspace deprecation flow is designed for design-intent
errors, not for one-line miscount fixes after lock. Logging here
preserves the original locked record as-is and adds the correction in
a discoverable location (`mock/research/notes/`).

## Recorded

2026-05-10. Senior reviewer (`pr-reviewer-senior`) report ID
`a8df7a2c33e3328eb` for round 202605110100 PR #32. Correction logged
post-merge to honor the audit-trail discipline without forcing a
ceremony round.
