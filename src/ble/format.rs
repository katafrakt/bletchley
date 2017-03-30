use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::str;
use std::io::{BufRead, BufReader};
use serde_json;

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

pub fn read_file(fname: &str) -> HashMap<&str,Vec<u8>> {
    let mut file = BufReader::new(File::open(fname).unwrap());
    let mut json_line = String::new();
    file.read_line(&mut json_line);
    let json: serde_json::Value = serde_json::from_slice(json_line.as_bytes()).unwrap();
    let key = json["key"].as_str().unwrap().to_string().into_bytes();

    let mut binary_content = Vec::new();
    file.read_to_end(&mut binary_content);

    let mut results = HashMap::new();
    results.insert("key", key);
    results.insert("content", binary_content);

    return results;
}
