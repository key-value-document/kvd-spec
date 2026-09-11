[KVD spec](../../README.md), section 11

## 10. Validation constraints

Status: **normative since 1.0**. The descriptor form (`type` plus `optional: true`) and the `validation` block (ranges, lengths, patterns) are both part of 1.0. A conforming verifier must enforce `validation` constraints as defined below.

### Motivation

Shape typing ([§05](05-values.md)) confirms that a value is an int, a string,
and so on, but not that it is a *sensible* int or string. Validation adds
optional, declarative constraints on top of the shape: ranges, lengths, and
patterns. Constraints are expressed entirely in the schema, never inline in
the data.

### Descriptor form

A schema leaf may be written either as a bare type name or as a descriptor
block. The descriptor is an indented block with reserved keys:

- `type` (required): a bare type name (`int`, `float`, `bool`, `str`,
  `dict`, `list`). For `type: list` a required `element` key gives the
  (uniform) item type; for `type: dict` an optional `element` key gives the
  (uniform) value type (absent means any value type passes).
- `description` (optional): a string documenting the field. It has no effect
  on verification; verifiers MUST accept any string value and ignore it.
- `deprecated` (optional): an indented block marking the field as deprecated.
  It has no effect on verification; verifiers MUST accept and ignore it.
  Allowed sub-keys, both optional strings: `reason` (why it is deprecated /
  what to use instead), `since` (version when deprecated, e.g. `"1.0"`).
  An unknown sub-key or a non-string sub-value is a malformed schema.
- `validation` (optional): an indented block of constraint keys.

```kvd
app:
  port:
    type: int
    description: "port to listen on"
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
  legacy_port:
    type: int
    optional: true
    deprecated:
      reason: "use port instead"
      since: "1.0"
```

The bare form `port: int` (required) is exactly equivalent to a descriptor
with only a `type` key and no `optional`/`description`/`deprecated`/`validation` blocks. A descriptor
with a `validation` block but no `type` key is a malformed schema:
`type` is required. Optionality is declared with `optional: true` (§05),
not a `?` suffix.

```kvd
# error: descriptor missing its required type key (malformed schema)
port:
  validation:
    min: 0
```

### Ambiguity rule

A schema leaf block is a descriptor if and only if it contains a `type` key.
Any other key in a leaf block is treated as nested node prefixes, not as a
constraint. This keeps the rule from [§05](05-values.md) unchanged: a leaf
block with `type` is a descriptor; a leaf block without `type` is nested
prefixes. The reserved keys `type`, `description`, `deprecated`, and `validation` have meaning only inside a
descriptor. A `type: dict` descriptor accepts an optional `element` type for
the dict values; a typed node subtree is written as nested prefixes (a block
without a `type` key).

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
| `list`, `dict` | `min_len` | collection length (item / entry count) >= min_len |
| `list`, `dict` | `max_len` | collection length (item / entry count) <= max_len |

Numeric bounds are compared on the mathematical value of the value's written
text interpreted as the declared numeric type; `min`/`max` are inclusive,
`exclusive_min`/`exclusive_max` are exclusive. For `int` bounds the bound
values must be ints; for `float` bounds they may be ints or floats.
`min_len`/`max_len` must be non-negative ints (a negative length is a
malformed schema). `pattern` must be a string holding a regular
expression matched against the full string value (equivalent to anchoring
the pattern with `^` and `$`). The pattern dialect is the RE2 syntax
(backtracking-free: no look-around, no backreferences), with the matching
semantics of the Rust `regex` crate. String length
is counted in Unicode scalar values (characters, not bytes). An unknown
constraint key for a given type (for example `pattern` on an `int`), or a
constraint value of the wrong shape, is a malformed schema (§08.3).

Valid bounded count with an out-of-range counterpart:

```kvd
count:
  type: int
  validation:
    min: 0
    max: 999
```

```kvd
# error: pattern does not apply to int (malformed schema)
count:
  type: int
  validation:
    pattern: "^[0-9]+$"
```

### Optionality and null

If the declared type is optional (`optional: true`, [§05](05-values.md)) and
the data value is `null` (or the key is absent), constraint checks are
skipped. In this case `null`/absence means "no value to validate". Constraints apply
only when a concrete value is present.

### Verification

Validation runs as part of the verify pass ([§08](08-operations.md)), after
shape typing. The order is: shape check, then constraint checks. A constraint
failure is reported as a `constraint` violation ([§06](06-errors.md)), carrying
the dotted path and a message such as `value 1000 exceeds max 999`.
