extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate openssl;

use std::path::Path;
use std::process::exit;
use std::fs::File;
use std::io::{Read, Write};
use std::iter::repeat;
use self::rustc_serialize::base64::{STANDARD, ToBase64};
use self::rand::{OsRng, Rng};
use self::crypto::aes::{self, KeySize};
use self::crypto::symmetriccipher::SynchronousStreamCipher;
use self::openssl::rsa::{Rsa, PKCS1_PADDING};

pub fn with_file_key(key: &str, input: &str) {
    let key_path = Path::new(key);
    if !key_path.exists() {
        fail("Key file does not exist.");
    }
    let input_path = Path::new(input);
    if !input_path.exists() {
        fail("Input file does not exist.");
    }

    let mut key_contents = String::new();
    let mut f = File::open(key).unwrap(); // TODO handle errors like insufficient permissions
    f.read_to_string(&mut key_contents).unwrap();

    encrypt_file(key_contents, input);
}

fn fail(message: &str) {
    print!("ERROR: {}\n", message);
    exit(1);
}

fn encrypt_file(rsa_key: String, input_path: &str) {
    // create random string which will serve as key to AES
    // source: http://zsiciarz.github.io/24daysofrust/book/vol1/day21.html
    let key_length = 80; // must fit into RSA encrypt max length of 117 (?) bytes
    let mut gen = OsRng::new().expect("Failed to get OS random generator");
    let mut key: Vec<u8> = repeat(0u8).take(key_length).collect();
    gen.fill_bytes(&mut key[..]);
    let mut nonce: Vec<u8> = repeat(0u8).take(key_length).collect();
    gen.fill_bytes(&mut nonce[..]);

    // read input
    let mut input = Vec::new();
    let mut f = File::open(input_path).expect("Cannot open input file");
    f.read_to_end(&mut input).expect("Cannot read input file");

    // encrypt input
    let mut cipher = aes::ctr(KeySize::KeySize128, &key, &nonce);
    let mut encrypted_file: Vec<u8> = repeat(0u8).take(input.len()).collect();
    cipher.process(&input, &mut encrypted_file[..]);

    // encrypt key
    let key_string = format!("{}|{}|{}|{}", key.len(),
        key.to_base64(STANDARD), nonce.len(), nonce.to_base64(STANDARD));
    let public_key = Rsa::public_key_from_pem(rsa_key.as_bytes()).unwrap(); // TODO don't assume it's public key
    let mut encrypted_key = vec![0; public_key.size()];
    let encrypted_bytes = public_key.public_encrypt(key_string.as_bytes(), &mut encrypted_key, PKCS1_PADDING).unwrap();
    encrypted_key.truncate(encrypted_bytes);

    // construct resulting string
    let encrypted_key_base64 = encrypted_key.to_base64(STANDARD);
    let encrypted_file_base64 = encrypted_file.to_base64(STANDARD);
    let output = format!("{}|{}|{}|{}", encrypted_key_base64.len(), encrypted_key_base64,
        encrypted_file_base64.len(), encrypted_file_base64);

    // write results
    let mut output_file = File::create("output.ble").expect("Cannot open output file for writing");
    output_file.write_all(output.as_bytes()).unwrap();
}
