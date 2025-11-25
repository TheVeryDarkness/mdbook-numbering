# mdbook-numbering

A mdBook preprocessor that adds numbering.

- [x] Adds numbers to chapter titles
- [ ] Configurable numbering formats (e.g., "1.", "1.1.", "I.", "A.", etc.)
- [x] Adds numbers to lines in code blocks.

## Configuration

Add the following to your `book.toml`:

```toml
[preprocessor.numbering]
```

Then configure as needed (see [Configuration]), for example:

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

## Note

- Using [`highlightjs-line-numbers.js`](https://github.com/yauhenipakala/highlightjs-line-numbers.js/tree/077386de760c62e43d05963fd16529bcbdb058c0) to add line numbers to code blocks in the rendered HTML. The license of `highlightjs-line-numbers.js` is MIT License, and is copied to [src/highlightjs/LICENSE](./src/highlightjs/LICENSE).

  Some modifications have been made to the original code to fit into this project.
