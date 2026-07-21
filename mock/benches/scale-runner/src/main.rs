//! The scale-runner: one shared driver for the beyond-cap regimes the harness
//! excludes (multi-million-node reachability today; arena-locality and other
//! large-working-set questions later). It is not a copy of a bench; it links
//! the same `vehje-bench-carrier` and reuses its generators, solvers, and
//! checksum, so a scale run shares every line of workload generation and
//! validation with the harness benches. Only the driver differs: no 16 KB
//! input cap, program generated behind an opaque boundary (a runtime RNG, so
//! the optimizer cannot fold it), multiple runs with min / median / spread, and
//! a mandatory CSV. It refuses to conclude without writing its data.
//!
//! Timing uses the same hardware counter as the harness (`CNTVCT_EL0` at 24 MHz
//! on aarch64), so the numbers share a basis with the cdylib benches.

use std::fs;
use std::path::Path;

use mockspace_bench_core::counter::{read_counter, ticks_to_ns};
use vehje_bench_carrier::reach::{
    gen_fanin, gen_layered, gen_random_dag, reach_checksum, reset_reach, solve_semi, solve_whole,
    Graph,
};

#[derive(Clone, Copy)]
enum Shape {
    RandomDag,
    Layered32,
    Fanin,
}

impl Shape {
    fn build(self, n: usize, seed: u64) -> Graph {
        match self {
            Shape::RandomDag => gen_random_dag(n, 4, seed),
            Shape::Layered32 => gen_layered(n, 32, 3, seed),
            Shape::Fanin => gen_fanin(n, seed),
        }
    }
    fn tag(self) -> &'static str {
        match self {
            Shape::RandomDag => "random_dag",
            Shape::Layered32 => "layered32",
            Shape::Fanin => "fanin",
        }
    }
}

struct RunStats {
    ns: Vec<f64>,
    rounds: u32,
    checksum: u64,
}

fn median(mut xs: Vec<f64>) -> f64 {
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = xs.len() / 2;
    if xs.len() % 2 == 0 {
        (xs[m - 1] + xs[m]) / 2.0
    } else {
        xs[m]
    }
}

/// Time `runs` whole-column solves over the graph, resetting reach between runs.
fn time_whole(g: &Graph, runs: usize, perturb: u64) -> RunStats {
    let mut reach = vec![0u64; g.n];
    let mut ns = Vec::with_capacity(runs);
    let mut rounds = 0;
    let mut checksum = 0;
    for _ in 0..runs {
        reset_reach(g, &mut reach, 64, perturb);
        let t0 = read_counter();
        rounds = solve_whole(g, &mut reach);
        let t1 = read_counter();
        ns.push(ticks_to_ns(t1.wrapping_sub(t0)));
        checksum = reach_checksum(&reach, perturb);
    }
    RunStats { ns, rounds, checksum }
}

/// Time `runs` semi-naive solves; buffers allocated once, reused across runs.
fn time_semi(g: &Graph, runs: usize, perturb: u64) -> RunStats {
    let mut reach = vec![0u64; g.n];
    let mut fr: Vec<u32> = Vec::with_capacity(g.n);
    let mut nx: Vec<u32> = Vec::with_capacity(g.n);
    let mut q = vec![false; g.n];
    let mut ns = Vec::with_capacity(runs);
    let mut rounds = 0;
    let mut checksum = 0;
    for _ in 0..runs {
        reset_reach(g, &mut reach, 64, perturb);
        let t0 = read_counter();
        rounds = solve_semi(g, &mut reach, &mut fr, &mut nx, &mut q);
        let t1 = read_counter();
        ns.push(ticks_to_ns(t1.wrapping_sub(t0)));
        checksum = reach_checksum(&reach, perturb);
    }
    RunStats { ns, rounds, checksum }
}

