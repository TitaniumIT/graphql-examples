use std::str::FromStr;

use juniper::{GraphQLScalar, InputValue, ScalarValue};

#[derive(GraphQLScalar, PartialEq, Clone,Debug)]
#[graphql(from_input_with = Self::from_input, transparent)]
pub struct EmailAddressScalar(String);


impl EmailAddressScalar {
    fn from_input<S>(input: &InputValue<S>) -> Result<Self, String>
    where
        S: ScalarValue
    {
       input.as_string_value().
         ok_or_else(|| format!("Expected `String`, found: {input}"))
         .and_then(
            |str| {
                if str.ends_with("@local.home") {
                   Ok( EmailAddressScalar( String::from_str(str).unwrap()))
                } else {
                   Err( format!("email address not ending with {input} @local.home"))
                }
        })
    }

}