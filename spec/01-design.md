[KVD spec](../README.md) — section 01

## 1. Design principles

1. Line-oriented. No flow collections: every value is one line, a `"""`
   block, or an indented subtree — the sole exceptions are the
   empty-collection literals `{}` and `[]` (§4). There is exactly one way
   to spell each thing.
2. Strict, fixed indentation: exactly 2 spaces per level. Tabs are illegal
   outside double-quoted strings.
3. No implicit coercion beyond a closed shape set. `yes`, `on`,
   `2026-08-20` are always errors — never silently bools.
4. Data files are self-describing at the shape level; types are an optional
   companion schema, never inline annotations.
5. Dotted keys are pure sugar for nesting; path collisions are hard errors.
6. No anchors/aliases, no merge keys, no tags, no directives, no
   multi-document streams, no duplicate keys. The only reserved namespace is
   metakeys (`__...__`, [§2](02-lexical.md)). The only bare tokens with
   non-string meaning in value position are integers and floats
   (`42`, `1_000`, `0.5`), the literals `true`, `false`, and `null`, and
   the empty-collection literals `{}` and `[]` — quote them to get strings.
   Bare type names (`int`, `float`, `bool`, `str`) have non-string meaning
   in schema position only ([§5](05-values.md)).
