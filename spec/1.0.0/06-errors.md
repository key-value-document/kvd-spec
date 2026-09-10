[KVD spec](../../README.md), section 07

## 6. Error model

Every error carries a `line:col` position and a category. The grammar is
deterministic enough that an invalid document has exactly one explanation.
Parse errors come from reading the text; verification errors come from
checking parsed data against a schema. `line:col` is the offending line and
the column where the offending token starts; for INDENT/DEDENT failures it
is the line that required the indent change, column 1; for a missing
terminator at EOF it is the last line's end. A `---` or `...` line is not a
document separator (single document only, §09): its `-` is a
`bad-list-marker` and its `.` an `unexpected-character`.

### Parse errors

Produced by the parser from document text:

| Category | Meaning |
|---|---|
| `bad-indent` | Indentation is not a multiple of 2, or a subtree is misaligned |
| `tab` | A tab is used for indentation, or appears outside a quoted string or a `"""` block (tabs are allowed only inside quoted content) |
| `bad-list-marker` | A `-` marker must be followed by exactly one space or a newline; a `-` at end of line opens a nested list. It is also an error when a `-` appears where a key was expected |
| `bad-dict-marker` | An `=` marker must be followed by exactly one space plus a quoted key on the same line; an `=` alone at end of line or anywhere a key was expected is an error |
| `misaligned-key` | A key in a list item does not align to the first key of that item; a dict entry in a list item does not align to the first entry |
| `missing-value` | A key has no value and no indented subtree |
| `duplicate-key` | The same key or dotted path appears twice |
| `leaf-interior-conflict` | A path is used as both a leaf value and an interior node |
| `bad-path` | A dotted path has an empty segment, a leading dot, or a trailing dot |
| `unterminated` | A `"..."` string or `"""` block has no closing delimiter |
| `unknown-metakey` | A `__name__` key is not a defined metakey |
| `metakey-outside-root` | A metakey appears outside the document root |
| `depth-limit` | Nesting exceeds the maximum depth (default 100) |
| `unexpected-character` | A character or token is not valid in this position |

Valid input parses; mis-indented input reports its category:

```kvd
a: "ok"
```

```kvd
# error: bad-indent (3 spaces)
a:
   b: "x"
```

### Verification errors

Produced by the schema verifier as `Violation` values (path plus message),
not as parse errors. Each carries the dotted path of the offending value.
Schema-shape problems (quoted or numbered type leaf, bare `dict`/`list`
leaf, descriptor missing its `type`, unknown type name, unknown descriptor
or constraint key, schema list with anything but exactly one element type)
are reported as a malformed schema (§08.3), distinct from document
violations:

- **unknown key**: a data key has no counterpart in the schema
- **missing key**: a required schema key is absent from the data
- **schema mismatch**: a value's shape does not match its declared type
- **unknown type**: a schema leaf names a type that is not a builtin
- **null without optional type**: `null` appears under a non-optional type
- **constraint**: a value violates a declared constraint
  (range, length, or pattern); see [§10](10-validation.md)
