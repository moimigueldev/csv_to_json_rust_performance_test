use std::{
    fs::File,
    io::BufWriter,
    sync::{mpsc, Arc},
    thread,
};

use serde_json::{json, to_writer, Value};

fn main() {
    // transmitter, reciver
    let file_path = std::env::args()
        .nth(1)
        .unwrap_or("../data/chicago_crimes_2024_min.csv".to_string());

    let num_of_threads = std::env::args()
        .nth(2)
        .unwrap_or("2".to_string())
        .parse()
        .unwrap_or(2);

    let mut reader = csv::Reader::from_path(file_path).expect("Error while reading file");
    let headers: Arc<Vec<String>> = Arc::new(
        reader
            .headers()
            .unwrap()
            .iter()
            .map(|h| h.to_string())
            .collect(),
    );

    let output_file = File::create("./output.json").expect("Unable to create");
    let mut writer = BufWriter::new(&output_file);
    // let json_list: Vec<Value> = Vec::new();

    let mut records: Vec<_> = reader
        .records()
        .collect::<Result<_, _>>()
        .expect("Failed to read records");

    // let num_of_threads = 2;

    let chunk_size = records.len() / num_of_threads;
    // dbg!(records.len());
    // dbg!(&chunk_size);

    let (tx, rx) = mpsc::channel();

    for _ in 0..num_of_threads {
        // println!("Looping:: current thread index: {thread_index}");
        let headers_copy = headers.clone();
        let tx_copy = tx.clone();

        let chunk = if records.len() > chunk_size {
            records.split_off(records.len() - chunk_size)
        } else {
            std::mem::take(&mut records)
        };

        thread::spawn(move || {
            // println!("Thread {} created", thread_index);

            // TODO: Rename item to better reflect the
            // descrition of what we're doing
            for item in chunk.into_iter() {
                let mut json_object = json!({});

                for (i, h) in headers_copy.iter().enumerate() {
                    let map = json_object.as_object_mut().expect("Expected a jons object");
                    map.insert(
                        h.to_string(),
                        Value::String(item.get(i).unwrap().to_string()),
                    );
                }

                tx_copy.send(json_object).unwrap();
            }

            // println!("Thread {} finished", thread_index);
        });
    }

    drop(tx);

    let mut batch = Vec::with_capacity(100); // Adjust batch size as needed
    for received in rx {
        batch.push(received);
        if batch.len() >= 100 {
            // When batch is full
            to_writer(&mut writer, &batch).expect("Unable to write batch to file");
            batch.clear(); // Clear for the next batch
        }
    }

    if !batch.is_empty() {
        to_writer(&mut writer, &batch).expect("Unable to write final batch to file");
    }
}
