extern crate pest;
#[macro_use]
extern crate pest_derive;

mod json;

use std::time::Instant;

use json::{parse_json, JSONValue};

fn main() {
    let start = Instant::now();
    let input_path = std::env::args()
        .nth(1)
        .expect("Please enter an input Json file path");
    let data = std::fs::read_to_string(input_path).expect("Could not read input file");

    let json = parse_json(&data).unwrap();
    assert!(matches!(json, JSONValue::Array(_)));
    let array = if let JSONValue::Array(array) = json { array } else { todo!() };
    assert!(array.len() == 5000);

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
