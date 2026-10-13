//! Drift guard: every KVD code block in the spec must parse, and the
//! section 7 example must verify against its companion schema.

use kvd_rs::{deserialize::from_str, schema::verify_from_str, serialize::to_string};

const DOCS_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn blocks(path: &str) -> Vec<String> {
    let text = std::fs::read_to_string(format!("{DOCS_DIR}/{path}")).unwrap();
    let mut out = Vec::new();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        let fence = line.trim();
        if fence == "```kvd" || fence == "```" {
            let mut block = Vec::new();
            for line in lines.by_ref() {
                if line.trim().starts_with("```") {
                    break;
                }
                block.push(line);
            }
            out.push(block.join("\n"));
        } else if fence.starts_with("```") {
            // Skip non-KVD fences (ebnf, text): consume until closer.
            for line in lines.by_ref() {
                if line.trim().starts_with("```") {
                    break;
                }
            }
        }
    }
    out
}

#[test]
fn spec_code_blocks_parse() {
    for file in [
        "README.md",
        "spec/1.1/01-design.md",
        "spec/1.1/02-lexical.md",
        "spec/1.1/03-tokens.md",
        "spec/1.1/04-grammar.md",
        "spec/1.1/05-values.md",
        "spec/1.1/06-errors.md",
        "spec/1.1/07-example.md",
        "spec/1.1/08-operations.md",
        "spec/1.1/09-validation.md",
        "spec/1.1/10-merge.md",
    ] {
        for (i, block) in blocks(file).iter().enumerate() {
            if block.contains(":=") || block.contains("# error") {
                continue; // grammar or an intentional-error sample
            }
            from_str(block).unwrap_or_else(|e| panic!("{file} block {i} fails to parse: {e}"));
        }
    }
}

#[test]
fn section7_example_verifies() {
    let b = blocks("spec/1.1/07-example.md");
    assert!(b.len() >= 2, "section 7 needs data and schema blocks");
    verify_from_str(&b[0], &b[1])
        .unwrap_or_else(|e| panic!("section 7 data does not satisfy its schema: {e}"));
}


/// §05: `bytes` is schema-level only. The parser must see a bytes field as an
/// ordinary string and the emitter must reproduce its base64 text verbatim.
#[test]
fn bytes_field_is_a_string_until_verified() {
    let doc = "cert: \"AQIDBA==\"\nempty: \"\"\n";
    let node = from_str(doc).expect("bytes field parses as a string");
    assert_eq!(
        to_string(&node).expect("bytes field emits"),
        doc,
        "a bytes value must round-trip as the identical string"
    );
}

/// §05: base64 must be quoted; `=`, `+`, and `/` are not valid in a bare token,
/// so an unquoted value is a parse error rather than a bytes literal.
#[test]
fn unquoted_base64_is_a_parse_error() {
    for src in ["cert: AQIDBA==\n", "cert: AQID\n", "cert: ab+c\n"] {
        assert!(
            from_str(src).is_err(),
            "{src:?} must not parse: bytes values are always quoted"
        );
    }
}

/// §05: line breaks in a wrapped `"""` block are layout, not content. The block
/// parses and is emitted unchanged; only `verify` drops the newlines.
#[test]
fn wrapped_base64_block_is_byte_stable() {
    let doc = "pem: \"\"\"\n  AQID\n  BA==\n\"\"\"\n";
    let node = from_str(doc).expect("a wrapped base64 block parses");
    assert_eq!(
        to_string(&node).expect("emits"),
        doc,
        "the emitter must not re-wrap a bytes value"
    );
}

/// §02: `'...'` is a literal alias; emit normalizes it to `"..."`.
#[test]
fn single_quotes_emit_as_double_quotes() {
    let node = from_str("a: 'x'\n").expect("single-quoted value parses");
    assert_eq!(
        to_string(&node).expect("emits"),
        "a: \"x\"\n",
        "single quotes are not canonical"
    );
}

/// §02: single quotes take no escapes, so `\\n` stays a backslash and an `n`.
#[test]
fn single_quotes_have_no_escapes() {
    let node = from_str("a: 'x\\ny'\n").expect("literal backslash parses");
    assert_eq!(
        to_string(&node).expect("emits"),
        "a: \"x\\\\ny\"\n",
        "`\\n` in a single-quoted string is not a newline"
    );
}

/// §02: a quoted key segment is literal in either form, so dots stay in the key.
#[test]
fn quoted_key_segments_are_literal() {
    let node = from_str("'a.b': 1\n").expect("single-quoted key parses");
    assert_eq!(
        to_string(&node).expect("emits"),
        "\"a.b\": 1\n",
        "a quoted key with a dot is one key, not a path"
    );
}

/// §2, §4: `'''` is the multi-line literal form. Backslashes stand for
/// themselves, and emit still produces a double-quoted string.
#[test]
fn sqtriple_block_is_literal_and_emits_canonical() {
    let doc = "script: '''\n  set -eu\n  cp C:\\dest\\f\n'''\n";
    let node = from_str(doc).expect("a ''' block parses");
    match &node {
        kvd_rs::value::Node::Map(m) => match m.get("script") {
            Some(kvd_rs::value::Node::Scalar(s)) => {
                assert_eq!(s.text, "set -eu\ncp C:\\dest\\f\n", "no escape processing");
            }
            other => panic!("script is not a scalar: {other:?}"),
        },
        other => panic!("not a map: {other:?}"),
    }
    // Emit uses a `"""` block, and since `"""` processes escapes the
    // literal backslashes are doubled on the way out.
    assert_eq!(
        to_string(&node).expect("emits"),
        "script: \"\"\"\n  set -eu\n  cp C:\\\\dest\\\\f\n\"\"\"\n",
        "emit uses a \"\"\" block, never '''"
    );
}
