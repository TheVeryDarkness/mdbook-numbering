use mdbook_preprocessor::book::{BookItem, Chapter, SectionNumber};
use prettydiff::basic::DiffOp;
use prettydiff::diff_lines;
use prettydiff::owo_colors::OwoColorize;

use crate::{CodeConfig, HeadingConfig, NumberingConfig, NumberingPreprocessor, NumberingStyle};

#[track_caller]
fn panic_on_error(err: mdbook_preprocessor::errors::Error) {
    panic!("{err}");
}

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
        panic!("{actual}");
    }
}

#[track_caller]
fn assert_chapter_eq(left: &Chapter, right: &Chapter) {
    assert_eq!(left.name, right.name);
    assert_string_eq(&left.content, &right.content);
    assert_eq!(left.number, right.number);
    assert_eq!(left.path, right.path);
    assert_eq!(left.sub_items.len(), right.sub_items.len());
    for (l, r) in left.sub_items.iter().zip(right.sub_items.iter()) {
        assert_book_item_eq(l, r);
    }
}

#[track_caller]
fn assert_book_item_eq(left: &BookItem, right: &BookItem) {
    match (left, right) {
        (BookItem::Chapter(lc), BookItem::Chapter(rc)) => assert_chapter_eq(lc, rc),
        _ => panic!("Mismatched BookItem variants"),
    }
}

#[test]
fn empty() {
    let chapter = Chapter {
        name: "Empty Chapter".to_string(),
        content: "".to_string(),
        number: Some([1, 2].into_iter().collect()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter);

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            code: CodeConfig { enable: false },
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            ..Default::default()
        },
        panic_on_error,
    );

    assert_eq!(
        item,
        BookItem::Chapter(Chapter {
            name: "Empty Chapter".to_string(),
            content: "".to_string(),
            number: Some(SectionNumber::new(vec![1, 2])),
            ..Default::default()
        }),
    );
}

#[test]
fn complex_title() {
    let chapter = Chapter {
        name: "Complex Title".to_string(),
        content: "# Complex Title with Inline Code `let x = 10;` and Inline Formulas $E=mc^2$"
            .to_string(),
        number: Some([1, 2].into_iter().collect()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter);

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            code: CodeConfig { enable: false },
            ..Default::default()
        },
        panic_on_error,
    );

    assert_book_item_eq(
        &item,
        &BookItem::Chapter(Chapter {
            name: "Complex Title".to_string(),
            content: "# Complex Title with Inline Code `let x = 10;` and Inline Formulas $E=mc^2$"
                .to_string(),
            number: Some(SectionNumber::new(vec![1, 2])),
            ..Default::default()
        }),
    );
}

#[test]
fn disabled() {
    let chapter = Chapter {
        name: "Empty Chapter".to_string(),
        content: "# abc\n\n$ abc $\n\n$$\n    E = mc^2\n$$".to_string(),
        number: Some([1].into_iter().collect()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter.clone());

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            code: CodeConfig { enable: false },
            heading: HeadingConfig {
                enable: false,
                numbering_style: NumberingStyle::Consecutive,
            },
            ..Default::default()
        },
        panic_on_error,
    );

    assert_book_item_eq(&item, &BookItem::Chapter(chapter));
}

#[test]
fn draft() {
    let chapter = Chapter {
        name: "Chapter 1".to_string(),
        content: "# Heading 1\n\nSome content.".to_string(),
        number: Some([1].into_iter().collect()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter);

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            code: CodeConfig { enable: false },
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            ..Default::default()
        },
        panic_on_error,
    );

    assert_book_item_eq(
        &item,
        &BookItem::Chapter(Chapter {
            name: "Chapter 1".to_string(),
            content: "# Heading 1\n\nSome content.".to_string(),
            number: Some(SectionNumber::new(vec![1])),
            ..Default::default()
        }),
    );
}

#[test]
fn level_1() {
    let chapter = Chapter {
        name: "Chapter 1".to_string(),
        content: "\
# Heading 1

Some content."
            .to_string(),
        number: Some([1].into_iter().collect()),
        path: Some("chapter_1.md".into()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter);

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            code: CodeConfig { enable: false },
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            ..Default::default()
        },
        panic_on_error,
    );

    assert_book_item_eq(
        &item,
        &BookItem::Chapter(Chapter {
            name: "Chapter 1".to_string(),
            content: r#"# <span class="heading numbering">1. </span>Heading 1 { data-numbering=1. }

Some content.

<style>span.heading.numbering{user-select:none;-webkit-user-select:none;cursor:default}
</style>
"#
            .to_string(),
            number: Some(SectionNumber::new(vec![1])),
            path: Some("chapter_1.md".into()),
            ..Default::default()
        }),
    );
}

