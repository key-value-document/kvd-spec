[KVD spec](../../README.md), section 01

## 1. Design principles

1. Entry-oriented. No flow collections: each key starts its entry on one
   line; its value is a scalar on that line, a `"""`
   block, or an indented `-` / `=` subtree. The only exceptions are the
   empty-collection literals `{}` and `[]` (§04). There is exactly one way
   to spell each thing.
2. Strict, fixed indentation: exactly 2 spaces per level. Tabs are illegal
   outside quoted strings (single-quoted literals, double-quoted strings,
   `"""` blocks).
3. No implicit coercion beyond a closed shape set. `yes`, `on`,
   `2026-08-20` are always errors and never silently become bools.
4. Data files are self-describing at the shape level; types are an optional
   companion schema, never inline annotations.
5. Keys are usually dotted paths (`a.b.c: value`); indenting is sugar for
   a shared node prefix. Intermediate nodes are not values: only a full
   key holds a scalar, list, or dict value; path collisions are hard errors.
6. Dicts are explicit values marked with `=` entries; dict keys are opaque
   and never split on dots. Lists are marked with `-` entries. The two
   markers compose for nested lists and dicts.
7. No anchors/aliases, no merge keys, no tags, no directives, no
   multi-document streams, no duplicate keys. The only reserved namespace is
   metakeys (`__...__`, [§02](02-lexical.md)). The only bare tokens with
   non-string meaning in value position are integers and floats
   (`42`, `1_000`, `0.5`), the literals `true`, `false`, and `null`, and
   the empty-collection literals `{}` and `[]`. Quote them to get strings.
   Bare type names (`int`, `float`, `bool`, `str`, `dict`, `list`) have
   non-string meaning in schema position only ([§05](05-values.md)).

Terms used throughout this spec: **document** (one file's content after
parsing; a trie of node prefixes ending in keys), **node** (an interior
prefix such as `path.to` in `path.to.key`; it carries no value),
**key** (a full path holding a value), **value** (a scalar, list, or dict),
**scalar** (a single int, float, bool, string, or null value), **shape**
(the runtime kind of a value), **type** (a schema declaration such as
`int`), **schema** (a companion document of allowed keys and types), and
**canonical form** (the single standard spelling `emit` produces, §08.2).
