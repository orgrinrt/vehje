## What each is

Pandoc is a Haskell library plus CLI parsing ~40 markup/document formats into one shared AST (`Pandoc Meta [Block]`) and rendering it to ~60 formats; the AST ships as a separately versioned package, `pandoc-types`. Djot is a smaller, later markup language by the same author (John MacFarlane), designed to parse in linear time with no backtracking, with two independent reference implementations: `djot.js` (TS) and `djoths` (Haskell, used by Pandoc's own `Djot` reader/writer). Djot's AST is far more structurally uniform than Pandoc's, and its filter API is a hand-written stack walker, not a typeclass.

## The AST

**Pandoc** (`pandoc-types`, `Text.Pandoc.Definition`):

```haskell
data Pandoc = Pandoc Meta [Block]
newtype Meta = Meta { unMeta :: M.Map Text MetaValue }
data MetaValue = MetaMap (M.Map Text MetaValue) | MetaList [MetaValue] | MetaBool Bool
               | MetaString Text | MetaInlines [Inline] | MetaBlocks [Block]
type Attr = (Text, [Text], [(Text, Text)])   -- id, classes, key-value pairs
newtype Format = Format Text                  -- case-insensitive Eq/Ord

data Block = Plain [Inline] | Para [Inline] | LineBlock [[Inline]]
    | CodeBlock Attr Text | RawBlock Format Text | BlockQuote [Block]
    | OrderedList ListAttributes [[Block]] | BulletList [[Block]]
    | DefinitionList [([Inline],[[Block]])] | Header Int Attr [Inline]
    | HorizontalRule | Table Attr Caption [ColSpec] TableHead [TableBody] TableFoot
    | Figure Attr Caption [Block] | Div Attr [Block]

data Inline = Str Text | Emph [Inline] | Underline [Inline] | Strong [Inline]
    | Strikeout [Inline] | Superscript [Inline] | Subscript [Inline] | SmallCaps [Inline]
    | Quoted QuoteType [Inline] | Cite [Citation] [Inline] | Code Attr Text
    | Space | SoftBreak | LineBreak | Math MathType Text | RawInline Format Text
    | Link Attr [Inline] Target | Image Attr [Inline] Target | Note [Block] | Span Attr [Inline]
```
14 Block, 19 Inline constructors. Only `Div`/`Span`/`CodeBlock`/`Header`/`Code`/`Link`/`Image`/`Table`/`Figure`/`Cell` carry `Attr`; `Str`, `Emph`, `Space`, `Math`, `Note` carry none, so which nodes are extensible is a per-constructor decision baked into the sum type.

**Djot** (`djot.js` `src/ast.ts`; `djoths` `src/Djot/AST.hs`): every node is attribute-bearing structurally. TS: `interface HasAttributes { attributes?; autoAttributes?; pos?; }`, all ~30 node interfaces extend it. Haskell wraps instead of fielding: `data Node a = Node Pos Attr a`, `type Inlines = Many (Node Inline)`, `type Blocks = Many (Node Block)`, `newtype Many a = Many (Seq a)`. Blocks: `Para | Section | Heading | BlockQuote | CodeBlock | Div | OrderedList | BulletList | TaskList | DefinitionList | ThematicBreak | Table | RawBlock`. Inlines: `Str | Emph | Strong | Highlight | Insert | Delete | Superscript | Subscript | Verbatim | Symbol | Math | Link | Image | Span | FootnoteReference | UrlLink | EmailLink | RawInline | NonBreakingSpace | Quoted | SoftBreak | HardBreak`. `Doc { docBlocks, docFootnotes :: NoteMap, docReferences/docAutoReferences :: ReferenceMap, docAutoIdentifiers }`. `div`/`span` match Pandoc's `Div`/`Span` as the generic containers, but attributes are uniform across every node, not special-cased.

## How a writer is written

The writer contract is one function type, dispatched by an association list, not a typeclass:

```haskell
data Writer m = TextWriter (WriterOptions -> Pandoc -> m Text)
              | ByteStringWriter (WriterOptions -> Pandoc -> m BL.ByteString)
writers :: PandocMonad m => [(Text, Writer m)]
```
A new writer is one function `WriterOptions -> Pandoc -> m Text` plus one tuple appended to `writers`. Nothing else required.

`Writers.Man` (379 lines) is the minimal case: `writeMan :: PandocMonad m => WriterOptions -> Pandoc -> m Text` runs `StateT WriterState m` (a small record: open footnotes, has-tables flag, font-feature flags), exhaustively matching all 14 `Block`/19 `Inline` constructors once each with no generic recursion (its own `blockListToMan`/`inlineListToMan` folds, bypassing `Walkable`). ~40 of 379 lines are import boilerplate; the rest is pure per-constructor rendering, since this style has no generic-traversal boilerplate at all.

`Writers.Markdown` is the complex case, split across `Markdown.hs` (976) + `Markdown/Inline.hs` (740) + `Markdown/Table.hs` (141) + `Markdown/Types.hs` (79) = 1936 lines, implementing four variants (markdown, commonmark, markua, **and plain**) as one shared `pandocToMarkdown` parameterized by `envVariant :: MarkdownVariant`:
```haskell
type MD m = ReaderT WriterEnv (StateT WriterState m)
writePlain opts document = evalMD (pandocToMarkdown opts document) def{ envVariant = PlainText } def
```
`Writers.Plain` does not exist as a standalone module; `writePlain` reuses ~1900 lines of markdown logic under a mode flag, not a separate small writer. Pandoc's own `Writers/Djot.hs` (300 lines) is a third shape: an adapter mapping the Pandoc AST into the standalone `djoths` package's `Djot.AST`, calling that library's `renderDjot`; most syntax knowledge lives outside Pandoc.

**Reader**, same treatment: `readMan :: (PandocMonad m, ToSources a) => ReaderOptions -> a -> m Pandoc` (561 lines), a two-stage pipeline: `lexRoff` tokenizes to `[RoffToken]`, then `ParsecT [RoffToken] ManState m` parses combinator-style. Readers build the AST via `Text.Pandoc.Builder`, a Monoid smart-constructor layer (not raw list-consing) shared by every reader.

## The Walkable traversal machinery (`Text.Pandoc.Walk`)

```haskell
class Walkable a b where
  walk  :: (a -> a) -> b -> b
  walk f = runIdentity . walkM (return . f)
  walkM :: (Monad m, Applicative m, Functor m) => (a -> m a) -> b -> m b
  query :: Monoid c => (a -> c) -> b -> c
  {-# MINIMAL walkM, query #-}
```
A two-parameter typeclass, "walk `a`-shaped things inside a `b`," hand-instantiated: ~90 explicit `instance Walkable a b` declarations cover every (needle, haystack) pair (`Inline`-in-`Block`, `[Block]`-in-`Pandoc`, `Inline`-in-`MetaValue`, `Block`-in-`Citation`, …), plus one generic `(Foldable t, Traversable t, Walkable a b) => Walkable a (t b)` for lists/`Maybe`, and one for pairs. Per-constructor recursion is a compiler-checked pattern match:
```haskell
walkBlockM f (Para xs)  = Para <$> walkM f xs
walkBlockM _ x@RawBlock {} = return x   -- leaves unchanged
walkBlockM f (Table attr capt as hs bs fs) = do
  capt' <- walkM f capt; hs' <- walkM f hs; bs' <- walkM f bs; fs' <- walkM f fs
  return $ Table attr capt' as hs' bs' fs'
```
`query` folds via Monoid, applying `f` to self and combining with recursion: `query f x = f x <> queryBlock f x`. Because `walkBlockM` is exhaustive, a new `Block` constructor forces a compile error until handled. Pandoc ships a second, slower generic traversal, `Text.Pandoc.Generic` (SYB reflection over `Data.Data`), which Walk.hs's own haddock says Walkable beats "by a factor of four or five"; the hand-written instances exist because the generic path was too slow, not because generics were unavailable.

## Filters and extension points

**Pandoc Lua**: a filter is a Lua table keyed by constructor name (`Str`, `Para`, `Meta`, `Pandoc`) plus wildcards `Inline`/`Block`. Default ("typewise") order is fixed type-tier bottom-up, not source order: "functions for Inline elements, then the `Inlines` filter function, then functions for Block elements, then `Blocks`, then `Meta`, then last `Pandoc`." `traverse = 'topdown'` switches to depth-first top-down; returning `false` as a second value there prunes children. Return protocol: nil = unchanged, one element = replace (same type), list = splice (empty = delete). Multiple `--lua-filter` flags run as separate passes in CLI order; chaining filters from one returned array still works but is discouraged.

**Djot.js**: an object keyed by lowercase `tag`, value a plain function (implicit bottom-up "exit") or `{enter, exit}` for real top-down/bottom-up pairing (example given: a counter on `enter`/`exit` around emph driving context-sensitive capitalization). Return semantics mirror Pandoc's, plus a unique `{stop: value}` an `enter` handler returns to both supply the final value and skip descending into children in one object. Traversal is a hand-written mutable-stack `Walker` (`stack: {node, childIndex}[]`) doing duck-typed `"children" in node` recursion, generalizing over any homogeneous `children: A[]` array rather than per-constructor dispatch; no Walkable-equivalent typeclass. A filter file may return an array of filter objects applied sequentially, its only multi-filter shape.

**Attr / Div / Span**: Pandoc's `Div [Block]`/`Span [Inline]` are documented in-source as "Generic block/inline container with attributes," the two constructors whose only payload beyond children is `Attr`. `Attr`'s kv-pairs are `Text -> Text`, so structured extension data is stringly-typed and re-parsed by convention (citation processors, syntax-highlight classes), no schema slot; every other Attr-bearing constructor (`Header`, `Link`, `Table`) is a dedicated typed constructor Attr merely augments. Djot inverts this: attributes attach to *every* node (`Node Pos Attr a` / `HasAttributes`), `div`/`span` stay the designated generic containers, but "can this carry `{#id .class k=v}`" stops being a per-constructor decision.

**RawBlock/RawInline `Format`**: `newtype Format Text`, case-insensitive `Eq`/`Ord`. Each writer decides per-node whether raw content is theirs by literal string match: `blockToMan _ b@(RawBlock f str) | f == Format "man" = return $ literal str | otherwise = report (BlockNotRendered b) >> return empty`. No central registry; convention plus a per-writer guard, falling through to a warning, not an error. Djot's raw nodes carry the equivalent `format: string` field directly.

## Mechanisms worth naming (with tradeoffs)

1. **Hand-instantiated `Walkable`, not derived.** 4-5x faster than the SYB fallback; costs manual threading of every new constructor through every relevant instance.
2. **`ReaderT WriterEnv (StateT WriterState m)` per writer.** Scoped context plus mutable accumulation via `asks`/`local`/`gets`/`modify`; couples each writer's helpers to its own stack (Man's bare `StateT` and Markdown's `MD m` are not interchangeable).
3. **Association-list writer dispatch, not a registry typeclass.** Trivial to extend; untyped, so an unmatched name is a runtime `PandocUnknownWriterError`, not a compile error.
4. **Djot's `Node Pos Attr a` wrapper vs. Pandoc's per-constructor field.** One place defines attribute attachment with a real `Semigroup` class-merge; a new node type gets it free by being wrapped, vs. Pandoc deciding extensibility constructor-by-constructor.
5. **Djot's mutable-stack `Walker` vs. Pandoc's exhaustive pattern match.** Array-surgery splice/prune generalizing over homogeneous children, but nothing forces every shape to be handled; Pandoc's `walkBlockM` fails to compile if a constructor is left unmatched.

## What would not transfer to no_std/no-alloc Rust

Every Pandoc list field (`[Block]`, `[Inline]`, `[[Block]]`) is a heap-allocated, GC-reclaimed cons list; `Meta` is a `Data.Map` tree; Djot's `Many a = Many (Seq a)` is a finger tree chosen for cheap `Semigroup` concatenation, and Pandoc's Builder module leans on the same idea. None have compile-time-known size; none exist without a GC or an allocator standing in for one. `Walkable`'s instance search is ad hoc polymorphism with no monomorphization-only Rust analogue; a port needs dynamic dispatch through a closed vtable of node kinds, or generated per-type-pair recursion, since no single trait covers ~90 pairs otherwise. `walk`'s reduction to `walkM` via `Identity` assumes closures are cheap heap objects under GHC's evaluation model: efficient for a GC'd language, not zero-allocation. `Text.Pandoc.Generic`'s SYB traversal is runtime reflection over `Data.Data` witnesses with no no_std analogue at all. `djoths`'s `Attr = [(ByteString,ByteString)]` is a heap-allocated association list per node, fresh-allocating on class-merge; `djot.js`'s tag/attribute tables are V8 hash maps. Man's Parsec reader is a backtracking combinator parser over an unbounded token list, relying on `Alternative`/laziness to discard abandoned branches under GC. Both systems answer "how is a document held and walked" uniformly: an unbounded recursive sum type, heap-allocated, walked by boxed closures or runtime reflection, with growable lists/maps/seqs at every level.

## Sources

- pandoc-types `Text.Pandoc.Definition`: https://raw.githubusercontent.com/jgm/pandoc-types/master/src/Text/Pandoc/Definition.hs
- pandoc-types `Text.Pandoc.Walk`: https://raw.githubusercontent.com/jgm/pandoc-types/master/src/Text/Pandoc/Walk.hs
- pandoc `Writers.Man`: https://raw.githubusercontent.com/jgm/pandoc/main/src/Text/Pandoc/Writers/Man.hs
- pandoc `Writers.Markdown` (+`/Inline.hs`, `/Table.hs`, `/Types.hs`): https://raw.githubusercontent.com/jgm/pandoc/main/src/Text/Pandoc/Writers/Markdown.hs
- pandoc `Writers.Djot`: https://raw.githubusercontent.com/jgm/pandoc/main/src/Text/Pandoc/Writers/Djot.hs
- pandoc `Writers` dispatch (`Writer`/`writers`/`getWriter`): https://raw.githubusercontent.com/jgm/pandoc/main/src/Text/Pandoc/Writers.hs
- pandoc `Readers.Man`: https://raw.githubusercontent.com/jgm/pandoc/main/src/Text/Pandoc/Readers/Man.hs
- Pandoc Lua filters: https://pandoc.org/lua-filters.html
- djot.js AST: https://raw.githubusercontent.com/jgm/djot.js/main/src/ast.ts
- djot.js filter API: https://raw.githubusercontent.com/jgm/djot.js/main/src/filter.ts
- djoths AST: https://raw.githubusercontent.com/jgm/djoths/main/src/Djot/AST.hs
- jgm/djot README: https://github.com/jgm/djot/blob/main/README.md
- MacFarlane, "Beyond Markdown": https://johnmacfarlane.net/beyond-markdown.html
