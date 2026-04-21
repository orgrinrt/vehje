# Linting Discipline

The compiler's linter framework is the core mechanism enforcing Tenet
8 (Clean Builds Or No Builds). Every category of bug that surfaces in
playtest should have been caught at compile time. When something slips
past, the fix includes a new lint — not just the bug fix.

This doc captures the workflow. Follow it every time.

## The rule

When a playtest bug is diagnosed and the root cause is something a
compile-time check could have detected, the commit that fixes the bug
MUST also include:

1. A new linter module (or extension of an existing one) that catches
   the same class of problem.
2. A positive fixture: input that MUST trigger the lint.
3. A negative fixture: input that MUST NOT trigger it.
4. If the lint surfaces pre-existing issues in external mods we can't
   fix: an entry in the external findings baseline with a comment
   explaining why the waiver is acceptable.

No exceptions. The lint catalog grows until playtest reveals nothing
new. Every lint we add makes the next build a little safer.

## Writing a new lint

Pick a scope based on what you need to see:

- **PatchLinter** — operates on one loaded Patch at a time. Use for
  input-defect detection: malformed argument shapes, references to
  unknown mods, broken macro names.
- **ItemLinter** — operates on one compiled Item + its CompileResult.
  Use for content-level checks on a single item: trigger nesting
  depth, deprecated syntax, bare tabs.
- **TreeLinter** — operates on the whole compiled tree at once. Use
  for cross-reference checks: undefined scripted_trigger usages,
  unused first-party definitions, LOC keys referenced but not defined.

Each lint is its own module under
`tools/megapatch/megapatch_compiler/linters/builtin/`. Minimal shape:

```python
from ..base import Finding, ItemLinter, LintContext, Ownership, Severity
from ...model import CompileResult, Item

class MyLint(ItemLinter):
    rule_id = "MY_RULE_ID"        # UPPER_SNAKE_CASE, globally unique
    severity = Severity.WARNING   # default severity for findings
    category = "correctness"      # "correctness" | "deprecation" | "style"

    def lint(self, item, result, ctx):
        if <condition that indicates bug>:
            yield Finding(
                rule_id=self.rule_id,
                severity=self.severity,
                ownership=<determine via ctx.resolve_ownership(...)>,
                message="<what's wrong, written for a human reader>",
                source_file="<file path if known>",
                source_line=<line if known>,
                directory=item.directory,
                item_id=item.item_id,
                category=self.category,
                suggested_fix="<one-line actionable hint>",
            )
```

The `discover()` function picks up the new module automatically —
no manual registration step. But there IS a test that asserts the
expected catalog matches reality: update the expected set in
`tests/unit/test_builtin_lints.py::TestBuiltinDiscovery::test_all_builtin_lints_discovered`.

## Writing fixtures

Every lint needs paired tests in
`tests/unit/test_builtin_lints.py`:

- `test_<lint>_positive_<case>` — input that must fire the lint.
- `test_<lint>_negative_<case>` — input that must NOT fire it.

Add edge cases where the lint could reasonably be wrong:

- Longer identifier that contains the tokens the lint matches (e.g.
  for `PREVPREV_IN_ACTIVE_CODE`, a variable named `prevprevious` must
  not match).
- The token appearing inside a comment or string literal.
- The lint firing on external content when ownership attribution
  should mark it EXTERNAL.

Without a negative test, a lint can silently become a
false-positive factory. Without positive tests, a regression can
silently break the lint without anyone noticing.

## Ownership attribution

Every Finding carries an `ownership` tag. Your lint decides whether
to emit FIRST_PARTY, EXTERNAL, or UNKNOWN by consulting the
`LintContext.resolve_ownership(...)` helper. Pass whichever signal
is strongest:

- Patch linters: `ctx.resolve_ownership(mod_id=patch.mod)` when the
  patch has a `mod:` field, otherwise by patch source_file.
- Item linters: attribute through the winning source's mod_id. See
  `_attribute_ownership` in `content_syntax.py` for the pattern.
