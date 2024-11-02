use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use super::Schema;


// {
//   "type": "object",
//   "properties": {
//     "id": {
//       "type": "integer"
//     },
//     "name": {
//       "type": "string",
//       "minLength": 1
//     },
//     "tags": {
//       "type": "array",
//       "items": { "type": "string" }
//     }
//   },
//   "patternProperties": {
//     "^x-": { "type": "string" }
//   },
//   "additionalProperties": false,
//   "required": ["id", "name"],
//   "propertyNames": {
//     "pattern": "^[a-zA-Z_][a-zA-Z0-9_]*$"
//   },
//   "minProperties": 2,
//   "maxProperties": 10,
//   "dependentRequired": {
//     "credit_card": ["billing_address"]
//   },
//   "dependentSchemas": {
//     "premium": {
//       "required": ["support_phone"]
//     }
//   },
//   "$defs": {
//     "address": {
//       "type": "object",
//       "properties": {
//         "street": { "type": "string" },
//         "city": { "type": "string" }
//       }
//     }
//   },
//   "title": "User",
//   "description": "A user in the system",
//   "default": {
//     "id": 0,
//     "name": "guest"
//   },
//   "examples": [
//     {
//       "id": 1,
//       "name": "John Doe"
//     }
//   ]
// }


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ObjectSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub properties: HashMap<String, Schema>,
    // XXX: Support the complete spec, eg.
}

