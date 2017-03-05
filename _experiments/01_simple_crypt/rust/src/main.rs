extern crate openssl;
use openssl::rsa::{Rsa, PKCS1_PADDING};

fn main() {
    let pkey = include_bytes!("../../private.pem");
    let private_key = Rsa::private_key_from_pem_passphrase(pkey, b"12345").unwrap();

    let key = include_bytes!("../../public.pem");
    let public_key = Rsa::public_key_from_pem(key).unwrap();

    let mut result = vec![0; public_key.size()];
    let secret_message = String::from("Daj się poznać!");

    public_key.public_encrypt(secret_message.as_bytes(), &mut result, PKCS1_PADDING).unwrap();

    print!("Encrypted:\n");
    print!("{:?}\n\n", result);

    let mut decrypted_text = vec![0; private_key.size()];
    private_key.private_decrypt(&result, &mut decrypted_text, PKCS1_PADDING).unwrap();

    print!("Decrypted:\n");
    print!("{}\n", std::str::from_utf8(&decrypted_text).unwrap());
}
