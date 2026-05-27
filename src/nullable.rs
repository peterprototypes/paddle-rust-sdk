use serde::Serialize;

/// Three-state type for PATCH request fields.
///
/// The Paddle API uses PATCH semantics where omitting a field means "don't change",
/// sending `null` means "clear the value", and sending a value means "set to this value".
///
/// - `Unchanged` - Field will be omitted from serialization (no change)
/// - `Null` - Field will be serialized as `null` (clear the value)
/// - `Value(T)` - Field will be serialized as the contained value
#[derive(Clone, Debug, PartialEq)]
pub enum Nullable<T> {
    Unchanged,
    Null,
    Value(T),
}

impl<T> Nullable<T> {
    pub fn is_unchanged(&self) -> bool {
        matches!(self, Nullable::Unchanged)
    }
}

impl<T> Default for Nullable<T> {
    fn default() -> Self {
        Nullable::Unchanged
    }
}

impl<T> From<T> for Nullable<T> {
    fn from(value: T) -> Self {
        Nullable::Value(value)
    }
}

impl From<&str> for Nullable<String> {
    fn from(value: &str) -> Self {
        Nullable::Value(value.to_string())
    }
}

macro_rules! nullable_from_string_like {
    ($($t:ty),* $(,)?) => {
        $(
            impl From<&str> for Nullable<$t> {
                fn from(s: &str) -> Self {
                    Nullable::Value(s.into())
                }
            }
            impl From<String> for Nullable<$t> {
                fn from(s: String) -> Self {
                    Nullable::Value(s.into())
                }
            }
        )*
    };
}

use paddle_rust_sdk_types::ids::{AddressID, BusinessID, CustomerID, DiscountID};
nullable_from_string_like!(CustomerID, AddressID, BusinessID, DiscountID);

impl<T: Serialize> Serialize for Nullable<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Nullable::Unchanged => serializer.serialize_none(),
            Nullable::Null => serializer.serialize_none(),
            Nullable::Value(v) => v.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct TestStruct {
        #[serde(skip_serializing_if = "Nullable::is_unchanged")]
        field: Nullable<String>,
    }

    #[derive(Serialize)]
    struct MultiField {
        #[serde(skip_serializing_if = "Nullable::is_unchanged")]
        a: Nullable<String>,
        #[serde(skip_serializing_if = "Nullable::is_unchanged")]
        b: Nullable<i32>,
        #[serde(skip_serializing_if = "Nullable::is_unchanged")]
        c: Nullable<bool>,
    }

    #[test]
    fn unchanged_is_omitted() {
        let s = TestStruct {
            field: Nullable::Unchanged,
        };
        let json = serde_json::to_value(&s).unwrap();
        assert_eq!(json, serde_json::json!({}));
    }

    #[test]
    fn null_serializes_as_null() {
        let s = TestStruct {
            field: Nullable::Null,
        };
        let json = serde_json::to_value(&s).unwrap();
        assert_eq!(json, serde_json::json!({"field": null}));
    }

    #[test]
    fn value_serializes_as_value() {
        let s = TestStruct {
            field: Nullable::Value("hello".to_string()),
        };
        let json = serde_json::to_value(&s).unwrap();
        assert_eq!(json, serde_json::json!({"field": "hello"}));
    }

    #[test]
    fn mixed_fields() {
        let s = MultiField {
            a: Nullable::Unchanged,
            b: Nullable::Null,
            c: Nullable::Value(true),
        };
        let json = serde_json::to_value(&s).unwrap();
        assert_eq!(json, serde_json::json!({"b": null, "c": true}));
    }

    #[test]
    fn default_is_unchanged() {
        let n: Nullable<String> = Nullable::default();
        assert!(n.is_unchanged());
    }

    #[test]
    fn from_value() {
        let n: Nullable<String> = "hello".to_string().into();
        assert_eq!(n, Nullable::Value("hello".to_string()));
    }
}
