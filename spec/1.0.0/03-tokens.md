[KVD spec](../../README.md), section 04

## 3. Tokens

The token grammar. `:=` is the definition operator (meta-syntax).

```ebnf
NL, INDENT, DEDENT, EOF
':'  '-'  '='  '"""'

key       := [A-Za-z0-9]
           | [A-Za-z0-9] [A-Za-z0-9_-]* [A-Za-z0-9]   ; no dots, no quotes
metakey   := "__" [a-z] [a-z0-9_-]* "__"

digit     := [0-9]
int       := [+-]? "0"
           | [+-]? [1-9] digit*
           | [+-]? [1-9] digit{0,2} ("_" digit{3})+
float     := [+-]? pint "." digit+ ([eE] [+-]? digit+)?
           | [+-]? pint [eE] [+-]? digit+
pint      := "0" | [1-9] digit*          ; ungrouped integer part

dquote    := '"' (escape | char)* '"'    ; char: any except '"' '\' NL
squote    := "'" char* "'"               ; literal: no escapes, no "'" inside
escape    := '\n' | '\t' | '\\' | '\"' | "\'" | '\u' hex{4}
hex       := [0-9A-Fa-f]

type      := [a-z] [a-z0-9_-]*          ; schema position only (optionality via descriptor, §05)

empty-map  := "{}"
empty-list := "[]"
```

Single-quoted `'...'` is a literal string with no escape processing (§02).
It is accepted for compatibility but is not canonical: `emit` always
produces `"..."`.

**Disambiguation:**
- In key position, `metakey` wins over `key`; `key` wins over `int`/`float`.
  `8080:` is a key; `8080` alone is an int.
- In value position, `type` tokens are accepted as bare strings. They are
  meaningful only in schema documents; in data documents they are strings
  whose value is the type name. A name that is not one of the six builtins
  is a malformed schema at verification time (`VerifyError::SchemaMalformed`,
  §05, §08.3), not a parse error and not a document violation.
- `{}` and `[]` are atomic tokens; `{`, `}`, `[`, `]` in any other context
  are errors.
- `=` is a token only as a dict entry marker at the start of a subtree
  line (`= ` plus the entry). Any other `=` is an error.
- `,` is not a token. A comma anywhere is an error.
- `#` is not a token. Comments are stripped during lexing.
- An unrecognized escape sequence is an `unexpected-character` error.
  Surrogate pairs (`\ud800` to `\udfff`) are always errors even if syntactically
  well-formed.
- `\'` is a valid escape in `"..."` (yields `'`). Single quotes have no
  special meaning inside double-quoted strings.
