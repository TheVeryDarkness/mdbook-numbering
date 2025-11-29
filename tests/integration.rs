//! Tests for mdbook-numbering preprocessor integration.

use std::io::Write;
use std::process::{Command, Stdio};
use std::str::FromStr;

use mdbook_numbering::NumberingPreprocessor;
use mdbook_preprocessor::book::{Book, BookItem, Chapter};
use mdbook_preprocessor::config::Config;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use text_diff::assert_diff;

fn run(ctx: &PreprocessorContext, book: Book, expected_stderr: &str) -> Book {
    let process = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "mdbook-numbering"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let mut process = process.expect("Failed to start mdbook-numbering preprocessor");
    write!(
        process.stdin.as_mut().unwrap(),
        "{}",
        serde_json::to_string(&(ctx, book)).unwrap()
    )
    .unwrap();
    let output = process.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_diff(&stderr, expected_stderr, "\n", 0);
    serde_json::from_slice(&output.stdout).unwrap()
}

fn assert_book_equal(expected: &Book, actual: &Book) {
    assert_eq!(
        expected.items.len(),
        actual.items.len(),
        "Book item count mismatch"
    );
    for (i, (exp_item, act_item)) in expected.items.iter().zip(actual.items.iter()).enumerate() {
        match (exp_item, act_item) {
            (BookItem::Chapter(exp_ch), BookItem::Chapter(act_ch)) => {
                assert_eq!(
                    exp_ch.name, act_ch.name,
                    "Chapter name mismatch at index {}",
                    i
                );
                assert_diff(&act_ch.content, &exp_ch.content, "\n", 0);
            }
            (BookItem::Separator, BookItem::Separator) => {}
            (BookItem::PartTitle(exp_title), BookItem::PartTitle(act_title)) => {
                assert_diff(&act_title, &exp_title, "\n", 0);
            }
            _ => panic!("Mismatched book item types at index {}", i),
        }
    }
    assert_eq!(expected, actual, "Books are not equal");
}