- Tree linters: use the CompileResult.source_label format.

When the ownership is genuinely ambiguous, return UNKNOWN — the
policy default treats it as FIRST_PARTY (fail-safe) so we don't
silently ship bugs attributed to "someone else's code".

## Baseline capture

Some lints inevitably fire on external mod source we can't fix. For
those:

1. Run the compiler with the new lint enabled against the real modlist:
   ```
   python3 -m megapatch_compiler assemble --mode debug
   ```
2. Extract external findings from the build manifest:
   ```
   jq '.findings[] | select(.ownership == "external" and .rule_id == "<MY_RULE_ID>")' \
       .debug/build_manifest.json
   ```
3. Review each finding manually. For any that are legitimate
   pre-existing conditions in external mods (not something our
   patches introduced), add a line to the baseline file at
   `tests/baselines/external_findings.jsonl` using the Finding's
   JSON representation.
4. Commit the baseline update with a comment explaining what the
   waived findings are and why they can't be fixed on our side.

Never add first-party findings to the baseline — if our own code
causes a finding, fix it. The baseline is strictly a record of
"external mods we've audited and decided to tolerate". It is not an
escape hatch for our own bugs.

## Disabling a lint temporarily

The `--disable-lint <RULE_ID>` CLI flag takes a rule out of play for
one build. Use this for:

- Investigating whether a lint is the actual cause of a build failure
- A lint that's demonstrably broken and needs to ship with a fix
  rather than block everyone

Do NOT use it to ship releases that would otherwise fail. If a lint
blocks release, either fix the underlying issue or document it in the
baseline.

## Debug mode

Release mode fails fast at the first class of blocking findings to
save work. Debug mode (`--mode debug`) runs everything to completion
and reports all findings together, so you can fix a whole batch of
issues in one pass instead of rebuilding after each one. Debug output
lands in `.debug/` and never touches `.dist/`.

## Severity and category conventions

- **ERROR** — correctness-affecting. Release build fails.
- **WARNING** — likely-incorrect or deprecated. Release build fails
  for first-party findings; external findings are baseline-checked.
- **INFO** — style/advisory. Release build fails only in strict
  configurations; baseline-applicable otherwise.

- **correctness** — breaks functionality at some point.
- **deprecation** — still works, will break in a future Stellaris version.
- **style** — consistency / readability — no functional impact.

## Regenerating the external baseline

External findings (things in workshop mods we can't fix ourselves) are
captured in `tests/baselines/external_findings.jsonl` so release builds
ignore known noise while catching genuinely-new regressions.

Regenerate after:
- landing a new lint rule that fires on pre-existing external content
- updating a workshop mod that now carries different surface issues
- expanding the active modlist (new mods → new noise)

Command:

```bash
megapatch lint --regen-baseline
```

Under the hood:
1. Runs a DEBUG-mode build so no findings block the pass.
2. Classifies every finding by ownership.
3. Writes every `EXTERNAL` finding to `tests/baselines/external_findings.jsonl`.
4. First-party findings are **never** baselined — those are always
   ours to fix.

The baseline file is sorted for stable diffs; commit it alongside the
commit that landed the new lint.

### Triage workflow when a new lint fires

For each finding category surfaced by a new lint:

1. **Genuine pre-existing noise** → accept into baseline with
   `--regen-baseline`.
2. **Real bug in external content** → file upstream, optionally add a
   comment in the baseline entry so future maintainers know why it's
   allowlisted.
3. **False positive from the lint itself** → refine the rule's regex
   or heuristic. Add a regression test. Rerun `--regen-baseline`.

### When baseline diverges from reality

If `tests/baselines/external_findings.jsonl` has entries for items
that no longer exist (removed mod, renamed item), those entries
silently pass through the policy check and produce no noise. That's
fine — stale allowlist entries are harmless. Clean them up
opportunistically during the next `--regen-baseline` sweep.
