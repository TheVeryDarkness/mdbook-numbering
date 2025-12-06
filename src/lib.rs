#![doc = include_str!("../README.md")]

use std::iter::once;
use std::marker::PhantomData;

use anyhow::anyhow;
pub use config::{CodeConfig, HeadingConfig, NumberingConfig, NumberingStyle};
use either::Either;
use mdbook_preprocessor::book::{Book, BookItem};
use mdbook_preprocessor::config::Config;
use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CowStr, Event, Parser, Tag};
use pulldown_cmark_to_cmark::{State, cmark_resume_with_options};

mod config;
#[cfg(test)]
mod tests;

static HIGHLIGHT_JS_LINE_NUMBERS_JS: &str = concat!(
    "<script defer>\n\
        window.addEventListener('DOMContentLoaded', function() { ",
    include_str!("highlightjs/line-numbers-min.js"),
    " });\n\
    </script>\n",
);

static HIGHLIGHT_JS_LINE_NUMBERS_CSS: &str = concat!(
    "<style>\n",
    include_str!("highlightjs/line-numbers-min.css"),
    "\n</style>\n",
);

static SECTION_NUMBERS_CSS: &'static str = concat!(
    "<style>",
    include_str!("heading/numbering-min.css"),
    "</style>\n"
);

static SECTION_NUMBERS_PRINT_HIDE_CSS: &'static str = concat!(
    "<style>",
    include_str!("heading/hide-min.css"),
    "</style>\n"
);

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
    fn parser_options() -> pulldown_cmark::Options {
        use pulldown_cmark::Options;
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
        options
    }
    fn render_book_item(item: &mut BookItem, config: &NumberingConfig, mut cb: impl FnMut(Error)) {
        let BookItem::Chapter(ch) = item else { return };
        if ch.is_draft_chapter() {
            return;
        }
        let c = &ch.content;

        let options = Self::parser_options();

        let tokenized = Parser::new_ext(c, options);

        let options = pulldown_cmark_to_cmark::Options::default();

        let mut buf = String::with_capacity(c.len());

        let mut state = State::default();

        if config.heading.enable {
            if let Some(a) = &ch.number {
                let name = ch.name.clone();
                let mut stack = a.clone();
                let events = tokenized.flat_map(|mut event| match event {
                    Event::Start(Tag::Heading {
                        level,
                        ref mut attrs,
                        ..
                    }) => {
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
                        if config.heading.numbering_style == NumberingStyle::Consecutive
                            && level_depth < a.len()
                        {
                            cb(anyhow!(
                                "\
                            Heading level {} found, \
                            but numbering \"{}\" for chapter \"{}\" has more levels. \
                            Consider using `numbering-style = \"top\"` in the config, \
                            if you want the top heading to be level 1.",
                                level,
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
                        attrs.push((
                            CowStr::from("data-numbering"),
                            Some(CowStr::from(format!("{stack}"))),
                        ));
                        Either::Right(
                            [
                                event,
                                Event::InlineHtml(CowStr::from(format!(
                                    "<span class=\"heading numbering\">{stack} </span>"
                                ))),
                            ]
                            .into_iter(),
                        )
                    }
                    _ => Either::Left(once(event)),
                });
                state = cmark_resume_with_options(events, &mut buf, Some(state), options.clone())
                    .unwrap();
                state = cmark_resume_with_options(
                    once(Event::InlineHtml(CowStr::from(SECTION_NUMBERS_CSS))),
                    &mut buf,
                    Some(state),
                    options.clone(),
                )
                .unwrap();

                if config.heading.numbering_style == NumberingStyle::Consecutive && a.len() > 1 {
                    state = cmark_resume_with_options(
                        once(Event::InlineHtml(CowStr::from(
                            SECTION_NUMBERS_PRINT_HIDE_CSS,
                        ))),
                        &mut buf,
                        Some(state),
                        options.clone(),
                    )
                    .unwrap();
                }
            } else {
                let events = tokenized.map(|mut event| match event {
                    Event::Start(Tag::Heading { ref mut attrs, .. }) => {
                        attrs.push((CowStr::from("data-numbering"), None));
                        event
                    }
                    _ => event,
                });
                state = cmark_resume_with_options(events, &mut buf, Some(state), options.clone())
                    .unwrap();
            }
        } else {
            state = cmark_resume_with_options(tokenized, &mut buf, Some(state), options.clone())
                .unwrap();
        };

        if config.code.enable {
            state = cmark_resume_with_options(
                [
                    Event::InlineHtml(CowStr::from(HIGHLIGHT_JS_LINE_NUMBERS_JS.as_ref())),
                    Event::InlineHtml(CowStr::from(HIGHLIGHT_JS_LINE_NUMBERS_CSS.as_ref())),
                ]
                .into_iter(),
                &mut buf,
                Some(state),
                options,
            )
            .unwrap();
        }

        state.finalize(&mut buf).unwrap();

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

        // eprintln!("mdbook-numbering: Using config: {config:#?}");
        // eprintln!("mdbook-numbering: Processing book...");
        // eprintln!("-----------------------------------");
        // eprintln!("Book before processing:\n{book:#?}");
        // eprintln!("-----------------------------------");

        book.for_each_mut(|item| {
            Self::render_book_item(item, &config, |err| eprintln!("mdbook-numbering: {err}"));
        });
        Ok(book)
    }
}
