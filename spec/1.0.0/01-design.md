[KVD spec](../../README.md), section 01

## 1. Design principles

1. Line-oriented. No flow collections: every value is one line, a `"""`
   block, or an indented subtree. The only exceptions are the
   empty-collection literals `{}` and `[]` (§5). There is exactly one way
   to spell each thing.
2. Strict, fixed indentation: exactly 2 spaces per level. Tabs are illegal
   outside quoted strings (single-quoted literals, double-quoted strings,
   `"""` blocks).
3. No implicit coercion beyond a closed shape set. `yes`, `on`,
   `2026-08-20` are always errors and never silently become bools.
4. Data files are self-describing at the shape level; types are an optional
   companion schema, never inline annotations.
5. Dotted keys are pure sugar for nesting; path collisions are hard errors.
6. No anchors/aliases, no merge keys, no tags, no directives, no
   multi-document streams, no duplicate keys. The only reserved namespace is
   metakeys (`__...__`, [§3](03-lexical.md)). The only bare tokens with
   non-string meaning in value position are integers and floats
   (`42`, `1_000`, `0.5`), the literals `true`, `false`, and `null`, and
   the empty-collection literals `{}` and `[]`. Quote them to get strings.
   Bare type names (`int`, `float`, `bool`, `str`) have non-string meaning
   in schema position only ([§6](06-values.md)).

Terms used throughout this spec: **document** (one file's content after
parsing; the root is always a map), **node** (any scalar, map, or list),
**scalar** (a single int, float, bool, string, or null value), **shape**
(the runtime kind of a value), **type** (a schema declaration such as
`int`), **schema** (a companion document of allowed keys and types), and
**canonical form** (the single standard spelling `emit` produces, §9.2).
