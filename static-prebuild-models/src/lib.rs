pub trait TextBuildable {
    fn to_code_text(&self) -> String;
}

impl TextBuildable for String {
    fn to_code_text(&self) -> String {
        format!("\"{}\".to_string()", self)
    }
}

impl TextBuildable for f64 {
    fn to_code_text(&self) -> String {
        format!("{}_f64", self)
    }
}

impl TextBuildable for f32 {
    fn to_code_text(&self) -> String {
        format!("{}_f32", self)
    }
}

impl TextBuildable for i64 {
    fn to_code_text(&self) -> String {
        format!("{}_i64", self)
    }
}

impl TextBuildable for i32 {
    fn to_code_text(&self) -> String {
        format!("{}_i32", self)
    }
}

impl TextBuildable for i16 {
    fn to_code_text(&self) -> String {
        format!("{}_i16", self)
    }
}

impl TextBuildable for i8 {
    fn to_code_text(&self) -> String {
        format!("{}_i8", self)
    }
}

impl TextBuildable for u64 {
    fn to_code_text(&self) -> String {
        format!("{}_u64", self)
    }
}

impl TextBuildable for u32 {
    fn to_code_text(&self) -> String {
        format!("{}_u32", self)
    }
}

impl TextBuildable for u16 {
    fn to_code_text(&self) -> String {
        format!("{}_u16", self)
    }
}

impl TextBuildable for u8 {
    fn to_code_text(&self) -> String {
        format!("{}_u8", self)
    }
}

impl TextBuildable for bool {
    fn to_code_text(&self) -> String {
        format!("{}", self)
    }
}

impl<T> TextBuildable for Option<T>
where
    T: TextBuildable,
{
    fn to_code_text(&self) -> String {
        match self {
            Some(value) => format!("Some({})", value.to_code_text()),
            None => "None".to_string(),
        }
    }
}

impl<T, Err> TextBuildable for Result<T, Err>
where
    T: TextBuildable,
    Err: TextBuildable,
{
    fn to_code_text(&self) -> String {
        match self {
            Ok(value) => format!("Ok({})", value.to_code_text()),
            Err(err) => format!("Err({})", err.to_code_text()),
        }
    }
}

impl<T> TextBuildable for Vec<T>
where
    T: TextBuildable,
{
    fn to_code_text(&self) -> String {
        let mut result = String::new();
        result.push_str("vec![\n");
        for value in self {
            result.push_str(&format!("{},\n", value.to_code_text()));
        }
        result.push(']');
        result
    }
}

impl<K, V> TextBuildable for std::collections::HashMap<K, V>
where
    K: TextBuildable,
    V: TextBuildable,
{
    fn to_code_text(&self) -> String {
        let mut result = String::new();
        result.push_str("let mut hm = std::collections::HashMap::new();\n");
        for (key, value) in self {
            result.push_str(&format!(
                "hm.insert({}, {});\n",
                key.to_code_text(),
                value.to_code_text()
            ));
        }
        result.push_str("hm");
        format!("{{\n{}\n}}", result)
    }
}
