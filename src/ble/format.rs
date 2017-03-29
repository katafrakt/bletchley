use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

const VERSION: i32 = 1;

pub fn create_file(info: HashMap<String,String>, fname: Option<&str>) {
    let filename = fname.unwrap_or("output.ble");
    let mut output_file = File::create(filename).expect("Cannot open output file for writing");

    let ble_contents = json!({
        "version": VERSION,
        "key": info.get("key"),
        "content": info.get("content")
    });

    output_file.write_all(ble_contents.to_string().as_bytes()).unwrap();
}
