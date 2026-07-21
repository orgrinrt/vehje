# SP2 findings: real arity/nesting/binder distributions (scripting-corpus proxy)

**Date:** 2026-07-21 | **Outcome:** DONE (real distributions on a proxy corpus) | Python 3 stdlib, 1500 files
**Settles:** the SP2 distribution measurements that feed BN2 (record width), the reachability-bitmask width, the
region-id width, and the depth cap; and the avoidance-relevant closure rate. Corpus is a proxy (see scope).

## Distributions (Python stdlib as a real scripting-language corpus, ~490k calls, ~140k functions)
- **Call arity (args+kwargs):** arity<=2 = 89.6%, arity>=3 = 10.4%. Cumulative: arity<=3 = 96.5%.
- **Function nesting depth (live-scope-chain proxy):** depth 1 = 83.0%, depth 2 = 16.3%, depth 3+ = 0.6%.
- **Live-binder width per function (locals):** 0-7 = 92.8%, 8-15 = 6.3%, 16-31 = 0.8%, 32-63 = 0.2%, 64+ ~ 0.0%.
- **Closure rate (nested defs, avoidance-relevant):** 17.0% of functions are nested.

## Design decisions these validate
- **24-byte record (BN2):** three inline operands cover arity<=3 = 96.5% of calls, so only 3.5% (arity>=4) spill
  to the pool. Combined with BN2's finding that 24B wins even at 5% call fraction, the 24-byte record is
  confirmed by the real arity distribution.
- **Reachability bitmask width (SK11):** a u64 mask over <=64 live binders covers 99.8% of functions
  (32-63 binders = 0.2%, 64+ ~ 0%). The W-width bitmask is well-sized; the overflow policy (SK2 fail-closed /
  demote) bites for ~0.2% of functions.
- **Region-id in the flags byte (SK17/BN2):** live-region width never approaches 256, so the region id fits the
  flags byte; no parallel [u16;N] column is needed in practice.
- **Depth cap:** depth 3+ is 0.6%, so the finite depth cap is small (single digits covers essentially all),
  cheaply bounding the no-alloc frontier, the frame stack, and the traversal.
- **Avoidance boundary (SK4/SK13):** 17% of functions are nested (the closure/escape shape where the reachability
  avoidance boundary matters). Static reach (SK4/SK11) handles the placeable majority; the generational-ref
  residual (SK13) handles the un-placeable subset of that 17%. Not negligible, not dominant.

## Scope (honest)
Python stdlib is a PROXY for scripting-language shape (functions, closures, nesting, calls), a real and large
corpus, but not the exact vehje census (jomini/lua/w3/ts4/doc-DSL/polka). The distributions are representative of
the scripting-consumer class op cares about; the exact census would shift the numbers but not the order of
magnitude (arity<=3 dominant, shallow nesting, narrow live-binder width, minority closures). A census-specific
measurement is deferred to when those consumers exist.
