//! Generate the full carrier composition-matrix from the typed `bench_matrix!`
//! declarations in the carrier library crate.
//!
//! The matrix is authored once, as real type-checked `setup` / `cell_<tag>` pub
//! fns in `vehje_bench_carrier::bench`, and each family's `matrix_decls()` returns
//! the `MatrixDecl` data. This binary concatenates them (`all_matrix_decls`) and
//! hands them to the upstream `generate_all`, which owns the single canonical
//! measurement template and writes the isolated variant crates plus the
//! `bench.toml` sections. There is no hand-rolled template here to drift: the four
//! measurement distortions the review panel found in the earlier string-template
//! generator cannot recur, because this binary emits no measurement logic at all.
//!
//! Run from `mock/benches/`: `cargo run --bin gen_matrix`.

use std::path::Path;

fn main() {
    let decls = vehje_bench_carrier::bench::all_matrix_decls();
    let benches = decls.len();
    let cells: usize = decls.iter().map(|d| d.cells.len()).sum();
    match mockspace_bench_matrix::generate_all(&decls, Path::new(".")) {
        Ok(()) => println!("generated {benches} matrix families, {cells} cells; wrote bench.toml"),
        Err(e) => {
            eprintln!("FAILED: {e}");
            std::process::exit(1);
        }
    }
}
