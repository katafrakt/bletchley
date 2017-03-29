use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::str;

const VERSION: i32 = 1;

pub fn create_file(info: HashMap<String,Vec<u8>>, fname: Option<&str>) {
    let filename = fname.unwrap_or("output.ble");
    let mut output_file = File::create(filename).expect("Cannot open output file for writing");
    let key_str = str::from_utf8(info.get("key").unwrap()).unwrap();

    let ble_contents = json!({
        "version": VERSION,
        "key": key_str
    });

    output_file.write_all(ble_contents.to_string().as_bytes()).unwrap();
    output_file.write_all("\n".as_bytes());
    output_file.write_all(info.get("content").unwrap());
}
