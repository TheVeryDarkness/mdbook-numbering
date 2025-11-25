use crate::{CodeConfig, HeadingConfig, NumberingConfig, NumberingPreprocessor, NumberingStyle};
use mdbook::book::{BookItem, Chapter, SectionNumber};

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
            command: (),
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: (),
        },
        |err| panic!("{err}"),
    );

    assert_eq!(
        item,
        BookItem::Chapter(Chapter {
            name: "Chapter 1".to_string(),
            content: "# Heading 1\n\nSome content.".to_string(),
            number: Some(SectionNumber(vec![1])),
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
            command: (),
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: (),
        },
        |err| panic!("{err}"),
    );

    assert_eq!(
        item,
        BookItem::Chapter(Chapter {
            name: "Chapter 1".to_string(),
            content: "\
# 1. Heading 1

Some content."
                .to_string(),
            number: Some(SectionNumber(vec![1])),
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
        number: Some(SectionNumber(vec![1])),
        path: Some("chapter_1.md".into()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter);

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            code: CodeConfig { enable: false },
            command: (),
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: (),
        },
        |err| panic!("{err}"),
    );

    assert_eq!(
        item,
        BookItem::Chapter(Chapter {
            name: "Chapter 1".to_string(),
            content: "\
# 1. Heading 1

## 1.1. Heading 2

Some content.

## 1.2. Heading 3

More content."
                .to_string(),
            number: Some([1].into_iter().collect()),
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
        number: Some(SectionNumber(vec![1])),
        path: Some("chapter_1.md".into()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter);

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            code: CodeConfig { enable: false },
            command: (),
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: (),
        },
        |err| panic!("{err}"),
    );

    assert_eq!(
        item,
        BookItem::Chapter(Chapter {
            name: "Chapter 1".to_string(),
            content: "\
# 1. Heading 1

### 1.0.1. Heading 2

Some content."
                .to_string(),
            number: Some([1].into_iter().collect()),
            path: Some("chapter_1.md".into()),
            ..Default::default()
        }),
    );
}
