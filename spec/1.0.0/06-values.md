[KVD spec](../../README.md), section 06

## 6. Values and types

Data model: a document is an ordered map of keys to nodes; a node is a
scalar, a map, or a list. Maps preserve insertion order. Metakeys are not
part of the data model.

Shape typing uses a closed, predictable set. Nothing else is coerced:

| Written                           | Result   |
|-----------------------------------|----------|
| `42`, `-5`, `+5`, `1_000`         | int      |
| `0.75`, `1e3`, `-5.5`             | float    |
| `true` / `false`                  | bool     |
| `"..."`, `'...'`                  | string   |
| `"""..."""` block                 | string   |
| `null`                            | null     |
| `{}`                              | empty map |
| `[]`                              | empty list |
| indented pairs                    | map      |
| `- ` items                        | list     |

All string values are double-quoted, single-quoted literal, or the `"""`
block form. The only bare (unquoted) tokens permitted in value position
are: `true`, `false`, `null`, integer and float literals, `{}`, `[]`, and
type names matching the `type` grammar (`[a-z][a-z0-9_-]*`). Type names are
meaningful only in schema documents; in data documents they parse as strings.
Any other unquoted token is an `unexpected-character` error.
`true`, `false`, and `null` are shape literals, not reserved words: unquoted
they are bools/null, `"true"` is the string. Keys named `true` are legal,
and keys may start with digits (`8080`, `2fa`). Keys are always strings.

Valid shapes with a date kept as a string:

```kvd
count: 42
name: "hello"
when: "2026-08-20"
flag: true
mode: yes
```kvd

```kvd
# error: date-like bare token (quote it for a string)
when: 2026-08-20
```kvd

```kvd
# error: capitalized bare word is not a shape, int, null, or type name
flag: Yes
```kvd

### Null and absence

`null` is valid only under an optional schema type (`optional: true`). The
verifier rejects it anywhere else. In a document without a schema, `null`
parses without error but has no declared type to satisfy; consumers treat
it as untyped null.

To express absence in a schema-less document: omit the key entirely, or use
`{}` / `[]` for an empty collection. When a schema is present, declare the
key optional (`optional: true`) and write `null` or omit the key.

The parser stores numbers verbatim: a number node is its exact written text
tagged with its shape, with no conversion to a machine integer or float.
Range and precision are the consumer's concern. `1e999` and
`99999999999999999999` are valid values.

```kvd
retries: null
```kvd

```kvd
# error: null under a required type (schema says port: int)
port: null
```kvd

### Schemas and types

Types are declared in a schema, never inline. The builtin type set is closed:
`int`, `float`, `bool`, `str`, `list`, `map`. There are no custom types. The
first four are scalar types; `list` and `map` are container types and may
appear only as the `type` of a descriptor. They give a list or map an
`optional` or `validation` slot (see below).

Type names are bare (unquoted) words in schema position only. A name that is
not one of the six builtins is an `unknown-type` error at verification time,
reported as a malformed schema (§9.3).

Verification is a separate pass over a parsed document; the parser itself is
registry-free:

- The schema mirrors the data document's structure: every data key must
  appear in the schema and vice versa. Dotted keys and nested blocks are
  interchangeable in the schema, exactly as in data.
- A leaf type name constrains the corresponding data value's shape: `int` maps
  to int, `float` maps to float, `bool` maps to bool, `str` maps to string
  (any quoted form).
  A `{}` leaf accepts any map (empty or not) and applies no checks to its
  contents; a `[]` leaf accepts any list (empty or not) and applies no
  checks to its items. These bare `{}`/`[]` leaves are always required.
- A list with a checked item type is declared either with the
  single-element list form (`key:` plus one `- <element>` line) or with a
  `type: list` descriptor carrying a required `element` type; every item in
  the data list must match that single `element` type (uniform). A
  `type: map` descriptor accepts any map (typed maps are written with the
  nested sub-schema form). Unlike the bare `{}`/`[]` leaves, `type: list` and
  `type: map` descriptors may carry `optional: true` and `validation`.
- Mismatches, unknown keys, missing keys, and unknown types are errors.

A schema leaf is either a bare type name, the bare `{}`/`[]` collection
literals, a single-element list, or a descriptor block. The bare forms are
required; the descriptor form adds `optional: true` and/or `validation`. An
optional key may be absent, present with a value of the declared type, or
present as `null`. A required key (no `optional: true`) that is absent is a
missing-key error and `null` is an error (`null` requires `optional: true`).
`optional` applies to any descriptor, including `type: list` and `type: map`
containers: a list or map key with a descriptor may be absent or `null`. The
bare `{}`/`[]` leaves remain always required; use `type: list`/`type: map`
to make a container optional.

```kvd
app:
  port: int                 # required
  retries:
    type: int
    optional: true         # absent, int, or null
```kvd

A list element may also be a descriptor with `optional: true`; items may
then be `null` (absence does not apply to items).

```kvd
items:
  - type: int
    optional: true
```kvd

The compact `- <element>` list form above is sugar for the explicit
`type: list` descriptor, which additionally allows `optional` and
`validation` on the container itself:

```kvd
items:
  type: list
  element: int        # required; all items must be int
  optional: true      # the list key may be absent or null
  validation:
    max_len: 100

cfg:
  type: map           # any map; the container may be optional
  optional: true
```kvd

```kvd
# error: unknown type name (malformed schema at verification time)
port: port
```

Optional per-value constraints such as numeric ranges, string/list lengths, and
regular-expression patterns are described in [§11](11-validation.md).

### Multi-line strings

The `"""` block form is defined in [§5](05-grammar.md).
