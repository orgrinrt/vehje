# SK22 findings: diagnostics via lazy provenance witness reconstruction

**Date:** 2026-07-21 | **Outcome:** WORKS | Zig 0.16.0 | artifact `witness.zig`
**Settles:** the diagnostics/provenance design (Cluster C Part 2): provenance cheap on the happy path, the
derivation witness reconstructed lazily on failure (Souffle proof-tree on the resident relations).

## Result
The reach fixpoint runs normally (no provenance cost on success). On a lease violation ("value `result(5,6)`
reaches binder b1 which does not outlive it"), the derivation witness is reconstructed from the resident
Child/VarUse relations, producing the escape PATH: `result(5,6) <- add(3,4) <- use_b1 uses binder b1`. So the
error names which binder escaped and the exact node chain it flowed through, from the same relations the analysis
already holds, with no extra bookkeeping on the happy path.

## Design impact
A mod author's first contact (a lease/effect error) is a real diagnostic naming the binder and the escape path,
reconstructed on demand, not a bare "inclusion failed". The schema needs span + binder-site info on the
marked-output relations (the six additions Cluster C named) so the reconstructed path can carry source spans;
this sketch confirms the reconstruction mechanism over the relations. Land the schema in the wire format.
