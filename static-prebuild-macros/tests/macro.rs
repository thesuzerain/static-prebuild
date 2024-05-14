use serde::{Deserialize, Serialize};
use static_prebuild_macros::include_json;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct TestStruct {
    test_string: String,
    test_number: f32,
}

#[test]
fn deserialize_with_macro() {
    let test_json_deserialized = include_json!("static-prebuild-macros/res/test.json");

    let test_struct = TestStruct {
        test_string: "test".to_string(),
        test_number: 35.5,
    };

    assert_eq!(test_json_deserialized, test_struct);
}
