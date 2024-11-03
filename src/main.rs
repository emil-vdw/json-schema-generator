use std::fs::File;
use std::io::BufReader;

use schema::{integer::IntegerSchema, number::NumberSchema, object::ObjectSchema, string::StringSchema, Schema};
use schema_generator::SchemaGenerator;
use serde_json::Value;

mod schema;
mod schema_generator;

fn main() {
    let nested_object = Schema::Object(
        ObjectSchema {
            title: Some(String::from("nested")),
            properties: vec![
                (String::from("foo"), Schema::Integer(IntegerSchema::default())),
                (String::from("bar"), Schema::String(StringSchema::default())),
            ].into_iter().collect(),
            description: None,
        }
    );


    let file = File::open("test_data/person_object.json").expect("The file should exist");
    let reader = BufReader::new(file);
    let source: Value = serde_json::from_reader(reader).expect("Should always deserialize");

    // let schema: Schema = serde_json::from_reader(reader).expect("Should always deserialize");

    let mut schema_generator = SchemaGenerator::new();

    let schema = schema_generator.derive_schema(&source);

    println!(
        "{}",
        serde_json::to_string_pretty(&schema).unwrap()
    );
}
