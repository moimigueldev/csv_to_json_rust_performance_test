use std::{fs::File, io::BufWriter};

use serde_json::{json, to_writer, Value};
use std::io::Write;

fn main() {
    let file_path = std::env::args()
        .nth(1)
        .unwrap_or("../data/chicago_crimes_2024_min.csv".to_string());

    let mut reader = csv::Reader::from_path(file_path).expect("Error while reading file");
    let output_file = File::create("./output_no_mem.json").expect("Unable to create");
    let mut writer = BufWriter::new(&output_file);

    write!(writer, "[").expect("Unable to write opening bracket");

    let headers: Vec<String> = reader
        .headers()
        .unwrap()
        .iter()
        .map(|h| h.to_string())
        .collect();

    for (i, line) in reader.records().enumerate() {
        let readable_line = line
            .unwrap()
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();

        let mut json_object = json!({});

        for (j, h) in headers.iter().enumerate() {
            let map = json_object.as_object_mut().expect("Expected a jons object");

            map.insert(h.to_string(), Value::String(readable_line[j].to_string()));
        }

        if i > 0 {
            write!(writer, ",").expect("Unable to write comma");
        }

        to_writer(&mut writer, &json_object).expect("Unable to write JSON data to file");
    }

    write!(writer, "]").expect("Unable to write closing bracket");
}
