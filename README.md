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

### Configuration Details

- `heading`: Configuration for heading numbering.
  - `enable`: Whether to enable heading numbering. Default is `true`.
  - `numbering-style`: The numbering style for headings. Can be either `"consecutive"` or `"top"`. Default is `"consecutive"`.
    - `"consecutive"`: Top-level headings should have consistent numbering with their chapter numbers. For example:
      - If a chapter is numbered `2`, its top-level heading should be `# Title` (`<h1>` in HTML).
      - If a chapter is numbered `2.3`, its top-level heading should be `## Title` (`<h2>` in HTML).
    - `"top"`: Top-level headings should always be in the form of `# Title` (`<h1>` in HTML).
- `code`: Configuration for code block line numbering.
  - `enable`: Whether to enable line numbering for code blocks. Default is `true`.

## `pulldown-cmark` Features

There are several optional features of `pulldown-cmark` (via [`Options`](https://docs.rs/pulldown-cmark/0.13.0/pulldown_cmark/struct.Options.html)) that can be enabled via flags. Some features conflicting with `mdbook` are disabled currently.

| Feature                                   | Flag                   | Enabled |
| ----------------------------------------- | ---------------------- | ------- |
| `ENABLE_TABLES`                           | `1 << 1`               | ✅      |
| `ENABLE_FOOTNOTES`                        | `1 << 2`               | ✅      |
| `ENABLE_STRIKETHROUGH`                    | `1 << 3`               | ✅      |
| `ENABLE_TASKLISTS`                        | `1 << 4`               | ✅      |
| `ENABLE_SMART_PUNCTUATION`                | `1 << 5`               |
| `ENABLE_HEADING_ATTRIBUTES`               | `1 << 6`               | ✅      |
| `ENABLE_YAML_STYLE_METADATA_BLOCKS`       | `1 << 7`               |
| `ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS` | `1 << 8`               |
| `ENABLE_OLD_FOOTNOTES`                    | `(1 << 9) \| (1 << 2)` |
| `ENABLE_MATH`                             | `1 << 10`              | ✅      |
| `ENABLE_GFM`                              | `1 << 11`              | ✅      |
| `ENABLE_DEFINITION_LIST`                  | `1 << 12`              |
| `ENABLE_SUPERSCRIPT`                      | `1 << 13`              | ✅      |
| `ENABLE_SUBSCRIPT`                        | `1 << 14`              | ✅      |
| `ENABLE_WIKILINKS`                        | `1 << 15`              |

These options must be enabled, otherwise some features of `mdbook` may not work as expected. See [pulldown-cmark-to-cmark/#106](https://github.com/Byron/pulldown-cmark-to-cmark/issues/106) for more details.

For example:

- If `ENABLE_TABLES` is not enabled, tables in markdown files will not be rendered correctly.
- If `ENABLE_MATH` is not enabled, `katex` preprocessor will not work correctly.

## Updates

### 0.4.0

- Support more `mdbook` features by enabling more `pulldown-cmark` options.

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
| 0.3.0+, 0.4.0+           | 0.5.0+         |

## Note

- Using [`highlightjs-line-numbers.js`](https://github.com/yauhenipakala/highlightjs-line-numbers.js/tree/077386de760c62e43d05963fd16529bcbdb058c0) to add line numbers to code blocks in the rendered HTML. The license of `highlightjs-line-numbers.js` is MIT License, and is copied to [src/highlightjs/LICENSE](https://github.com/TheVeryDarkness/mdbook-numbering/tree/master/src/highlightjs/LICENSE).

  Some modifications have been made to the original code to fit into this project.
