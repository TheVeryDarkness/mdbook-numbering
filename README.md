# mdbook-numbering

A mdBook preprocessor that adds numbering.

- [x] Adds numbers prior to chapter titles.
- [ ] Configurable numbering formats (e.g., "1.", "1.1.", "I.", "A.", etc.).
- [x] Adds numbers to lines in code blocks.

## Configuration

Add the following to your `book.toml`:

```toml
[preprocessor.numbering]
```

Then configure as needed (see [`NumberingConfig`](https://docs.rs/mdbook-numbering/latest/mdbook_numbering/struct.NumberingConfig.html)), for example:

```toml
[preprocessor.numbering]
# Configuration for heading numbering
heading = {
  enable          = true,
  numbering_style = "consecutive", # "consecutive" or "top"
}
# Configuration for code block line numbering
code = {
  enable          = true,
}
```

## Updates

### 0.2.1

- Show a warning if this preprocessor is not set to run after `katex` preprocessor when `katex` is used.
- Minor code cleanup and documentation improvements.
- Minify the JavaScript file and the CSS file before including them in the preprocessed markdown.

### 0.2.0

- Added support for adding line numbers to code blocks using `highlightjs-line-numbers.js`.

### 0.1.0

- Initial release with support for adding numbers to chapter titles.

## Note

- Using [`highlightjs-line-numbers.js`](https://github.com/yauhenipakala/highlightjs-line-numbers.js/tree/077386de760c62e43d05963fd16529bcbdb058c0) to add line numbers to code blocks in the rendered HTML. The license of `highlightjs-line-numbers.js` is MIT License, and is copied to [src/highlightjs/LICENSE](https://github.com/TheVeryDarkness/mdbook-numbering/tree/master/src/highlightjs/LICENSE).

  Some modifications have been made to the original code to fit into this project.
