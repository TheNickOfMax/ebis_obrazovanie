use json::JsonValue;

pub trait FromJson<T> {
    fn from_json_array(value: JsonValue) -> Vec<T>;
}

impl FromJson<String> for Vec<String> {
    fn from_json_array(value: JsonValue) -> Vec<String> {
        value
            .members()
            .map(|s| s.as_str().unwrap_or_default().to_string())
            .collect()
    }
}

impl FromJson<i32> for Vec<i32> {
    fn from_json_array(value: JsonValue) -> Vec<i32> {
        value
            .members()
            .map(|s| s.as_i32().unwrap_or_default())
            .collect()
    }
}

impl FromJson<JsonValue> for Vec<JsonValue> {
    fn from_json_array(value: JsonValue) -> Vec<JsonValue> {
        value.members().map(|j| j.to_owned()).collect()
    }
}

impl FromJson<Vec<String>> for Vec<Vec<String>> {
    fn from_json_array(value: JsonValue) -> Vec<Vec<String>> {
        value
            .members()
            .map(|j| Vec::<String>::from_json_array(j.to_owned()))
            .collect()
    }
}
