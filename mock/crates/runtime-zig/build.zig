// vehje runtime build script.
//
// Produces libvehje_runtime.{so|dylib|dll} as a dynamic library
// that the Rust compiler loads via dlopen at distribution time.

const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const mod = b.createModule(.{
        .root_source_file = b.path("src/runtime.zig"),
        .target = target,
        .optimize = optimize,
    });

    const lib = b.addLibrary(.{
        .name = "vehje_runtime",
        .root_module = mod,
        .linkage = .dynamic,
    });

    b.installArtifact(lib);

    // `zig build test` runs the runtime's own unit tests.
    const mod_tests = b.addTest(.{ .root_module = mod });
    const run_mod_tests = b.addRunArtifact(mod_tests);
    const test_step = b.step("test", "Run runtime unit tests");
    test_step.dependOn(&run_mod_tests.step);
}
