# Does the data support the conclusion (John Carmack)

Independent read of `first-look-results.md` against the raw TSVs
(`mock/benches/.bench_history/abi_soa_win_real_n*.tsv`, `abi_cross_scalar_real_n*.tsv`) and the code that
produced them (`mock/benches/carrier-runtime/src/lib.rs`, `mock/benches/carrier/src/bench/boundary/{soa,cross,common}.rs`).
No timing runs were executed (a run is in progress); everything below is from the committed data and the source.

## Verdict in one line

The 2.4x is real but it is a property of the carrier's SoA-8 interpreter, not of the ABI entry; the W>=8
threshold is the `soa_batch` while-loop condition restated as a measurement; and "not by crossing
amortisation" is asserted for the one tier (native) these numbers do not contain, so the conclusion's causal
chain is about half right and the half that is right was mostly known before the bench ran.

## Where the causal inference is unsupported or conflated

1. **The W<8 rows of `soa_payload` are the scalar code path by construction, so "no win below W=8" is a
   tautology, not a finding.** `soa_batch` (`carrier-runtime/src/lib.rs:118-134`) is `while i + LANES <= w`
   (line 121) SoA passes plus a scalar remainder, with `LANES = 8` a fixed const (line 43). For w in
   {1, 2, 4} the SoA loop body never executes and every record runs `interpret_predecoded` through the
   remainder, i.e. `cr_execute_soa_runtime_w` at W<8 IS `scalar_batch` behind a different symbol. The
   table's W=1/2/4 soa rows (2.375/2.438/2.363 ms vs scalar 2.461/2.416/2.363 ms) are scalar-vs-scalar and
   carry zero information about SoA, the ABI, or hardware. The harness's own doc says this outright
   ("the two 1/W knees separate by construction", lib.rs:13-18; soa.rs:11-18); the results memo then
   re-fuses the two widths into a causal headline the code explicitly decoupled.

2. **The threshold "8" is carrier-runtime's internal chunk constant, not "the SIMD lane width" and not an
   ABI property.** On aarch64, `Simd<u64, 8>` is four 2-lane NEON registers; the vert8 win is chunked
   ILP plus amortised interpreter control flow, not an 8-lane hardware fact. Had `soa_batch` carried a
   vert4 fallback, W=4 would show a partial win and the "threshold" would move. Worse for the conclusion:
   the runtime-W entry itself proves the ABI does NOT need to know the number 8. The entry takes any w and
   chunks internally. So "the ABI's minimum useful batch is the SIMD width" is calling-convention guidance
   about a runtime implementation parameter, not an ABI shape input. Baking 8 into the ABI surface would be
   encoding one interpreter's loop constant into a contract.

3. **"A scalar per-record boundary forecloses the win" is supported only under an unstated design premise
   the bench never tests: the runtime is stateless across calls and must complete its records' work
   synchronously within the delivering call.** Under that premise the claim is true by contract and needed
   no benchmark. Outside it there are at least two unmeasured shapes where a narrow entry does not
   foreclose vectorisation: (a) a runtime-resident column, where records already live runtime-side and a
   thin trigger call vectorises over the resident data (per-call W decoupled from vector width entirely);
   (b) accumulate-and-flush, where W=1 calls append and evaluation batches at flush (this one changes
   result-availability semantics, and the reserve/commit sink contract plausibly rules it out, but that is
   a design ruling, not a measured one). No cell in `abi_soa_win` represents either. The data quantifies
   the size of the win given in-call vectorisation; it does not establish that the boundary shape is what
   gates vectorisation.

4. **"The crossing is negligible" is true but the quoted number is not the crossing, and the negligibility
   does not transfer to the tier the decision is about.** `null_entry` is fill_seeds(256) + fold + k
   crossings, and fill runs in the timed cell (`common.rs:77-81`, soa.rs:73-74). The W-sweep slope isolates
   the actual per-crossing cost: bench 1 gives (4055-3501)/63 ≈ 8.8 ns, bench 3 gives (5482-3221)/255 ≈
   8.9 ns per warm, single-target, perfectly predicted `blr` including small-W marshalling. The ~2-5 us the
   memo cites as "the crossing" is the W-invariant harness intercept (mostly fill_seeds). Against the
   8.5 us/record scalar interpret (2.17 ms / 256) the ~9 ns crossing is 0.1%, fine. But at the native /
   copy-and-patch tier a 256-node residual is plausibly ~50-150 ns/record, so a per-record crossing is
   ~6-18% warm and worse cold or target-varying (the `cold.rs` machinery built to measure exactly that
   penalty is absent from these numbers, as are the sink families with their two reverse crossings per
   record). The memo's body concedes the decision "lives at the cheap-payload end"; the concluding
   paragraph then asserts "justified by the vectorisation it enables, not by crossing amortisation", a
   negative claim about precisely the regime with no data. At the native tier, amortisation plausibly
   BECOMES a load-bearing justification. The batched entry likely survives, but for a reason these numbers
   cannot yet name.

