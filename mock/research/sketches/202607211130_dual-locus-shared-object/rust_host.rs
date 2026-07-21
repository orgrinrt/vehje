// Rust dev-side locus links the compiled Zig engine over FFI.
#[link(name = "vehje_engine")] extern "C" { fn vehje_engine_reachhash(b: *const u32, n: usize, seed: u64) -> u64; }
fn main() {
    let binders = [0u32, 5, 17, 42, 63, 1];
    let h = unsafe { vehje_engine_reachhash(binders.as_ptr(), binders.len(), 0xABCD) };
    println!("{}", h);
}
