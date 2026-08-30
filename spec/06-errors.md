[KVD spec](../README.md) — section 06

## 6. Error model

Every error carries a `line:col` position and a category. The grammar is
deterministic enough that an invalid document has exactly one explanation.

### Parse errors

Produced by the parser from document text:

| Category | Meaning |
|---|---|
| `bad-indent` | Indentation is not a multiple of 2, or a subtree is misaligned |
| `tab` | A tab is used for indentation, or appears outside a double-quoted string or a `"""` block (tabs are allowed only inside `"""` content) |
| `bad-list-marker` | A `-` marker must be followed by exactly one space or a newline; a `-` at end of line opens a nested list. It is also an error when a `-` appears where a mapping key was expected |
| `misaligned-key` | A key in a mapping list item does not align to the first key of that item |
| `missing-value` | A key has no value and no indented subtree |
| `duplicate-key` | The same key or dotted path appears twice |
| `leaf-interior-conflict` | A path is used as both a leaf value and an interior node |
| `bad-path` | A dotted path has an empty segment, a leading dot, or a trailing dot |
| `unterminated` | A `"..."` string or `"""` block has no closing delimiter |
| `unknown-metakey` | A `__name__` key is not a defined metakey |
| `metakey-outside-root` | A metakey appears outside the document root |
| `depth-limit` | Nesting exceeds the maximum depth (default 100) |
| `unexpected-character` | A character or token is not valid in this position |

### Verification errors

Produced by the schema verifier as `Violation` values (path + message),
not as parse errors. Each carries the dotted path of the offending value:

- **unknown key** — a data key has no counterpart in the schema
- **missing key** — a required schema key is absent from the data
- **schema mismatch** — a value's shape does not match its declared type
- **unknown type** — a schema leaf names a type that is not a builtin
- **null without optional type** — `null` appears under a non-optional type
- **constraint** (planned, 1.1) — a value violates a declared constraint
  (range, length, or pattern); see [§10](10-validation.md)
