use serde::de::{Deserializer, Error, SeqAccess, Unexpected, Visitor};

pub fn deserialize_string_or_int_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(Deserializei32WithVisitor)
}

pub fn deserialize_string_or_int_to_vec_i32<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeVeci32WithVisitor)
}

pub fn deserialize_string_or_float_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(Deserializef32WithVisitor)
}

struct DeserializeVeci32WithVisitor;
impl<'de> Visitor<'de> for DeserializeVeci32WithVisitor {
    type Value = Vec<i32>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("an integer or a string that can be converted to a integer")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match i32::try_from(v) {
            Ok(v) => Ok(vec![v]),
            Err(_) => Err(E::custom(format!(
                "overflow: Unable to convert i64 value `{v:?}` to i32"
            ))),
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match i32::try_from(v) {
            Ok(v) => Ok(vec![v]),
            Err(_) => Err(E::custom(format!(
                "overflow: Unable to convert u64 value `{v:?}` to i32"
            ))),
        }
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match i32::try_from(v.round() as u64) {
            Ok(v) => Ok(vec![v]),
            Err(_) => Err(E::custom(format!(
                "overflow: Unable to convert f64 value `{v:?}` to i32"
            ))),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if let Ok(n) = v.parse::<f64>() {
            self.visit_f64(n)
        } else if v.is_empty() {
            Ok(vec![])
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values: Vec<i32> = Vec::new();
        while let Ok(Some(el)) = seq.next_element::<String>() {
            if let Ok(n) = el.parse::<i32>() {
                values.push(n)
            } else {
                return Err(Error::invalid_type(Unexpected::Seq, &self));
            }
        }
        Ok(values)
    }

    /// If `null` value; then an empty vec is returned
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(vec![])
    }
}

struct Deserializei32WithVisitor;

impl<'de> Visitor<'de> for Deserializei32WithVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string that could be parsed to an integer")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if let Ok(v) = i32::try_from(v) {
            Ok(v)
        } else {
            Err(E::custom(format!(
                "overflow: Unable to convert u64 value `{v:?}` to i32"
            )))
        }
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if let Ok(v) = i32::try_from(v.round() as i64) {
            Ok(v)
        } else {
            Err(E::custom(format!(
                "overflow: Unable to convert f64 value `{v:?}` to i32"
            )))
        }
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if let Ok(v) = i32::try_from(v) {
            Ok(v)
        } else {
            Err(E::custom(format!(
                "overflow: Unable to convert i64 value `{v:?}` to i32"
            )))
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if let Ok(n) = v.parse::<i32>() {
            Ok(n)
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }
}

struct Deserializef32WithVisitor;

impl<'de> Visitor<'de> for Deserializef32WithVisitor {
    type Value = f32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string that could be parsed to an integer")
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v < f32::MIN as f64 || v > f32::MAX as f64 {
            Err(E::custom(format!(
                "overflow: Unable to convert f64 value `{v:?}` to f32"
            )))
        } else {
            Ok(v as f32)
        }
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if let Ok(v) = i32::try_from(v) {
            Ok(v as f32)
        } else {
            Err(E::custom(format!(
                "overflow: Unable to convert i64 value `{v:?}` to f32"
            )))
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if let Ok(v) = i32::try_from(v) {
            Ok(v as f32)
        } else {
            Err(E::custom(format!(
                "overflow: Unable to convert u64 value `{v:?}` to f32"
            )))
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if let Ok(n) = v.parse::<f32>() {
            Ok(n)
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
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
    fn it_deserializes_string_or_float_to_f32() {
        let data: serde::de::value::StrDeserializer<_> = r#"5.5"#.into_deserializer();
        let value: Result<f32, serde_json::Error> = deserialize_string_or_float_to_f32(data);

        match value {
            Ok(x) => assert_eq!(x, 5.5),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn it_deserializes_string_or_int_to_vec_i32() {
        let data = serde_json::json!(["1", "2", "3"]);
        let deserializer = data.into_deserializer();

        let value: Result<Vec<i32>, serde_json::Error> =
            deserialize_string_or_int_to_vec_i32(deserializer);

        match value {
            Ok(x) => assert_eq!(x.len(), 3),
            Err(_) => assert!(false),
        }
    }
}