fn run_experiment(shape: Shape, n: usize, runs: usize, out_dir: &Path) {
    let seed = 0x5ca1e_0000u64 ^ (n as u64).wrapping_mul(0x9e37_79b9);
    let perturb = 0x42;
    eprintln!("  building {} n={}...", shape.tag(), n);
    let g = shape.build(n, seed);
    let edges = g.edges.len();

    let w = time_whole(&g, runs, perturb);
    let s = time_semi(&g, runs, perturb);

    // cross-validation: whole and semi must reach the identical fixpoint.
    if w.checksum != s.checksum {
        eprintln!(
            "  !! CROSS-VAL FAIL {} n={}: whole=0x{:x} semi=0x{:x}",
            shape.tag(),
            n,
            w.checksum,
            s.checksum
        );
        std::process::exit(2);
    }

    let wm = median(w.ns.clone());
    let sm = median(s.ns.clone());
    let wmin = w.ns.iter().cloned().fold(f64::INFINITY, f64::min);
    let smin = s.ns.iter().cloned().fold(f64::INFINITY, f64::min);
    let wspread = (w.ns.iter().cloned().fold(0.0, f64::max) - wmin) / wm * 100.0;
    let sspread = (s.ns.iter().cloned().fold(0.0, f64::max) - smin) / sm * 100.0;

    // mandatory CSV: refuse to conclude without writing the data.
    let path = out_dir.join(format!("reach_{}_n{}.csv", shape.tag(), n));
    let mut csv = String::from("shape,n,edges,strategy,run,ns,rounds,checksum\n");
    for (strat, st) in [("whole", &w), ("semi", &s)] {
        for (i, &t) in st.ns.iter().enumerate() {
            csv.push_str(&format!(
                "{},{},{},{},{},{:.1},{},0x{:x}\n",
                shape.tag(),
                n,
                edges,
                strat,
                i,
                t,
                st.rounds,
                st.checksum
            ));
        }
    }
    fs::write(&path, csv).expect("scale-runner refuses to conclude without writing its CSV");

    // cost-model sanity line: whole-column does rounds x edges edge-ops.
    let whole_ops = w.rounds as f64 * edges as f64;
    let ns_per_op = wm / whole_ops;
    let cyc_per_op = ns_per_op * 3.2; // ns * GHz
    println!(
        "{:<11} n={:<8} E={:<9} | whole {:>10.0} us ({} rounds, {:.1} cyc/edge-op) | semi {:>10.0} us | semi/whole {:.2}x | spread w{:.0}%/s{:.0}%",
        shape.tag(),
        n,
        edges,
        wm / 1000.0,
        w.rounds,
        cyc_per_op,
        sm / 1000.0,
        sm / wm,
        wspread,
        sspread,
    );
}

fn main() {
    let out_dir = Path::new("results/scale");
    fs::create_dir_all(out_dir).expect("create results/scale");
    let runs = 5;

    // The matrix. Sizes chosen so whole-column stays feasible: random DAGs have
    // small diameter (few rounds), layered32 is 32 rounds over a wide graph,
    // fan-in is one round. Deep chains at these scales are intentionally absent
    // (whole-column would take millions of rounds); the harness covers the deep
    // regime at small n, and semi-naive's win there is already established.
    let matrix: &[(Shape, &[usize])] = &[
        (Shape::RandomDag, &[1_000_000, 4_000_000, 8_000_000]),
        (Shape::Layered32, &[1_000_000, 4_000_000]),
        (Shape::Fanin, &[8_000_000]),
    ];

    println!("vehje scale-runner: reachability whole-column vs semi-naive at scale");
    println!("timing: CNTVCT_EL0 (24 MHz), {} runs, median reported\n", runs);
    let args: Vec<String> = std::env::args().skip(1).collect();
    for (shape, sizes) in matrix {
        for &n in *sizes {
            // allow a single-experiment filter: `scale-runner random_dag 1000000`
            if !args.is_empty() {
                if args[0] != shape.tag() {
                    continue;
                }
                if args.len() > 1 && args[1].parse::<usize>().ok() != Some(n) {
                    continue;
                }
            }
            run_experiment(*shape, n, runs, out_dir);
        }
    }
    println!("\nscale CSVs written to results/scale/");
}
