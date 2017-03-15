mod crypto;
use crypto::key_generator;

fn main() {
    let pair = key_generator::generate_pair("rsa");
    match pair {
        Some(p) => p.write_keys("test"),
        None => print!("Could not generate keys")
    }
}
