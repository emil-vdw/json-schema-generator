use serde::{Serialize, Deserialize};

// {
//   "type": "number",
//   "minimum": 0.0,
//   "maximum": 1.0,
//   "exclusiveMinimum": 0.0,
//   "exclusiveMaximum": 1.0,
//   "multipleOf": 0.1,
//   "enum": [0.1, 0.2, 0.3],
//   "default": 0.5,
//   "title": "Decimal Field",
//   "description": "A decimal number between 0 and 1",
//   "examples": [0.25, 0.5, 0.75]
// }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumberSchema {
    minimum: Option<f64>,
    maximum: Option<f64>,
    exclusive_minimum: Option<f64>,
    exclusive_maximum: Option<f64>,
    multiple_of: Option<f64>,
    #[serde(rename = "enum")]
    r#enum: Option<Vec<f64>>,
    title: Option<String>,
    description: Option<String>,
    examples: Option<Vec<f64>>,
    // XXX: Support the complete spec, eg.
}
