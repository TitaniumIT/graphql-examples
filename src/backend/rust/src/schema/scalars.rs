use core::fmt;

use juniper::{graphql_scalar, InputValue, ScalarValue, Value};
use serde::{de, Deserialize, Deserializer, Serialize};


#[derive(Clone, Debug, PartialEq, ScalarValue, Serialize)]
#[serde(untagged)]
pub enum DefaultScalarValue {
    #[value(as_float, as_int)]
    Int(i32),
    #[value(as_float)]
    Float(f64),
    #[value(as_str, as_string, into_string)]
    String(String),
    #[value(as_bool)]
    Boolean(bool),
}

impl<'de> Deserialize<'de> for DefaultScalarValue {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = DefaultScalarValue;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a valid input value")
            }

            fn visit_bool<E: de::Error>(self, b: bool) -> Result<Self::Value, E> {
                Ok(DefaultScalarValue::Boolean(b))
            }

            fn visit_i32<E: de::Error>(self, n: i32) -> Result<Self::Value, E> {
                Ok(DefaultScalarValue::Int(n))
            }

            fn visit_u32<E: de::Error>(self, n: u32) -> Result<Self::Value, E> {
                if n <= i32::MAX as u32 {
                    self.visit_i32(n.try_into().unwrap())
                } else {
                    self.visit_u64(n.into())
                }
            }

            fn visit_u64<E: de::Error>(self, n: u64) -> Result<Self::Value, E> {
                if n <= i64::MAX as u64 {
                    self.visit_i64(n.try_into().unwrap())
                } else {
                    // Browser's `JSON.stringify()` serialize all numbers
                    // having no fractional part as integers (no decimal
                    // point), so we must parse large integers as floating
                    // point, otherwise we would error on transferring large
                    // floating point numbers.
                    Ok(DefaultScalarValue::Float(n as f64))
                }
            }

            fn visit_f64<E: de::Error>(self, f: f64) -> Result<Self::Value, E> {
                Ok(DefaultScalarValue::Float(f))
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                self.visit_string(s.into())
            }

            fn visit_string<E: de::Error>(self, s: String) -> Result<Self::Value, E> {
                Ok(DefaultScalarValue::String(s))
            }
        }

        de.deserialize_any(Visitor)
    }
}

#[graphql_scalar(with= email_address_scalar,parse_token(String) , scalar=DefaultScalarValue)]
pub type EmailAddressScalar = shared_types::EmailAddress;

mod email_address_scalar {
    use super::*;

    pub(super) fn to_output<S: ScalarValue>(v: &EmailAddressScalar) -> Value<S> {
        Value::scalar(v.to_string())
    }

    pub(super) fn from_input<S: ScalarValue>(input: &InputValue<S>) -> Result<EmailAddressScalar, String> {
        input
            .as_string_value()
            .ok_or_else(|| format!("Expected `String`, found: {input}"))
            .and_then(|str| EmailAddressScalar::new(str))
    }
}
