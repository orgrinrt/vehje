# Grounding the contracts (Glenn Fiedler)

## The judgement in one line

The determinism promise the whole design rests on is not merely unstated, it is **falsified by shipped code
in 151 committed files**, and the commit gate's three legs are the wrong three: the leg the round assumed
was policy is verified fact, the leg it called parity is false as written, and the leg nobody named (the
gate is client-side local config with no server-side re-execution) is the one that actually carries the risk.

## The contract ledger

| contract | promised | stated? | established? | citation / verdict |
|---|---|---|---|---|
| Byte-identical output from unchanged inputs | implicitly, everywhere a diff is called meaningful | **nowhere** | **FALSE TODAY** | `render_design.rs:105-117` writes `now_rfc3339()` into every generated header. UNSUPPORTED, and worse, contradicted. |
| Snapshot version skew semantics | reader detects and refuses/upgrades | no | ESTABLISHED as a discipline, unadopted here | [SQLite file format](https://www.sqlite.org/fileformat2.html) §1.2 read/write version bytes; [Cap'n Proto evolution rules](https://capnproto.org/language.html#evolving-your-protocol) |
| Snapshot staleness detection | fingerprint mismatch forces rebuild | prose only (Giesen adj. 5) | NOVEL only in composition; each class established | Bazel source/generated split; [Nix fixed-output derivations](https://nixos.org/manual/nix/stable/language/advanced-attributes) for the pinned-oracle class |
| Gate: "nothing reaching `dev` is unchecked" | operational | yes (Arntzen) | **PARTIALLY FALSE** | see below; verified against live rulesets and the generated hook |
| Checker parity, gate vs local run | identical checker | assumed | **FALSE** | hook passes `--scope`; `lint/mod.rs:130-131` skips out-of-scope crates |
| Invalidation on transitive registry change | gate re-checks readers | red test proposed, unwritten | **FALSE TODAY** | hook derives scope from staged path prefixes only |
| mockspace(`std`) → no-alloc crates | borrowed views, never forced allocation | no API shape anywhere | UNSUPPORTED | Arntzen correct; no precedent found for the no-heap half |
| Upstream stability (`arvo`, `hilavitkutin-str`) | none owed | yes, by omission | ESTABLISHED as deliberate | `no-legacy-shims-pre-1.0.md`; Q-F-A pins are mitigation, not contract |
| vehje fit | assumed | named as a bet | UNSUPPORTED, honestly labelled | keep the label |
| Epoch-pinned citations | cited bytes immutable | Giesen mech. 1 | ESTABLISHED per leg | [Software Heritage SWHIDs](https://www.swhid.org/) content-address to line granularity; the diff-driven frontier is the unlocated part |

## Determinism end to end, and what it actually requires

The design leans on determinism at four separate points: a diff is meaningful, the variation seed is stable,
the churn counter measures something, and the dirty set can trust a fingerprint. None of those survive a
generator that is not reproducible, and the shipped one is not.

**Verified:** `render_design.rs:480-490` shells out to `date -u` and embeds the result at
`generation_header_md`. Committed instances carrying it: arvo 27, hilavitkutin 19, vehje 19, viola 12,
`ikiuni_renderer` 74. **Every regeneration dirties every generated document**, unconditionally. The failure
is doubled: the function shells to an external binary and, on failure, silently substitutes `"unknown"`, so
output content depends on `PATH`.

The fix is a solved problem with a specification. `SOURCE_DATE_EPOCH` is the reproducible-builds standard for
exactly this: a build embedding a timestamp reads it from the environment and falls back to a fixed value,
never the wall clock ([spec](https://reproducible-builds.org/specs/source-date-epoch/)). `grep` finds zero
occurrences in `mockspace/src`. The stronger option is to delete the field: a generated file's provenance is
its input hash, not its birth time, and the input hash is a fingerprint the design already needs.

What determinism actually requires here, per the reproducible-builds literature (Lamb and Zacchiroli,
"Reproducible Builds: Increasing the Integrity of Software Supply Chains," *IEEE Software* 39(2), 2022,
[arXiv](https://arxiv.org/abs/2104.06020)), which enumerates the recurring non-determinism sources:

1. **No wall clock.** Broken today.
2. **Ordered iteration.** In good shape and worth stating as an invariant rather than luck: 17 source files
   use `BTreeMap`/`BTreeSet`; the only `HashMap`/`HashSet` sites (`config.rs:737-739`,
   `render_agent.rs:2231/2253`) are lookup and membership, never iterated into output. That is a real
   property today and one refactor from being lost silently.
3. **Sorted directory reads.** `fs::read_dir` returns filesystem order. Every derived class Karis proposes
   (441 rows from source scans) and the `crates::` probe at `resolve.rs:359-368` reads directories. Sorting is
   required at each, not assumed.
4. **No locale.** Case folding and collation in `~` and `sort` must be byte-wise or explicitly `C`.
5. **Hash ordering.** The structural hash is fine; the hash-to-slot *table* must never determine output
   order. Tatarchuk's T2-C keeps the arena index as identity, which preserves this if insertion order is
   input order.
6. **No path leakage.** `mock_rel` in the header is repo-relative already; keep it that way.

Absolute paths, build IDs, and locale are precisely the classes Debian's programme found dominate in
practice ([reproducible-builds.org/docs/](https://reproducible-builds.org/docs/)).

## Where a single contract serving three consumers fails

Giesen's adjudication 5 makes the snapshot one artefact answering the warm path, the interchange boundary,
and the `std`-to-no-alloc edge. That consolidation is right, and it fails in one specific place, which is the
place a single contract serving three consumers always fails: **the three have different version-skew
tolerances, and one contract can only encode one.**

- **Warm cache.** Skew tolerance: zero, and the correct response is *silent regeneration*. A cache that
  refuses on a version bump is a broken cache.
- **Interchange to vehje.** Skew tolerance: high, and the correct response is *refuse loudly*. Silently
  regenerating someone else's input is how you ship a wrong document.
- **`std`-to-no-alloc.** Skew tolerance: zero, and the correct response is *fail at link or load*, because
  the no-alloc side cannot allocate a migration buffer to upgrade an old snapshot even if it wanted to.

One version field cannot mean "rebuild me", "reject me", and "I cannot upgrade in place" at once. SQLite
solved this by carrying **two** numbers, a read version and a write version, so a reader learns whether it
may read, may write, or must refuse, independently. Adopt that shape: a format version plus a minimum-reader
version, plus a separate content generation counter. The generation counter answers staleness; the version
pair answers compatibility; conflating them gives the cache the interchange format's brittleness or gives
the interchange format the cache's silent-rebuild behaviour.

Second failure, narrower: the no-alloc consumer forces the snapshot to be **self-describing without
allocation**, which rules out any layout needing a fix-up pass or a built index at load. Recommendation A
(hand-rolled flat tables) satisfies this and rkyv's validation layer does not, which is a stronger argument
for A than the dependency-surface one the round actually made.

## What the gate can and cannot carry as a proof

I verified the three legs rather than inheriting them. The result rearranges the risk.

**Leg 1, rulesets. VERIFIED TRUE, and the round underrated it.** `gh api repos/orgrinrt/arvo/rulesets`
returns `trunk-protection` (`enforcement: active`) covering `~DEFAULT_BRANCH`, `refs/heads/dev`,
`refs/heads/main` with rules `deletion`, `non_fast_forward`, `pull_request`. This is server-side, live, and
not policy. Same on `hiisi-digital/mockspace`.

**Leg 2, checker parity. FALSE, in shipped code.** The generated pre-commit hook
(`bootstrap.rs:1289-1327`) computes `CHANGED_CRATES` from staged path prefixes and invokes
`cargo mock --lint-only --commit --scope "$CHANGED_CRATES"`. `lint/mod.rs:130-131` then skips every crate not
in scope. The gate runs a **strict subset** of a local run, and the subset is chosen by which paths happen to
be staged. Staging only registry rows yields empty `CHANGED_CRATES` and `--scope infra`, so Arntzen's red
test does not merely fail, it is routed to a different scope entirely. Renaming a crate directory stages two
paths and checks those two crates, while the 2286 `crates::` references that just re-resolved live in other
crates' templates and in registry rows, outside scope. Giesen's adjudication-2 case 2 is demonstrably broken
today, not hypothetically.

**Leg 3, `--no-verify`. Unverifiable policy, as stated. Correctly labelled.**

**Leg 4, unnamed by anyone, and the one that matters.** The gate is a bash script reached through
`core.hooksPath`, set by `git config --local` (`bootstrap.rs:200-218`). Local config is not cloned. A fresh
clone, a CI runner, or any contributor who never triggered bootstrap has **no gate at all**. And because
`trunk-protection` carries no `required_status_checks` (verified: only `deletion`, `non_fast_forward`,
`pull_request`; the one `required_status_checks` ruleset is `main-source-restriction`, scoped to `main`), the
server never re-runs the check. So the single server-side leg enforces *process* and the checking is
*entirely client-side and opt-in*.

The verification literature is unambiguous about what this can carry. A check whose execution is controlled
by the party being checked carries no proof about that party; this is Thompson's point in
["Reflections on Trusting Trust"](https://dl.acm.org/doi/10.1145/358198.358210) (Turing Award lecture, 1984)
and the reason reproducible builds exist as a discipline at all: the answer to an untrusted local build is an
**independent rebuild by a verifier**, as formalised in Wheeler's Diverse Double-Compiling
([wheeler.com/trusting-trust](https://dwheeler.com/trusting-trust/)).

So the honest claim is narrower than Arntzen's and the fix is one line of ruleset JSON: **add
`required_status_checks` on `dev` running the full-scope checker.** Then the gate becomes a proof carrier
about the trunk (still not about a working copy), leg 4 dissolves, leg 3 stops mattering, and leg 2 is
forced to parity because the server has no staged-path prefix to scope by. Until then the honest sentence is
"a contributor with hooks installed is warned about the crates they touched."

## Corrections to Wronski, Wyman, and earlier phases

**Wyman's 6628 is correct; I reproduced it independently.** Full prefix histogram over
`ikiuni_renderer/mock/registry`: 3475 `seed::`, 2286 `crates::`, 650 `reference::`, 93 `spike::`, 66 `bench::`,
31 `tripwire::`, 7 `constant::`, 5 `ruling::`, 3 each `vocab::`/`reg::`/`law::`/`equation::`, 2 `technique::`,
1 `facet::`. Row-to-row is 861, and 867 counting `vocab::`/`reg::`. Per his provocation 4, the command is
`grep -rhoE '\b[a-z_][a-z0-9_]*::' . | sort | uniq -c | sort -rn`. Fourth count, last count.

**Arntzen's leg 1 should be promoted from assumption to verified fact**, and his leg list extended to four.
His judgement that the proof split "does not dissolve" stands and is strengthened: it does not dissolve
*while checking is client-side*, and it partially dissolves the moment a required status check exists.

**Giesen's "the snapshot is one boundary contract" needs the two-version correction above.** The
consolidation is right; the single version field is not.

**Wronski's DBSP scoping is right and understates the consequence.** A derivative computable from a query
plan is worth nothing if the *inputs* are not fingerprinted, and today the largest input class (derived
namespaces, 2286 references) has no fingerprint and the gate cannot see it change.

## Open provocations for the synthesiser

1. **Delete `now_rfc3339` from the generated header, or gate it behind `SOURCE_DATE_EPOCH`.** It is the
   cheapest fix in the round and it is currently falsifying the design's foundational promise across 151
   committed files.
2. **State determinism as a contract with a test**, not a hope: a regenerate-twice-and-diff test in CI. Every
   requirement above is cheap; none is guaranteed by anything written down.
3. **Add `required_status_checks` to `trunk-protection` on `dev`.** One ruleset edit converts three
   unproven operational legs into one verified one.
4. **Split the snapshot's version field into read-version, minimum-writer-version, and generation counter**
   before anything consumes it, per SQLite. Three consumers, three skew semantics, one field is one too few.
5. **Fix the hook's scope derivation before writing the red tests**, or the tests pin behaviour the gate
   cannot reach. Scope must come from the dependency graph, not from staged path prefixes.
