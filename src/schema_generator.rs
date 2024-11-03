use std::collections::HashMap;

use serde_json::Value;

use crate::schema::{
    boolean::BooleanSchema, integer::IntegerSchema, number::NumberSchema, object::ObjectSchema,
    string::StringSchema, Schema,
};

#[derive(Debug, Clone)]
enum NodeRelation {
    ObjectAttribute(String),
    ArrayElement,
}

#[derive(Debug)]
struct SchemaNode<'a> {
    // We link the node to it's parent by index in the frontier to
    // avoid dangling references when the frontier gets re-allocated
    // for instance.
    parent_index: Option<usize>,
    relation: Option<NodeRelation>,
    content: Option<PartialContent>,
    source: &'a Value,
}

#[derive(Debug, Clone)]
enum PartialContent {
    Object(HashMap<String, Schema>),
    Array(Vec<Schema>),
}

#[derive(Debug)]
pub struct SchemaGenerator<'a> {
    schema: Option<Schema>,
    frontier: Vec<SchemaNode<'a>>,
}

impl SchemaNode<'_> {
    pub fn to_schema(&mut self) -> Schema {
        let content = self.content.take().unwrap();

        match content {
            PartialContent::Object(object_properties) => {
                Schema::Object(ObjectSchema::from(object_properties))
            }

            PartialContent::Array(_) => {
                todo!("Implement array schema")
            }

            _ => panic!("Schema cannot be created from empty node"),
        }
    }

    fn add_attribute(&mut self, key: &str, value: Schema) {
        if self.content.is_none() {
            let mut object_attributes = HashMap::new();
            object_attributes.insert(key.to_owned(), value);
            self.content = Some(PartialContent::Object(object_attributes));
        } else if let Some(PartialContent::Object(ref mut object_attributes)) = self.content {
            object_attributes.insert(key.to_owned(), value);
        } else {
            panic!("Attempt to add attribute to non object node");
        }
    }

    fn add_element(&mut self, value: Schema) {
        if let Some(PartialContent::Array(ref mut array_elements)) = self.content {
            array_elements.push(value);
        } else {
            panic!("Attempt to add element to non array node");
        }
    }

    pub fn is_array_node(&self) -> bool {
        matches!(self.content, Some(PartialContent::Array(_)))
    }

    pub fn is_object_node(&self) -> bool {
        matches!(self.content, Some(PartialContent::Object(_)))
    }
}

impl<'a> SchemaGenerator<'a> {
    pub fn new() -> Self {
        Self {
            schema: None,
            frontier: Vec::new(),
        }
    }

    pub fn expand_schema(&mut self, object: &'a Value) {
        if self.schema.is_none() {
            // We have no existing Schema.
            self.schema = Some(self.derive_schema(object));
        }
    }

    fn merge_schemas(schema: &Schema, other_schema: &Schema) -> Schema {
        todo!()
    }

    pub fn derive_schema(&mut self, object: &'a Value) -> Schema {
        if Self::is_basic_value(object) {
            return Self::derive_basic_schema(object);
        }

        self.frontier.push(SchemaNode {
            parent_index: None,
            content: None,
            relation: None,
            source: object,
        });

        while let Some(mut current_node) = self.frontier.pop() {
            if current_node.content.is_some() {
                // We've processed the node before so we are moving back up the tree.
                // Now we build the actual schema.
                let schema = current_node.to_schema();

                match (current_node.parent_index, &current_node.relation) {
                    (None, _) => {
                        return schema;
                    }
                    (Some(parent_index), Some(NodeRelation::ObjectAttribute(node_key))) => {
                        let parent = self.frontier.get_mut(parent_index).expect(
                            "Parent should always live longer on the frontier than the parent",
                        );
                        parent.add_attribute(node_key, schema);
                    }
                    (Some(parent_index), Some(NodeRelation::ArrayElement)) => {
                        let parent = self.frontier.get_mut(parent_index).expect(
                            "Parent should always live longer on the frontier than the parent",
                        );
                        parent.add_element(schema);
                    }
                    _ => panic!("Invalid parent relation state"),
                }

                continue;
            }

            match current_node.source {
                Value::Object(source_attributes) => {
                    let mut child_nodes: Vec<SchemaNode> = Vec::new();

                    current_node.content = Some(PartialContent::Object(HashMap::new()));

                    self.frontier.push(current_node);
                    let parent_index = Some(self.frontier.len() - 1);

                    for (attribute, value) in source_attributes {
                        if Self::is_basic_value(value) {
                            // For basic values, directly add to parent's properties
                            if let Some(schema_node) = self.frontier.last_mut() {
                                schema_node
                                    .add_attribute(attribute, Self::derive_basic_schema(value));
                            }
                        } else {
                            child_nodes.push(SchemaNode {
                                parent_index,
                                content: None,
                                source: value,
                                relation: Some(NodeRelation::ObjectAttribute(attribute.to_owned())),
                            });
                        }
                    }

                    if !child_nodes.is_empty() {
                        // The schema for the current node cannot be created yet because
                        // we need to process children.
                        self.frontier.extend(child_nodes.into_iter());
                    }
                }

                Value::Array(_) => todo!("Implement array"),

                _ => unreachable!(),
            }
        }

        todo!()
    }

    fn derive_basic_schema(object: &Value) -> Schema {
        match object {
            Value::Null => Schema::Null,
            Value::Bool(_) => Schema::Boolean(BooleanSchema::default()),
            Value::Number(number) if number.is_i64() => Schema::Integer(IntegerSchema::default()),
            Value::Number(number) if number.is_f64() => Schema::Number(NumberSchema::default()),
            Value::String(_) => Schema::String(StringSchema::default()),
            _ => panic!("All basic types are covered"),
        }
    }

    fn is_basic_value(object: &Value) -> bool {
        !matches!(object, Value::Object(_) | Value::Array(_))
    }
}
