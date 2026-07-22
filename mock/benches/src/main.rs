//! vehje bench binary. Byte-shaped benches (branch-strategy comparison and,
//! progressively, the rest of the vehje bench set) run through the generic
//! mockspace bench driver: variant cdylibs are isolated per-subprocess+dlopen,
//! timed with hardware counters under a realistic surrounding workload, and
//! reported to CSV + findings. A new bench needs only variant crates under
//! `variants/<name>/` plus a `[bench.<name>]` section in `bench.toml`.
use std::process::ExitCode;

use mockspace_bench_core::byte_routine_dispatch;
use mockspace_bench_harness::driver::{drive, DriverRegistry};
use mockspace_bench_harness::{self as harness, BenchConfig, RoutineSpec, Workload};

/// Realistic surrounding workload: the measured call embedded in scalar
/// dependency chains, pointer-chase graph work, cache pressure, and branchy
/// context, so the timed call sees a real calling environment rather than an
/// empty loop.
fn build_workload(_name: &str, _n: usize) -> Workload {
    let mut workload = Workload::new();
    workload.program("realistic", |b| {
        b.stage(vec![
            harness::algo_call(),
            harness::scalar_work(48),
            harness::graph_work(32),
            harness::heavy_memory(384),
            harness::branch_work(24),
            harness::light_scalar(),
        ]);
    });
    workload
}

/// All benches here are byte-shaped; the generic byte dispatch serves them
/// (`may_differ` comes from the manifest, not a name list).
fn routine_for(_config: &BenchConfig) -> Option<RoutineSpec> {
    None
}

fn main() -> ExitCode {
    drive(&DriverRegistry {
        build_workload,
        routine_for,
        byte_dispatch: byte_routine_dispatch!(
            out = 8,
            sizes = [64, 256, 1024, 2048, 3072, 4096, 6144, 8192, 16384]
        ),
    })
}
