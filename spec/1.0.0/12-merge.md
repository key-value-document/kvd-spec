[KVD spec](../../README.md), section 12

# 12. Document merging (planned for 1.x)

**Status: planned, not in 1.0.** This section specifies a future `merge`
operation. It is additive (a later minor version); 1.0 ships only
`get` / `set` / `remove` / `remove_recursive` (§9.5). The design here is
normative for the future operation but may be refined before implementation.

Like all operations, `merge` is defined by its **observable behavior on the
KVD data model**, not by any language API. A conforming implementation in any
language must produce identical results given identical inputs.

## 12.1 Motivation

Real configuration workflows stack documents by precedence. Examples are chart defaults,
user overrides, and CLI flags (the Helm model). A single global merge policy
cannot fit every field: maps usually deep-merge, but some lists must append
and others must replace, and lists of objects often need element-level
patching. Therefore the merge policy is declared **per-field in the schema**,
where it belongs with the field's type, rather than passed as a parameter at
each call site.

## 12.2 Operation

```text
merge(base, overlay)            # uses base's embedded __schema__
merge(base, overlay, schema)    # uses an explicit schema
```kvd

- **base**: the lower-precedence document (for example chart defaults).
- **overlay**: the higher-precedence document (for example user values).
- **schema** (optional): the schema whose per-field `merge` descriptors
  drive the behavior. If omitted, the base document's `__schema__` metakey is
  used (the `merge_embedded` form, analogous to `verify_embedded` in §9.3); if neither is present, the global default applies
  to every field.

**Output:** a new document that is the merge of `base` and `overlay`. `base` and
`overlay` are not mutated.

**Stacking:** repeated `merge` in precedence order reproduces Helm-style
value stacking:

```text
doc = merge(defaults, userA)
doc = merge(doc, userB)
doc = merge(doc, cliOverrides)
```kvd

## 12.3 Policy model

Each field's merge behavior is taken from its schema descriptor's `merge`
attribute. The descriptor already carries `type`, `optional`, and constraints
(§6); `merge` is an additional, optional attribute. Allowed descriptor keys
in a merge-aware schema are `type`, `optional`, `validation`, `element`,
plus `merge` and (for `by-key` only) `key`. A `by-key` descriptor without
`key`, or a `key` on any other strategy, is a malformed merge descriptor.

```kvd
__schema__:
  database:
    type: map
    merge: deep
  ports:
    type: list
    merge: append
  containers:
    type: list
    merge: by-key
    key: name
  name:
    type: str
    merge: replace
```

### Strategies

| Strategy | Applies to | Behavior |
|---|---|---|
| `replace` | any | Overlay value completely replaces the base value. |
| `deep` | maps | Recursively merge maps key-by-key (§12.4). On a non-map value, see §12.6. |
| `append` | lists | Concatenate: base elements followed by overlay elements. |
| `union` | lists | Like `append`, then dedupe by whole-element value equality. |
| `by-key` | lists of maps | Match elements by `key`; deep-merge matches, keep unmatched from both sides (§12.5). |

### Default policy

When a field has no `merge` attribute, or no schema is supplied, the policy is
**`replace`** for every shape. This is the safe default. No implicit deep
merge occurs unless explicitly declared. To obtain Helm-like behavior (map
deep-merge, list replace), set `merge: deep` on the relevant maps; lists
default to `replace`, which already matches Helm.

## 12.4 `deep` (maps)

For a map field with `merge: deep`, merge proceeds key-by-key:

- A key present only in `base` is kept.
- A key present only in `overlay` is added.
- A key present in both recurses with the policy of that sub-field (looked up
  from the schema at the deeper path; falls back to the default if undeclared).

## 12.5 `by-key` (lists of maps)

For a list-of-maps field with `merge: by-key`, the descriptor must also
declare `key: <field>`. This is the name of the identity field within each element.

Algorithm:

1. Index the overlay list by the value of `key`.
2. For each base element:
   - if a matching overlay element exists, deep-merge the pair (using
     per-field policies for the element's sub-fields) and emit the merged
     element;
   - otherwise emit the base element unchanged.
3. Append overlay elements that had no base match.

**Errors (strict):**

- **Missing key**: any list element (base or overlay) that is not a map, or
  lacks the declared `key` field, is a `MergeError::MissingKey`. The merger
  reports an error on lack of key and never silently drops or treats the
  element as unmatched.
- **Duplicate key**: two elements within the *same* list sharing the same
  `key` value is a `MergeError::DuplicateKey` (the match would be ambiguous).

`key` is a single top-level field name in this version; nested key paths are
deferred.

## 12.6 Shape and policy conflicts

- A strategy applied to an incompatible actual shape is a
  `MergeError::ShapeMismatch` (for example `deep` on a list, `append` on a map,
  `by-key` on a list of scalars). Merge is strict and performs no implicit
  coercion (consistent with §1).
- **Parent/child resolution:** a parent map's `merge: deep` causes recursion
  into it; an individual child field may override with its own `merge`
  attribute (for example a child list under a deep map may declare `append`). A
  child's explicit strategy takes precedence over the parent's recursion.

## 12.7 Errors

`merge` returns `MergeError`:

- `SchemaMalformed`: a `merge` attribute with an unknown strategy, a `by-key`
  without `key`, a `key` on a non-`by-key` strategy, or any other invalid
  merge descriptor. Mirrors `VerifyError::SchemaMalformed` (§9.3).
- `MissingKey`: a `by-key` element lacks the declared key field (§12.5).
- `DuplicateKey`: duplicate key value within a single list (§12.5).
- `ShapeMismatch`: a strategy applied to an incompatible shape (§12.6).

## 12.8 Open questions (to resolve before implementation)

- **`null` in overlay:** under an optional field, should a `null` overlay
  value delete the base key (Helm `--set key=null` semantics) or set it to
  `null`? Currently unspecified.
- **Nested `key` path:** `by-key` `key` is a single field name; nested paths
  (for example `spec.name`) deferred.
- **Cross-shape base/overlay:** if `base` and `overlay` disagree on shape at a
  `deep` / `append` field, the current rule is `ShapeMismatch`; an alternative
  is "overlay wins" (`replace`). To be finalized.
