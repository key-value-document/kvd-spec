[KVD spec](../../README.md), section 05

## 5. Values and types

Data model: a document is an ordered map of keys to nodes; a node is a
scalar, a map, or a list. Maps preserve insertion order. Metakeys are not
part of the data model.

Shape typing uses a closed, predictable set. Nothing else is coerced:

| Written                           | Result   |
|-----------------------------------|----------|
| `42`, `-5`, `+5`, `1_000`         | int      |
| `0.75`, `1e3`, `-5.5`             | float    |
| `true` / `false`                  | bool     |
| `"..."`                           | string   |
| `"""..."""` block                 | string   |
| `null`                            | null     |
| `{}`                              | empty map |
| `[]`                              | empty list |
| indented pairs                    | map      |
| `- ` items                        | list     |

All string values are double-quoted or use the `"""` block form. The only
bare (unquoted) tokens permitted in value position are: `true`, `false`,
`null`, integer and float literals, `{}`, `[]`, and type names matching the
`type` grammar (`[a-z][a-z0-9_-]*`). Type names are
meaningful only in schema documents; in data documents they parse as strings.
Any other unquoted token is an `unexpected-character` error.
`true`, `false`, and `null` are shape literals, not reserved words: unquoted
they are bools/null, `"true"` is the string. Keys named `true` are legal,
and keys may start with digits (`8080`, `2fa`). Keys are always strings.

### Null and absence

`null` is valid only under an optional schema type (`optional: true`). The
verifier rejects it anywhere else. In a document without a schema, `null`
parses without error but cannot carry useful meaning because there is no type
declaration to make the key optional.

To express absence in a schema-less document: omit the key entirely, or use
`{}` / `[]` for an empty collection. When a schema is present, declare the
key optional (`optional: true`) and write `null` or omit the key.

The parser stores numbers verbatim: a number node is its exact written text
tagged with its shape, with no conversion to a machine integer or float.
Range and precision are the consumer's concern. `1e999` and
`99999999999999999999` are valid values.

### Schemas and types

Types are declared in a schema, never inline. The builtin type set is closed:
`int`, `float`, `bool`, `str`, `list`, `map`. There are no custom types. The
first four are scalar types; `list` and `map` are container types and may
appear only as the `type` of a descriptor. They give a list or map an
`optional` or `validation` slot (see below).

Type names are bare (unquoted) words in schema position only. A name that is
not one of the six builtins is an `unknown-type` error at verification time.

Verification is a separate pass over a parsed document; the parser itself is
registry-free:

- The schema mirrors the data document's structure: every data key must
  appear in the schema and vice versa. Dotted keys and nested blocks are
  interchangeable in the schema, exactly as in data.
-   A leaf type name constrains the corresponding data value's shape: `int` →
  int, `float` → float, `bool` → bool, `str` → string (any quoted form).
  A `{}` leaf accepts any map (empty or not) and applies no checks to its
  contents; a `[]` leaf accepts any list (empty or not) and applies no
  checks to its items. These bare `{}`/`[]` leaves are always required.
- A list is declared with `type: list` and a required `element` type; every
  item in the data list must match that single `element` type (uniform). A
  `type: map` descriptor accepts any map (typed maps are written with the
  nested sub-schema form). Unlike the bare `{}`/`[]` leaves, `type: list` and
  `type: map` descriptors may carry `optional: true` and `validation`.
- Mismatches, unknown keys, missing keys, and unknown types are errors.

A schema leaf is either a bare type name, the bare `{}`/`[]` collection
literals, or a descriptor block. The bare forms are required; the descriptor
form adds `optional: true` and/or `validation`. An optional key may be
absent, present with a value of the declared type, or present as `null`. A
required key (no `optional: true`) that is absent is a missing-key error and
`null` is an error (`null` requires `optional: true`). `optional` applies to
any descriptor, including `type: list` and `type: map` containers. So a list
or map key may now be absent or `null`. The bare `{}`/`[]` leaves remain
always required; use `type: list`/`type: map` to make a container optional.

```
app:
  port: int                 # required
  retries:
    type: int
    optional: true         # absent, int, or null
```

A list element may also be a descriptor with `optional: true`; items may
then be `null` (absence does not apply to items).

```
items:
  - type: int
    optional: true
```

The compact `- <element>` list form above is sugar for the explicit
`type: list` descriptor, which additionally allows `optional` and
`validation` on the container itself:

```
items:
  type: list
  element: int        # required; all items must be int
  optional: true      # the list key may be absent or null
  validation:
    max_len: 100

cfg:
  type: map           # any map; the container may be optional
  optional: true
```

Optional per-value constraints such as numeric ranges, string/list lengths, and
regular-expression patterns are described in [§10](10-validation.md).

### Multi-line strings

The `"""` block form is defined in [§4](04-grammar.md).
