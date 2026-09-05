[KVD spec](../../README.md), section 11

## 11. Validation constraints

Status: **normative since 1.0**. The descriptor form (`type` plus `optional: true`) and the `validation` block (ranges, lengths, patterns) are both part of 1.0. A conforming verifier must enforce `validation` constraints as defined below.

### Motivation

Shape typing ([§6](06-values.md)) confirms that a value is an int, a string,
and so on, but not that it is a *sensible* int or string. Validation adds
optional, declarative constraints on top of the shape: ranges, lengths, and
patterns. Constraints are expressed entirely in the schema, never inline in
the data.

### Descriptor form

A schema leaf may be written either as a bare type name or as a descriptor
block. The descriptor is an indented map with two reserved keys:

- `type` (required): a bare type name (`int`, `float`, `bool`, `str`,
  `list`, `map`). For `type: list` a required `element` key gives the
  (uniform) item type; for `type: map` no field keys are required (typed
  maps use the nested sub-schema form).
- `validation` (optional): an indented map of constraint keys.

```kvd
app:
  port:
    type: int
    optional: true
    validation:
      min: 0
      max: 999
  name:
    type: str
    validation:
      min_len: 1
      max_len: 64
      pattern: "^[a-z][a-z0-9_-]*$"
  retries:
    type: int
    optional: true
```kvd

The bare form `port: int` (required) is exactly equivalent to a descriptor
with only a `type` key and no `optional`/`validation` blocks. A descriptor
with a `validation` block but no `type` key is a malformed schema:
`type` is required. Optionality is declared with `optional: true` (§6),
not a `?` suffix.

```kvd
# error: descriptor missing its required type key (malformed schema)
port:
  validation:
    min: 0
```kvd

### Ambiguity rule

A schema leaf map is a descriptor if and only if it contains a `type` key.
Any other key in a leaf map is treated as a nested sub-schema, not as a
constraint. This keeps the rule from [§6](06-values.md) unchanged: a leaf
map with `type` is a descriptor; a leaf map without `type` is a nested
schema. The reserved keys `type` and `validation` have meaning only inside a
descriptor. A `type: map` descriptor accepts any map; a typed map is
written as a nested sub-schema (a map without a `type` key).

### Constraints by type

| Type | Constraint | Meaning |
|------|-----------|---------|
| `int`, `float` | `min` | value >= min |
| `int`, `float` | `max` | value <= max |
| `int`, `float` | `exclusive_min` | value > exclusive_min |
| `int`, `float` | `exclusive_max` | value < exclusive_max |
| `str` | `min_len` | string length >= min_len |
| `str` | `max_len` | string length <= max_len |
| `str` | `pattern` | string matches the regex (full match) |
| `list`, `map` | `min_len` | collection length (key count) >= min_len |
| `list`, `map` | `max_len` | collection length (key count) <= max_len |

Numeric bounds are compared on the value's written text interpreted as the
declared numeric type; `min`/`max` are inclusive, `exclusive_min`/
`exclusive_max` are exclusive. For `int` bounds the bound values must be
ints; for `float` bounds they may be ints or floats. `min_len`/`max_len`
must be non-negative ints. `pattern` must be a string holding a regular
expression matched against the full string value (equivalent to anchoring
the pattern with `^` and `$`). The pattern dialect is a Perl/PCRE-style
regular expression restricted to the backtracking-free subset (no
look-around, no backreferences). This is the syntax of RE2 and the Rust
`regex` crate. The exact engine is an implementation detail. String length
is counted in Unicode scalar values (characters, not bytes). An unknown
constraint key for a given type (for example `pattern` on an `int`), or a
constraint value of the wrong shape, is a malformed schema (§9.3).

Valid bounded count with an out-of-range counterpart:

```kvd
count:
  type: int
  validation:
    min: 0
    max: 999
```kvd

```kvd
# error: pattern does not apply to int (malformed schema)
count:
  type: int
  validation:
    pattern: "^[0-9]+$"
```

### Optionality and null

If the declared type is optional (`optional: true`, [§6](06-values.md)) and
the data value is `null` (or the key is absent), constraint checks are
skipped. In this case `null`/absence means "no value to validate". Constraints apply
only when a concrete value is present.

### Verification

Validation runs as part of the verify pass ([§9](09-operations.md)), after
shape typing. The order is: shape check, then constraint checks. A constraint
failure is reported as a `constraint` violation ([§7](07-errors.md)), carrying
the dotted path and a message such as `value 1000 exceeds max 999`.
