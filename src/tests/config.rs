use crate::{NumberingConfig, NumberingPreprocessor, NumberingStyle};
use mdbook::Config;

#[test]
fn all() {
    let config = toml::toml! {
        numbering-style = "consecutive"
    };

    let config: NumberingConfig = config.try_into().unwrap();

    assert_eq!(
        config,
        NumberingConfig {
            numbering_style: NumberingStyle::Consecutive,
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
            [preprocessor.numbering]
            numbering-style = "consecutive"
        },
        NumberingConfig {
            numbering_style: NumberingStyle::Consecutive,
        },
    );

    test_config(
        toml::toml! {
            [preprocessor.numbering]
        },
        NumberingConfig::default(),
    );
}
