use std::{
    fs::File,
    io::{BufWriter, Write},
    thread,
};

use crossbeam_channel::bounded;
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

    let bounded_size = std::env::args()
        .nth(2)
        .unwrap_or("5".to_string())
        .parse()
        .unwrap_or(5);

    let output_file = File::create("./output.json").expect("Unable to create");
    let mut reader = csv::Reader::from_path(file_path).expect("Error while reading file");
    let headers: Vec<String> = reader
        .headers()
        .unwrap()
        .iter()
        .map(|h| h.to_string())
        .collect();

    let (sender, receiver) = bounded(bounded_size);

    let writer_thread = thread::spawn(move || {
        let mut writer = BufWriter::new(&output_file);

        write!(writer, "[").expect("Unable to write closing bracket");

        let mut first = true;

        for batch in receiver {
            if !first {
                write!(writer, ",").expect("unable to write comma");
                first = false;
            }

            to_writer(&mut writer, &batch).expect("Unable to write batch to file");
        }

        write!(writer, "]").expect("Unable to write closing bracket");
    });

    let records = reader
        .records()
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to read records");

    records
        .par_chunks(batch_size)
        .for_each_with(sender, |s, chunk| {
            let batch: Vec<Value> = chunk
                .iter()
                .map(|record| {
                    let mut json_object = json!({});
                    for (index, header) in headers.iter().enumerate() {
                        json_object.as_object_mut().unwrap().insert(
                            header.clone(),
                            Value::String(record.get(index).unwrap().to_string()),
                        );
                    }

                    json_object
                })
                .collect();

            s.send(batch).expect("Failed to send batch");
        });

    writer_thread.join().expect("Writer thread panicked")
}
