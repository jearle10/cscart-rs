use serde::{
    de::{Deserializer, Error},
    Deserialize,
};
use serde_json::Value;

pub fn deserialize_string_or_int_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let parse_result = match value {
        Value::String(string) => string.parse::<i64>().ok(),
        Value::Number(number) => number.as_i64(),
        _ => None,
    };

    match parse_result {
        Some(number) => Ok(number as i32),
        None => Err(serde::de::Error::custom("Could not parse")),
    }
}

pub fn deserialize_string_or_int_to_vec_i32<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    // Convert each variant to an array before parsing
    let value_array = match value.clone() {
        Value::Array(x) => x,
        Value::String(y) => vec![Value::String(y)],
        Value::Number(z) => vec![Value::Number(z)],
        _ => vec![],
    };

    let mut numbers: Vec<i32> = vec![];

    for item in value_array {
        let parsed_value = match item {
            Value::String(string) => string.parse::<i64>().ok(),
            Value::Number(number) => number.as_i64(),
            _ => None,
        };

        match parsed_value {
            Some(number) => numbers.push(number as i32),
            None => return Err(Error::custom(format!("Couldn't parse i32, {}", value))),
        }
    }
    Ok(numbers)
}

pub fn deserialize_string_or_float_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let parse_result = match value {
        Value::String(string) => string.parse::<f64>().ok(),
        Value::Number(number) => number.as_f64(),
        _ => None,
    };

    match parse_result {
        Some(number) => Ok(number as f32),
        None => Err(serde::de::Error::custom("Could not parse data")),
    }
}

#[cfg(test)]
mod product_unit_tests {
    use serde::de::IntoDeserializer;

    use super::*;

    #[test]
    fn it_deserializes_string_or_int_to_i32() {
        let data: serde::de::value::StrDeserializer<_> = r#"10"#.into_deserializer();
        let value: Result<i32, serde_json::Error> = deserialize_string_or_int_to_i32(data);

        match value {
            Ok(x) => assert_eq!(x, 10),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn it_deserializes_string_or_float_to_i32() {
        let data: serde::de::value::StrDeserializer<_> = r#"5.5"#.into_deserializer();
        let value: Result<f32, serde_json::Error> = deserialize_string_or_float_to_f32(data);

        match value {
            Ok(x) => assert_eq!(x, 5.5),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn it_deserializes_string_or_int_to_vec_i32() {
        let data = r#"[1, 2, 3]"#;
        let json_value: serde_json::Value = serde_json::from_str(data).unwrap();
        let deserializer = json_value.into_deserializer();

        let value: Result<Vec<i32>, serde_json::Error> =
            deserialize_string_or_int_to_vec_i32(deserializer);

        match value {
            Ok(x) => assert_eq!(x.len(), 3),
            Err(_) => assert!(false),
        }
    }
}