5. **The headline "2.56x at W=8" is computed against the sweep's noisiest baseline point.** scalar_payload
   should be W-invariant (its per-crossing delta is ~9 ns x at most 255 crossings ≈ 2 us, i.e. 0.1% of
   2.17 ms), yet it spans 2.151-2.489 ms across the sweep, a 14% swing with the W=8 median (2.489 ms, CI
   2.414-2.570) sitting far off the 2.15-2.20 ms plateau. Bench 1 shows the same pattern: at W=64-256 all
   three heavy cells (ffi, inproc_direct, inproc_fnptr) drift up ~10% together with widening CIs, and ffi
   comes out FASTER than in-process at W=256 (2.374 vs 2.389 ms). That is machine-state drift across
   cells run minutes apart, not a W effect. Consequences: the honest ratio is the plateau ratio, ~2.175 ms
   / ~0.897 ms ≈ 2.42x, not 2.56x; and nothing in this run can resolve differences under roughly +/-10%,
   which pre-emptively voids any entry-form (mono vs runtime-W vs dispatch) conclusion drawn from this
   payload class.

6. **Bench 3 re-measures a known in-process fact through an FFI window and attributes it to the window.**
   The 2.4x is the carrier's own vert8-vs-scalar interpreter ratio (lib.rs:41-43 calls it "the measured
   column-eval win", past tense, because the carrier's vertical bench measured it in-process). Both bench-3
   cells cross identically; the delta is interpreter-vs-interpreter. The genuinely new information in bench
   3 is narrower and worth having: the win SURVIVES the C ABI crossing and pointer+len marshalling intact.
   That is a preservation result, not an enablement result. "The batched ABI unlocks 2.4x" inverts the
   causality: the SoA interpreter produces the 2.4x; the batched call is one (cheap, sensible) way to feed
   it 8 or more records under the synchronous-call premise of point 3.

## What the data does legitimately support

1. A warm, single-target cross-object indirect call costs ~9 ns including marshalling a pointer+len batch;
   against any interpret-class payload (8.5 us/record here) every boundary and entry-form choice is noise.
   Two independent families give the same slope; the low-W anchors are clean.
2. The SoA-8 vertical interpreter is ~2.42x the scalar interpreter on the `real` profile at 256 nodes, and
   that win crosses the C ABI undamaged. Cross-validation (byte-exact host-vs-inproc folds, soa.rs tests)
   makes this trustworthy as far as it goes. Caveat noted for completeness: the keep-alive checksum is also
   vectorised in the SoA path, but at ~2% of payload it does not move the ratio materially.
3. Given a runtime that vectorises synchronously over the records delivered per call, a call must carry at
   least the runtime's internal chunk width for any win, and delivering more (up to the whole column) costs
   nothing and saturates it. Flat 897-904 us across W=16..256 is a clean saturation curve.
4. Handing the runtime the whole resident column per call (W=256) is free relative to any smaller W. Since
   the column already exists contiguous host-side in every cell, the maximally batched entry is the
   zero-regret point of the measured sweep.

## The sharpest correction to the conclusion as written

Replace "a batched entry unlocks a ~2.4x SoA vectorisation win that a scalar per-record entry forecloses"
with: **the SoA interpreter is worth ~2.4x, the boundary provably does not eat it, and under the settled
synchronous-call, stateless-across-calls ABI contract, per-call batch width is the vectorisation set, so
the entry takes a column (pointer + length, any W); the runtime chunks internally and W >= its internal
vector width is performance guidance, not an ABI parameter.** Strike "not by crossing amortisation": that
clause is a claim about the native tier, which is absent from these numbers, and at ~9 ns warm crossing
versus an estimated ~100 ns/record native payload (plus two reverse sink crossings per record in the
per-record sink shape), amortisation is likely a second independent justification there, to be confirmed by
the cold-target and sink families in the full run. The decision itself (expose the batched column entry) is
almost certainly right, and was nearly free to make before the bench: the column is already resident and
contiguous host-side, pointer+len costs nothing over a scalar argument, and it strictly dominates the
per-record shape in every measured and every projected regime. What the first look actually bought is the
preservation result (2) and the crossing constant (1); the memo should claim those, not enablement.
