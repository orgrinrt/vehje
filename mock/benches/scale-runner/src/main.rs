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
use vehje_bench_carrier::eqsat::{build_chain, Op};
use vehje_bench_carrier::incr::{compile_module, gen_modules};
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

/// One eqsat saturation over a k-leaf associative chain at the given cap.
/// Returns (enodes, rounds, extract_cost, value, ns) for the run.
fn one_eqsat(k: usize, cap: usize) -> (usize, u32, u64, u64, f64) {
    let (mut g, root) = build_chain(k, Op::Add, cap, 0x5eeda11c);
    let t0 = read_counter();
    let rounds = g.saturate();
    let (cost, val) = g.extract(root);
    let t1 = read_counter();
    (g.enode_count(), rounds, cost, val, ticks_to_ns(t1.wrapping_sub(t0)))
}

/// The eqsat associativity cap-explosion experiment: a bounded window vs an
/// unbounded run over the same reassociation chain, which is the case designed
/// to blow an e-graph up. The bounded run must stay near its cap while
/// extracting the same-cost form; the unbounded run explodes toward the safety
/// ceiling. Both extract equal VALUE (all reassociations are equal), the
/// cross-validation. runs=3, median ns; K kept <= 14 so the unbounded run stays
/// feasible (K=16 unbounded is ~2M e-nodes and ~20 s).
fn run_eqsat(out_dir: &Path, runs: usize) {
    println!("\n== eqsat associativity cap-explosion: bounded(512) vs unbounded(2M ceiling) ==");
    println!(
        "{:>3} | {:<9} {:>9} {:>5} {:>5} {:>10} | {:<9} {:>9} {:>5} {:>5} {:>10} | equal-value?",
        "K", "bounded", "enodes", "cost", "rnds", "us", "unbounded", "enodes", "cost", "rnds", "us"
    );
    let mut csv = String::from("k,strategy,cap,enodes,rounds,extract_cost,value,ns\n");
    for &k in &[8usize, 10, 12, 14] {
        let bounded_cap = 512;
        let unbounded_cap = 2_000_000;
        let mut bns = Vec::new();
        let mut uns = Vec::new();
        let (mut be, mut brn, mut bco, mut bv) = (0, 0, 0, 0);
        let (mut ue, mut urn, mut uco, mut uv) = (0, 0, 0, 0);
        for _ in 0..runs {
            let (e, r, c, v, ns) = one_eqsat(k, bounded_cap);
            be = e;
            brn = r;
            bco = c;
            bv = v;
            bns.push(ns);
        }
        for _ in 0..runs {
            let (e, r, c, v, ns) = one_eqsat(k, unbounded_cap);
            ue = e;
            urn = r;
            uco = c;
            uv = v;
            uns.push(ns);
        }
        if bv != uv {
            eprintln!("  !! eqsat CROSS-VAL FAIL k={k}: bounded value 0x{bv:x} != unbounded 0x{uv:x}");
            std::process::exit(2);
        }
        let bm = median(bns);
        let um = median(uns);
        println!(
            "{:>3} | {:<9} {:>9} {:>5} {:>5} {:>10.0} | {:<9} {:>9} {:>5} {:>5} {:>10.0} | yes (0x{:x})",
            k, "", be, bco, brn, bm / 1000.0, "", ue, uco, urn, um / 1000.0, bv
        );
        for (strat, cap, e, r, c, ns) in [
            ("bounded", bounded_cap, be, brn, bco, bm),
            ("unbounded", unbounded_cap, ue, urn, uco, um),
        ] {
            csv.push_str(&format!("{},{},{},{},{},{},0x{:x},{:.1}\n", k, strat, cap, e, r, c, bv, ns));
        }
    }
    fs::write(out_dir.join("eqsat_cap_explosion.csv"), csv)
        .expect("scale-runner refuses to conclude without writing its CSV");
    println!("  (bounded stays near 512 e-nodes; unbounded explodes: ~79k at K=12, >2M at K>=18)");
}