#[test]
fn level_2() {
    let chapter = Chapter {
        name: "Chapter 1".to_string(),
        content: "\
# Heading 1

## Heading 2

Some content.

## Heading 3

More content.
"
        .to_string(),
        number: Some(SectionNumber::new(vec![1])),
        path: Some("chapter_1.md".into()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter);

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            code: CodeConfig { enable: false },
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            ..Default::default()
        },
        panic_on_error,
    );

    assert_book_item_eq(
        &item,
        &BookItem::Chapter(Chapter {
            name: "Chapter 1".to_string(),
            content: r#"# <span class="heading numbering">1. </span>Heading 1 { data-numbering=1. }

## <span class="heading numbering">1.1. </span>Heading 2 { data-numbering=1.1. }

Some content.

## <span class="heading numbering">1.2. </span>Heading 3 { data-numbering=1.2. }

More content.

<style>span.heading.numbering{user-select:none;-webkit-user-select:none;cursor:default}
</style>
"#
            .to_string(),
            number: Some([1].into_iter().collect()),
            path: Some("chapter_1.md".into()),
            ..Default::default()
        }),
    );
}

#[test]
fn level_2_consecutive() {
    let chapter = Chapter {
        name: "Chapter 1".to_string(),
        content: "## Heading 1

### Heading 2

Some content.

### Heading 3

More content.
"
        .to_string(),
        number: Some(SectionNumber::new(vec![1, 2])),
        path: Some("chapter_1.md".into()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter);

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            code: CodeConfig { enable: false },
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            ..Default::default()
        },
        panic_on_error,
    );

    assert_book_item_eq(
        &item,
        &BookItem::Chapter(Chapter {
            name: "Chapter 1".to_string(),
            content:
                "## <span class=\"heading numbering\">1.2. </span>Heading 1 { data-numbering=1.2. }

### <span class=\"heading numbering\">1.2.1. </span>Heading 2 { data-numbering=1.2.1. }

Some content.

### <span class=\"heading numbering\">1.2.2. </span>Heading 3 { data-numbering=1.2.2. }

More content.

<style>span.heading.numbering{user-select:none;-webkit-user-select:none;cursor:default}
</style>
<style>\
    @media print{\
        h1:not([data-numbering]),\
        h2:not([data-numbering]),\
        h3:not([data-numbering]),\
        h4:not([data-numbering]),\
        h5:not([data-numbering]),\
        h6:not([data-numbering]){\
            display:none\
        }\
    }
</style>
"
                .to_string(),
            number: Some([1, 2].into_iter().collect()),
            path: Some("chapter_1.md".into()),
            ..Default::default()
        }),
    );
}

#[test]
fn level_2_top() {
    let chapter = Chapter {
        name: "Chapter 1".to_string(),
        content: "\
# Heading 1

## Heading 2

Some content.

## Heading 3

More content.
"
        .to_string(),
        number: Some(SectionNumber::new(vec![1, 2])),
        path: Some("chapter_1.md".into()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter);

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            code: CodeConfig { enable: false },
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Top,
            },
            ..Default::default()
        },
        panic_on_error,
    );

    assert_book_item_eq(
        &item,
        &BookItem::Chapter(Chapter {
            name: "Chapter 1".to_string(),
            content:
                r#"# <span class="heading numbering">1.2. </span>Heading 1 { data-numbering=1.2. }

## <span class="heading numbering">1.2.1. </span>Heading 2 { data-numbering=1.2.1. }

Some content.

## <span class="heading numbering">1.2.2. </span>Heading 3 { data-numbering=1.2.2. }

More content.

<style>span.heading.numbering{user-select:none;-webkit-user-select:none;cursor:default}
</style>
"#
                .to_string(),
            number: Some([1, 2].into_iter().collect()),
            path: Some("chapter_1.md".into()),
            ..Default::default()
        }),
    );
}

#[test]
#[should_panic = "\
    Heading level h3 found, \
    but only 1 levels in numbering \"1.\" for chapter \"Chapter 1\".\
"]
fn inconsecutive() {
    let chapter = Chapter {
        name: "Chapter 1".to_string(),
        content: "\
# Heading 1

### Heading 2

Some content."
            .to_string(),
        number: Some(SectionNumber::new(vec![1])),
        path: Some("chapter_1.md".into()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter);

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            code: CodeConfig { enable: false },
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            ..Default::default()
        },
        panic_on_error,
    );
}
