use mdbook_preprocessor::config::Config;
use serde::de::IgnoredAny;

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
            code: CodeConfig { enable: true },
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            ..NumberingConfig::default()
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
            code: CodeConfig { enable: true },
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            ..NumberingConfig::default()
        },
    );

    test_config(
        toml::toml! {
            [preprocessor.numbering]
        },
        NumberingConfig::default(),
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
fn preprocessors_not_a_list() {
    let config = toml::toml! {
        [preprocessor.numbering]
        enable = "true"
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
