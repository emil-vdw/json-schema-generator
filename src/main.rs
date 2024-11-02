use std::fs::File;
use std::io::BufReader;

use schema::{integer::IntegerSchema, number::NumberSchema, object::ObjectSchema, string::StringSchema, Schema};

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

    let file = File::open("test_data/complex_object.json").expect("The file should exist");
    let reader = BufReader::new(file);

    let schema: Schema = serde_json::from_reader(reader).expect("Should always deserialize");

    println!(
        "{}",
        serde_json::to_string_pretty(&schema).unwrap()
    );
}
