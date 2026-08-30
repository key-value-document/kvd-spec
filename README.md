# KVD — Key-Value Document format

Name: **kvd**. Status: 1.0.
Goal: a general-purpose, opinionated config/data format that keeps YAML's
readability without its pitfalls, and is small enough to implement in ~200
lines of parser plus error handling.

Versioning: the format follows SemVer. Any change that makes a previously
valid document invalid, or changes the shape of a value, bumps the major
version; backward-compatible additions bump the minor version; everything
else is patch-level.

## Contents

1. [Design principles](spec/01-design.md) — closed shape set, no implicit coercion
2. [Lexical rules](spec/02-lexical.md) — encoding, indentation, comments, keys, strings, numbers
3. [Tokens](spec/03-tokens.md) — token grammar
4. [Grammar](spec/04-grammar.md) — EBNF, schema documents, structural rules
5. [Values and types](spec/05-values.md) — shape typing, schemas, multi-line strings
6. [Error model](spec/06-errors.md) — error categories
7. [Full example](spec/07-example.md) — a data document with its companion schema
8. [Operations](spec/08-operations.md) — parse, emit, verify, typed round-trip
9. [Non-goals](spec/09-non-goals.md) — features that are permanently out of scope
10. [Validation constraints](spec/10-validation.md) — ranges, lengths, patterns (planned for 1.1)
11. [Document merging](spec/11-merge.md) — per-field merge policy (planned for 1.x)
