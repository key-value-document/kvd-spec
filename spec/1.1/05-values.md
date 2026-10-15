[KVD spec](../../README.md), section 06

## 5. Values and types

A document is a trie[^trie] of node prefixes ending in keys; a key
holds one value: a scalar, a list, or a dict. Nodes carry no value.
Keys are usually written as dotted paths (`a.b.c: value`); an indented
block is sugar for keys sharing a prefix.
Implementations MUST preserve insertion order.

Shape typing uses a closed, predictable set. Nothing else is coerced:

| Written                           | Result   |
|-----------------------------------|----------|
| `42`, `-5`, `+5`, `1_000`         | int      |
| `0.75`, `1e3`, `-5.5`             | float    |
| `true` / `false`                  | bool     |
| `"..."`, `'...'`                  | string   |
| `"""..."""`, `'''...'''` blocks     | string   |
| `null`                            | null     |
| `{}`                              | empty dict |
| `[]`                              | empty list |
| indented pairs                    | node prefixes |
| `-` items                        | list     |
| `=` entries                      | dict     |

All string values are double-quoted, single-quoted literal, or one of the two
multi-line block forms. The only bare (unquoted) tokens permitted in value position
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
```

```kvd
# error: date-like bare token (quote it for a string)
when: 2026-08-20
```

```kvd
# error: capitalized bare word is not a shape, int, null, or type name
flag: Yes
```

Dict values hold opaque keys with scalar, list, or dict values:

```kvd
metrics:
  = "a.b.c/name": 99.9
  = "errors/total": 3
```

```kvd
groups:
  = "team-a":
    - "amy"
    - "bo"
```

```kvd
outer:
  = "a":
    = "b": 1
```

```kvd
items:
  - = "a.b/c": 1
    = "d": 2
```

### Null and absence

`null` is valid only under an optional schema type (`optional: true`). The
verifier rejects it anywhere else. In a document without a schema, `null`
parses without error but has no declared type to satisfy; consumers treat
it as untyped null.

To express absence in a schema-less document: omit the key entirely, or use
`{}` / `[]` for an empty collection. When a schema is present, declare the
key optional (`optional: true`) and write `null` or omit the key.

The parser MUST store numbers verbatim: a number node is its exact written
text tagged with its shape, with no conversion to a machine integer or
float. Range and precision are the consumer's concern. `1e999` and
`99999999999999999999` are valid values.

```kvd
retries: null
```

```kvd
# error: null under a required type (schema says port: int)
port: null
```

### Schemas and types

Types are declared in a schema, never inline. The builtin type set is closed:
`int`, `float`, `bool`, `str`, `bytes`, `dict`, `list`. There are no custom
types. The first five are scalar types; `dict` and `list` are container types and may
appear only as the `type` of a descriptor. `bytes` is a schema-level type with
no token of its own (see *Binary data* below); like `null`, it is meaningful
only against a schema. They give a dict or list an
`optional`, `description`, `deprecated`, or `validation` slot (see below).

Type names are bare (unquoted) words in schema position only. A name that is
not one of the seven builtins is an `unknown-type` error at verification time,
reported as a malformed schema (§08.3).

Verification is a separate pass over a parsed document; the parser itself is
registry-free:

- The schema mirrors the data document's structure outside dict values:
  every data key outside a dict must appear in the schema and vice versa.
  Dotted keys and nested blocks are interchangeable in the schema, exactly
  as in data. Intermediate nodes need no type; only full keys carry types.
  Inside a dict value the open-world rule below applies instead of mirroring.
- A leaf type name constrains the corresponding data value's shape: `int` maps
  to int, `float` maps to float, `bool` maps to bool, `str` maps to string
  (any quoted form), and `bytes` maps to string (any quoted form) whose text
  is valid padded base64 (*Binary data* below).
  A `{}` leaf accepts any dict (empty or not) and applies no checks to its
  contents; a `[]` leaf accepts any list (empty or not) and applies no
  checks to its items. These bare `{}`/`[]` leaves are always required.
- A list with a checked item type is declared either with the
  single-element list form (`key:` plus one `- <element>` line) or with a
  `type: list` descriptor carrying a required `element` type; every item in
  the data list must match that single `element` type (uniform).
- A dict declares any number of entries (`key:` plus `= "name": <type>`
  lines). Each declared key present in the data is checked against its
  type; declared keys may be absent (optional) and undeclared data keys
  pass unchecked with any value type. Use a `type: dict` descriptor with an
  optional `element` type when every value must share one type (uniform);
  without `element` any dict value type passes.
- Mismatches, unknown keys outside dict values, missing keys, and unknown
  types are errors. Undeclared keys inside a dict value pass unchecked.

A schema leaf is either a bare type name, the bare `{}`/`[]` collection
literals, a single-element list, a dict of any number of entries, or a descriptor block.
The bare forms are required; the descriptor form adds `optional: true`,
`description`, `deprecated`, and/or `validation`. An optional key may be absent, present with a value of
the declared type, or present as `null`. A required key (no `optional: true`)
that is absent is a missing-key error and `null` is an error (`null` requires
`optional: true`). `optional` applies to any descriptor, including
`type: dict` and `type: list` containers: a dict or list key with a descriptor
may be absent or `null`. The bare `{}`/`[]` leaves remain always required; use
`type: dict`/`type: list` to make a container optional.

```kvd
app:
  port: int                 # required
  retries:
    type: int
    optional: true         # absent, int, or null
