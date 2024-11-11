use std::{fs::File, io::BufWriter};

use rayon::prelude::*;
use serde_json::{json, to_writer, Value};

fn main() {
    let file_path = std::env::args()
        .nth(1)
        .unwrap_or("../data/chicago_crimes_2024_min.csv".to_string());

    let output_file = File::create("./output.json").expect("Unable to create");
    let mut reader = csv::Reader::from_path(file_path).expect("Error while reading file");
    let headers: Vec<String> = reader
        .headers()
        .unwrap()
        .iter()
        .map(|h| h.to_string())
        .collect();
    let mut writer = BufWriter::new(&output_file);
    let records: Vec<_> = reader
        .records()
        .collect::<Result<_, _>>()
        .expect("Failed to read records");

    let json_list: Vec<_> = records
        .into_par_iter()
        .map(|line| {
            let mut json_object = json!({});
            for (index, header) in headers.iter().enumerate() {
                let map = json_object.as_object_mut().expect("Expected a jons object");
                map.insert(
                    header.to_string(),
                    Value::String(line.get(index).unwrap().to_string()),
                );
            }

            json_object
        })
        .collect();

    to_writer(&mut writer, &json_list).expect("Unable to write final batch to file");
}
