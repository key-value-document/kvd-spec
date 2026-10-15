[KVD spec](../../README.md), section 05

## 4. Grammar (EBNF)

`:=` is EBNF's definition operator. It is meta-syntax, not part of the format.
`NL` is a newline, `INDENT`/`DEDENT` mark nesting changes, `EOF` is end of
file, `SP{keycol}` means spaces up to the key's column.

```ebnf
document  := pairs? EOF

pairs     := pair (NL pair)*
pair      := path ':' value
path      := keyseg ('.' keyseg)*
keyseg    := key | dquote | squote

value     := ' ' scalar
           | ' ' '"""' NL triple      ; triple-quoted string (see below)
           | " ' '''" NL sqtriple    ; multi-line literal (see below)
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
           | "- ' '''" NL sqtriple    ; multi-line literal as list item
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
sqtriple  := sq-line* closer-sq
content-line := any line that is not the closer (escapes per `"..."`)
sq-line   := any line that is not the `'''` closer (verbatim, no escapes)
closer    := SP{keycol} '"""' NL          ; standalone: trailing \n included
           | content-line '"""' NL        ; inline: no trailing \n
closer-sq := SP{keycol} "'''" NL          ; standalone: trailing \n included
           | sq-line "'''" NL             ; inline: no trailing \n
```

`SP{keycol}` is the exact run of spaces ending at the key's column (for a
list-item `"""`, the marker's column). `content-line` excludes the closer
forms above.

Quoted keys (§02) may appear as any `keyseg` in `path`; a quoted
`"__name__"` segment is an ordinary key like any other.

Dict entry keys (`dkey`) are always quoted and always opaque: `"a.b.c/name"`
is one key. A bare word is never a dict key; write `= "k": v` even when `k`
looks bare. The `=` marker always carries its entry on the same line; an
`=` alone at end of line is a `bad-dict-marker` error.

### Schema documents

A schema is a separate document that mirrors a data document's structure
with builtin type names in place of values. A schema file (by convention
`name.schema.kvd`) is a bare KVD tree:

```kvd
# app.schema.kvd
server:
  port: int
  host: str
```

Schemas are never embedded in data documents. Values in schema position are a
builtin type name, a descriptor block, a single-element list declaring its
item type, a dict of any number of entries declaring per-key value types,
or the empty literals `{}` / `[]`. A bare `dict`/`list` leaf is a malformed
schema (use a `type: dict`/`type: list` descriptor or a `{}`/`[]` leaf).
The builtin type names are `int`, `float`, `bool`, `str`, `bytes`, `dict`,
`list`; `dict` and `list` appear only inside a descriptor's `type` key (they make a
container optional/validatable). Numbers, booleans, `null`, and quoted
strings are malformed-schema errors in schema position. A type name is
written bare; optionality is declared with `optional: true` in a descriptor
block. `bytes` is a type name only: it has no token in the data grammar, and
a `bytes`-typed key holds a quoted string holding base64 text (§05). See
[§05](05-values.md).

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

There are two multi-line forms, `"""` and `'''`, differing only in escape
processing. Everything else — the opener, the indentation, the two closer
forms, and the common-indent rule — is identical, and is specified once here.

The opener MUST be followed by a newline (inline `"""x"""` is an
`unexpected-character` error, and so is `'''x'''`); content lines MUST be
indented at least two columns past the key's column.

```kvd
key: """
  line one
  line two
"""
```

Two closer forms exist for each delimiter. The position of the closer controls
whether a trailing newline is included:

- **Standalone closer**: `"""` (or `'''`) alone on a line at exactly the key's
  column. The string includes a trailing `\n`. Any other indentation is a
  `bad-indent` error.
- **Inline closer**: the delimiter appended directly to the last content line.
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

The common indentation of non-blank content lines is stripped: the parser
MUST compute the longest common space prefix of the non-blank content lines
(at least the opener's content indent) and remove it. Blank lines
are kept as empty lines and excluded from the common-indent computation.

Inside `"""`, escapes are processed per line (same rules as `"..."`):
`\n` in content becomes a newline, so backslashes in embedded scripts or SQL
must be doubled (`\\`). Inside `'''`, **no escape is processed** — a backslash
is a backslash, which is what makes the literal form the right choice for a
script, a SQL statement, or a regular expression:

```kvd
# standalone `'''`: value is "#!/bin/sh\nset -eu\n"
script: '''
  #!/bin/sh
  set -eu
'''
```

```kvd
# inline `'''`: value is "a\tb\n" with no escape processing at all
literal: '''
  a\tb
  C:\dest\file'''
```

The same form works as a list item; the closer sits at the marker's column:

```kvd
hooks:
  - """
    #!/bin/sh
    set -e
    """
  - "plain"
  - '''
    #!/bin/sh
    set -eu
    '''
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
