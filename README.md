# KVD: Key-Value Document format

Name: **kvd**. Status: 1.0.
Goal: a general-purpose, opinionated config/data format that keeps YAML's
readability without its pitfalls, and is small enough to implement in ~200
lines of parser plus error handling.

Versioning: the format follows SemVer. Any change that makes a previously
valid document invalid, or changes the shape of a value, bumps the major
version; backward-compatible additions bump the minor version; everything
else is patch-level.

## Contents

1. [Design principles](spec/1.0.0/01-design.md): closed shape set, no implicit coercion
2. [User guide](spec/1.0.0/02-guide.md): reading and writing KVD (non-normative)
3. [Lexical rules](spec/1.0.0/03-lexical.md): encoding, indentation, comments, keys, strings, numbers
4. [Tokens](spec/1.0.0/04-tokens.md): token grammar
5. [Grammar](spec/1.0.0/05-grammar.md): EBNF, schema documents, structural rules
6. [Values and types](spec/1.0.0/06-values.md): shape typing, schemas, multi-line strings
7. [Error model](spec/1.0.0/07-errors.md): error categories
8. [Full example](spec/1.0.0/08-example.md): a data document with its companion schema
9. [Operations](spec/1.0.0/09-operations.md): parse, emit, verify, typed round-trip
10. [Non-goals](spec/1.0.0/10-non-goals.md): features that are permanently out of scope
11. [Validation constraints](spec/1.0.0/11-validation.md): ranges, lengths, patterns
12. [Document merging](spec/1.0.0/12-merge.md): per-field merge policy (planned for 1.x)