```

A list element may also be a descriptor with `optional: true`; items may
then be `null` (absence does not apply to items).

```kvd
items:
  - type: int
    optional: true
```

A dict value type may also be a descriptor with `optional: true`; values may
then be `null` (absence does not apply to values).

```kvd
metrics:
  = "errors/total": int
```

The compact `- <element>` list form above is sugar for the explicit
`type: list` descriptor, which additionally allows `optional`, `description`, `deprecated`, and
`validation` on the container itself. A `type: dict` descriptor with an
`element` type enforces one uniform value type instead:

```kvd
items:
  type: list
  element: int        # required; all items must be int
  optional: true      # the list key may be absent or null
  validation:
    max_len: 100

cfg:
  type: dict          # any dict; the container may be optional
  optional: true
```

```kvd
# error: unknown type name (malformed schema at verification time)
port: port
```

Optional per-value constraints such as numeric ranges, string/list lengths, and
regular-expression patterns are described in [§09](09-validation.md).

### Binary data: `bytes`

**Status: new in 1.1.** `bytes` is a **schema-level** type. It adds no token to
the data grammar and no shape to the parser: a key declared `bytes` holds an
ordinary quoted string whose **text is padded base64** (RFC 4648 §4, standard
alphabet `A-Za-z0-9+/` with `=` padding). The parser yields a `Str` scalar and
the schema is what makes it binary.

```kvd
# data (tls.kvd)
tls.cert: "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8A"
tls.key: ""
tls.chain: null
```

```kvd
# schema (tls.schema.kvd)
tls:
  cert:
    type: bytes
    validation:
      max_len: 4096
  key:
    type: bytes
    optional: true
  chain:
    type: bytes
    optional: true
