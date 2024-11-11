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
    let mut json_list: Vec<Value> = Vec::new();

    let mut records: Vec<_> = reader
        .records()
        .collect::<Result<_, _>>()
        .expect("Failed to read records");

    // let num_of_threads = 2;

    let chunk_size = records.len() / num_of_threads;
    // dbg!(records.len());
    // dbg!(&chunk_size);

    let (tx, rx) = mpsc::channel();

    for thread_index in 0..num_of_threads {
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

    for received in rx {
        json_list.push(received);
    }

    to_writer(&mut writer, &json_list).expect("Unable to write JSON data to file");
}

// fn main() {
//     let file_path = std::env::args()
//         .nth(1)
//         .unwrap_or("../data/chicago_crimes_2024_min.csv".to_string());
//
//     let mut reader = csv::Reader::from_path(file_path).expect("Error while reading file");
//     let headers: Vec<String> = reader
//         .headers()
//         .unwrap()
//         .iter()
//         .map(|h| h.to_string())
//         .collect();
//     let headers = Arc::new(headers);
//     let output_file = File::create("./output_no_mem.json").expect("Unable to create");
//     let mut writer = BufWriter::new(&output_file);
//     let mut json_list: Arc<Mutex<Vec<Value>>> = Arc::new(Mutex::new(Vec::new()));
//     let mut thread_list = vec![];
//     let num_of_threads = 4;
//     let mut records: Vec<_> = reader
//         .records()
//         .collect::<Result<_, _>>()
//         .expect("failed to read records");
//
//     let chunk_size = records.len() / num_of_threads;
//     dbg!(records.len());
//     dbg!(&chunk_size);
//
//     for i in 0..num_of_threads {
//         let json_list_copy = json_list.clone();
//         let headers_copy = headers.clone();
//
//         let chunk = if records.len() > chunk_size {
//             records.split_off(records.len() - chunk_size)
//         } else {
//             // Take the remainignt elements if less than chunk size
//             std::mem::take(&mut records)
//         };
//
//         let handle = thread::spawn(move || {
//             println!("Started thread: {}", i); // Immediate println! at the start of each thread
//
//             // Simulate processing time to observe parallel execution
//             std::thread::sleep(std::time::Duration::from_secs(1));
//             let mut lock_list = json_list_copy.lock().unwrap();
//
//             for item in chunk.into_iter() {
//                 let mut json_object = json!({});
//                 for (j, h) in headers_copy.iter().enumerate() {
//                     let map = json_object.as_object_mut().expect("Expected a jons object");
//                     map.insert(
//                         h.to_string(),
//                         Value::String(item.get(j).unwrap().to_string()),
//                     );
//                 }
//                 lock_list.push(json_object);
//             }
//
//             println!("Finished processing in thread: {}", i);
//         });
//
//         thread_list.push(handle);
//     }
//
//     for thread in thread_list.into_iter() {
//         thread.join().unwrap();
//     }
//
//     to_writer(&mut writer, &*json_list.lock().unwrap()).expect("Unable to write JSON data to file");
// }
