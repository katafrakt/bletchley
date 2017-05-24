extern crate openssl;
extern crate rustc_serialize;
extern crate crypto;

use std::collections::HashMap;
use self::openssl::rsa::{Rsa, PKCS1_PADDING};
use std::fs::File;
use std::io::Read;
use std::str;
use std::fmt::Display;
use std::iter::repeat;
use self::rustc_serialize::base64::{STANDARD, FromBase64};
use self::crypto::aes::{self, KeySize};

pub fn decrypt_from_file(data_hash: HashMap<&str,Vec<u8>>, key: &str) {
    let mut key_contents = String::new();
    let mut f = File::open(key).unwrap(); // TODO handle errors like insufficient permissions
    f.read_to_string(&mut key_contents).unwrap();

    let private_key = Rsa::private_key_from_pem(key_contents.as_bytes()).unwrap();
    let mut encrypted_key = vec![0; 1000];
    let data_base64 = data_hash.get("key").unwrap();
    let data = data_base64.from_base64().expect("Cannot decode the key");
    let encrypted_bytes = private_key.private_decrypt(&data, &mut encrypted_key, PKCS1_PADDING).unwrap();
    encrypted_key.truncate(encrypted_bytes);
    let enc_key_str = str::from_utf8(encrypted_key.as_slice()).unwrap();
    let parts: Vec<&str> = enc_key_str.split("|").collect();
    let aes_key_base64 = parts[0];
    let aes_nonce_base64 = parts[1];
    let aes_key = aes_key_base64.from_base64().unwrap();
    let aes_nonce = aes_nonce_base64.from_base64().unwrap();

    let mut cipher = aes::ctr(KeySize::KeySize128, &aes_key, &aes_nonce);
    let input = data_hash.get("content").unwrap();
    let mut encrypted_file: Vec<u8> = repeat(0u8).take(input.len()).collect();
    cipher.process(&input, &mut encrypted_file[..]);
    let plaintext_str = String::from_utf8_lossy(&encrypted_file);
    print!("{}", plaintext_str);
}
