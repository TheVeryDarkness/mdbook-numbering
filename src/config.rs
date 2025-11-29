use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};

/// The numbering style to be used by the `mdbook-numbering` preprocessor.
///
/// Should be placed under the `numbering-style` field
/// in the `[preprocessor.numbering]` section in `book.toml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub enum NumberingStyle {
    /// There should be no more than one top heading (the heading with the highest level)
    /// in the chapter, and it should has the same level as the chapter numbering.
    ///
    /// For example, if the numbering of the chapter is `1.2.3`, the top heading in the chapter
    /// should be level 3 (i.e., `### Chapter 1.2.3`).
    ///
    /// This is the default behavior of `mdbook-numbering`. And it works well with [mdbook-pdf]
    /// in regard to generating the table of contents.
    ///
    /// [mdbook-pdf]: https://github.com/HollowMan6/mdbook-pdf
    Consecutive,
    /// There should be no more than one top heading (the heading with the highest level)
    /// in the chapter, and it should be level 1 (i.e., `# Chapter 1.2.3`),
    /// regardless of the chapter numbering.
    ///
    /// This style is more flexible, but may lead to inconsistent heading levels across chapters.
    /// And using it you may get a flat table of contents when generating PDF with [mdbook-pdf].
    ///
    /// By the way, this is how [the documentation of mdbook] is structured.
    ///
    /// [mdbook-pdf]: https://github.com/HollowMan6/mdbook-pdf
    /// [the documentation of mdbook]: https://github.com/rust-lang/mdBook/tree/master/guide
    Top,
    // Future numbering styles can be added here.
}

impl NumberingStyle {
    /// Create a new `NumberingStyle` with default value.
    pub const fn new() -> Self {
        Self::Consecutive
    }
}

impl Default for NumberingStyle {
    fn default() -> Self {
        Self::new()
    }
}

fn bool_true() -> bool {
    true
}

/// Configuration for heading numbering style.
///
/// Should be placed under the `heading` field
/// in the `[preprocessor.numbering]` section in `book.toml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct HeadingConfig {
    /// Whether to enable heading numbering.
    #[serde(default = "bool_true")]
    pub enable: bool,
    /// Whether to treat warnings as errors.
    #[serde(default)]
    pub numbering_style: NumberingStyle,
    // Future configuration options can be added here.
}

impl HeadingConfig {
    /// Create a new `HeadingConfig` with default values.
    pub const fn new() -> Self {
        Self {
            enable: true,
            numbering_style: NumberingStyle::new(),
        }
    }
}

impl Default for HeadingConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for code block line numbering.
///
/// Should be placed under the `code` field
/// in the `[preprocessor.numbering]` section in `book.toml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct CodeConfig {
    /// Whether to enable code numbering.
    #[serde(default = "bool_true")]
    pub enable: bool,
    // Future configuration options can be added here.
}

impl CodeConfig {
    /// Create a new `CodeConfig` with default values.
    pub const fn new() -> Self {
        Self { enable: true }
    }
}

impl Default for CodeConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Preprocessor list of interests.
///
/// May be placed under `preprocessor.*.before` or `preprocessor.*.after` in `book.toml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Preprocessors {
    /// Whether to include `mdbook-katex` in the list.
    pub katex: bool,
    /// Whether to include `mdbook-numbering` in the list.
    pub numbering: bool,
    // Future preprocessors can be added here.
}

impl Preprocessors {
    /// Create a new `Preprocessors` with default values.
    pub const fn new() -> Self {
        Self {
            katex: false,
            numbering: false,
        }
    }
}

impl Default for Preprocessors {
    fn default() -> Self {
        Self::new()
    }
}

impl Serialize for Preprocessors {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut vec = Vec::new();
        if self.katex {
            vec.push("katex");
        }
        if self.numbering {
            vec.push("numbering");
        }
        vec.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Preprocessors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec: Vec<String> = Vec::deserialize(deserializer)?;
        let mut preprocessors = Preprocessors::default();
        for item in vec {
            match item.as_str() {
                "katex" => preprocessors.katex = true,
                "numbering" => preprocessors.numbering = true,
                _ => {}
            }
        }
        Ok(preprocessors)
    }
}

/// Configuration for the `mdbook-numbering` preprocessor.
///
/// Should be placed under the `[preprocessor.numbering]` section in `book.toml`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct NumberingConfig {
    /// Those preprocessors that `mdbook-numbering` should run after.
    #[serde(default)]
    pub after: Preprocessors,
    /// Those preprocessors that `mdbook-numbering` should run before.
    #[serde(default)]
    pub before: Preprocessors,
    /// Configuration for line numbering in code blocks.
    #[serde(default)]
    pub code: CodeConfig,
    /// Placeholder to ignore unused fields.
    #[serde(default, skip_serializing)]
    pub command: IgnoredAny,
    /// Configuration for heading numbering.
    #[serde(default)]
    pub heading: HeadingConfig,
    /// Placeholder to ignore unused fields.
    #[serde(default, skip_serializing)]
    pub optional: IgnoredAny,
    /// Placeholder to ignore unused fields.
    #[serde(default, skip_serializing)]
    pub renderers: IgnoredAny,
    // Future configuration options can be added here.
}

impl NumberingConfig {
    /// Create a new `NumberingConfig` with default values.
    pub const fn new() -> Self {
        Self {
            after: Preprocessors::new(),
            before: Preprocessors::new(),
            code: CodeConfig::new(),
            command: IgnoredAny,
            heading: HeadingConfig::new(),
            optional: IgnoredAny,
            renderers: IgnoredAny,
        }
    }
}

impl Default for NumberingConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for NumberingConfig {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.heading == other.heading
    }
}
impl Eq for NumberingConfig {}
