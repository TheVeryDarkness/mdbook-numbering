use mdbook::book::{BookItem, Chapter, SectionNumber};
use serde::de::IgnoredAny;

use crate::{CodeConfig, HeadingConfig, NumberingConfig, NumberingPreprocessor, NumberingStyle};

#[test]
fn empty() {
    let chapter = Chapter {
        name: "Empty Chapter".to_string(),
        content: "".to_string(),
        number: Some([1].into_iter().collect()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter);

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            after: Vec::new(),
            before: Vec::new(),
            code: CodeConfig { enable: false },
            command: IgnoredAny,
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: IgnoredAny,
        },
        |err| panic!("{err}"),
    );

    assert_eq!(
        item,
        BookItem::Chapter(Chapter {
            name: "Empty Chapter".to_string(),
            content: "".to_string(),
            number: Some(SectionNumber(vec![1])),
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
            after: Vec::new(),
            before: Vec::new(),
            code: CodeConfig { enable: false },
            command: IgnoredAny,
            heading: HeadingConfig {
                enable: false,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: IgnoredAny,
        },
        |err| panic!("{err}"),
    );

    assert_eq!(item, BookItem::Chapter(chapter));
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
            after: Vec::new(),
            before: Vec::new(),
            code: CodeConfig { enable: false },
            command: IgnoredAny,
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: IgnoredAny,
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
            after: Vec::new(),
            before: Vec::new(),
            code: CodeConfig { enable: false },
            command: IgnoredAny,
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: IgnoredAny,
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
            after: Vec::new(),
            before: Vec::new(),
            code: CodeConfig { enable: false },
            command: IgnoredAny,
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: IgnoredAny,
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
fn level_2_consecutive() {
    let chapter = Chapter {
        name: "Chapter 1".to_string(),
        content: "\
## Heading 1

### Heading 2

Some content.

### Heading 3

More content.
"
        .to_string(),
        number: Some(SectionNumber(vec![1, 2])),
        path: Some("chapter_1.md".into()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter);

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            after: Vec::new(),
            before: Vec::new(),
            code: CodeConfig { enable: false },
            command: IgnoredAny,
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: IgnoredAny,
        },
        |err| panic!("{err}"),
    );

    assert_eq!(
        item,
        BookItem::Chapter(Chapter {
            name: "Chapter 1".to_string(),
            content: "\
## 1.2. Heading 1

### 1.2.1. Heading 2

Some content.

### 1.2.2. Heading 3

More content."
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
        number: Some(SectionNumber(vec![1, 2])),
        path: Some("chapter_1.md".into()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter);

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            after: Vec::new(),
            before: Vec::new(),
            code: CodeConfig { enable: false },
            command: IgnoredAny,
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Top,
            },
            optional: IgnoredAny,
        },
        |err| panic!("{err}"),
    );

    assert_eq!(
        item,
        BookItem::Chapter(Chapter {
            name: "Chapter 1".to_string(),
            content: "\
# 1.2. Heading 1

## 1.2.1. Heading 2

Some content.

## 1.2.2. Heading 3

More content."
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
        number: Some(SectionNumber(vec![1])),
        path: Some("chapter_1.md".into()),
        ..Default::default()
    };
    let mut item = BookItem::Chapter(chapter);

    NumberingPreprocessor::render_book_item(
        &mut item,
        &NumberingConfig {
            after: Vec::new(),
            before: Vec::new(),
            code: CodeConfig { enable: false },
            command: IgnoredAny,
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: IgnoredAny,
        },
        |err| panic!("{err}"),
    );
}
