//! Extract the node-count metric the cheap_lowering variants emit in their
//! output high-32-bits but the harness does not surface to CSV. Takes ONE size
//! as argv and loads+runs both variants for it, so each size runs in a fresh
//! process: the variant's OnceLock program cache is keyed on the first N it
//! sees, and macOS dlopen dedups a library within a process, so extracting
//! multiple sizes in one process would freeze the program at the first size
//! (exactly the subprocess isolation the harness itself uses per variant/size).
#[repr(C)]
struct FfiBenchCall {
    run_ticks: u64,
}
type Entry = unsafe extern "C" fn(*const u8, *mut u8, usize) -> FfiBenchCall;

fn node_count(lib_path: &str, n: usize) -> u64 {
    unsafe {
        let lib = libloading::Library::new(lib_path).expect("load variant dylib");
        let entry: libloading::Symbol<Entry> = lib.get(b"bench_entry").expect("bench_entry");
        let input = vec![0u8; n];
        let mut output = [0u8; 8];
        let _ = entry(input.as_ptr(), output.as_mut_ptr(), n);
        u64::from_le_bytes(output) >> 32
    }
}

fn main() {
    let n: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).expect("usage: nodecount-probe <n>");
    let fo = node_count("variants/cl_foldonly/target/release/libcl_foldonly.dylib", n);
    let fc = node_count("variants/cl_foldcse/target/release/libcl_foldcse.dylib", n);
    let red = if fo > 0 { (fo - fc) as f64 / fo as f64 * 100.0 } else { 0.0 };
    println!("{},{},{},{:.1}", n, fo, fc, red);
}
