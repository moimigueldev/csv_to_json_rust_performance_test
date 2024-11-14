use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
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

    std::fs::create_dir_all("./batch_files").expect("Failed to create directory");

    let output_file = File::create("./output.json").expect("Unable to create");
    let mut output_writer = BufWriter::new(output_file);
    write!(output_writer, "[").expect("Unable to write opening bracket"); // Write the opening bracket for JSON array

    let mut reader = csv::Reader::from_path(file_path).expect("Error while reading file");
    let headers: Vec<String> = reader
        .headers()
        .unwrap()
        .iter()
        .map(|h| h.to_string())
        .collect();

    reader
        .records()
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to read records")
        .par_chunks(batch_size)
        .enumerate()
        .for_each(|(i, chunk)| {
            let batch_file_name = format!("./batch_files/output-{}.json", i);
            let batch_file = File::create(&batch_file_name).expect("Unable to create batch file");
            let mut writer = BufWriter::new(batch_file);

            for (index, line) in chunk.iter().enumerate() {
                let mut json_object = json!({});
                for (j, header) in headers.iter().enumerate() {
                    json_object.as_object_mut().unwrap().insert(
                        header.clone(),
                        Value::String(line.get(j).unwrap().to_string()),
                    );
                }

                if index > 0 {
                    write!(writer, ",").expect("Failed to write comma");
                }

                to_writer(&mut writer, &json_object).expect("Could not write to file");
            }
        });

    let mut first_file = true;

    for entry in fs::read_dir("./batch_files").expect("./batch_files dir not found") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file() {
            if !first_file {
                write!(output_writer, ",").expect("Unable to write comma");
            } else {
                first_file = false;
            }

            let batch_file = File::open(&path).expect("Failed to open batch file");
            let mut batch_reader = BufReader::new(batch_file);

            let mut content = String::new();
            batch_reader
                .read_to_string(&mut content)
                .expect("Failed to read batch file");

            write!(output_writer, "{}", content).expect("Failed to write batch content to output");
        }
    }

    write!(output_writer, "]").expect("Unable to write closing bracket");

    fs::remove_dir_all("./batch_files").expect("Failed to delete batch files directory");
}
