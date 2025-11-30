# mdbook-numbering

[![CI](https://github.com/TheVeryDarkness/mdbook-numbering/actions/workflows/ci.yml/badge.svg)](https://github.com/TheVeryDarkness/mdbook-numbering/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/mdbook-numbering.svg)](https://crates.io/crates/mdbook-numbering)
[![Docs.rs](https://docs.rs/mdbook-numbering/badge.svg)](https://docs.rs/mdbook-numbering/)
[![codecov](https://codecov.io/gh/TheVeryDarkness/mdbook-numbering/graph/badge.svg?token=BVVO5ZJN0X)](https://codecov.io/gh/TheVeryDarkness/mdbook-numbering)

<!-- [![License](https://img.shields.io/crates/l/mdbook-numbering.svg)](https://github.com/TheVeryDarkness/mdbook-numbering/blob/master/LICENSE) -->

A mdBook preprocessor that adds numbering.

- [x] Adds numbers prior to chapter titles.
- [ ] Configurable numbering formats (e.g., "1.", "1.1.", "I.", "A.", etc.).
- [x] Adds numbers to lines in code blocks.

## Configuration

Add the following to your `book.toml`:

```toml
[preprocessor.numbering]
```

Then configure as needed (see [`NumberingConfig`](https://docs.rs/mdbook-numbering/latest/mdbook_numbering/struct.NumberingConfig.html) for details), for example:

```toml
[preprocessor.numbering]

[preprocessor.numbering.heading] # Configuration for heading numbering
enable          = true
numbering-style = "consecutive"  # "consecutive" or "top"

[preprocessor.numbering.code]    # Configuration for code block line numbering
enable          = true
```

Or if you don't like the flattened style, which also occupies more lines, you can also write it like this:

```toml
[preprocessor.numbering]
heading = { enable = true, numbering-style = "consecutive" }
code    = { enable = true }
```

### Details

- `heading`: Configuration for heading numbering.
  - `enable`: Whether to enable heading numbering. Default is `true`.
  - `numbering-style`: The numbering style for headings. Can be either `"consecutive"` or `"top"`. Default is `"consecutive"`.
    - `"consecutive"`: Top-level headings should have consistent numbering with their chapter numbers. For example:
      - If a chapter is numbered `2`, its top-level heading should be `# Title` (`<h1>` in HTML).
      - If a chapter is numbered `2.3`, its top-level heading should be `## Title` (`<h2>` in HTML).
    - `"top"`: Top-level headings should always be in the form of `# Title` (`<h1>` in HTML).
- `code`: Configuration for code block line numbering.
  - `enable`: Whether to enable line numbering for code blocks. Default is `true`.

## Updates

### 0.3.1

- Fix misleading configuration examples in `README.md`.

### 0.3.0

- Update to support `mdbook` version 0.5.0 and above.
- Improve warning messages for better clarity.
- No longer skip chapters without chapter numbers when other features are enabled.
- Improve performance when popping heading levels.

### 0.2.1

- Show a warning if this preprocessor is not set to run after `katex` preprocessor when `katex` is used.
- Minor code cleanup and documentation improvements.
- Minify the JavaScript file and the CSS file before including them in the preprocessed markdown.

### 0.2.0

- Added support for adding line numbers to code blocks using `highlightjs-line-numbers.js`.

### 0.1.0

- Initial release with support for adding numbers to chapter titles.

## Compatibility

### mdBook Version

This preprocessor is compatible with `mdbook` version 0.5.0 and above.

| mdbook-numbering version | mdBook Version |
| ------------------------ | -------------- |
| 0.1.0, 0.2.0+            | 0.4.37+        |
| 0.3.0+                   | 0.5.0+         |

### Katex Preprocessor

If you are using the `katex` preprocessor in your `mdbook`, please ensure that the `numbering` preprocessor is set to run **after** the `katex` preprocessor.

This is important because like most preprocessors, `numbering` is using

## Note

- Using [`highlightjs-line-numbers.js`](https://github.com/yauhenipakala/highlightjs-line-numbers.js/tree/077386de760c62e43d05963fd16529bcbdb058c0) to add line numbers to code blocks in the rendered HTML. The license of `highlightjs-line-numbers.js` is MIT License, and is copied to [src/highlightjs/LICENSE](https://github.com/TheVeryDarkness/mdbook-numbering/tree/master/src/highlightjs/LICENSE).

  Some modifications have been made to the original code to fit into this project.
