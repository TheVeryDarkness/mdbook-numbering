use std::marker::PhantomData;
use std::sync::LazyLock;

use anyhow::anyhow;
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::{BookItem, Config};
use pulldown_cmark::{CowStr, Event, Tag, TagEnd};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// The numbering style to be used by the `mdbook-numbering` preprocessor.
///
/// Should be placed under the `numbering-style` field
/// in the `[preprocessor.numbering]` section in `book.toml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub enum NumberingStyle {
    /// There should be no more than one top heading (the heading with the highest level)
    /// in the chapter, and it should has the same level as the chapter numbering.
    ///
    /// For example, if the numbering of the chapter is `1.2.3`, the top heading in the chapter
    /// should be level 3 (i.e., `### Chapter 1.2.3`).
    ///
    /// This is the default behavior of `mdbook-numbering`. And it works well with [mdbook-pdf]
    /// in regard to generating the table of contents.
    ///
    /// [mdbook-pdf]: https://github.com/HollowMan6/mdbook-pdf
    Consecutive,
    /// There should be no more than one top heading (the heading with the highest level)
    /// in the chapter, and it should be level 1 (i.e., `# Chapter 1.2.3`),
    /// regardless of the chapter numbering.
    ///
    /// This style is more flexible, but may lead to inconsistent heading levels across chapters.
    /// And using it you may get a flat table of contents when generating PDF with [mdbook-pdf].
    ///
    /// By the way, this is how [the documentation of mdbook] is structured.
    ///
    /// [mdbook-pdf]: https://github.com/HollowMan6/mdbook-pdf
    /// [the documentation of mdbook]: https://github.com/rust-lang/mdBook/tree/master/guide
    Top,
    // Future numbering styles can be added here.
}

impl NumberingStyle {
    /// Create a new `NumberingStyle` with default value.
    pub const fn new() -> Self {
        Self::Consecutive
    }
}

impl Default for NumberingStyle {
    fn default() -> Self {
        Self::new()
    }
}

fn bool_true() -> bool {
    true
}

/// Configuration for heading numbering style.
///
/// Should be placed under the `heading` field
/// in the `[preprocessor.numbering]` section in `book.toml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct HeadingConfig {
    /// Whether to enable heading numbering.
    #[serde(default = "bool_true")]
    enable: bool,
    /// Whether to treat warnings as errors.
    #[serde(default)]
    numbering_style: NumberingStyle,
    // Future configuration options can be added here.
}

impl HeadingConfig {
    /// Create a new `HeadingConfig` with default values.
    pub const fn new() -> Self {
        Self {
            enable: true,
            numbering_style: NumberingStyle::new(),
        }
    }
}

impl Default for HeadingConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for code block line numbering.
///
/// Should be placed under the `code` field
/// in the `[preprocessor.numbering]` section in `book.toml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct CodeConfig {
    /// Whether to enable code numbering.
    #[serde(default = "bool_true")]
    enable: bool,
    // Future configuration options can be added here.
}

impl CodeConfig {
    /// Create a new `CodeConfig` with default values.
    pub const fn new() -> Self {
        Self { enable: true }
    }
}

impl Default for CodeConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for the `mdbook-numbering` preprocessor.
///
/// Should be placed under the `[preprocessor.numbering]` section in `book.toml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct NumberingConfig {
    /// Configuration for line numbering in code blocks.
    #[serde(default)]
    code: CodeConfig,
    /// Placeholder to ignore unused fields.
    #[serde(skip)]
    command: (),
    /// Configuration for heading numbering.
    #[serde(default)]
    heading: HeadingConfig,
    /// Placeholder to ignore unused fields.
    #[serde(skip)]
    optional: (),
    // Future configuration options can be added here.
}

impl NumberingConfig {
    /// Create a new `NumberingConfig` with default values.
    pub const fn new() -> Self {
        Self {
            code: CodeConfig::new(),
            command: (),
            heading: HeadingConfig::new(),
            optional: (),
        }
    }
}

