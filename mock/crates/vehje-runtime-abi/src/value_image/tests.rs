//! Tests for the value-image writer and reader.

use super::*;
use arvo::Identity;

/// Build the arena for `record { unit, true, 7 }`, write it, and read it
/// back. Children come before their parent, which is the ordering the
/// acyclicity check depends on.
fn record_arena_bytes(out: &mut [u8]) -> USize { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test wire buffer; tracked: #207
    let blob: [u8; 9] = [1, 7, 0, 0, 0, 0, 0, 0, 0]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test scalar payload bytes; tracked: #207
    let nodes = [
        ValueNode {
            tag: ValueTag::Unit,
            region: RegionId(USize::ZERO),
            children: ValueList::EMPTY,
            blob: BlobSpan::EMPTY,
        },
        ValueNode {
            tag: ValueTag::Bool,
            region: RegionId(USize::ZERO),
            children: ValueList::EMPTY,
            blob: BlobSpan { offset: USize(0), len: USize(1) },
        },
        ValueNode {
            tag: ValueTag::Int,
            region: RegionId(USize::ZERO),
            children: ValueList::EMPTY,
            blob: BlobSpan { offset: USize(1), len: USize(INT_BYTES) },
        },
        ValueNode {
            tag: ValueTag::Record,
            region: RegionId(USize::ZERO),
            children: ValueList { start: USize::ZERO, len: USize(3) },
            blob: BlobSpan::EMPTY,
        },
    ];
    let pool = [ValueRef::new(USize(0)), ValueRef::new(USize(1)), ValueRef::new(USize(2))];
    let regions = [Region { start: USize::ZERO, len: USize(4) }];
    let arena = ValueArena::new(&nodes, &pool, &blob, &regions, ValueRef::new(USize(3)));
    match write_value(&arena, out) {
        Maybe::Is(n) => n,
        Maybe::Isnt => panic!("buffer should hold the image"),
    }
}

#[test]
fn a_value_round_trips_through_the_image() {
    let mut buf = [0u8; 256]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test wire buffer; tracked: #207
    let written = record_arena_bytes(&mut buf);
    let img = match ValueImage::parse(&buf[..written.0]) {
        Maybe::Is(i) => i,
        Maybe::Isnt => panic!("the written image should parse"),
    };
    assert!(img.validate().0);
    assert_eq!(img.node_count(), USize(4));

    let root = match img.try_node(img.root()) {
        Maybe::Is(n) => n,
        Maybe::Isnt => panic!("root should read"),
    };
    assert_eq!(root.tag, ValueTag::Record);
    assert_eq!(root.children.len, USize(3));

    let leaf = |k: USize| match img.child(root.children, k) {
        Maybe::Is(c) => match img.try_node(c) {
            Maybe::Is(n) => n,
            Maybe::Isnt => panic!("child should read"),
        },
        Maybe::Isnt => panic!("child index should resolve"),
    };
    assert_eq!(leaf(USize(0)).tag, ValueTag::Unit);
    assert_eq!(img.as_bool(leaf(USize(1))), Maybe::Is(Bool(true)));
    assert_eq!(img.as_int(leaf(USize(2))), Maybe::Is(Int::<64, Hot>::from_raw(7)));
}

#[test]
fn the_writer_refuses_a_buffer_one_byte_too_small() {
    let mut big = [0u8; 256]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test wire buffer; tracked: #207
    let written = record_arena_bytes(&mut big);
    let mut small = [0u8; 256]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test wire buffer; tracked: #207
    let nodes = [ValueNode {
        tag: ValueTag::Unit,
        region: RegionId(USize::ZERO),
        children: ValueList::EMPTY,
        blob: BlobSpan::EMPTY,
    }];
    let arena = ValueArena::new(&nodes, &[], &[], &[], ValueRef::new(USize::ZERO));
    let need = HEADER_WORDS * WORD + NODE_WORDS * WORD;
    assert_eq!(write_value(&arena, &mut small[..need - 1]), Maybe::Isnt);
    assert!(matches!(write_value(&arena, &mut small[..need]), Maybe::Is(_)));
    assert!(written.0 > need);
}

#[test]
fn a_wrong_magic_is_refused() {
    let mut buf = [0u8; 256]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test wire buffer; tracked: #207
    let written = record_arena_bytes(&mut buf);
    put_u32(&mut buf, 0, MAGIC ^ 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: corrupting the magic under test; tracked: #207
    assert_eq!(ValueImage::parse(&buf[..written.0]), Maybe::Isnt);
}

#[test]
fn a_header_claiming_more_than_it_carries_is_refused() {
    let mut buf = [0u8; 256]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test wire buffer; tracked: #207
    let written = record_arena_bytes(&mut buf);
    put_u32(&mut buf, 2 * WORD, 99); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: overstating the node count under test; tracked: #207
    assert_eq!(ValueImage::parse(&buf[..written.0]), Maybe::Isnt);
}

#[test]
fn a_child_at_or_above_its_parent_is_refused() {
    // The writer emits children before parents, so a child index that is not
    // strictly below its parent's is the cycle the ordering forbids.
    let blob: [u8; 0] = []; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test wire blob bytes; tracked: #207
    let nodes = [
        ValueNode {
            tag: ValueTag::Unit,
            region: RegionId(USize::ZERO),
            children: ValueList::EMPTY,
            blob: BlobSpan::EMPTY,
        },
        ValueNode {
            tag: ValueTag::Seq,
            region: RegionId(USize::ZERO),
            children: ValueList { start: USize::ZERO, len: USize(1) },
            blob: BlobSpan::EMPTY,
        },
    ];
    // The sequence at index 1 names index 1 as its own child.
    let pool = [ValueRef::new(USize(1))];
    let regions = [Region { start: USize::ZERO, len: USize(2) }];
    let arena = ValueArena::new(&nodes, &pool, &blob, &regions, ValueRef::new(USize(1)));
    let mut buf = [0u8; 256]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test wire buffer; tracked: #207
    let written = match write_value(&arena, &mut buf) {
        Maybe::Is(n) => n,
        Maybe::Isnt => panic!("buffer should hold the image"),
    };
    let img = match ValueImage::parse(&buf[..written.0]) {
        Maybe::Is(i) => i,
        Maybe::Isnt => panic!("the header is well-formed"),
    };
    assert!(!img.validate().0);
}

#[test]
fn spans_past_their_section_are_refused() {
    let mut buf = [0u8; 256]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test wire buffer; tracked: #207
    let written = record_arena_bytes(&mut buf);
    let img = match ValueImage::parse(&buf[..written.0]) {
        Maybe::Is(i) => i,
        Maybe::Isnt => panic!("the written image should parse"),
    };
    let past = BlobSpan { offset: USize(0), len: USize(999) };
    assert_eq!(img.blob(past), Maybe::Isnt);

    // An Int whose payload is not eight bytes is refused rather than read
    // from whatever bytes happen to follow.
    let narrow = ValueNode {
        tag: ValueTag::Int,
        region: RegionId(USize::ZERO),
        children: ValueList::EMPTY,
        blob: BlobSpan { offset: USize(0), len: USize(2) },
    };
    assert_eq!(img.as_int(narrow), Maybe::Isnt);
}
