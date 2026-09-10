[KVD spec](../../README.md), section 10

## 9. Non-goals

The following features are permanently out of scope. They will not be added
in any future version of KVD.

### YAML compatibility features

- **Multi-document streams** (`---` / `...` separators). KVD files are
  single documents.
- **Anchors and aliases** (`&anchor`, `*alias`). There is no reference
  mechanism; duplication must be done in the consuming application or a
  pre-processor.
- **Merge keys** (`<<`). No implicit key merging.
- **Tags** (`!!str`, `!custom`). Types are an optional companion schema,
  never inline annotations.
- **Directives** (`%YAML`, `%TAG`). KVD has no directive layer.
- **Flow collections** (`{...}`, `[...]` as inline maps/lists). The only
  uses of `{}` and `[]` are the empty-collection literals.
- **Implicit coercion**. `2026-08-20`, `0x1f`, `0o17`,
  `1_000.5`, `Yes` are always errors. Lowercase bare words matching the
  `type` grammar (`yes`, `on`, `port`) parse as strings in data (§05),
  never as another shape. The value set is closed and explicit.
- **Block scalars** other than `"""` (`|`, `|-`, `>`, `>-`). The `"""`
  form covers all multi-line string needs.

### Other features

- **Custom types**. The builtin type set (`int`, `float`, `bool`, `str`, `dict`, `list`)
  is closed. There are no user-defined or pluggable types.
- **Single-quoted strings as canonical form**. Double-quote is the only
  canonical string delimiter; `'...'` is an accepted literal alias (§02)
  but is never emitted.
- **Inline annotations**. Types, comments, and metadata are never mixed
  into the value syntax.
- **Streaming / incremental parsing**. A KVD document is a complete unit;
  there is no append or patch protocol.
