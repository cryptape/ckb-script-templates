// Include your tests here
// See https://github.com/xxuejie/ckb-native-build-sample/blob/main/tests/src/tests.rs for examples

use super::Loader;

#[test]
fn test_binary_exists() {
    Loader::default().load_binary("{{project-name}}");
}
