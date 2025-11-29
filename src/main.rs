#![doc = include_str!("../README.md")]

use std::io;

use mdbook_numbering::NumberingPreprocessor;
use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::{Preprocessor, parse_input};

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

fn handle_preprocessing() -> Result<(), Error> {
    let pre = NumberingPreprocessor::new();
    let (ctx, book) = parse_input(io::stdin())?;

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
