//! Tests for mdbook-numbering preprocessor integration.

use std::io::Write;
use std::process::{Command, Stdio};
use std::str::FromStr;

use mdbook_numbering::NumberingPreprocessor;
use mdbook_preprocessor::book::{Book, BookItem, Chapter};
use mdbook_preprocessor::config::Config;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};

use prettydiff::basic::DiffOp;
use prettydiff::diff_lines;
use prettydiff::owo_colors::OwoColorize as _;

#[track_caller]
fn assert_string_eq(actual: &str, expected: &str) {
    let diff = diff_lines(actual, expected);
    let diff = diff.set_trim_new_lines(false);
    let diff = diff.diff();

    for diff in diff {
        match diff {
            DiffOp::Equal(a) => {
                for i in a.iter() {
                    println!("    {}", i);
                }
            }
            DiffOp::Insert(b) => {
                for i in b.iter() {
                    println!("{}   {}", '+'.bright_green(), i.bright_green());
                }
            }
            DiffOp::Remove(a) => {
                for i in a.iter() {
                    println!("{}   {}", '-'.bright_red(), i.bright_red());
                }
            }
            DiffOp::Replace(a, b) => {
                for i in a.iter() {
                    println!("{}   {}", '-'.bright_red(), i.bright_red());
                }
                for i in b.iter() {
                    println!("{}   {}", '+'.bright_green(), i.bright_green());
                }
            }
        }
    }
    if actual != expected {
        panic!("{actual}")
    }
}

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
    assert_string_eq(&stderr, expected_stderr);
    serde_json::from_slice(&output.stdout).unwrap()
}

#[track_caller]
fn assert_book_equal(actual: &Book, expected: &Book) {
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
                assert_string_eq(&act_ch.content, &exp_ch.content);
            }
            (BookItem::Separator, BookItem::Separator) => {}
            (BookItem::PartTitle(exp_title), BookItem::PartTitle(act_title)) => {
                assert_string_eq(act_title, exp_title);
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
    assert_book_equal(&preprocessed, &book);

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
    assert_book_equal(&preprocessed, &expected);
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
    assert_book_equal(&preprocessed, &expected);

    let book = Book {
        items: vec![
            BookItem::Chapter(Chapter {
                name: "Test1".to_string(),
                content: include_str!("./md/test5.input.md").to_string(),
                number: Some(vec![1, 2].into_iter().collect()),
                path: Some("./md/test5.input.md".into()),
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
                content: include_str!("./md/test5.output.md").to_string(),
                number: Some(vec![1, 2].into_iter().collect()),
                path: Some("./md/test5.input.md".into()),
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    assert_book_equal(&preprocessed, &expected);
}

#[test]
fn alerts_compatibility() {
    let ctx = PreprocessorContext::new(
        file!().into(),
        Config::from_str(
            r#"
[book]

[preprocessor.numbering.code]
enable = false

[preprocessor.numbering.heading]
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
                content: include_str!("./md/test4.input.md").to_string(),
                number: Some(vec![1].into_iter().collect()),
                path: Some("./md/test4.input.md".into()),
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
                content: include_str!("./md/test4.output.md").to_string(),
                number: Some(vec![1].into_iter().collect()),
                path: Some("./md/test4.input.md".into()),
                ..Default::default()
            }),
            BookItem::Separator,
            BookItem::PartTitle("Title 1".to_string()),
        ],
    };
    assert_book_equal(&preprocessed, &expected);
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
    assert_book_equal(&preprocessed, &expected);
}

#[test]
fn illformed_enable() {
    let ctx = PreprocessorContext::new(
        file!().into(),
        Config::from_str(
            r#"
[book]

[preprocessor.numbering]
enable = "true"
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
        "\
Using default config for mdbook-numbering due to config error: \
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
    assert_book_equal(&preprocessed, &expected);
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
    assert_book_equal(&preprocessed, &expected);
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
fn wrong_argument() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--bin",
            "mdbook-numbering",
            "--",
            "support",
        ])
        .output()
        .expect("Failed to execute process");
    assert!(!output.status.success());
    assert_eq!(output.stdout, b"");
    assert_eq!(output.stderr, b"unknown argument: support\n");
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