```

Rules:

- **No new syntax.** There is no `b"..."` literal, no `0x...` literal, and no
  inline type marker ([§01](01-design.md)). Binary data is text until a
  schema declares it `bytes`, which keeps the shape set closed and keeps type
  annotations out of the value syntax.
- **Quoting is required.** The value MUST be written as a quoted string, in
  either quote form. The base64 alphabet (`A-Za-z0-9+/=`) lies almost entirely
  outside the bare-token grammar: `=`, `+`, and `/` cannot appear in a bare
  token, and an uppercase word is not a bare type name. An unquoted base64 value
  is an `unexpected-character` **parse error**, never a bytes literal.
- **Bare words are still strings.** The one exception is text made only of
  lowercase letters with no `+`, `/`, or padding, which parses as a bare word
  per §01: `cert: abcd` is the string `abcd`, and it satisfies `type: bytes`
  only because that text happens to be valid base64. Quote it anyway; the
  difference between a bytes field and a str field is never the quoting.
- **Only a string is bytes.** A bare type name is the string `bytes` (§05
  above), and a number is a number: `cert: bytes` and `cert: 12345` are both
  schema mismatches under `type: bytes`.
- **Line breaks are ignored when decoding.** CR and LF are layout, not data, so
  a value may be wrapped across lines and a `"""` block works with either
  closer form ([§04](04-grammar.md)): a standalone closer appends a trailing
  `\n` that decoding simply drops. This is scoped to `bytes` and to the decode
  step only: under `type: str` a newline is always content, and no other type
  changes meaning because of this rule. Whitespace handling is a property of
  the declared type, never of the text.
- **Spaces and tabs are content.** Unlike line breaks they are NOT ignored, so
  `cert: "AQ IDBA=="` is an `invalid bytes` violation: a space in the middle of
  a blob is a bad paste far more often than it is layout. KVD carries no digest,
  so a silently corrupted value would otherwise verify clean.
- **Lenient parse, strict verify.** Parse accepts any quoted string and stores
  it verbatim (the same rule numbers follow); emit re-emits the identical text.
  A string that is not valid base64 parses without error and is reported only
  by `verify`, as an `invalid bytes` violation ([§06](06-errors.md)).
- **Strict base64.** After line breaks are dropped, the remaining text decodes
  only when its length is a multiple of 4, `=` appears only as trailing padding
  of at most two characters, every character is in the standard alphabet, and
  the pad bits are zero (the unused low bits of the last encoded character, per
  RFC 4648 §3.5). The last rule is stricter than many built-in decoders, which
  accept `AR==` for the octet `0x01`; KVD requires canonical base64 so that a
  decoded value has one representation modulo line breaks and re-serializing it
  is deterministic. The empty string `""` is valid base64 and decodes to zero
  octets.
- **Canonical output is the consumer's job.** The emitter MUST NOT re-encode,
  re-wrap, or reformat the value ([§08](08-operations.md)), so a document is
  byte-stable. Canonical base64 is a single unwrapped line; a consumer that
  re-serializes decoded octets SHOULD produce that form.
- **No schema, no bytes.** In a schema-less document a `bytes` field and a
  `str` field are the same thing: a string. Only `type: bytes` distinguishes
  them, and it is what makes the text meaningful as data.
- **Constraints.** `min_len` / `max_len` count **decoded octets**, not
  characters and not base64 characters ([§09](09-validation.md)). `min`,
  `max`, `exclusive_min`, `exclusive_max`, and `pattern` do not apply to
  `bytes` and are a malformed schema on it.
- **Optionality is unchanged.** `null` and absence still require
  `optional: true`, exactly as for any other type.
- **Typed round-trip** maps an octet-array type (`Vec<u8>` and equivalents) to
  and from a `bytes` field, encoding base64 in the text form
  ([§08](08-operations.md)).
- **Compatibility.** A 1.0 verifier MUST report `type: bytes` as
  `unknown-type` (a malformed schema), exactly as it reports the
  [§10](10-merge.md) `merge` and `key` descriptor keys
  ([§08](08-operations.md)). Data written for a 1.1 schema is unchanged in
  1.0: it is valid KVD either way, only its interpretation differs.

Valid base64:

```kvd
cert: "AQIDBA=="
empty: ""
```

Wrapped, which is the usual shape of a pasted PEM body. Both closer forms work,
and the value is the same octets either way:

```kvd
pem: """
  AQIDBA==
"""
pem_wrapped: """
  AQID
  BA==
"""
```

```kvd
# error: not valid base64 (length is not a multiple of 4)
cert: "AQIDB"
```

```kvd
# error: not valid base64 (non-zero discarded bits in the last character)
cert: "AR=="
```

```kvd
# error: not valid base64 (space is content, only line breaks are ignored)
cert: "AQ IDBA=="
```

```kvd
# error: unquoted base64 — a parse error, not a bytes literal
cert: AQIDBA==
```

### Multi-line strings

The `"""` block form is defined in [§04](04-grammar.md).

---

[^trie]: A trie (from *retrieval*, pronounced "try") is a prefix tree: keys
sharing a prefix share the nodes for that prefix. `a.b.c` and `a.b.d`
share the nodes `a` and `a.b`, then branch.
