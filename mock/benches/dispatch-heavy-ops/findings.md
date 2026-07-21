# BN1-heavy (expansion): switch vs tail-threading across op-body weight

**Date:** 2026-07-21 | Zig 0.16.0 | data `heavy.csv` | expands BN1.
**Question:** does tail-threading's independent-register-allocation benefit emerge as op bodies get heavier?

## Result (ns/op, median of 7, body = W dependent arithmetic ops per dispatch)
| body weight W | switch | tail |
|---|---|---|
| 1 | 4.48 | 6.69 |
| 4 | 4.69 | 6.85 |
| 16 | 7.61 | 10.18 |
| 64 | 47.60 | 51.82 |

Switch is faster at EVERY body weight (checksums match, correctness confirmed). Tail-threading's overhead
(~2-4 ns/op: per-op frame, argument passing, no preserve_none) is a constant that never gets repaid; as the body
grows, both slow down but switch stays ahead. Tail-threading does not win even when the op body dominates.

## Reading and the one untested regime
For vehje's workload class (few op-kinds, arithmetic bodies, no runtime type dispatch), switch is the right
dispatch regardless of op-body weight. The literature's tail-threading win comes from a regime this does not
test: MANY distinct op-kinds (100+ opcodes), where the giant switch function spills registers and the branch
predictor struggles, which is where per-op independent register allocation pays off. But SP1 measured vehje's
primitive vocabulary at ~25 (bounded, low tens), so vehje is NOT in the many-opcode regime. Within vehje's actual
op count, switch wins across all body weights. If a consumer ever pushed the runtime op count into the hundreds,
this bench should be re-run over op-COUNT (not body weight) to locate that crossover; for the designed vocabulary
it does not arise.

## Design impact
Confirms and strengthens BN1: ship the interpreter with switch dispatch. Tail-threading is not worth building for
vehje's bounded op vocabulary at any op-body weight. preserve_none stays off the critical path.
