# KVD: Key-Value Document format

KVD is a small format for configuration files and structured data. If you
can write a shopping list, you can write KVD: each line is a `key: value`
pair, and indenting with 2 spaces groups lines together.

## A first look

```kvd
server:
  host: "localhost"
  port: 8080
  debug: false
```

Three things to notice:

- `server:` with nothing after it starts a group; the indented lines below
  belong to it.
- Text goes in double quotes (`"localhost"`). Numbers (`8080`),
  `true`/`false`, and `null` stand alone without quotes.
- Indentation is always 2 spaces per level. Tabs are not allowed.

Deeper nesting just indents further:

```kvd
server:
  tls:
    enabled: true
    cert: "/etc/app/cert.pem"
```

As a shortcut, dots in a key mean the same as nesting, so
`server.port: 8080` and the two-line form above are identical.

## Lists

A key with list items underneath holds a list. Each item starts with `- `:

```kvd
tags:
  - "web"
  - "api"
```

Items can be groups too. Put the first key on the `- ` line and line the
rest up under it:

```kvd
endpoints:
  - path: "/health"
    method: "GET"
  - path: "/ready"
    method: "GET"
```

## Values at a glance

| What you write        | What it means              |
|-----------------------|----------------------------|
| `"hello"`             | text                       |
| `8080`, `4.5`, `-3`   | numbers                    |
| `true`, `false`       | yes/no                     |
| `null`                | no value (see below)       |
| `{}`                  | an empty group             |
| `[]`                  | an empty list              |
| `"""` ... `"""`       | text spanning many lines   |

`null` means "no value". If a setting is simply not needed, leave the key
out; use `null` when you want to say explicitly that it is unset:

```kvd
retries: null
labels: {}
search: []
```

Anything else must be quoted. Dates, yes/no words, and version numbers are
text, so they need quotes:

```kvd
when: "2026-08-20"
flag: "yes"
version: "1.10"
```

Lines starting with `#` are comments for humans and are ignored:

```kvd
# Increase this if the server is slow.
timeout: 30
```

## Checking your config with a schema

A schema describes what a valid config looks like: it uses type names
(`str`, `int`, `bool`, `float`, `null`, `map`, `list`, `any`) where data
has values.

```kvd
server:
  port: int
  host: str
```

The config below satisfies that schema; changing `port` to `"eighty"`
would fail with a type error instead of breaking the server at startup:

```kvd
server:
  port: 8080
  host: "localhost"
```

Schemas can also mark keys optional and set allowed ranges, lengths, and
text patterns. Start with the [user guide](spec/1.0.0/02-guide.md), then
read [validation](spec/1.0.0/11-validation.md) when you need constraints.

## Contents

1. [Design principles](spec/1.0.0/01-design.md): closed shape set, no implicit coercion
2. [User guide](spec/1.0.0/02-guide.md): reading and writing KVD (non-normative)
3. [Lexical rules](spec/1.0.0/03-lexical.md): encoding, indentation, comments, keys, strings, numbers
4. [Tokens](spec/1.0.0/04-tokens.md): token grammar
5. [Grammar](spec/1.0.0/05-grammar.md): EBNF, schema documents, structural rules
6. [Values and types](spec/1.0.0/06-values.md): shape typing, schemas, multi-line strings
7. [Error model](spec/1.0.0/07-errors.md): error categories
8. [Full example](spec/1.0.0/08-example.md): a data document with its companion schema
9. [Operations](spec/1.0.0/09-operations.md): parse, emit, verify, typed round-trip, programmatic editing
10. [Non-goals](spec/1.0.0/10-non-goals.md): features that are permanently out of scope
11. [Validation constraints](spec/1.0.0/11-validation.md): ranges, lengths, patterns
12. [Document merging](spec/1.0.0/12-merge.md): per-field merge policy (planned for 1.x; null-deletes, literal by-key, strict ShapeMismatch resolved)

## Versioning

The format follows SemVer. Any change that makes a previously valid
document invalid, or changes the shape of a value, bumps the major version;
backward-compatible additions bump the minor version; everything else is
patch-level.