#[test]
fn full_featured() {
    let ctx = PreprocessorContext::new(
        file!().into(),
        Config::from_str("[book]\n\n[preprocessor.numbering]").unwrap(),
        "html".into(),
    );
    let preprocessor = NumberingPreprocessor::new();

    let book = Book::default();
    let preprocessed = run(&ctx, book.clone(), "");
    assert_eq!(book, preprocessed);

    let book = Book {
        items: vec![
            BookItem::Chapter(Chapter {
                name: "Test".to_string(),
                content:
                    "# Heading\n\n```c\nint main() {\n    printf(\"Hellow, world!\");\n}\n```\n"
                        .to_string(),
                number: Some(vec![1].into_iter().collect()),
                path: None,
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    let preprocessed = run(&ctx, book.clone(), "");
    assert_book_equal(&book, &preprocessed);

    let book = Book {
        items: vec![
            BookItem::Chapter(Chapter {
                name: "Test1".to_string(),
                content: include_str!("./md/test1.input.md").to_string(),
                number: Some(vec![1].into_iter().collect()),
                path: Some("./md/test1.input.md".into()),
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    let preprocessed = preprocessor.run(&ctx, book).unwrap();
    let expected = Book {
        items: vec![
            BookItem::Chapter(Chapter {
                name: "Test1".to_string(),
                content: include_str!("./md/test1.output.md").to_string(),
                number: Some(vec![1].into_iter().collect()),
                path: Some("./md/test1.input.md".into()),
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    assert_book_equal(&expected, &preprocessed);
}

#[test]
fn chapter() {
    let ctx = PreprocessorContext::new(
        file!().into(),
        Config::from_str("[book]\n\n[preprocessor.numbering.code]\nenable = false").unwrap(),
        "html".into(),
    );
    let preprocessor = NumberingPreprocessor::new();

    let book = Book::default();
    let preprocessed = run(&ctx, book.clone(), "");
    assert_eq!(book, preprocessed);

    let book = Book {
        items: vec![
            BookItem::Chapter(Chapter {
                name: "Test1".to_string(),
                content: include_str!("./md/test2.input.md").to_string(),
                number: Some(vec![1].into_iter().collect()),
                path: Some("./md/test2.input.md".into()),
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    let preprocessed = preprocessor.run(&ctx, book).unwrap();
    let expected = Book {
        items: vec![
            BookItem::Chapter(Chapter {
                name: "Test1".to_string(),
                content: include_str!("./md/test2.output.md").to_string(),
                number: Some(vec![1].into_iter().collect()),
                path: Some("./md/test2.input.md".into()),
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    assert_book_equal(&expected, &preprocessed);
}

#[test]
fn katex_order_check() {
    let ctx = PreprocessorContext::new(
        file!().into(),
        Config::from_str(
            r#"
[book]

[preprocessor.katex]
after=["links"]

[preprocessor.numbering.code]
enable = false
"#,
        )
        .unwrap(),
        "html".into(),
    );

    let book = Book {
        items: vec![
            BookItem::Chapter(Chapter {
                name: "Test1".to_string(),
                content: include_str!("./md/test2.input.md").to_string(),
                number: Some(vec![1].into_iter().collect()),
                path: Some("./md/test2.input.md".into()),
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    let preprocessed = run(
        &ctx,
        book,
        "mdbook-numbering: \
Detected KaTeX usage, \
but 'katex' is not included in the 'after' list, \
or equivalently 'numbering' is not included in the 'before' list of the KaTeX preprocessor. \
KaTeX may not work correctly after processing by pulldown-cmark. \
Consider adding 'katex' to the 'after' list in the configuration.
",
    );
    let expected = Book {
        items: vec![
            BookItem::Chapter(Chapter {
                name: "Test1".to_string(),
                content: include_str!("./md/test2.output.md").to_string(),
                number: Some(vec![1].into_iter().collect()),
                path: Some("./md/test2.input.md".into()),
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    assert_book_equal(&expected, &preprocessed);
}

#[test]
fn config_error() {
    let ctx = PreprocessorContext::new(
        file!().into(),
        Config::from_str(
            r#"
[book]

[preprocessor.numbering.code]
enable = "false"
"#,
        )
        .unwrap(),
        "html".into(),
    );

    let book = Book {
        items: vec![
            BookItem::Chapter(Chapter {
                name: "Test1".to_string(),
                content: include_str!("./md/test1.input.md").to_string(),
                number: Some(vec![1].into_iter().collect()),
                path: Some("./md/test1.input.md".into()),
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    let preprocessed = run(
        &ctx,
        book,
        "Using default config for mdbook-numbering due to config error: \
Failed to deserialize `preprocessor.numbering`
",
    );
    let expected = Book {
        items: vec![
            BookItem::Chapter(Chapter {
                name: "Test1".to_string(),
                content: include_str!("./md/test1.output.md").to_string(),
                number: Some(vec![1].into_iter().collect()),
                path: Some("./md/test1.input.md".into()),
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    assert_book_equal(&expected, &preprocessed);
}

#[test]
fn illformed_before() {
    let ctx = PreprocessorContext::new(
        file!().into(),
        Config::from_str(
            r#"
[book]

[preprocessor.katex]
before = "numbering"

[preprocessor.numbering]
"#,
        )
        .unwrap(),
        "html".into(),
    );

    let book = Book {
        items: vec![
            BookItem::Chapter(Chapter {
                name: "Test1".to_string(),
                content: include_str!("./md/test1.input.md").to_string(),
                number: Some(vec![1].into_iter().collect()),
                path: Some("./md/test1.input.md".into()),
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    let preprocessed = run(
        &ctx,
        book,
        "mdbook-numbering: Failed to deserialize `preprocessor.katex.before`
",
    );
    let expected = Book {
        items: vec![
            BookItem::Chapter(Chapter {
                name: "Test1".to_string(),
                content: include_str!("./md/test1.output.md").to_string(),
                number: Some(vec![1].into_iter().collect()),
                path: Some("./md/test1.input.md".into()),
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    assert_book_equal(&expected, &preprocessed);
}

#[test]
fn not_numbered() {
    let ctx = PreprocessorContext::new(
        file!().into(),
        Config::from_str(
            r#"
[book]

[preprocessor.numbering]
"#,
        )
        .unwrap(),
        "html".into(),
    );

    let book = Book {
        items: vec![
            BookItem::Chapter(Chapter {
                name: "Test1".to_string(),
                content: include_str!("./md/test3.input.md").to_string(),
                number: None,
                path: Some("./md/test3.input.md".into()),
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    let preprocessed = run(&ctx, book, "");
    let expected = Book {
        items: vec![
            BookItem::Chapter(Chapter {
                name: "Test1".to_string(),
                content: include_str!("./md/test3.output.md").to_string(),
                number: None,
                path: Some("./md/test3.input.md".into()),
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    assert_book_equal(&expected, &preprocessed);
}

#[test]
fn supports() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--bin",
            "mdbook-numbering",
            "--",
            "supports",
        ])
        .output()
        .expect("Failed to execute process");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"");
    assert_eq!(output.stderr, b"");
}

#[test]
fn empty() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "mdbook-numbering"])
        .output()
        .expect("Failed to execute process");
    assert!(!output.status.success());
    assert_eq!(output.stdout, b"");
    assert_eq!(output.stderr, b"Unable to parse the input\n",);
}
