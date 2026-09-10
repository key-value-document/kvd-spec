[KVD spec](../../README.md), section 05

## 4. Grammar (EBNF)

`:=` is EBNF's definition operator. It is meta-syntax, not part of the format.
`NL` is a newline, `INDENT`/`DEDENT` mark nesting changes, `EOF` is end of
file, `SP{keycol}` means spaces up to the key's column.

```ebnf
document  := pairs? EOF

pairs     := pair (NL pair)*
pair      := (metakey | path) ':' value
path      := key ('.' key)*

value     := ' ' scalar
           | ' ' '"""' NL triple      ; triple-quoted string (see below)
           | NL INDENT pairs DEDENT   ; nested node prefixes
           | NL INDENT items DEDENT   ; list
           | NL INDENT entries DEDENT ; dict

scalar    := int | float
           | 'true' | 'false' | 'null'
           | dquote
           | squote
           | empty-map | empty-list
           | type                     ; bare type name: meaningful in schema,
                                      ; plain string in data (§05)

items     := item (NL item)*
item      := '- ' scalar
           | '- ' '"""' NL triple     ; triple-quoted string as list item
           | '- ' pair                 ; inline first path of a list item
           | '- ' entry                ; inline first dict entry of a list item
           | '- ' items               ; compact nested list  (- - "x")
           | '- ' entries             ; compact nested dict  (- = "k": 1)
           | '-' NL INDENT pairs DEDENT
           | '-' NL INDENT items DEDENT
           | '-' NL INDENT entries DEDENT

entries   := entry (NL entry)*
entry     := '= ' dkey ':' value
dkey      := dquote | squote           ; opaque dict key: dots never split

triple    := content-line* closer
closer    := SP{keycol} '"""' NL          ; standalone: trailing \n included
           | content-line '"""' NL        ; inline: no trailing \n
```

Quoted keys (§02) may appear as any `key` segment in `path`; a quoted
`"__name__"` segment is a literal key, never a metakey. `metakey` pairs are
retained in the root; the same pattern anywhere else is a
`metakey-outside-root` error. The parser needs only one token of lookahead.

Dict entry keys (`dkey`) are always quoted and always opaque: `"a.b.c/name"`
is one key. A bare word is never a dict key; write `= "k": v` even when `k`
looks bare. The `=` marker always carries its entry on the same line; an
`=` alone at end of line is a `bad-dict-marker` error.

### Schema documents

A schema mirrors a data document's structure with builtin type names in place
of values. A standalone schema file (by convention `name.schema.kvd`) is a
bare KVD tree with no metakeys:

```kvd
# app.schema.kvd
server:
  port: int
  host: str
```

A data file may embed its schema under `__schema__`:

```kvd
__schema__:
  server:
    port: int
server:
  port: 8080
```

`__schema__` is the only defined metakey. Values in schema position are a
builtin type name, a descriptor block, a single-element list declaring its
item type, a single-entry dict declaring its value type, or the empty
literals `{}` / `[]`.
The builtin type names are `int`, `float`, `bool`, `str`, `dict`, `list`;
`dict` and `list` appear only inside a descriptor's `type` key (they make a
container optional/validatable). Numbers, booleans, `null`, and quoted
strings are malformed-schema errors in schema position. A type name is
written bare; optionality is declared with `optional: true` in a descriptor
block. See [§05](05-values.md).

Metakeys are excluded from the data tree before verification, so an embedded
schema never appears as a data key.

### Structural rules

- Keys are usually dotted paths; nested blocks are sugar for a shared
  prefix. Both spellings expand to the same node prefixes at parse time. Distinct pairs
  that expand onto the same key are `duplicate-key` errors (for example
  `server.port: 8080` on one line and a nested `server:` block holding
  `port` on another).
- Collisions are hard errors: duplicate keys, leaf-vs-interior conflicts
  (`a: 1` and `a.b: 2`), and empty path segments (`.a`, `a.`, `a..b`,
  all `bad-path`).
- Intermediate nodes are prefixes only: they hold no value and need no
  schema entry. Only full keys hold values.
- The document root is a trie; a top-level list or dict marker is an error.
- A `key:` with nothing on the line and no indented subtree below is a
  `missing-value` error.
- In a list item holding node prefixes, the first path may share the `- `
  line (`- path: "/health"`); every subsequent key in that item must align
  to that first key's column. In a list item holding dict entries, the first
  entry may share the `- ` line (`- = "k": 1`); subsequent entries align to
  that first entry's column. Correct alignment, then the error form:

```kvd
endpoints:
  - path: "/health"
    method: "GET"
```

```kvd
# error: misaligned-key
endpoints:
  - path: "/health"
  method: "GET"
```

- Dotted paths inside a list item are relative to that item's root, not the
  document root.
- `null` is meaningful only under an optional schema type (`optional: true`).
  In a schema-less document it parses without error; when a schema is present,
  the verifier rejects `null` anywhere a non-optional type applies.

### Multi-line strings

`"""` is the only multi-line string form. The opener must be followed by a
newline (inline `"""x"""` is an error); content lines sit two columns past
the key's column.

```kvd
key: """
  line one
  line two
"""
```

Two closer forms exist. The position of `"""` controls whether a trailing newline
is included:

- **Standalone closer**: `"""` alone on a line at exactly the key's column.
  The string includes a trailing `\n`.
- **Inline closer**: `"""` appended directly to the last content line.
  No trailing `\n` is added.

```kvd
# standalone: value is "line one\nline two\n"
standalone: """
  line one
  line two
"""
```

```kvd
# inline: value is "line one\nline two"
inline: """
  line one
  line two"""
```

The common indentation of non-blank content lines is stripped. Blank lines
are kept as empty lines and excluded from the common-indent computation.
Escapes are processed per line (same rules as `"..."`). Because `"""` is
escape-processed, backslashes in embedded scripts or SQL must be doubled
(`\\`), and `\n` in content becomes a newline.

The same form works as a list item; the closer sits at the marker's column:

```kvd
hooks:
  - """
    #!/bin/sh
    set -e
    """
  - "plain"
```

### Nesting combinations

`-` and `=` compose. A value under either marker may be a scalar, a list, or
a dict:

```kvd
matrix:
  -
    - 1
    - 2
  -
    - 3
    - 4
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
