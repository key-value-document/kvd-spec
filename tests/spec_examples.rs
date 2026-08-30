//! Drift guard: every KVD code block in the spec must parse, and the
//! section 7 example must verify against its companion schema.

use kvd_rs::{deserialize::from_str, schema::verify_from_str};

const DOCS_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn blocks(path: &str) -> Vec<String> {
    let text = std::fs::read_to_string(format!("{DOCS_DIR}/{path}")).unwrap();
    let mut out = Vec::new();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        if line.trim() == "```" {
            let mut block = Vec::new();
            for line in lines.by_ref() {
                if line.trim() == "```" {
                    break;
                }
                block.push(line);
            }
            out.push(block.join("\n"));
        }
    }
    out
}

#[test]
fn spec_code_blocks_parse() {
    for file in [
        "README.md",
        "spec/02-lexical.md",
        "spec/03-tokens.md",
        "spec/04-grammar.md",
        "spec/05-values.md",
        "spec/06-errors.md",
        "spec/07-example.md",
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
    let b = blocks("spec/07-example.md");
    assert!(b.len() >= 2, "section 7 needs data and schema blocks");
    verify_from_str(&b[0], &b[1])
        .unwrap_or_else(|e| panic!("section 7 data does not satisfy its schema: {e}"));
}
