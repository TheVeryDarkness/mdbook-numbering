//! This is a demonstration of an mdBook preprocessor which parses markdown
//! and removes any instances of emphasis.

use mdbook::BookItem;
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use pulldown_cmark::{CowStr, Event, Tag, TagEnd::Heading};
use std::io;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            // Supports all renderers.
            return;
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    if let Err(e) = handle_preprocessing() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

struct NumberingPreprocessor;

impl Preprocessor for NumberingPreprocessor {
    fn name(&self) -> &str {
        "numbering"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            let BookItem::Chapter(ch) = item else { return };
            if ch.is_draft_chapter() {
                return;
            }
            let Some(a) = &ch.number else { return };
            let c = &ch.content;
            let tokenized = mdbook::utils::new_cmark_parser(c, false);

            let mut in_heading = false;

            let mut stack = a.clone();

            let events = tokenized.map(|event| match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    in_heading = true;
                    if level as usize > stack.len() {
                        eprintln!("Warning: Heading level {} found, but only {} levels in numbering {:?} for chapter '{}'.", level, stack.len(), a, ch.name);
                    }
                    while level as usize >= stack.len() {
                        stack.push(0);
                    }
                    while (level as usize) < stack.len() {
                        stack.pop();
                    }
                    if stack.len() > a.len() {
                        stack[level as usize - 1] += 1;
                    }
                    event
                }
                Event::Text(s) if in_heading => {
                    let new_content = format!("{stack} {s}");
                    Event::Text(CowStr::from(new_content))
                }
                Event::End(Heading(_)) => {
                    in_heading = false;
                    event
                }
                _ => event,
            });

            let mut buf = String::with_capacity(c.len() + a.to_string().len());
            pulldown_cmark_to_cmark::cmark(events, &mut buf).expect("cmark parsing failed");

            ch.content = buf;
        });
        Ok(book)
    }
}

pub fn handle_preprocessing() -> Result<(), Error> {
    let pre = NumberingPreprocessor;
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
