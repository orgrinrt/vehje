// vehje runtime build script.
//
// Produces libvehje_runtime.{so|dylib|dll} as a dynamic library
// that the Rust compiler loads via dlopen at distribution time.
// Stub for R2; full build logic lands with Phase 6 impl.

const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const lib = b.addSharedLibrary(.{
        .name = "vehje_runtime",
        .root_source_file = b.path("src/runtime.zig"),
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(lib);
}
