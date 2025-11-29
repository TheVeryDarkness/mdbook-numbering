use mdbook_preprocessor::Preprocessor;

use crate::NumberingPreprocessor;

#[test]
fn name() {
    assert_eq!(NumberingPreprocessor::new().name(), "numbering");
    assert_eq!(NumberingPreprocessor::default().name(), "numbering");
}
