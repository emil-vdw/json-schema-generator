pub mod integer;
pub mod number;
pub mod string;
pub mod boolean;
pub mod object;

use std::fmt;

use object::ObjectSchema;
use serde::{Deserialize, Serialize};

use boolean::BooleanSchema;
use integer::IntegerSchema;
use number::NumberSchema;
use string::StringSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JsonType {
    Null,
    Boolean,
    // In Python, for example, this is the `float` type.
    Number,
    Integer,
    String,
    Array,
    Object,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Schema {
    String(StringSchema),
    Boolean(BooleanSchema),
    Number(NumberSchema),
    Integer(IntegerSchema),
    Object(ObjectSchema),
}

impl fmt::Display for JsonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("Serialization should never fail"),
        )
    }
}

