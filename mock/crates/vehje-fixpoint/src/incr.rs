//! The incremental scaling layer: differential maintenance over a host set.
//!
//! This is the engine's differential evaluation mode plus content-addressed
//! keying, a module here rather than a separate crate because it is the same
//! engine in differential mode, not a new mechanism. Cross-artifact dedup keys
//! shared subtrees by opaque content-hash values the client supplies (an
//! `arvo` content hash a client's structural hashing produces), so a shared
//! subtree compiles once and a cache hit is a proven-valid content-addressed
//! artifact. The engine keys on content addresses without naming any IR type,
//! so its layering holds.
//!
//! The framework owns the mechanism (keying, dedup, differential recompute).
//! The host owns the load policy and the storage backend, expressed as the
//! `Store` and `ArtifactSet` traits, so no I/O is baked in.

use arvo::strategy::Hot;
use arvo::{Bits, Bool, USize};
use arvo_hash::{ContentHash, xxhash3_64};
use hilavitkutin_api::ColumnValue;
use notko::Maybe;

/// The host-lent storage backend, keyed by content address.
///
/// The host chooses where artifacts live (memory, mmap, a cache tier); the
/// engine only asks it to look up and store by content key. A `get` miss is
/// `Maybe::Isnt`; a `put` reports whether the key was newly stored.
pub trait Store {
    /// The artifact payload stored under a content key.
    type Artifact: ColumnValue;

    /// Look up the artifact for `key`, or `Maybe::Isnt` on a miss.
    fn get(&self, key: ContentHash) -> Maybe<Self::Artifact>;

    /// Store `artifact` under `key`. `Bool::TRUE` when newly stored.
    fn put(&mut self, key: ContentHash, artifact: Self::Artifact) -> Bool;
}

/// The host-declared set of artifacts to maintain.
///
/// The host enumerates the artifacts (each named by its content key) that the
/// incremental layer should ensure are present. The engine walks the set; the
/// host decides its membership and order.
pub trait ArtifactSet {
    /// The number of artifacts in the set.
    fn len(&self) -> USize;
    /// The content key of the artifact at position `at` (`at < len`).
    fn key_at(&self, at: USize) -> ContentHash;
}

/// Differential maintenance of the compile-stage relations over a host set.
///
/// Holds the count of content addresses recomputed since construction (the
/// dedup and invalidation counter). An edit re-derives only its transitive
/// dependents; a shared subtree keyed to an already-stored content address is
/// a cache hit and is not recomputed.
pub struct Incremental {
    recomputed: USize,
}

impl Incremental {
    /// A fresh incremental layer with nothing recomputed yet.
    pub fn new() -> Self {
        Self { recomputed: USize(0) }
    }

    /// The count of content addresses recomputed since construction.
    pub fn recomputed(&self) -> USize {
        self.recomputed
    }

    /// Content-address a byte payload via the substrate hash.
    ///
    /// The engine treats the result as opaque: it keys on the content address
    /// and never inspects what produced it, so the layering that keeps this
    /// crate free of any IR type holds. A client with its own structural hash
    /// supplies the `ContentHash` directly instead of calling this.
    pub fn key_of(bytes: &[u8]) -> ContentHash { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a byte image is the content-hash input; bytes are the hash unit; tracked: #207
        // the 64-bit content-hash word at its construction boundary.
        Bits::<64, Hot>::from_raw(xxhash3_64(bytes)) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: content-hash raw 64-bit word at the arvo-hash construction boundary; tracked: #207
    }

    /// Warm-load the artifact set additively, recomputing only content misses.
    ///
    /// For each artifact in the set, a store hit is a proven-valid
    /// content-addressed artifact reused as-is; a miss is produced (via
    /// `produce`) and stored, and counts as one recompute. Returns the running
    /// recompute total. This is the additive-edit case: an edit adds and
    /// re-derives its transitive dependents.
    // FIXME: general edit with retraction (a deletion that must un-derive what
    // depended on it) needs counted differential dataflow and is deferred
    // until a consumer's edit pattern needs it (see BACKLOG). This ships the
    // additive case only.
    pub fn warm_load<A, S, P>(&mut self, set: &A, store: &mut S, produce: P) -> USize
    where
        A: ArtifactSet,
        S: Store,
        P: Fn(ContentHash) -> S::Artifact,
    {
        let len = set.len();
        let mut i = USize(0);
        while i < len {
            let key = set.key_at(i);
            if store.get(key).isnt() {
                let artifact = produce(key);
                if store.put(key, artifact).0 {
                    self.recomputed = self.recomputed + USize(1);
                }
            }
            i = i + USize(1);
        }
        self.recomputed
    }
}

impl Default for Incremental {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ArrayStore {
        keys: [ContentHash; 4],
        vals: [USize; 4],
        len: USize,
    }
    impl ArrayStore {
        fn new() -> Self {
            Self {
                keys: [Incremental::key_of(b""); 4],
                vals: [USize(0); 4],
                len: USize(0),
            }
        }
        fn find(&self, key: ContentHash) -> Maybe<USize> {
            let mut i = USize(0);
            while i < self.len {
                if self.keys[i.0] == key {
                    return Maybe::Is(self.vals[i.0]);
                }
                i = i + USize(1);
            }
            Maybe::Isnt
        }
    }
    impl Store for ArrayStore {
        type Artifact = USize;
        fn get(&self, key: ContentHash) -> Maybe<USize> {
            self.find(key)
        }
        fn put(&mut self, key: ContentHash, artifact: USize) -> Bool {
            if self.find(key).is() {
                return Bool(false);
            }
            self.keys[self.len.0] = key;
            self.vals[self.len.0] = artifact;
            self.len = self.len + USize(1);
            Bool(true)
        }
    }

    struct TwoArtifacts {
        a: ContentHash,
        b: ContentHash,
    }
    impl ArtifactSet for TwoArtifacts {
        fn len(&self) -> USize {
            USize(2)
        }
        fn key_at(&self, at: USize) -> ContentHash {
            if at == USize(0) { self.a } else { self.b }
        }
    }

    #[test]
    fn warm_load_computes_each_key_once() {
        let set = TwoArtifacts {
            a: Incremental::key_of(b"alpha"),
            b: Incremental::key_of(b"beta"),
        };
        let mut store = ArrayStore::new();
        let mut incr = Incremental::new();

        // first load: both are misses, both recomputed.
        let first = incr.warm_load(&set, &mut store, |_k| USize(7));
        assert_eq!(first, USize(2));

        // second load: both are hits, recompute total unchanged.
        let second = incr.warm_load(&set, &mut store, |_k| USize(7));
        assert_eq!(second, USize(2));
        assert_eq!(incr.recomputed(), USize(2));
    }

    #[test]
    fn key_of_is_stable_and_distinguishing() {
        assert_eq!(Incremental::key_of(b"same"), Incremental::key_of(b"same"));
        assert!(Incremental::key_of(b"one") != Incremental::key_of(b"two"));
    }
}
