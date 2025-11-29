#![doc = include_str!("../README.md")]

use std::marker::PhantomData;
use std::sync::LazyLock;

use anyhow::anyhow;
pub use config::{CodeConfig, HeadingConfig, NumberingConfig, NumberingStyle};
use mdbook_preprocessor::book::{Book, BookItem};
use mdbook_preprocessor::config::Config;
use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CowStr, Event, Parser, Tag, TagEnd};
use serde::de::IgnoredAny;

use crate::config::Preprocessors;

mod config;
#[cfg(test)]
mod tests;

static HIGHLIGHT_JS_LINE_NUMBERS_JS: LazyLock<String> = LazyLock::new(|| {
    format!(
        "<script defer>\n\
            window.addEventListener('DOMContentLoaded', function() {{ {} }});\n\
        </script>\n",
        include_str!("highlightjs/line-numbers-min.js"),
    )
});

static HIGHLIGHT_JS_LINE_NUMBERS_CSS: LazyLock<String> = LazyLock::new(|| {
    format!(
        "<style>\n{}\n</style>\n",
        include_str!("highlightjs/line-numbers-min.css"),
    )
});

/// mdbook preprocessor for adding numbering to headings and code blocks.
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
        let c = &ch.content;
        let tokenized = Parser::new(c);

        let events: Box<dyn Iterator<Item = Event>> = if let Some(a) = &ch.number
            && config.heading.enable
        {
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
                    stack.truncate(level_depth);
                    // while level_depth < stack.len() {
                    //     stack.pop();
                    // }
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

        // eprintln!("--- Chapter '{}' Processed ---", ch.name);
        // eprintln!("vvv Original Below\n{c:?}\n^^^ Original Above");
        // eprintln!("vvv Processed Below\n{buf:?}\n^^^ Processed Above");
        // eprintln!("-------------------------------");

        ch.content = buf;
    }

    fn get_config(config: &Config, mut cb: impl FnMut(&Error)) -> NumberingConfig {
        config.get("preprocessor.numbering").map_or_else(
            |err| {
                cb(&err);
                NumberingConfig::default()
            },
            |cfg| cfg.unwrap_or_default(),
        )
    }

    fn validate_config(config: &NumberingConfig, original: &Config, mut cb: impl FnMut(Error)) {
        if !config.after.katex
            && original.get("preprocessor.katex").map_or_else(
                |err| {
                    // Actually impossible as it's deserialized as `IgnoredAny`.
                    cb(err);
                    false
                },
                |katex: Option<IgnoredAny>| katex.is_some(),
            )
            && original.get("preprocessor.katex.before").map_or_else(
                |err| {
                    cb(err);
                    false
                },
                |before| before.is_none_or(|before: Preprocessors| !before.numbering),
            )
        {
            cb(anyhow!(
                "Detected KaTeX usage, \
                but 'katex' is not included in the 'after' list, \
                or equivalently 'numbering' is not included \
                in the 'before' list of the KaTeX preprocessor. \
                KaTeX may not work correctly after processing by pulldown-cmark. \
                Consider adding 'katex' to the 'after' list in the configuration."
            ));
        }
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

        Self::validate_config(&config, &ctx.config, |err| {
            eprintln!("mdbook-numbering: {err}");
        });

        book.for_each_mut(|item| {
            Self::render_book_item(item, &config, |err| eprintln!("mdbook-numbering: {err}"));
        });
        Ok(book)
    }
}
