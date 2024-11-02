use serde::{Serialize, Deserialize};

// {
//   "type": "integer",
//   "minimum": -100,
//   "maximum": 100,
//   "exclusiveMinimum": -100,
//   "exclusiveMaximum": 100,
//   "multipleOf": 5,
//   "enum": [-50, 0, 50],
//   "default": 0,
//   "title": "Integer Field",
//   "description": "An integer between -100 and 100, multiple of 5",
//   "examples": [-25, 0, 75]
// }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegerSchema {
    minimum: Option<i64>,
    maximum: Option<i64>,
    exclusive_minimum: Option<i64>,
    exclusive_maximum: Option<i64>,
    multiple_of: Option<usize>,
    #[serde(rename = "enum")]
    r#enum: Option<Vec<i64>>,
    title: Option<String>,
    description: Option<String>,
    examples: Option<Vec<i64>>,
    // XXX: Support the complete spec, eg.
}
