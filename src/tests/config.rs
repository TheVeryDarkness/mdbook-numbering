use crate::{CodeConfig, HeadingConfig, NumberingConfig, NumberingPreprocessor, NumberingStyle};
use mdbook::Config;

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
            command: (),
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: (),
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
            command: (),
            heading: HeadingConfig {
                enable: true,
                numbering_style: NumberingStyle::Consecutive,
            },
            optional: (),
        },
    );

    test_config(
        toml::toml! {
            [preprocessor.numbering]
        },
        NumberingConfig::default(),
    );
}
