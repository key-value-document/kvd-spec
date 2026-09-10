[KVD spec](../../README.md), section 01

## 1. Design principles

### Data model

A document is a set of entries, and each entry pairs a key with exactly one value. A value is a scalar, a list, or a dict: a scalar is a single int, float, bool, string, or null, a list is an ordered sequence of values, and a dict maps opaque keys to values. The full value model is defined in §04 and §05.

The smallest entries pair a flat key with a scalar:

```kvd
port: 8080
name: "hello"
debug: true
```

Values come from a closed shape set, and the parser never coerces text from one shape into another. Date-like and other non-shape bare tokens are errors: `2026-08-20`, `0x1f`, `1_000.5`, and `Yes` never become strings, bools, or numbers. Only certain bare tokens are permitted in value position: integers and floats (`42`, `1_000`, `0.5`), the literals `true`, `false`, and `null`, the empty-collection literals `{}` and `[]`, and bare words matching the `type` grammar (`[a-z][a-z0-9_-]*`). Lowercase bare words parse as strings in data documents (`yes`, `on`, `port`) and never coerce to another shape. Bare type names (`int`, `float`, `bool`, `str`, `dict`, `list`) carry non-string meaning in schema position only, as defined in §05; in data documents they parse as strings. Quoting a bare token forces the string spelling.

A date must therefore be quoted to become a string:

```kvd
when: "2026-08-20"
```

Lists are marked with `-` entries and dicts with `=` entries, and the two markers compose for nested lists and dicts. Dict keys are opaque and never split on dots, so open-ended names stay intact.

### Nesting and indentation

Keys can form a tree. A dotted key such as `a.b.c` nests inside its prefixes, so `a` and `a.b` are nodes and `a.b.c` is the key. A node is a shared prefix and carries no value. Only the full key holds a value, and any path collision is a hard error. An indented block is sugar for keys sharing a prefix: both spellings expand to the same tree.

The flat spelling writes each full path on its own line:

```kvd
server.port: 8080
server.host: "localhost"
```

The nested spelling groups both keys under their shared prefix:

```kvd
server:
  port: 8080
  host: "localhost"
```

A scalar is a leaf: it holds one datum and nothing below it, but other kinds of values can nest as well. A list holds an ordered sequence of values, and each item can itself be a scalar, a list, or a dict. A dict maps opaque keys to values, and each value can again be a scalar, a list, or a dict. An entry can therefore hold a whole subtree and not just a leaf.

A list of strings nests items under its key:

```kvd
tags:
  - "web"
  - "api"
```

A list of lists nests `-` blocks, with each bare `-` opening one inner list:

```kvd
matrix:
  -
    - 1
    - 2
  -
    - 3
    - 4
```

A dict of numbers keeps dotted names intact as single opaque keys:

```kvd
metrics:
  = "errors/total": 3
  = "a.b.c/name": 99.9
```

A dict of lists composes both markers, with the list nested under the entry:

```kvd
groups:
  = "team-a":
    - "amy"
    - "bo"
```

A dict of dicts nests `=` blocks, with each entry holding another dict:

```kvd
outer:
  = "a":
    = "b1": 1
    = "b2": 2
  = "c":
    = "d1": 3
    = "d2": 4
```

Nesting composes without limit except the depth cap in §02. Nested levels are spelled with exactly 2 spaces per level, so structure is always visible and unambiguous. Tabs are illegal outside quoted strings, which are single-quoted literals, double-quoted strings, and `"""` blocks.

### Schemas are separate

A data file describes its own shapes, so a reader can tell an int from a string without extra help. Types live in an optional companion schema and never appear as inline annotations on data values.

A schema mirrors the data tree with type names in place of values:

```kvd
server:
  port: int
  host: str
```

### No aliases, tags, or streams

KVD has no anchors, aliases, merge keys, tags, directives, multi-document streams, or duplicate keys. The only reserved namespace is metakeys (`__...__`), defined in §02.

Terms used throughout this spec are document, node, key, value, scalar, shape, type, schema, and canonical form. A document is one file's content after parsing, a trie of node prefixes ending in keys. A node is an interior prefix such as `path.to` in `path.to.key` and carries no value, while a key is a full path holding a value. A value is a scalar, a list, or a dict, and a scalar is a single int, float, bool, string, or null value. A shape is the runtime kind of a value, a type is a schema declaration such as `int`, a schema is a companion document of allowed keys and types, and a canonical form is the single standard spelling `emit` produces, defined in §08.2.
