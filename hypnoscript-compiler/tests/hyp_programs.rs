//! End-to-end integration tests over the sample programs in
//! `hypnoscript-tests/`.
//!
//! Every `.hyp` file in that directory must be listed either in
//! [`EXPECTED_PASS`] (it is lexed, parsed and interpreted and must succeed)
//! or in [`KNOWN_UNSUPPORTED`] (it exercises syntax or behavior the toolchain
//! does not support yet, with a reason). A new sample program that is not
//! categorized fails the sweep test, so coverage cannot silently rot.

use hypnoscript_compiler::Interpreter;
use hypnoscript_lexer_parser::{Lexer, Parser};
use std::path::PathBuf;

/// Sample programs that must run successfully end-to-end.
const EXPECTED_PASS: &[&str] = &[
    "test_assertions.hyp",
    "test_async_system.hyp",
    "test_basic.hyp",
    "test_channels.hyp",
    "test_compiler.hyp",
    "test_extended_builtins.hyp",
    "test_new_features.hyp",
    "test_parallel_execution.hyp",
    "test_pattern_matching.hyp",
    "test_pendulum_debug.hyp",
    "test_rust_demo.hyp",
    "test_scoping.hyp",
    "test_simple.hyp",
    "test_simple_features.hyp",
    "test_simple_new_features.hyp",
    "test_tranceify.hyp",
];

/// Sample programs that are known not to run, with the reason. These document
/// gaps rather than hiding them; when a gap is closed, move the file to
/// [`EXPECTED_PASS`].
const KNOWN_UNSUPPORTED: &[(&str, &str)] = &[
    (
        "medium_test.hyp",
        "uses 'drift' as a statement keyword, which the parser does not support",
    ),
    (
        "simple_test.hyp",
        "file is not valid UTF-8 (UTF-16 encoded legacy file)",
    ),
    (
        "test.hyp",
        "legacy file with a byte-order mark / stray tokens before 'Focus'",
    ),
    (
        "test_advanced.hyp",
        "uses generic array type syntax 'array[...]' that the parser does not support",
    ),
    (
        "test_all_new_features.hyp",
        "uses nullable type suffix 'type?' that the parser does not support",
    ),
    (
        "test_async.hyp",
        "uses nullable type suffix 'type?' that the parser does not support",
    ),
    (
        "test_comprehensive.hyp",
        "uses 'drift' as a statement keyword, which the parser does not support",
    ),
    (
        "test_enterprise_features.hyp",
        "uses 'sharedTrance' with a bare identifier instead of a declaration keyword",
    ),
    (
        "test_enterprise_v3.hyp",
        "uses 'sharedTrance' with a bare identifier instead of a declaration keyword",
    ),
    (
        "test_extended_features.hyp",
        "runs an unbounded loop / suspend and never terminates",
    ),
    (
        "test_new_language_features.hyp",
        "uses multi-expression 'whisper ... command ...' syntax the parser does not support",
    ),
];

fn samples_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("hypnoscript-tests")
}

fn run_program(source: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex().map_err(|e| format!("lex error: {}", e))?;
    let mut parser = Parser::new(tokens);
    let ast = parser
        .parse_program()
        .map_err(|e| format!("parse error: {}", e))?;
    let mut interpreter = Interpreter::new();
    interpreter
        .execute_program(ast)
        .map_err(|e| format!("runtime error: {}", e))
}

#[test]
fn expected_pass_programs_run_successfully() {
    let dir = samples_dir();
    let mut failures = Vec::new();

    for name in EXPECTED_PASS {
        let path = dir.join(name);
        let source = match std::fs::read_to_string(&path) {
            Ok(source) => source,
            Err(e) => {
                failures.push(format!("{}: cannot read file: {}", name, e));
                continue;
            }
        };
        if let Err(e) = run_program(&source) {
            failures.push(format!("{}: {}", name, e));
        }
    }

    assert!(
        failures.is_empty(),
        "sample programs failed:\n{}",
        failures.join("\n")
    );
}

#[test]
fn every_sample_program_is_categorized() {
    let dir = samples_dir();
    let mut uncategorized = Vec::new();
    let mut seen = Vec::new();

    for entry in std::fs::read_dir(&dir).expect("hypnoscript-tests directory must exist") {
        let entry = entry.expect("readable directory entry");
        let file_name = entry.file_name().to_string_lossy().into_owned();
        if !file_name.ends_with(".hyp") {
            continue;
        }
        seen.push(file_name.clone());

        let known = EXPECTED_PASS.contains(&file_name.as_str())
            || KNOWN_UNSUPPORTED.iter().any(|(name, _)| *name == file_name);
        if !known {
            uncategorized.push(file_name);
        }
    }

    assert!(
        !seen.is_empty(),
        "no .hyp files found in {} — wrong directory?",
        dir.display()
    );
    assert!(
        uncategorized.is_empty(),
        "new sample programs must be added to EXPECTED_PASS or KNOWN_UNSUPPORTED: {:?}",
        uncategorized
    );

    // Listed files must actually exist, so stale entries are caught too.
    for name in EXPECTED_PASS
        .iter()
        .chain(KNOWN_UNSUPPORTED.iter().map(|(name, _)| name))
    {
        assert!(
            seen.iter().any(|f| f == name),
            "listed sample program '{}' does not exist in {}",
            name,
            dir.display()
        );
    }
}
