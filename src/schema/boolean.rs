use serde::{Serialize, Deserialize};

// {
//   "type": "boolean",
//   "default": false,
//   "enum": [true, false],
//   "title": "User Status",
//   "description": "Indicates if the user account is active",
//   "examples": [true, false]
// }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BooleanSchema {
    title: Option<String>,
    description: Option<String>,
    examples: Option<Vec<f64>>,
    // XXX: Support the complete spec, eg.
}
