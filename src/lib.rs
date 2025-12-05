#![doc = include_str!("../README.md")]

use std::marker::PhantomData;
use std::sync::LazyLock;

use anyhow::anyhow;
pub use config::{CodeConfig, HeadingConfig, NumberingConfig, NumberingStyle};
use either::Either;
use mdbook_preprocessor::book::{Book, BookItem};
use mdbook_preprocessor::config::Config;
use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CowStr, Event, Options, Parser, Tag};

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
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
        options.insert(Options::ENABLE_MATH);
        options.insert(Options::ENABLE_GFM);
        options.insert(Options::ENABLE_SUPERSCRIPT);
        options.insert(Options::ENABLE_SUBSCRIPT);

        let tokenized = Parser::new_ext(c, options);

        let mut events: Box<dyn Iterator<Item = Event>> = if let Some(a) = &ch.number
            && config.heading.enable
        {
            let name = ch.name.clone();
            let mut stack = a.clone();
            Box::new(tokenized.flat_map(move |event| match event {
                Event::Start(Tag::Heading { level, .. }) => {
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
                    Either::Left(
                        [event, Event::Text(CowStr::from(format!("{stack} ")))].into_iter(),
                    )
                }
                _ => Either::Right([event].into_iter()),
            }))
        } else {
            Box::new(tokenized)
        };

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

    fn validate_config(config: &NumberingConfig, original: &Config, cb: impl FnMut(Error)) {
        let _ = config;
        let _ = original;
        // Add validation logic here if needed in the future.
        let _ = cb;
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
