[KVD spec](../../README.md), section 02

## 2. User guide: reading and writing KVD

This section is non-normative. It teaches the syntax by example. For exact
rules see §3 (lexical), §5 (grammar), and §6 (values).

### Maps, scalars, and nesting

A KVD file is a list of `key: value` pairs. Indent by 2 spaces to nest:

```kvd
server:
  host: "localhost"
  port: 8080
  debug: true
```kvd

Strings use double quotes. Numbers, `true`/`false`, and `null` stay bare.
Anything else bare is an error, so dates and yes/no words must be quoted.
`null` means "no value": without a schema just omit the key, or use `{}`
/ `[]` for an empty collection:

```kvd
when: "2026-08-20"
flag: "yes"
retries: null
```kvd

Dotted keys are shorthand for nesting. These two spellings mean the same:

```kvd
server.port: 8080
```kvd

```kvd
server:
  port: 8080
```kvd

### Lists

List items start with `- ` under their key, indented 2 spaces past it:

```kvd
tags:
  - "web"
  - "api"
```kvd

Objects in a list put the first key on the `- ` line and align the rest
under it:

```kvd
endpoints:
  - path: "/health"
    method: "GET"
  - path: "/ready"
    method: "GET"
```kvd

Empty collections have literals: `{}` is an empty map, `[]` is an empty
list:

```kvd
labels: {}
search: []
```kvd

### Multi-line strings

Open with `"""` after the colon, write content indented, close with `"""`.
A closer alone on its line keeps a trailing newline; a closer at the end
of the last content line does not:

```kvd
greeting: """
  hello
  world
"""
```kvd

### Comments, schemas, and validation

`#` starts a comment. A schema file uses type names where data has values:

```kvd
# app.schema.kvd
server:
  port: int
  host: str
```kvd

Append `optional: true` to allow a key to be absent or `null`, and add a
`validation` block for ranges, lengths, and patterns:

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
```kvd

With that schema, `host` may be omitted, be `null`, or hold a matching
string:

```kvd
server:
  port: 8080
  host: null
```

For exact rules see §11 (validation).

Common mistakes: tabs for indentation, `key:value` without a space,
`00` or `1_2` as numbers, and unquoted `2026-08-20`. Each is an error.
