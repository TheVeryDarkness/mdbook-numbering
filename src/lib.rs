#![doc = include_str!("../README.md")]
//!
//! [Configuration]: config::NumberingConfig

use std::marker::PhantomData;
use std::sync::LazyLock;

use anyhow::anyhow;
pub use config::{CodeConfig, HeadingConfig, NumberingConfig, NumberingStyle};
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::{BookItem, Config};
use pulldown_cmark::{CowStr, Event, Tag, TagEnd};

mod config;
#[cfg(test)]
mod tests;

static HIGHLIGHT_JS_LINE_NUMBERS_JS: LazyLock<String> = LazyLock::new(|| {
    format!(
        "<script defer>\nwindow.addEventListener('DOMContentLoaded', function() {{ {} }});\n</script>\n",
        include_str!("highlightjs/line-numbers-min.js"),
    )
});

static HIGHLIGHT_JS_LINE_NUMBERS_CSS: LazyLock<String> = LazyLock::new(|| {
    format!(
        "<style>\n{}\n</style>\n",
        include_str!("highlightjs/line-numbers-min.css"),
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

        // eprintln!("--- Chapter '{}' Processed ---", ch.name);
        // eprintln!("vvv Original Below\n{c:?}\n^^^ Original Above");
        // eprintln!("vvv Processed Below\n{buf:?}\n^^^ Processed Above");
        // eprintln!("-------------------------------");

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

    fn validate_config(config: &NumberingConfig, has_katex: bool, mut cb: impl FnMut(Error)) {
        if has_katex
            && !config
                .after
                .iter()
                .find(|s| s.as_str() == "katex")
                .is_some()
        {
            cb(anyhow!(
                "mdbook-numbering: Detected KaTeX usage, \
                but 'katex' is not included in the 'after' list. \
                Line numbering may not work correctly with KaTeX. \
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

        Self::validate_config(
            &config,
            ctx.config.get_preprocessor("katex").is_some(),
            |err| {
                eprintln!("Warning: {err}");
            },
        );

        book.for_each_mut(|item| {
            Self::render_book_item(item, &config, |err| eprintln!("Warning: {err}"));
        });
        Ok(book)
    }
}
