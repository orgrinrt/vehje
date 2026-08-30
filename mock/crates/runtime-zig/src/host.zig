//! The host boundary: how a value reaches a family handler and comes back.
//!
//! A family operation is dispatched to whoever declared the family, so the
//! runtime's job here is marshalling rather than meaning. A scalar crosses in
//! the operand's payload word, a string as bytes plus a length, and a compound
//! as a value image plus its length, which is the string convention generalised
//! rather than a new one, so the operand record keeps one shape for all three.
//!
//! A compound crosses as a value image in BOTH directions. That is what keeps
//! one decoder at the boundary: the image arriving from a handler is validated
//! by the same reader that validates a value leaving for the host, rather than
//! by a second decoder written to the same specification and trusted to agree.

const rt = @import("runtime.zig");
const value_image = @import("value_image.zig");
const varena = @import("value_arena.zig");

const Value = rt.Value;
const EvalError = rt.EvalError;
const Session = rt.Session;

/// One operand crossing to a handler: a kind tag, a payload word, and a bytes
/// pointer.
///
/// For a scalar the pointer is null and the payload is the value; for a string
/// the pointer is the bytes and the payload is their length. One record serves
/// both, so the common case is unchanged in size and there is no second
/// parallel form to keep in agreement. The tag values are the value image's
/// tags, so one vocabulary describes a value wherever it appears.
pub const VehjeOperand = extern struct {
    tag: u32,
    payload: i64,
    bytes: ?[*]const u8 = null,
};

/// Operand tag codes, matching `vehje-runtime-abi`'s `ValueTag`.
pub const SCALAR_UNIT: u32 = 0;
pub const SCALAR_BOOL: u32 = 1;
pub const SCALAR_INT: u32 = 2;
pub const SCALAR_STR: u32 = 3;
/// The compound tags. A compound operand's `bytes` is a value image and its
/// `payload` is that image's byte length, which is the string convention
/// generalised rather than a new one, so the operand record keeps its shape.
pub const COMPOUND_SEQ: u32 = 4;
pub const COMPOUND_RECORD: u32 = 5;

/// The host a family operation is dispatched to.
///
/// One callback plus opaque userdata, mirroring the sink: the host owns the
/// context, and the runtime retains nothing past the call. The callback returns
/// zero on success, having written the produced operand through `out`.
pub const VehjeHost = extern struct {
    call: *const fn (
        userdata: ?*anyopaque,
        family: u32,
        args: [*]const VehjeOperand,
        argc: usize,
        out: *VehjeOperand,
    ) callconv(.c) i32,
    userdata: ?*anyopaque,
};

/// The operand form of a value, or an error when it has none (a closure).
///
/// A compound is written out as a value image into the session scratch, so the
/// handler receives exactly the format the runtime itself reads back. That is
/// what keeps one decoder at the boundary instead of two agreeing by contract.
pub fn toOperand(v: Value, arena: *const varena.ValueArena, session: ?*Session) EvalError!VehjeOperand {
    return switch (v) {
        .unit => VehjeOperand{ .tag = SCALAR_UNIT, .payload = 0 },
        .boolean => |b| VehjeOperand{ .tag = SCALAR_BOOL, .payload = @intFromBool(b) },
        .int => |n| VehjeOperand{ .tag = SCALAR_INT, .payload = n },
        .str => |b| VehjeOperand{ .tag = SCALAR_STR, .payload = @intCast(b.len), .bytes = b.ptr },
        .compound => |root| blk: {
            const sess = session orelse return EvalError.NoScratch;
            const room = sess.scratch[sess.used..];
            const n = varena.writeTree(room, arena, root) catch return EvalError.ScratchFull;
            const img = room[0..n];
            sess.used += n;
            const tag: u32 = switch (arena.node(root).tag) {
                .seq => COMPOUND_SEQ,
                else => COMPOUND_RECORD,
            };
            break :blk VehjeOperand{ .tag = tag, .payload = @intCast(n), .bytes = img.ptr };
        },
        .closure => EvalError.Unsupported,
    };
}

/// The value a returned operand denotes.
///
/// A returned string's bytes exist only for the duration of the call, so they
/// are copied into the session scratch and the value names the copy.
pub fn fromOperand(o: VehjeOperand, session: ?*Session, arena: ?*varena.ValueArena) EvalError!Value {
    return switch (o.tag) {
        SCALAR_UNIT => Value.unit,
        SCALAR_BOOL => Value{ .boolean = o.payload != 0 },
        SCALAR_INT => Value{ .int = o.payload },
        SCALAR_STR => blk: {
            const p = o.bytes orelse return EvalError.Unsupported;
            const s = session orelse return EvalError.NoScratch;
            break :blk Value{ .str = try s.keep(p[0..@intCast(o.payload)]) };
        },
        COMPOUND_SEQ, COMPOUND_RECORD => blk: {
            const p = o.bytes orelse return EvalError.Unsupported;
            const a = arena orelse return EvalError.ValueArenaFull;
            break :blk Value{ .compound = try materialise(p[0..@intCast(o.payload)], a) };
        },
        else => EvalError.Unsupported,
    };
}

/// Walk an untrusted value image into the value arena, returning the root's
/// arena index.
///
/// The image is validated first, so every index and span is in range and the
/// children-before-parents order holds. That order is what makes the walk a
/// single forward pass: a node's children already have arena indices by the
/// time the node itself is built, so nothing is back-patched and no recursion
/// is needed.
fn materialise(bytes: []const u8, arena: *varena.ValueArena) EvalError!u32 {
    const r = value_image.Reader.parse(bytes) catch return EvalError.BadImage;
    r.validate() catch return EvalError.BadImage;

    // maps an image node index to its arena index; the forward pass fills it in
    // order, and a child is always at a lower index than its parent.
    var map: [MATERIALISE_CAP]u32 = undefined;
    if (r.node_count > MATERIALISE_CAP) return EvalError.ValueArenaFull;

    var i: u32 = 0;
    while (i < r.node_count) : (i += 1) {
        const tag = r.tagOf(i) catch return EvalError.BadImage;
        const blob = r.blob(i) catch return EvalError.BadImage;
        map[i] = switch (tag) {
            .unit => try arena.allocUnit(),
            .boolean => try arena.allocBool(blob.len > 0 and blob[0] != 0),
            .int => try arena.allocInt(readI64(blob)),
            .str => try arena.allocStr(blob),
            .seq, .record, .outcome => blk: {
                var kids: [MATERIALISE_ARITY]u32 = undefined;
                const len = r.childLen(i);
                if (len > MATERIALISE_ARITY) return EvalError.ValueArenaFull;
                var k: u32 = 0;
                while (k < len) : (k += 1) {
                    const c = r.child(i, k) catch return EvalError.BadImage;
                    kids[k] = map[c];
                }
                break :blk switch (tag) {
                    .seq => try arena.allocSeq(kids[0..len]),
                    else => try arena.allocRecord(kids[0..len]),
                };
            },
        };
    }
    return map[r.root];
}

/// The widest image a handler may return, and the widest compound in it. Both
/// are lent bounds like every other here: exceeding one is a named refusal, not
/// a growth point.
const MATERIALISE_CAP: usize = 1024;
const MATERIALISE_ARITY: usize = 64;

fn readI64(b: []const u8) i64 {
    var bits: u64 = 0;
    var i: usize = 0;
    while (i < b.len and i < 8) : (i += 1) bits |= @as(u64, b[i]) << @intCast(i * 8);
    return @bitCast(bits);
}
