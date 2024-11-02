use serde::{Serialize, Deserialize};

// {
//   "type": "string",
//   "minLength": 1,
//   "maxLength": 100,
//   "pattern": "^[A-Za-z0-9]+$",
//   "format": "email",
//   "contentEncoding": "base64",
//   "contentMediaType": "image/png",
//   "enum": ["value1", "value2"],
//   "const": "specificValue",
//   "default": "defaultValue",
//   "formatMinimum": "2020-01-01",
//   "formatMaximum": "2025-12-31",
//   "formatExclusiveMinimum": true,
//   "formatExclusiveMaximum": false,
//   "title": "String Field",
//   "description": "A string field with various constraints",
//   "examples": ["example1", "example2"]
// }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StringSchema {
    #[serde(rename = "minLenght")]
    min_length: Option<usize>,
    
    #[serde(rename = "maxLength")]
    max_length: Option<usize>,

    pattern: Option<String>,
    
    #[serde(rename = "enum")]
    r#enum: Option<Vec<String>>,
    
    title: Option<String>,
    description: Option<String>,
    examples: Option<Vec<String>>,
    // XXX: Support the complete spec, eg.
    // format: Option<StringFormat>,
    // const: Option<String>,
}
