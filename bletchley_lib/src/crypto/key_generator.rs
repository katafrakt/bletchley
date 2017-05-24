extern crate openssl;

use self::openssl::rsa::{Rsa, PKCS1_PADDING};
use std::string::String;
use std::fs::File;
use std::io::Write;

pub struct KeyPair {
    private: String,
    public: String
}

impl KeyPair {
    pub fn write_keys(&self, name: &str) {
        self.write_file(name, &self.private);
        let file_name = format!("{}{}", name, ".pub");
        self.write_file(&file_name, &self.public);
    }

    fn write_file(&self, name: &str, content: &String) {
        let mut f = File::create(name).unwrap();
        let bytes = content.as_bytes();
        f.write_all(bytes).unwrap();
    }
}

pub fn generate_pair(algo: &str) -> Option<KeyPair> {
    match algo {
        "rsa" => return Some(generate_rsa_pair()),
        &_ => return None
    }
}

fn generate_rsa_pair() -> KeyPair {
    let key = Rsa::generate(2048).unwrap();

    // generate private key
    let priv_pem = key.private_key_to_pem().unwrap();

    // generate public key
    let pub_pem = key.public_key_to_pem().unwrap();

    return KeyPair { public: String::from_utf8(pub_pem).unwrap(), private: String::from_utf8(priv_pem).unwrap() };
}
