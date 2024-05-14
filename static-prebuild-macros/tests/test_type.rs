use static_prebuild_macros::TextBuildable;
use static_prebuild_models::TextBuildable;

#[test]
fn test_numbers() {
    #[derive(TextBuildable)]
    pub struct TestType {
        pub integer_32: i32,
        pub integer_64: i64,
        pub integer_16: i16,
        pub integer_8: i8,
        pub unsigned_integer_32: u32,
        pub unsigned_integer_64: u64,
        pub unsigned_integer_16: u16,
        pub unsigned_integer_8: u8,
        pub float_32: f32,
        pub float_64: f64,
        pub boolean: bool,
    }

    let text_type = TestType {
        integer_32: 59942211,
        integer_64: 618943,
        integer_16: 123,
        integer_8: 12,
        unsigned_integer_32: 68942211,
        unsigned_integer_64: 718943,
        unsigned_integer_16: 223,
        unsigned_integer_8: 32,
        float_32: 35.5,
        float_64: 38.5,
        boolean: true,
    };

    assert_eq!(
        without_whitespace(&text_type.to_code_text()),
        without_whitespace(
            r#"
TestType {
    integer_32: 59942211_i32,
    integer_64: 618943_i64,
    integer_16: 123_i16,
    integer_8: 12_i8,
    unsigned_integer_32: 68942211_u32,
    unsigned_integer_64: 718943_u64,
    unsigned_integer_16: 223_u16,
    unsigned_integer_8: 32_u8,
    float_32: 35.5_f32,
    float_64: 38.5_f64,
    boolean: true,
}"#
        )
    );
}

#[test]
fn test_string() {
    #[derive(TextBuildable)]
    pub struct TestType {
        pub s: String,
    }

    let text_type = TestType {
        s: "test".to_string(),
    };

    assert_eq!(
        without_whitespace(&text_type.to_code_text()),
        without_whitespace(
            r#"
TestType {
    s: "test".to_string(),
}"#
        )
    );
}

#[test]
fn test_option_result() {
    #[derive(TextBuildable)]
    pub struct TestType {
        pub string_option_some: Option<String>,
        pub string_option_none: Option<String>,
        pub result_ok: Result<i32, String>,
        pub result_err: Result<i32, String>,
    }

    let text_type = TestType {
        string_option_some: Some("optional_test".to_string()),
        string_option_none: None,
        result_ok: Ok(123),
        result_err: Err("error".to_string()),
    };

    assert_eq!(
        without_whitespace(&text_type.to_code_text()),
        without_whitespace(
            r#"
TestType {
    string_option_some: Some("optional_test".to_string()),
    string_option_none: None,
    result_ok: Ok(123_i32),
    result_err: Err("error".to_string()),
}"#
        )
    );
}

#[test]
fn test_vec() {
    #[derive(TextBuildable)]
    pub struct TestType {
        pub vec: Vec<i32>,
        pub vec_string: Vec<String>,
    }

    let text_type = TestType {
        vec: vec![1, 2, 3],
        vec_string: vec!["one".to_string(), "two".to_string(), "three".to_string()],
    };

    assert_eq!(
        without_whitespace(&text_type.to_code_text()),
        without_whitespace(
            r#"
TestType {
    vec: vec![1_i32, 2_i32, 3_i32,],
    vec_string: vec!["one".to_string(), "two".to_string(), "three".to_string(),],
}"#
        )
    );
}

#[test]
fn test_hashmap() {
    #[derive(TextBuildable)]
    pub struct TestType {
        pub hm: std::collections::HashMap<String, i32>,
    }

    let text_type = TestType {
        hm: {
            let mut hm = std::collections::HashMap::new();
            // One entry to avoid ordering issues
            hm.insert("one".to_string(), 1);
            hm
        },
    };

    assert_eq!(
        without_whitespace(&text_type.to_code_text()),
        without_whitespace(
            r#"
TestType {
    hm: {
        let mut hm = std::collections::HashMap::new();
        hm.insert("one".to_string(), 1_i32);
        hm
    },
}"#
        )
    );
}

#[test]
fn test_struct() {
    #[derive(TextBuildable)]
    pub struct InnerTestType {
        pub s: String,
        pub integer_32: i32,
    }

    #[derive(TextBuildable)]
    pub struct TestType {
        pub inner: InnerTestType,
    }

    let text_type = TestType {
        inner: InnerTestType {
            s: "inner_test".to_string(),
            integer_32: 123,
        },
    };

    assert_eq!(
        without_whitespace(&text_type.to_code_text()),
        without_whitespace(
            r#"
TestType {
    inner: InnerTestType {
        s: "inner_test".to_string(),
        integer_32: 123_i32,
    },
}"#
        )
    );
}

#[test]
fn test_option_struct() {
    #[derive(TextBuildable)]
    pub struct InnerTestType {
        pub s: String,
        pub integer_32: i32,
    }

    #[derive(TextBuildable)]
    pub struct TestType {
        pub inner_option_some: Option<InnerTestType>,
        pub inner_option_none: Option<InnerTestType>,
    }

    let text_type = TestType {
        inner_option_some: Some(InnerTestType {
            s: "inner_test_some".to_string(),
            integer_32: 124,
        }),
        inner_option_none: None,
    };

    assert_eq!(
        without_whitespace(&text_type.to_code_text()),
        without_whitespace(
            r#"
TestType {
    inner_option_some: Some(InnerTestType {
        s: "inner_test_some".to_string(),
        integer_32: 124_i32,
    }),
    inner_option_none: None,
}"#
        )
    );
}

fn without_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
