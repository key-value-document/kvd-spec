[KVD spec](../../README.md), section 04

## 4. Grammar (EBNF)

`:=` is EBNF's definition operator. It is meta-syntax, not part of the format.

```
document  := pairs? EOF

pairs     := pair (NL pair)*
pair      := (metakey | path) ':' value
path      := key ('.' key)*

value     := ' ' scalar
           | ' ' '"""' NL triple      ; triple-quoted string (see below)
           | NL INDENT pairs DEDENT   ; nested mapping
           | NL INDENT items DEDENT   ; list

scalar    := int | float
           | 'true' | 'false' | 'null'
           | dquote
           | empty-map | empty-list

items     := item (NL item)*
item      := '- ' scalar
           | '- ' '"""' NL triple     ; triple-quoted string as list item
           | '- ' pair                 ; inline first key of a mapping item
           | '- ' items               ; compact nested list  (- - "x")
           | '-' NL INDENT pairs DEDENT
           | '-' NL INDENT items DEDENT

triple    := content-line* closer
closer    := SP{keycol} '"""' NL          ; standalone: trailing \n included
           | content-line '"""' NL        ; inline: no trailing \n
```

`metakey` pairs are retained in the root map; the same pattern
anywhere else is an error. The parser needs only one token of lookahead.

### Schema documents

A schema mirrors a data document's structure with builtin type names in place
of values. A standalone schema file (by convention `name.schema.kvd`) is a
bare KVD tree with no metakeys:

```
# app.schema.kvd
server:
  port: int
  host: str
```

A data file may embed its schema under `__schema__`:

```
__schema__:
  server:
    port: int
server:
  port: 8080
```

`__schema__` is the only defined metakey. Values in schema position are a
builtin type name, a descriptor block, or the empty literals `{}` / `[]`.
The builtin type names are `int`, `float`, `bool`, `str`, `list`, `map`;
`list` and `map` appear only inside a descriptor's `type` key (they make a
container optional/validatable). numbers, booleans, `null`, and quoted
strings are errors in schema position. A type name is written bare;
optionality is declared with `optional: true` in a descriptor block. See
[§5](05-values.md).

Metakeys are excluded from the data tree before verification, so an embedded
schema never appears as a data key.

### Structural rules

- Dotted paths and nested blocks are interchangeable spellings of the same
  tree and may mix freely in one document.
- Collisions are hard errors: duplicate keys, duplicate paths via different
  spellings (`a.b: 1` and `a: b: 1`), leaf-vs-interior conflicts (`a: 1`
  and `a.b: 2`), and empty path segments (`.a`, `a.`, `a..b`).
- The document root must be a mapping; a top-level list item is an error.
- A `key:` with nothing on the line and no indented subtree below is an
  error.
- In a mapping list item, the first key may share the `- ` line
  (`- path: "/health"`); every subsequent key in that item must align to
  that first key's column. Classic misalignment:

  ```
  endpoints:
    - path: "/health"
    method: "GET"     # error: misaligned-key
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

```
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

```
# standalone: value is "line one\nline two\n"
standalone: """
  line one
  line two
"""
```

```
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

```
hooks:
  - """
    #!/bin/sh
    set -e
    """
  - "plain"
```
