# Key-Value Document Specification

> Configuration files and structured data as `key: value` entries. One key per entry, explicitly typed.

## Contents

- [Why KVD?](#why-kvd)
- [Quick start](#quick-start)
- [Guide](#guide)
  - [1. Keys are dotted paths](#1-keys-are-dotted-paths)
  - [2. Indentation is grouping sugar](#2-indentation-is-grouping-sugar)
  - [3. Values](#3-values)
  - [4. Lists](#4-lists)
  - [5. Dicts for open-ended names](#5-dicts-for-open-ended-names)
  - [6. Multiline text](#6-multiline-text)
  - [7. Comments, empty collections, and null](#7-comments-empty-collections-and-null)
  - [8. Schemas and validation](#8-schemas-and-validation)
  - [9. Advanced nesting](#9-advanced-nesting)
- [Normative specification](#normative-specification)
- [Using KVD](#using-kvd)
- [Versioning](#versioning)
- [Contributing](#contributing)
- [License](#license)

## Why KVD?

- **Single entry model:** each entry is a `key: value` pair. Dotted paths address fixed structure, `-` builds lists, `=` builds dicts.
- **No implicit coercion:** numbers, booleans, and `null` stand alone. Everything else is quoted text. Dates, `yes`/`no`, and versions never silently change type.
- **Closed shape set:** scalars, lists, and dicts only.
- **Built-in validation:** schemas are written in KVD and support optional keys, ranges, lengths, and patterns.
- **Tested examples:** every example in this repository is parsed by CI against the reference implementation.

## Quick start

Basic example:

```kvd
server.host: "localhost"
server.port: 8080
server.debug: false
```

Nested form. Indentation with 2 spaces groups keys under a shared prefix. Both forms define the same keys:

```kvd
server:
  host: "localhost"
  port: 8080
  debug: false
```

Lists, dicts, and comments:

```kvd
# Increase this if the server is slow.
timeout: 30
tags:
  - "web"
  - "api"
metrics:
  = "a.b.c/name": 99.9
  = "errors/total": 3
```

The Guide below defines each feature. The formal rules are in [Normative specification](#normative-specification).

## Guide

### 1. Keys are dotted paths

Keys are usually dotted paths holding one value:

```kvd
a.b.c: "x"
a.b.d: "y"
```

`a` and `a.b` are nodes: shared prefixes, not values. Only the full keys (`a.b.c`, `a.b.d`) hold values.

A path used as both a value and a prefix is an error:

```kvd
# error: leaf-interior-conflict
a: 1
a.b: 2
```

### 2. Indentation is grouping sugar

Indentation with 2 spaces groups keys sharing a prefix. This block is equivalent to the previous example:

```kvd
a:
  b:
    c: "x"
    d: "y"
```

Flat and nested forms are equivalent. Both define `server.port`:

```kvd
server.port: 8080
```

```kvd
server:
  port: 8080
```

Rules:

- Use 2 spaces per level. Tabs are allowed only inside quoted content, never for indentation.
- Blank lines are fine anywhere.
- Flat and nested forms can be mixed in one document.

### 3. Values

| What you write      | What it means            |
|---------------------|--------------------------|
| `8080`, `-3`, `0`   | integers                 |
| `4.5`, `-2.7`       | floats                   |
| `true`, `false`     | boolean                  |
| `null`              | no value (see below)     |
| `"hello"`           | text                     |
| `'hello'`           | text, taken literally (no escapes) |
| `"""` ... `"""`     | text spanning many lines |
| `'''` ... `'''`     | literal text spanning many lines |
| `{}`                | an empty dict            |
| `[]`                | an empty list            |

Text goes in double quotes. Numbers, `true`/`false`, and `null` stand alone.
Lowercase bare words (`yes`, `port`) also parse as strings in data; anything
else bare (dates, `Yes`, versions) is an error, so quote them to be safe:

```kvd
when: "2026-08-20"
flag: "yes"
version: "1.10"
```

**Double or single quotes?** Two independent axes: the quote character picks
whether escapes are processed, and the quote count picks whether the value may
span lines. Double is canonical and the default; single is fully literal.
Emitting a document always writes double quotes.

| | escapes | literal |
|---|---|---|
| single line | `"..."` | `'...'` |
| multi line | `"""..."""` | `'''...'''` |

```kvd
greeting: 'say "hello"'      # single: literal, nothing to escape
apostrophe: "it's fine"       # double: a ' needs no escape either
newline: "one\ntwo"          # double: escapes are processed here
path: 'C:\Users\me'          # single: backslashes stay exactly as written
```

Inside `"""` a backslash must be doubled; inside `'''` it never is, which makes
`'''` the right form for an embedded script or a SQL statement:

```kvd
script: '''
  #!/bin/sh
  set -eu
  cp "$SRC" C:\dest\file
'''
```

Two rules to know about single quotes: there is **no** way to put a `'` inside
`'...'`, and `'''` blocks follow the same indentation and closer rules as `"""`.
Inside any quoted form, `#` is ordinary text rather than a comment, and a tab is
ordinary text rather than an indentation error.

### 4. Lists

A key with `-` items underneath holds a list:

```kvd
tags:
  - "web"
  - "api"
```

### 5. Dicts for open-ended names

A key with `=` entries underneath holds a dict. Dict keys are always quoted and opaque: dots and slashes never split, so `"a.b.c/name"` is one key:

```kvd
metrics:
  = "a.b.c/name": 99.9
  = "errors/total": 3
```

When to use which:

- Use **dicts** for open-ended names you do not control: labels, annotations, metric names.
- Use **dotted paths** for fixed structure you control: `server.port`, `app.name`.

### 6. Multiline text

Open with `"""` after the colon, write content indented, close with `"""`:

```kvd
greeting: """
  hello
  world
"""
```

Trailing newline rule: a closer alone on its line keeps a trailing newline; a closer at the end of the last content line does not. See [Grammar](spec/1.1/04-grammar.md) for the exact rule.

### 7. Comments, empty collections, and null

Lines starting with `#` are comments and are ignored:

```kvd
# Increase this if the server is slow.
timeout: 30
```

`{}` is an empty dict, `[]` is an empty list:

```kvd
labels: {}
search: []
```

`null` denotes an explicitly unset value. Without a schema it parses as
untyped null; with a schema it requires `optional: true` (§05). Omit the key
if the setting is not needed:

```kvd
retries: null
```

### 8. Schemas and validation

A schema defines the allowed keys and value types using type names (`str`, `int`, `bool`, `float`, `bytes`, `dict`, `list`):

```kvd
server:
  port: int
  host: str
```

Example data matching the schema:

```kvd
server:
  port: 8080
  host: "localhost"
```

A value of `"eighty"` for `port` fails verification with a type error.

Schemas also support optional keys, ranges, lengths, and patterns:

```kvd
server:
  port:
    type: int
    validation:
      min: 1
      max: 65535
  host:
    type: str
    optional: true
    validation:
      min_len: 1
      max_len: 253
      pattern: "^[a-z][a-z0-9.-]*$"
```

With that schema, `host` may be omitted, be `null`, or hold a matching string:

```kvd
server:
  port: 8080
  host: null
```

Dict schemas declare per-key value types. Declared keys present in the data are checked; declared keys may be absent and undeclared data keys pass with any value type:

```kvd
metrics:
  = "errors/total": int
  = "a.b.c/name": float
```

See [Validation constraints](spec/1.1/09-validation.md) for the full list.

Binary data (`bytes`, new in 1.1) has no literal form: it is a base64 string in
the document plus a `type: bytes` declaration in the schema. There is no `b"..."`
or `0x...` spelling, so the value syntax stays closed. The value is always
quoted; line breaks inside it are ignored, so a wrapped or pasted PEM body works,
but a space or tab is content and invalidates the value.

```kvd
tls:
  cert:
    type: bytes
    validation:
      max_len: 4096
  key:
    type: bytes
    optional: true
```

See [Binary data](spec/1.1/05-values.md) for the decoding rules and the
`min_len`/`max_len` semantics.

### 9. Advanced nesting

`-` and `=` compose for nested shapes. A value under either marker may be a scalar, a list, or a dict.

<details>
<summary>Show all four combinations</summary>

A list of lists nests `-` blocks:

```kvd
matrix:
  -
    - 1
    - 2
  -
    - 3
    - 4
```

A dict of lists puts the list under the entry:

```kvd
groups:
  = "team-a":
    - "amy"
    - "bo"
  = "team-b":
    - "cy"
```

A dict of dicts nests `=` blocks:

```kvd
outer:
  = "a":
    = "b": 1
    = "c": 2
```

A list of dicts puts the first entry on the `-` line and aligns the rest under it:

```kvd
items:
  - = "a.b/c": 1
    = "d": 2
  - = "a.b/c": 3
    = "d": 4
```

</details>

## Normative specification

The Guide above is non-normative. The normative documents are in `spec/1.1/`:

| # | Document | Covers |
| --- | ---------- | -------- |
| 1 | [Design principles](spec/1.1/01-design.md) | Closed shape set, no implicit coercion |
| 2 | [Lexical rules](spec/1.1/02-lexical.md) | Encoding, indentation, comments, keys, strings, numbers |
| 3 | [Tokens](spec/1.1/03-tokens.md) | Token grammar |
| 4 | [Grammar](spec/1.1/04-grammar.md) | EBNF, schema documents, structural rules |
| 5 | [Values and types](spec/1.1/05-values.md) | Shape typing, schemas, multi-line strings |
| 6 | [Error model](spec/1.1/06-errors.md) | Error categories |
| 7 | [Full example](spec/1.1/07-example.md) | A data document with its companion schema |
| 8 | [Operations](spec/1.1/08-operations.md) | Parse, emit, verify, typed round-trip, programmatic editing |
| 9 | [Validation constraints](spec/1.1/09-validation.md) | Ranges, lengths, patterns |
| 10 | [Document merging](spec/1.1/10-merge.md) | Per-field merge policy (planned for 1.x) |

For data and schema side by side, see [Full example](spec/1.1/07-example.md).

## Using KVD

This repository holds the specification and its examples. The reference implementation is [`kvd-rs`](https://github.com/key-value-document/kvd-rs).

Repository layout:

```text
README.md        This guide
spec/1.1/        Normative documents 01-10
tests/           Drift guard: every spec example must parse
src/lib.rs       Empty crate target so cargo test runs
```

The spec directory is named for the format version and carries major and minor
only. Patch releases change nothing about what is valid, so they do not get a
directory: a `spec/1.1/` document set is the current one until the next minor.
The crate in `Cargo.toml` keeps a full SemVer `major.minor.patch`, which Cargo
requires.

Run the drift guard locally:

```bash
cargo test
```

It parses every KVD block in this README and the spec, and verifies the section 7 example against its companion schema.

## Versioning

The format follows SemVer:

- **Major:** a previously valid document becomes invalid, or a value changes shape.
- **Minor:** backward-compatible additions.
- **Patch:** clarifications and fixes that change nothing valid.

## Contributing

Spec fixes and clarifications are welcome. Keep examples minimal, and ensure `cargo test` passes before opening a PR. Intentional error samples must contain `# error` so the drift guard skips them.

## License

MIT. See [LICENSE](LICENSE).
