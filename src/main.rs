use std::{
    fs::File,
    io::{BufWriter, Write},
};

use rayon::prelude::*;
use serde_json::{json, to_writer, Value};

fn main() {
    let file_path = std::env::args()
        .nth(1)
        .unwrap_or("../data/chicago_crimes_2024_min.csv".to_string());

    let batch_size = std::env::args()
        .nth(2)
        .unwrap_or("100".to_string())
        .parse()
        .unwrap_or(100);

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

    let batches: Vec<_> = records
        .par_chunks(batch_size)
        .map(|chunk| {
            let mut batch = Vec::new();

            for line in chunk {
                let mut json_object = json!({});
                for (index, header) in headers.iter().enumerate() {
                    let map = json_object.as_object_mut().expect("Expected a jons object");
                    map.insert(
                        header.to_string(),
                        Value::String(line.get(index).unwrap().to_string()),
                    );
                }

                batch.push(json_object);
            }

            batch
        })
        .collect();

    let mut first_batch = true;

    write!(writer, "[").expect("Unable to write closing bracket");
    for batch in batches {
        for json_obect in batch {
            if !first_batch {}
            write!(&mut writer, ",").expect("Unable to write comma");
            first_batch = false;
            to_writer(&mut writer, &json_obect).expect("Unable to write final batch to file");
        }
    }

    write!(writer, "]").expect("Unable to write closing bracket");

    // to_writer(&mut writer, &json_list).expect("Unable to write final batch to file");
}
