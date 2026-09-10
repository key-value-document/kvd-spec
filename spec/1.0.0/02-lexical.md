[KVD spec](../../README.md), section 03

## 2. Lexical rules

### Encoding and line endings

- UTF-8 only. A BOM is an error.
- CRLF is normalized to LF before parsing; a lone `CR` is an error.
- The last line may omit its terminating newline.

### Indentation

- Spaces only, exactly 2 per level. Any other count is an error
  (`bad-indent`).
- A tab anywhere outside a quoted string (single-quoted literal,
  double-quoted string, or `"""` block) is a `tab` error.
- Blank lines are allowed anywhere between tokens and carry no indent
  semantics. Trailing whitespace on any line is ignored.

### Comments

- `#` to end of line, allowed anywhere except inside a quoted string.
- There are no block comments.

### Newlines and structure

- Newlines are structural: they terminate lines and drive INDENT/DEDENT.
  Values never span lines except inside quoted strings (`\n` escape) and
  `"""` blocks ([§04](04-grammar.md)).

### Strings

- String values use double quotes (`"..."`), the single-quoted literal
  form (`'...'`, no escapes), or the multi-line block (`"""..."""`,
  [§04](04-grammar.md)).
- Non-ASCII characters in strings may be written literally or escaped as
  `\uXXXX` (Unicode code points U+0000 to U+FFFF). Code points above U+FFFF
  must be written literally as UTF-8; there is no surrogate-pair escape.
  Surrogate escapes (`\ud800` to `\udfff`) are always errors.

### Numbers

- A leading `+` or `-` followed by a digit starts a number (`+5`, `-5.5`).
  Any other use of `+` or `-` as a value token is an error.
- A token beginning with a digit must match the `int` or `float` grammar
  exactly. Partial matches (`5foo`, `1_2`, `00`, `1_000.5`) are errors.
- Thousands separators: `_` groups exactly 3 digits after the first 1 to 3
  digits (`1_000`, `45_678_112`); any other grouping is an error.
  Thousands separators are allowed only in integers, not in floats.

### Keys

- Bare keys are restricted to ASCII alphanumerics plus `-` and `_`, must
  not start or end with `-` or `_`, and may start with a digit (`8080`,
  `2fa`).
- A key that is not a valid bare key (for example one containing `.`, `/`,
  or `:`) is written quoted with `"` or `'`; the quoted spelling denotes
  that literal key.
- Dots are path separators, not key characters. A bare key can never
  contain a dot. Inside a dict entry (`=`), the entry key is opaque: a
  quoted `"a.b.c/name"` is one key and dots never split it.

### Metakeys

- A bare key matching `__name__` (double underscores, a lowercase letter,
  then lowercase letters/digits/`-`/`_`, double underscores) is a metakey.
- Metakeys are reserved and allowed only at the document root. The only
  defined metakey is `__schema__` ([§04](04-grammar.md)), which is retained
  in the root ([§08.1](08-operations.md)) but excluded from verification
  ([§08.3](08-operations.md)). Any other bare metakey is an
  `unknown-metakey` error. A quoted `"__name__"` is an ordinary literal
  key, not a metakey.

### Separators and markers

- The `:` separator is followed by exactly one space when the value is on the
  same line (`key: "value"`, `key: {}`, `key: """`). `key:value` is an error.
  When the value is an indented subtree, `:` is followed immediately by
  end of line.
- List marker: `- ` (dash plus exactly one space), sitting at the parent key's
  indent plus 2. A `-` alone at end of line introduces a nested list item
  ([§04](04-grammar.md)). Any other standalone `-` is a `bad-list-marker`
  error.
- Dict marker: `= ` (equals plus exactly one space), sitting at the parent
  key's indent plus 2. An `=` alone at end of line is a `bad-dict-marker`
  error; dict entries always carry their key on the marker line
  (`= "k": value`, [§04](04-grammar.md)). Any other standalone `=` is a
  `bad-dict-marker` error.

### Empty collections and limits

- `{}` is an empty dict; `[]` is an empty list. They are atomic tokens.
  `{`, `}`, `[`, `]` may not appear in any other context.
- An empty document (or a comments-only document) parses as an empty trie
  (no keys).
- Max nesting depth: 100. The limit is configurable in implementations; it
  counts indent levels and dotted path segments together. There are no
  aliases, so there is no billion-laughs expansion class.
