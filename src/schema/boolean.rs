use serde::{Serialize, Deserialize};

// {
//   "type": "boolean",
//   "default": false,
//   "enum": [true, false],
//   "title": "User Status",
//   "description": "Indicates if the user account is active",
//   "examples": [true, false]
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooleanSchema {
    #[serde(rename = "enum")]
    r#enum: Option<Vec<bool>>,
    
    title: Option<String>,
    description: Option<String>,
    examples: Option<Vec<f64>>,
    // XXX: Support the complete spec, eg.
}
