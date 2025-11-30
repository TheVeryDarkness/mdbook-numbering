use mdbook_preprocessor::config::Config;
use serde::de::IgnoredAny;

use crate::config::Preprocessors;
use crate::{CodeConfig, HeadingConfig, NumberingConfig, NumberingPreprocessor, NumberingStyle};

#[test]
fn from_str() {
    let config: Config = toml::from_str(
        r#"
[preprocessor.numbering]
"#,
    )
    .unwrap();
    assert_eq!(
        config
            .get::<NumberingConfig>("preprocessor.numbering")
            .unwrap()
            .unwrap(),
        NumberingConfig::new()
    );

    let config: Config = toml::from_str(
        r#"
[preprocessor.numbering]

[preprocessor.numbering.heading] # Configuration for heading numbering
enable          = true
numbering-style = "consecutive"  # "consecutive" or "top"

[preprocessor.numbering.code]    # Configuration for code block line numbering
enable          = true
"#,
    )
    .unwrap();
    assert_eq!(
        config
            .get::<NumberingConfig>("preprocessor.numbering")
            .unwrap()
            .unwrap(),
        NumberingConfig::new()
    );

    let config: Config = toml::from_str(
        r#"
[preprocessor.numbering]
heading = { enable = true, numbering-style = "consecutive" }
code    = { enable = true }
"#,
    )
    .unwrap();
    assert_eq!(
        config
            .get::<NumberingConfig>("preprocessor.numbering")
            .unwrap()
            .unwrap(),
        NumberingConfig::new()
    );
}

#[test]
fn all() {
    let config = toml::toml! {
        heading.numbering-style = "consecutive"
    };

    let config: NumberingConfig = config.try_into().unwrap();

    assert_eq!(
        config,
        NumberingConfig {
            after: Preprocessors::new(),
            before: Preprocessors::new(),
            code: CodeConfig { enable: true },
            command: IgnoredAny,
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: IgnoredAny,
            renderers: IgnoredAny,
        }
    );
}

#[test]
fn full() {
    fn test_config(value: toml::Value, expected: NumberingConfig) {
        let config: Config = value.try_into().unwrap();
        let config = NumberingPreprocessor::get_config(&config, |err| panic!("{err}"));

        assert_eq!(config, expected);
    }

    test_config(
        toml::toml! {
            [preprocessor.numbering.heading]
            numbering-style = "consecutive"
        },
        NumberingConfig {
            after: Preprocessors::new(),
            before: Preprocessors::new(),
            code: CodeConfig { enable: true },
            command: IgnoredAny,
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: IgnoredAny,
            renderers: IgnoredAny,
        },
    );

    test_config(
        toml::toml! {
            [preprocessor.numbering]
        },
        NumberingConfig::default(),
    );

    test_config(
        toml::toml! {
            [preprocessor.numbering]

            [preprocessor.katex]
            after = ["links"]
            before = ["numbering"]
        },
        NumberingConfig::default(),
    );

    test_config(
        toml::toml! {
            [preprocessor.numbering]
            after = ["katex"]

            [preprocessor.katex]
            after = ["links"]
        },
        NumberingConfig {
            after: Preprocessors {
                katex: true,
                numbering: false,
            },
            ..NumberingConfig::default()
        },
    );

    test_config(
        toml::toml! {
            [preprocessor.numbering]
            before = ["katex"]
        },
        NumberingConfig {
            before: Preprocessors {
                katex: true,
                numbering: false,
            },
            ..NumberingConfig::default()
        },
    );

    test_config(
        toml::toml! {
            [preprocessor.numbering]
            after = ["links"]
        },
        NumberingConfig {
            before: Preprocessors {
                katex: false,
                numbering: false,
            },
            ..NumberingConfig::default()
        },
    );
}

#[test]
fn cmp() {
    assert_eq!(
        NumberingConfig::default().heading.numbering_style,
        NumberingStyle::default(),
    );
}

#[test]
fn serialize() {
    assert_eq!(
        Preprocessors::default(),
        Preprocessors {
            katex: false,
            numbering: false,
        },
    );

    assert_eq!(
        toml::ser::to_string(&Preprocessors {
            katex: false,
            numbering: false,
        })
        .as_deref()
        .unwrap(),
        "[]",
    );
    assert_eq!(
        toml::ser::to_string(&Preprocessors {
            katex: true,
            numbering: false,
        })
        .as_deref()
        .unwrap(),
        "[\"katex\"]",
    );
    assert_eq!(
        toml::ser::to_string(&Preprocessors {
            katex: false,
            numbering: true,
        })
        .as_deref()
        .unwrap(),
        "[\"numbering\"]",
    );
    assert_eq!(
        toml::ser::to_string(&Preprocessors {
            katex: true,
            numbering: true,
        })
        .as_deref()
        .unwrap(),
        "[\"katex\", \"numbering\"]",
    );
}

#[test]
fn preprocessors_not_a_list() {
    let config = toml::toml! {
        [preprocessor.numbering]
        after = "katex"
    };

    let config: Config = config.try_into().unwrap();
    let numbering_config = NumberingPreprocessor::get_config(&config, |err| {
        assert_eq!(
            err.to_string(),
            "Failed to deserialize `preprocessor.numbering`"
        )
    });

    assert_eq!(numbering_config, NumberingConfig::default());
}
