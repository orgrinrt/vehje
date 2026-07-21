// SK1 side 1: "Rust emits VALIDATED DATA" (not Zig source). rustc typing this
// program is certification 1: a well-typed run can only emit consistent data.
// A language definition = families (name,id) + a target's supported family set.
// Validate (unique ids, id < 64 for the bitmask), then emit pure const DATA.
struct Family { name: &'static str, id: u8 }
fn main() {
    let families = [
        Family { name: "Core", id: 0 },
        Family { name: "Doc", id: 1 },
        Family { name: "Clausewitz", id: 2 },
    ];
    let target_supports: u64 = (1 << 0) | (1 << 1); // Core + Doc, NOT Clausewitz
    let mut seen = std::collections::HashSet::new();
    for f in &families {
        assert!(f.id < 64, "family id must fit the u64 mask");
        assert!(seen.insert(f.id), "duplicate family id {}", f.id);
    }
    let mut out = String::from("// AUTO-EMITTED by emit_data.rs (SK1). Pure const DATA.\n");
    out.push_str("pub const Family = struct { name: [:0]const u8, id: u8 };\n");
    out.push_str("pub const FAMILIES = [_]Family{\n");
    for f in &families { out.push_str(&format!("    .{{ .name = \"{}\", .id = {} }},\n", f.name, f.id)); }
    out.push_str("};\n");
    out.push_str(&format!("pub const TARGET_SUPPORTS: u64 = {};\n", target_supports));
    std::fs::create_dir_all("gen").unwrap();
    std::fs::write("gen/families_data.zig", out).unwrap();
    println!("cert-1 (rustc-typed emitter) ran: emitted {} families, target_supports={:#b}", families.len(), target_supports);
}
