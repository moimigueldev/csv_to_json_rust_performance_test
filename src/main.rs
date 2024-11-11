use std::{
    fs::File,
    io::BufWriter,
    sync::{Arc, Mutex},
    thread,
};

use serde_json::{json, to_writer, Value};
use std::io::Write;

fn main() {
    let file_path = std::env::args()
        .nth(1)
        .unwrap_or("../data/chicago_crimes_2024_min.csv".to_string());

    let num_of_threads = std::env::args()
        .nth(2)
        .unwrap_or("1".to_string())
        .parse()
        .unwrap_or(1);

    let mut reader = csv::Reader::from_path(file_path).expect("Error while reading file");
    let headers: Vec<String> = reader
        .headers()
        .unwrap()
        .iter()
        .map(|h| h.to_string())
        .collect();
    let headers = Arc::new(headers);
    let output_file = File::create("./output_no_mem.json").expect("Unable to create");
    let mut writer = BufWriter::new(&output_file);
    let mut json_list: Arc<Mutex<Vec<Value>>> = Arc::new(Mutex::new(Vec::new()));
    let mut thread_list = vec![];
    // let num_of_threads = 4;
    let mut records: Vec<_> = reader
        .records()
        .collect::<Result<_, _>>()
        .expect("failed to read records");

    let chunk_size = records.len() / num_of_threads;
    // dbg!(records.len());
    // dbg!(&chunk_size);

    for i in 0..num_of_threads {
        let json_list_copy = json_list.clone();
        let headers_copy = headers.clone();

        let chunk = if records.len() > chunk_size {
            records.split_off(records.len() - chunk_size)
        } else {
            // Take the remainignt elements if less than chunk size
            std::mem::take(&mut records)
        };

        let handle = thread::spawn(move || {
            // println!("Started thread: {}", i); // Immediate println! at the start of each thread

            // Simulate processing time to observe parallel execution
            // std::thread::sleep(std::time::Duration::from_secs(1));

            for item in chunk.into_iter() {
                let mut json_object = json!({});
                for (j, h) in headers_copy.iter().enumerate() {
                    let map = json_object.as_object_mut().expect("Expected a jons object");
                    map.insert(
                        h.to_string(),
                        Value::String(item.get(j).unwrap().to_string()),
                    );
                }
                json_list_copy.lock().unwrap().push(json_object);
            }

            // println!("Finished processing in thread: {}", i);
        });

        thread_list.push(handle);
    }

    for thread in thread_list.into_iter() {
        thread.join().unwrap();
    }

    to_writer(&mut writer, &*json_list.lock().unwrap()).expect("Unable to write JSON data to file");
}
