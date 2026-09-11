[KVD spec](../../README.md), section 09

## 8. Operations

This section defines the four operations a conforming KVD implementation must
provide. They are listed in dependency order: each operation builds on the
previous one.

### 9.1 Parse

**Input:** UTF-8 text.
**Output:** a document that is a trie of node prefixes ending in keys whose
values are scalars, lists, or dicts (§04).
**Errors:** parse errors carrying a `line:col` position and a category from
§06. A document with any parse error has no defined value.

Normalization performed during parse (the parser MUST perform all three;
the emitter MUST NOT produce comments or trailing whitespace):

- CRLF is normalized to LF.
- Comments and trailing whitespace are discarded.
- Dotted keys are expanded into node prefixes.

The output is a lossless node tree: re-emitting it (§08.2) produces canonical
KVD text that parses back to an equal tree.

```kvd
a.b: 1
```

```kvd
# error: a document with any parse error has no defined value
a: "unterminated
```

### 9.2 Emit

**Input:** a document node tree.
**Output:** canonical KVD text.
**Errors:** only if a key cannot be represented as a valid KVD key (for example
empty string).

Canonical form rules:

- All string scalars are double-quoted, except the six builtin type names
  (`int`, `float`, `bool`, `str`, `dict`, `list`), which stay bare. They are
  meaningful only in schema position (§05). A data string whose text equals one
  of these names is therefore also emitted bare, but it re-parses to an equal
  `Str` scalar, so the value round-trips.
- Multi-line strings use `"""` blocks.
- Keys are emitted bare when they satisfy the key grammar (§03); otherwise
  they are double-quoted so they round-trip (for example
  `"app.kubernetes.io/name"`).
- Dict entry keys are always double-quoted, even when they look bare.
- Indentation is exactly 2 spaces per level.
- List items are prefixed with `- `; dict entries are prefixed with `= `.
- No trailing whitespace; exactly one trailing newline. (Parse accepts a
  missing final newline per §02; emit always writes one.)
- No comments are emitted: the emitter MUST NOT produce any.

Emit is deterministic: equal node trees MUST produce identical text.
Round-trip invariant: `emit(parse(text))` equals
`emit(parse(emit(parse(text))))`.

### 9.3 Verify

**Input:** a document node tree and a schema node tree (both produced by
parse).
**Output:** success, or a non-empty list of violations. Each violation carries
the dotted path of the offending node and a human-readable message (§06).
**Errors:** `verify` returns `VerifyError`. When either input fails to parse
it is `ParseDoc` / `ParseSchema`. Once both parse, a *malformed schema* (for example a
quoted or numbered type leaf, a bare `dict`/`list` leaf, a descriptor missing its
`type`, an unknown type name, an unknown descriptor or constraint key, or a
schema list with anything but exactly one element type) is reported as
`VerifyError::SchemaMalformed`. A 1.0 verifier MUST report the §11 `merge`
and `key` descriptor keys as unknown (malformed schema); they become valid
only under the future merge operation.
This is distinct from `VerifyError::Violations`, which covers a well-formed schema
applied to a non-conforming document.

Verification is a separate pass, never part of parse. A document that parses
without error may still fail verification.

The verifier walks the schema tree in schema insertion order and for every
path checks:

- Outside dict values, the data has no keys absent from the schema.
- All required schema keys are present in the data.
- Each scalar's shape matches its declared builtin type.
- `null` only appears under an optional type (`optional: true`); a `null`
  under a required type is a violation before any constraint check.
- Each present non-`null` value satisfies its `validation` constraints, if any
  ([§10](10-validation.md)); `null`/absent values skip constraint checks.

Violations are reported in walk order (schema insertion order, lists in index
order). Each violation carries the dotted path of the offending node and a
human-readable message (§06); the message text after the path is
implementation-defined but MUST name the failed rule.

### 9.4 Typed round-trip (serde)

**Input (deserialize):** KVD text and a target type `T`.
**Output:** a value of type `T`.
**Input (serialize):** a value of type `T`.
**Output:** canonical KVD text.

This operation is composed from the primitives above: serialize builds a node
tree from `T`, then emits it; deserialize parses the text into a node tree,
then maps it onto `T`. Shape enforcement is strict and allows no coercion between
shapes (for example an integer field rejects a quoted-string value).

This operation is optional for implementations that do not support a type
system or reflection layer.

### 9.5 Document operations (programmatic editing)

Beyond the four core operations, an implementation may expose a small
editing surface over the parsed node tree (the `Node`/`Value` model of
§05). This section specifies the path syntax and the operations; it is
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

- Node keys are dotted segments: `a.b.c`
- List items use bracket indices: `a[0]`, `a.b[2]`
- Dict entries use a quoted segment for the entry key: `labels."app.kubernetes.io/name"`.
  A bare dict segment (`labels.team`) is sugar for the quoted form when the
  entry key satisfies the key grammar (§03); otherwise the segment must be
  quoted.
- Segments combine freely: `a.b[0].c`, and nested lists are allowed
  (`a[0][1]`)
- A key segment may be quoted (`"..."` or `'...'`) to address a literal
  key containing `.`, `[`, or other special characters, decoded with the
  same rules as document keys

Rules:

- Bare keys must satisfy the key grammar (§03); a bare key containing a dot
  is impossible because the dot is the segment separator.
- Indices are zero-based and non-negative. A negative, non-numeric, or
  out-of-range index is a path error, never silent truncation or append.
- A non-empty path string addresses a node; the document root itself is
  the empty segment sequence (addressable by implementations directly, not
  by parsing an empty path string, which is a `BadPath` error). `set` and
  `remove` on the root are errors; `get` on it returns the root.

#### Operations

| Operation | Effect |
|---|---|
| `get(doc, path)` | Returns the node or value at `path`, or absent if any segment is missing or mistyped. A path ending at an interior node returns that subtree. |
| `get_mut(doc, path)` | Returns a mutable borrow of the node or value at `path`, or absent. |
| `set(doc, path, value)` | Writes `value` at `path`. Intermediate nodes are created as needed (like dotted-key expansion at parse time). A list index in the path must already exist; an out-of-range index is an error. An existing value at the target is replaced. |
| `remove(doc, path)` | Removes and returns the value at `path`. Ancestor nodes that become empty are **left in place** as empty prefixes (not pruned). |
| `remove_recursive(doc, path)` | Like `remove`, but prunes now-empty ancestor **nodes** recursively up the chain. List elements are never pruned by emptiness. |

Errors (carried as an operation error, distinct from parse errors): a
malformed path (`BadPath`), a missing key (`MissingKey`), an out-of-range
list index (`IndexOutOfBounds`), a missing dict entry (`MissingEntry`), or a
node of the wrong shape where a list or dict was required (`NotAList` /
`NotADict`).

#### Document merging: deferred to §11

Merging two documents is **out of scope for 1.0**. The design is specified in
[§11](11-merge.md) (planned): the merge policy is declared **per-field in
the schema** (not passed as a parameter), with strategies `replace`, `deep`,
`append`, `union`, and `by-key`, and a safe default of `replace`. Until then,
`set` (which replaces a target node wholesale) is the only write primitive.
