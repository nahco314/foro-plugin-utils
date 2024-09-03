use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::BTreeMap;

pub trait JsonGetter: Sized {
    fn process(value: &Value) -> Option<Self>;

    fn get_value_opt<const N: usize>(mut value: &Value, name: [&str; N]) -> Option<Self> {
        for part in name {
            value = value.as_object()?.get(part)?;
        }

        Self::process(value)
    }

    fn get_value<const N: usize>(value: &Value, name: [&str; N]) -> Result<Self> {
        Self::get_value_opt(value, name).context("data-json format error")
    }
}

impl JsonGetter for bool {
    fn process(value: &Value) -> Option<Self> {
        value.as_bool()
    }
}

impl JsonGetter for u64 {
    fn process(value: &Value) -> Option<Self> {
        value.as_u64()
    }
}

impl JsonGetter for i64 {
    fn process(value: &Value) -> Option<Self> {
        value.as_i64()
    }
}

impl JsonGetter for f64 {
    fn process(value: &Value) -> Option<Self> {
        value.as_f64()
    }
}

impl JsonGetter for String {
    fn process(value: &Value) -> Option<Self> {
        Some(value.as_str()?.to_string())
    }
}

impl<T: JsonGetter> JsonGetter for Vec<T> {
    fn process(value: &Value) -> Option<Self> {
        value.as_array()?.iter().map(|v| T::process(v)).collect()
    }
}

impl<T: JsonGetter> JsonGetter for BTreeMap<String, T> {
    fn process(value: &Value) -> Option<Self> {
        value
            .as_object()?
            .iter()
            .map(|(k, v)| Some((k.clone(), T::process(v)?)))
            .collect()
    }
}

impl JsonGetter for Value {
    fn process(value: &Value) -> Option<Self> {
        Some(value.clone())
    }
}

impl<T: JsonGetter> JsonGetter for Option<T> {
    fn process(value: &Value) -> Option<Self> {
        if value.is_null() {
            Some(None)
        } else {
            Some(Some(T::process(value)?))
        }
    }
}

pub fn merge(value: &mut Value, other: &Value) {
    match (value, other) {
        (Value::Object(a), Value::Object(b)) => {
            for (key, value) in b {
                merge(a.entry(key.to_string()).or_insert(Value::Null), value);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_json_utils::JsonGetter;
    use std::collections::BTreeMap;

    #[test]
    fn test_get() {
        let obj = serde_json::json!({
            "a": true,
            "b": [
                1,
                2,
                3,
            ],
            "c": {
                "d": {
                    "f": false
                },
                "e": {
                    "f": "aaa"
                }
            },
        });

        assert_eq!(bool::get_value(&obj, ["a"]).unwrap(), true);
        assert_eq!(Vec::<u64>::get_value(&obj, ["b"]).unwrap(), vec![1, 2, 3]);
        assert_eq!(
            String::get_value(&obj, ["c", "e", "f"]).unwrap(),
            "aaa".to_string()
        );
        assert!(BTreeMap::<String, serde_json::Value>::get_value(&obj, ["c"]).is_ok());
    }

    #[test]
    fn test_merge() {
        let mut a = serde_json::json!({
            "a": 1,
            "b": 2,
            "c": {
                "d": 3,
                "e": 4,
            },
        });

        let b = serde_json::json!({
            "b": 5,
            "c": {
                "e": 6,
                "f": 7,
            },
        });

        super::merge(&mut a, &b);

        assert_eq!(
            a,
            serde_json::json!({
                "a": 1,
                "b": 5,
                "c": {
                    "d": 3,
                    "e": 6,
                    "f": 7,
                },
            })
        );
    }
}