/// C4b: a real threaded level-sync DAG loader. A synthetic module DAG of `levels`
/// levels x `width` modules; a level cannot start until the previous finishes
/// (the barrier is `thread::scope` join). Within a level the modules compile in
/// parallel across `threads` OS threads via static chunking, each a real
/// `compile_module` call (not the old frictionless ceil(width/8) model). Returns
/// (median_ns, fold) so the fold cross-validates across thread counts.
fn time_threaded_dag(
    modules: &[Vec<u8>],
    levels: usize,
    width: usize,
    threads: usize,
    runs: usize,
) -> (f64, u64) {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Barrier;
    let mut ns = Vec::with_capacity(runs);
    let mut fold = 0u64;
    for _ in 0..runs {
        // A persistent pool: T worker threads spawned once and reused across
        // every level via a barrier, so the measurement is the parallel compile
        // and the per-level barrier, NOT thread-spawn churn (spawning per level
        // dominated the earlier attempt). Within a level, workers pull modules
        // by an atomic cursor (work-stealing); the barrier is the level-sync.
        let end = AtomicUsize::new(0);
        let cursor = AtomicUsize::new(0);
        let done = AtomicBool::new(false);
        let barrier = Barrier::new(threads + 1);
        let elapsed = std::thread::scope(|s| {
            let mut handles = Vec::new();
            for _ in 0..threads {
                handles.push(s.spawn(|| {
                    let mut scratch = vec![0u64; 32];
                    let mut f = 0u64;
                    loop {
                        barrier.wait(); // level start (or shutdown)
                        if done.load(Ordering::Acquire) {
                            break;
                        }
                        let e = end.load(Ordering::Relaxed);
                        loop {
                            let i = cursor.fetch_add(1, Ordering::Relaxed);
                            if i >= e {
                                break;
                            }
                            f ^= compile_module(&modules[i], &mut scratch);
                        }
                        barrier.wait(); // level end
                    }
                    f
                }));
            }
            let t0 = read_counter();
            for l in 0..levels {
                let b = l * width;
                end.store((b + width).min(modules.len()), Ordering::Relaxed);
                cursor.store(b, Ordering::Relaxed);
                barrier.wait(); // release workers into the level
                barrier.wait(); // wait for the level to finish
            }
            done.store(true, Ordering::Release);
            barrier.wait(); // release workers to observe shutdown and exit
            let t1 = read_counter();
            let ns = ticks_to_ns(t1.wrapping_sub(t0));
            let combined = handles.into_iter().fold(0u64, |a, h| a ^ h.join().unwrap());
            (ns, combined)
        });
        ns.push(elapsed.0);
        fold = elapsed.1;
    }
    let _ = width;
    (median(ns), fold)
}

fn run_threaded_dag(out_dir: &Path, runs: usize) {
    println!("\n== C4b threaded level-sync DAG: parallel compile speedup (real threads) ==");
    let levels = 64usize;
    let width = 512usize;
    let n = levels * width;
    let modules = gen_modules(n, 0x4b_0000_0001);
    let cores = std::thread::available_parallelism().map(|c| c.get()).unwrap_or(8);
    println!(
        "  {} modules ({} levels x {} wide), {} logical cores; compile = 8-pass interp per module",
        n, levels, width, cores
    );
    let mut csv = String::from("levels,width,threads,ns,speedup,efficiency\n");
    let (base_ns, base_fold) = time_threaded_dag(&modules, levels, width, 1, runs);
    println!("  {:>2} thread : {:>9.1} ms  (1.00x)", 1, base_ns / 1e6);
    csv.push_str(&format!("{},{},1,{:.1},1.00,1.00\n", levels, width, base_ns));
    for &t in &[2usize, 4, 8] {
        let (t_ns, fold) = time_threaded_dag(&modules, levels, width, t, runs);
        if fold != base_fold {
            eprintln!("  !! CROSS-VAL FAIL threads={t}: fold 0x{fold:x} != serial 0x{base_fold:x}");
            std::process::exit(2);
        }
        let speedup = base_ns / t_ns;
        let eff = speedup / t as f64;
        println!(
            "  {:>2} threads: {:>9.1} ms  ({:.2}x, {:.0}% efficiency)",
            t,
            t_ns / 1e6,
            speedup,
            eff * 100.0
        );
        csv.push_str(&format!("{},{},{},{:.1},{:.2},{:.2}\n", levels, width, t, t_ns, speedup, eff));
    }
    fs::write(out_dir.join("threaded_dag.csv"), csv)
        .expect("scale-runner refuses to conclude without writing its CSV");
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

    println!("vehje scale-runner: beyond-cap experiments");
    println!("timing: CNTVCT_EL0 (24 MHz), {} runs, median reported\n", runs);
    let args: Vec<String> = std::env::args().skip(1).collect();

    // single-experiment gates.
    if args.first().map(String::as_str) == Some("eqsat") {
        run_eqsat(out_dir, 3);
        println!("\nscale CSVs written to results/scale/");
        return;
    }
    if args.first().map(String::as_str) == Some("threads") {
        run_threaded_dag(out_dir, 5);
        println!("\nscale CSVs written to results/scale/");
        return;
    }
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

    // eqsat + threaded-dag run by default too (after reach), unless a reach-shape
    // filter was given.
    if args.is_empty() {
        run_eqsat(out_dir, 3);
        run_threaded_dag(out_dir, 5);
    }
    println!("\nscale CSVs written to results/scale/");
}