impl Default for NumberingConfig {
    fn default() -> Self {
        Self::new()
    }
}

static HIGHLIGHT_JS_LINE_NUMBERS_JS: LazyLock<String> = LazyLock::new(|| {
    format!(
        "<script defer>\
            window.addEventListener('DOMContentLoaded', function() {{ {} }});\
        </script>\n",
        include_str!("highlightjs/line-numbers.js"),
    )
});

static HIGHLIGHT_JS_LINE_NUMBERS_CSS: LazyLock<String> = LazyLock::new(|| {
    format!(
        "<style>\
            {}\
        </style>\n",
        include_str!("highlightjs/line-numbers.css"),
    )
});

pub struct NumberingPreprocessor(PhantomData<()>);

impl NumberingPreprocessor {
    /// Create a new `NumberingPreprocessor`.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl Default for NumberingPreprocessor {
    fn default() -> Self {
        Self::new()
    }
}

impl NumberingPreprocessor {
    fn render_book_item(item: &mut BookItem, config: &NumberingConfig, mut cb: impl FnMut(Error)) {
        let BookItem::Chapter(ch) = item else { return };
        if ch.is_draft_chapter() {
            return;
        }
        let Some(a) = &ch.number else { return };
        let c = &ch.content;
        let tokenized = mdbook::utils::new_cmark_parser(c, false);

        let events: Box<dyn Iterator<Item = Event>> = if config.heading.enable {
            let name = ch.name.clone();
            let mut in_heading = false;
            let mut stack = a.clone();
            Box::new(tokenized.map(move |event| match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    in_heading = true;
                    let level_depth = match config.heading.numbering_style {
                        NumberingStyle::Consecutive => level as usize,
                        NumberingStyle::Top => level as usize + a.len() - 1,
                    };
                    if level_depth > stack.len() + 1 {
                        cb(anyhow!(
                            "\
                            Heading level {} found, \
                            but only {} levels in numbering \"{}\" for chapter \"{}\".",
                            level,
                            stack.len(),
                            stack,
                            name,
                        ));
                    }
                    while level_depth > stack.len() {
                        stack.push(0);
                    }
                    while level_depth < stack.len() {
                        stack.pop();
                    }
                    if level_depth > a.len() {
                        stack[level_depth - 1] += 1;
                    }
                    event
                }
                Event::Text(s) if in_heading => {
                    let new_content = format!("{stack} {s}");
                    Event::Text(CowStr::from(new_content))
                }
                Event::End(TagEnd::Heading(_)) => {
                    in_heading = false;
                    event
                }
                _ => event,
            }))
        } else {
            Box::new(tokenized)
        };

        let mut events: Box<dyn Iterator<Item = Event>> = Box::new(events);

        if config.code.enable {
            events = Box::new(events.chain([
                Event::InlineHtml(CowStr::from(HIGHLIGHT_JS_LINE_NUMBERS_JS.as_ref())),
                Event::InlineHtml(CowStr::from(HIGHLIGHT_JS_LINE_NUMBERS_CSS.as_ref())),
            ]))
        }

        let mut buf = String::with_capacity(c.len());
        pulldown_cmark_to_cmark::cmark(events, &mut buf).expect("cmark parsing failed");

        ch.content = buf;
    }

    fn get_config(config: &Config, mut cb: impl FnMut(&toml::de::Error)) -> NumberingConfig {
        config
            .get("preprocessor.numbering")
            .map_or_else(Default::default, |cfg| {
                cfg.clone()
                    .try_into()
                    .inspect_err(|err| cb(err))
                    .unwrap_or_default()
            })
    }
}

impl Preprocessor for NumberingPreprocessor {
    fn name(&self) -> &str {
        "numbering"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let config: NumberingConfig = Self::get_config(&ctx.config, |err| {
            eprintln!("Using default config for mdbook-numbering due to config error: {err}")
        });
        book.for_each_mut(|item| {
            Self::render_book_item(item, &config, |err| eprintln!("Warning: {err}"));
        });
        Ok(book)
    }
}
