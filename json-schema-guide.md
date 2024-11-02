# JSON Schema Structure Guide

## Basic Schema Structure
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Product",
  "description": "A product in the catalog",
  "type": "object",
  "properties": {
    "id": {
      "type": "integer",
      "description": "Product identifier"
    }
  }
}
```

## Core Data Types
- `string`
- `number`
- `integer`
- `boolean`
- `array`
- `object`
- `null`

## String Formats and Constraints
```json
{
  "type": "string",
  "minLength": 2,
  "maxLength": 100,
  "pattern": "^[A-Za-z]+$",
  "format": "email",
  "enum": ["draft", "published", "archived"]
}
```

Common formats:
- `date-time`
- `date`
- `time`
- `email`
- `ipv4`
- `ipv6`
- `uri`
- `uuid`

## Number Constraints
```json
{
  "type": "number",
  "minimum": 0,
  "maximum": 100,
  "exclusiveMinimum": 0,
  "exclusiveMaximum": 100,
  "multipleOf": 5
}
```

## Arrays
```json
{
  "type": "array",
  "items": {
    "type": "string"
  },
  "minItems": 1,
  "maxItems": 10,
  "uniqueItems": true
}
```

### Tuple Validation
```json
{
  "type": "array",
  "prefixItems": [
    { "type": "string" },
    { "type": "number" }
  ],
  "minItems": 2,
  "maxItems": 2
}
```

## Objects
```json
{
  "type": "object",
  "properties": {
    "name": { "type": "string" },
    "age": { "type": "integer" }
  },
  "required": ["name"],
  "additionalProperties": false,
  "minProperties": 1,
  "maxProperties": 10
}
```

## Multiple Types (oneOf, anyOf, allOf)

### oneOf (exactly one must match)
```json
{
  "oneOf": [
    { "type": "string" },
    { "type": "number" }
  ]
}
```

### anyOf (at least one must match)
```json
{
  "anyOf": [
    { "type": "string", "maxLength": 5 },
    { "type": "string", "minLength": 10 }
  ]
}
```

### allOf (all must match)
```json
{
  "allOf": [
    { "type": "object", "properties": { "name": { "type": "string" } } },
    { "type": "object", "properties": { "age": { "type": "integer" } } }
  ]
}
```

## Conditional Validation
```json
{
  "type": "object",
  "properties": {
    "type": { "enum": ["user", "admin"] },
    "permissions": { "type": "array" }
  },
  "if": {
    "properties": { "type": { "const": "admin" } }
  },
  "then": {
    "properties": {
      "permissions": { "minItems": 1 }
    },
    "required": ["permissions"]
  }
}
```

## References and Definitions
```json
{
  "$defs": {
    "address": {
      "type": "object",
      "properties": {
        "street": { "type": "string" },
        "city": { "type": "string" }
      }
    }
  },
  "type": "object",
  "properties": {
    "billing_address": { "$ref": "#/$defs/address" },
    "shipping_address": { "$ref": "#/$defs/address" }
  }
}
```

## Pattern Properties
```json
{
  "type": "object",
  "patternProperties": {
    "^S_": { "type": "string" },
    "^I_": { "type": "integer" }
  },
  "additionalProperties": false
}
```

## Dependencies
```json
{
  "type": "object",
  "properties": {
    "credit_card": { "type": "string" },
    "billing_address": { "type": "string" }
  },
  "required": ["credit_card"],
  "dependentRequired": {
    "credit_card": ["billing_address"]
  }
}
```

## Advanced Features

### Content Encoding and Media Type
```json
{
  "type": "string",
  "contentMediaType": "image/png",
  "contentEncoding": "base64"
}
```

### Custom Vocabulary and Meta-Schemas
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$vocabulary": {
    "https://json-schema.org/draft/2020-12/vocab/core": true,
    "https://json-schema.org/draft/2020-12/vocab/applicator": true,
    "https://json-schema.org/draft/2020-12/vocab/validation": true
  }
}
```

### Default Values
```json
{
  "type": "object",
  "properties": {
    "status": {
      "type": "string",
      "default": "pending"
    }
  }
}
```

### Constant Values
```json
{
  "properties": {
    "version": {
      "const": "1.0.0"
    }
  }
}
```

### Comments and Descriptions
```json
{
  "title": "Person",
  "description": "A person object",
  "$comment": "Internal comment for schema maintainers",
  "type": "object"
}
```

### Format Assertions
```json
{
  "type": "string",
  "format": "date-time",
  "formatMaximum": "2024-12-31T23:59:59Z"
}
```
