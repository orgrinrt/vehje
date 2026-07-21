const std = @import("std");
pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    // the native content-validation tool
    const exe = b.addExecutable(.{
        .name = "validate_content",
        .root_module = b.createModule(.{ .root_source_file = b.path("validate_content.zig"), .target = target, .optimize = optimize }),
    });
    const run = b.addRunArtifact(exe);
    // wire it as a build step so `zig build validate` runs the native content validation
    const step = b.step("validate", "run native content validation (the three-loci content step)");
    step.dependOn(&run.step);
    // also make the default build depend on it (content validation gates the build)
    b.getInstallStep().dependOn(&run.step);
}
