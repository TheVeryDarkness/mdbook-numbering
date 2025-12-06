## mdBook

Below are some examples demonstrating the compatibility of the numbering plugin with mdBook, together with other
Common Markdown Extensions.

### Texts and Paragraphs

This section demonstrates that the numbering plugin works correctly with [normal texts and paragraphs](https://rust-lang.github.io/mdBook/format/markdown.html#text-and-paragraphs).

```md
This section demonstrates that the numbering plugin works correctly with [normal texts and paragraphs](https://rust-lang.github.io/mdBook/format/markdown.html#text-and-paragraphs).
```

### Lists

This section demonstrates that the numbering plugin works correctly with [lists](https://rust-lang.github.io/mdBook/format/markdown.html#lists).

- milk
- eggs
- butter

1. carrots
1. celery
1. radishes

```md
- milk
- eggs
- butter

1. carrots
1. celery
1. radishes
```

### Headings

#### Example Heading

```md
#### Example Heading
```

### Links

This section demonstrates that the numbering plugin works correctly with [links](https://rust-lang.github.io/mdBook/format/markdown.html#links).

### Images

This section demonstrates that the numbering plugin works correctly with [images](https://rust-lang.github.io/mdBook/format/markdown.html#images).

![Rust Logo](https://www.rust-lang.org/logos/rust-logo-512x512.png)

```md
![Rust Logo](https://www.rust-lang.org/logos/rust-logo-512x512.png)
```

### Extensions

#### Strikethrough

This section demonstrates that the numbering plugin works ~~badly~~ correctly with [strikethrough] syntax.

An example of ~~strikethrough text~~.

```md
An example of ~~strikethrough text~~.
```

[strikethrough]: https://rust-lang.github.io/mdBook/format/markdown.html#strikethrough

#### Footnotes

This section demonstrates that the numbering plugin works correctly with [footnotes].

This is an example of a footnote[^note].

[^note]:
    This text is the contents of the footnote, which will be rendered
    towards the bottom.

```md
This is an example of a footnote[^note].

[^note]:
    This text is the contents of the footnote, which will be rendered
    towards the bottom.
```

[footnotes]: https://rust-lang.github.io/mdBook/format/markdown.html#footnotes

#### Tables

This section demonstrates that the numbering plugin works correctly with [tables].

| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   |

```md
| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   |
```

[tables]: https://rust-lang.github.io/mdBook/format/markdown.html#tables

#### Task Lists

This section demonstrates that the numbering plugin works correctly with [task lists].

- [x] Task 1
- [ ] Task 2

```md
- [x] Task 1
- [ ] Task 2
```

[task lists]: https://rust-lang.github.io/mdBook/format/markdown.html#task-lists

#### Smart Punctuation

This section demonstrates that the numbering plugin works correctly with [smart punctuation].

"Smart" quotes and dashes — are they rendered correctly?

```md
"Smart" quotes and dashes — are they rendered correctly?
```

| ASCII sequence |           Unicode            |
| :------------: | :--------------------------: |
|      `--`      |              –               |
|     `---`      |              —               |
|     `...`      |              …               |
|      `"`       | “ or ”, depending on context |
|      `'`       | ‘ or ’, depending on context |

[smart punctuation]: https://rust-lang.github.io/mdBook/format/markdown.html#smart-punctuation

#### Heading Attributes

This section demonstrates that the numbering plugin works correctly with [heading attributes].[^heading-attrbutes]

##### Example heading 1 { #first .class1 .class2 }

##### Example heading 2 { data-numbering=custom }

```md
##### Example heading 1 { #first .class1 .class2 }

##### Example heading 2 { data-numbering=custom }
```

[^heading-attrbutes]: `mdbook-numbering` is using this feature to store the numbering data in the `data-numbering` attribute.

[heading attributes]: https://rust-lang.github.io/mdBook/format/markdown.html#heading-attributes

#### Definition Lists

This section demonstrates the compatibility of [definition lists] with the heading numbering plugin.

Term 1
: Definition for term 1.
Term 2
: Definition for term 2.

```md
Term 1
: Definition for term 1.
Term 2
: Definition for term 2.
```

[definition lists]: https://rust-lang.github.io/mdBook/format/markdown.html#definition-lists

#### Admonitions

This section demonstrates that the numbering plugin works correctly within [admonitions][^1][^2].

> [!WARNING]
> This section demonstrates that the numbering plugin works correctly within alert blocks.

```md
> [!WARNING]
> This section demonstrates that the numbering plugin works correctly within alert blocks.
```

And here are other types of admonitions:

> [!NOTE]
> General information or additional context.

> [!TIP]
> A helpful suggestion or best practice.

> [!IMPORTANT]
> Key information that shouldn't be missed.

> [!WARNING]
> Critical information that highlights a potential risk.

> [!CAUTION]
> Information about potential issues that require caution.

```md
> [!NOTE]
> General information or additional context.

> [!TIP]
> A helpful suggestion or best practice.

> [!IMPORTANT]
> Key information that shouldn't be missed.

> [!WARNING]
> Critical information that highlights a potential risk.

> [!CAUTION]
> Information about potential issues that require caution.
```

[admonitions]: https://rust-lang.github.io/mdBook/format/markdown.html#admonitions

[^1]: Also known as [GFM-style alerts].

[^2]: In mdbook 0.4.0+, this is handled by [mdbook-alerts]. And `mdbook-numbering@<=0.2` is compatible with it.

[GFM-style alerts]: https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#alerts
[mdbook-alerts]: https://crates.io/crates/mdbook-alerts
