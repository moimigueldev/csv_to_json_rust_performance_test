use std::fs::File;

use serde_json::{json, to_writer, Value};

fn main() {
    let file_path = std::env::args()
        .nth(1)
        .unwrap_or("../data/chicago_crimes_2024_min.csv".to_string());

    let mut reader = csv::Reader::from_path(file_path).expect("Error while reading file");
    let mut json_vec: Vec<Value> = vec![];
    let headers: Vec<String> = reader
        .headers()
        .unwrap()
        .iter()
        .map(|h| h.to_string())
        .collect();

    for line in reader.records() {
        let readable_line = line
            .unwrap()
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();

        let mut json_object = json!({});

        for (index, h) in headers.iter().enumerate() {
            let map = json_object.as_object_mut().expect("Expected a jons object");

            map.insert(
                h.to_string(),
                Value::String(readable_line[index].to_string()),
            );
        }

        json_vec.push(json_object);
    }

    let output_file = File::create("./output.json").expect("Unable to create");
    to_writer(&output_file, &json_vec).expect("Unable to write JSON data to file");
}
