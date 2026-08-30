[KVD spec](../README.md) — section 08

## 8. Operations

This section defines the four operations a conforming KVD implementation must
provide. They are listed in dependency order: each operation builds on the
previous one.

### 8.1 Parse

**Input:** UTF-8 text.  
**Output:** a document — a root mapping node whose values are scalars, nested
mappings, or lists (§4).  
**Errors:** parse errors carrying a `line:col` position and a category from
§6. A document with any parse error has no defined value.

Normalization performed during parse:

- CRLF is normalized to LF.
- Comments and trailing whitespace are discarded.
- Dotted keys are expanded into nested maps.
- The `__schema__` metakey, if present, is validated and kept as an ordinary
  entry of the root map.

The output is a lossless node tree: re-emitting it (§8.2) produces canonical
KVD text that parses back to an equal tree.

### 8.2 Emit

**Input:** a document node tree.  
**Output:** canonical KVD text.  
**Errors:** only if a map key cannot be represented as a valid KVD key (e.g.
empty string).

Canonical form rules:

- All string scalars are double-quoted, except the six builtin type names
  (`int`, `float`, `bool`, `str`, `list`, `map`), which stay bare — they are
  meaningful only in schema position (§5). A data string whose text equals one
  of these names is therefore also emitted bare, but re-parses to an equal
  `Str` scalar, so the value round-trips.
- Multi-line strings use `"""` blocks.
- Keys are unquoted when they satisfy the key grammar (§3), quoted otherwise.
- Indentation is exactly 2 spaces per level.
- List items are prefixed with `- `.
- No trailing whitespace; exactly one trailing newline.
- No comments are emitted.

Emit is deterministic: equal node trees produce identical text. Round-trip
invariant: `emit(parse(text))` equals `emit(parse(emit(parse(text))))`.

### 8.3 Verify

**Input:** a document node tree and a schema node tree (both produced by
parse).  
**Output:** success, or a non-empty list of violations. Each violation carries
the dotted path of the offending node and a human-readable message (§6).  
**Errors:** `verify` returns `VerifyError`. When either input fails to parse
it is `ParseDoc` / `ParseSchema`. Once both parse, a *malformed schema* (e.g. a
quoted or numbered type leaf, a bare `list`/`map` leaf, a descriptor missing its
`type`, or an unknown type name) is reported as `VerifyError::SchemaMalformed` —
distinct from `VerifyError::Violations`, which covers a well-formed schema
applied to a non-conforming document. A self-describing document can be checked
against its own `__schema__` entry with `verify_embedded` (spec §8.1).

Verification is a separate pass, never part of parse. A document that parses
without error may still fail verification.

The verifier walks the schema tree and for every path checks:

- The data has no keys absent from the schema.
- All required schema keys are present in the data.
- Each scalar's shape matches its declared builtin type.
- `null` only appears under an optional type (`optional: true`).

### 8.4 Typed round-trip (serde)

**Input (deserialize):** KVD text and a target type `T`.  
**Output:** a value of type `T`.  
**Input (serialize):** a value of type `T`.  
**Output:** canonical KVD text.

This operation is composed from the primitives above: serialize builds a node
tree from `T`, then emits it; deserialize parses the text into a node tree,
then maps it onto `T`. Shape enforcement is strict — no coercion between
shapes (e.g. an integer field rejects a quoted-string value).

This operation is optional for implementations that do not support a type
system or reflection layer.

### 8.5 Document operations (programmatic editing)

Beyond the four core operations, an implementation may expose a small
editing surface over the parsed node tree (the `Node`/`Map`/`List` model of
§5). This section specifies the path syntax and the operations; it is
normative for any implementation that claims to provide them.

These operations are defined by their **observable behavior on the KVD data
model**, not by any programming-language API. The names `get`, `set`,
`remove`, and `remove_recursive` are conventional labels for the described
semantics; a conforming implementation in any language must produce the same
results (same node located, same replacement, same pruning behavior) given
the same document and path. The Rust `kvd::ops` module is one such
implementation, not the definition.

#### Path syntax

A **path** addresses a node inside a document. It is a sequence of
segments, written as a single string:

- Map keys are dotted segments: `a.b.c`
- List items use bracket indices: `a[0]`, `a.b[2]`
- Segments combine freely: `a.b[0].c`, and nested lists are allowed
  (`a[0][1]`)

Rules:

- Keys must satisfy the key grammar (§3); a key containing a dot is
  impossible because the dot is the segment separator.
- Indices are zero-based and non-negative. A negative, non-numeric, or
  out-of-range index is a path error, never silent truncation or append.
- An empty path refers to the document root.

#### Operations

| Operation | Effect |
|---|---|
| `get(doc, path)` | Returns the node at `path`, or absent if any segment is missing or mistyped. |
| `get_mut(doc, path)` | Returns a mutable borrow of the node at `path`, or absent. |
| `set(doc, path, value)` | Writes `value` at `path`. Intermediate maps are created as needed (like dotted-key expansion at parse time). A list index in the path must already exist; an out-of-range index is an error. An existing node at the target is replaced. |
| `remove(doc, path)` | Removes and returns the node at `path`. Ancestor maps that become empty are **left in place** as `{}` (not pruned). |
| `remove_recursive(doc, path)` | Like `remove`, but prunes now-empty ancestor **maps** recursively up the chain. List elements are never pruned by emptiness. |

Errors (carried as an operation error, distinct from parse errors): a
malformed path (`BadPath`), a missing map key (`MissingKey`), an out-of-range
list index (`IndexOutOfBounds`), or a node of the wrong shape where a map or
list was required (`NotAMap` / `NotAList`).

#### Document merging — deferred to §11

Merging two documents is **out of scope for 1.0**. The design is specified in
[§11](spec/11-merge.md) (planned): the merge policy is declared **per field in
the schema** (not passed as a parameter), with strategies `replace`, `deep`,
`append`, `union`, and `by-key`, and a safe default of `replace`. Until then,
`set` (which replaces a target node wholesale) is the only write primitive.
