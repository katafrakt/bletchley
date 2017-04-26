#[macro_use] extern crate clap;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate qml;

mod crypto;
mod ble;

fn main() {
    let matches = clap_app!(bletchley =>
        (version: "0.1")
        (author: "Paweł Świątkowski")
        (@subcommand gen =>
            (about: "Generates keys")
            (@arg name: -n --name +takes_value "Base of the filenames of created keys (default: \"id\")")
        )
        (@subcommand encrypt =>
            (@arg key: -k --key +takes_value +required "key to encrypt with")
            (@arg FILE: +required)
        )
        (@subcommand decrypt =>
            (@arg key: -k --key +takes_value +required "key to decrypt with")
            (@arg FILE: +required)
        )
    ).get_matches();


    if let Some(matches) = matches.subcommand_matches("gen") {
        let name = matches.value_of("name").unwrap_or("id");

        let pair = crypto::key_generator::generate_pair("rsa");
        match pair {
            Some(p) => p.write_keys(name),
            None => print!("Could not generate keys")
        }
    } else if let Some(matches) = matches.subcommand_matches("encrypt") {
        let key_file = matches.value_of("key").unwrap();
        let input = matches.value_of("FILE").unwrap();

        let crypt_result = crypto::encryptor::with_file_key(key_file, input);
        ble::format::create_file(crypt_result, None);
    } else if let Some(matches) = matches.subcommand_matches("decrypt") {
        let key_file = matches.value_of("key").unwrap();
        let input = matches.value_of("FILE").unwrap();

        let contents = ble::format::read_file(input);
        crypto::decryptor::decrypt_from_file(contents, key_file);
    } else {
        let mut engine = qml::QmlEngine::new();
        engine.load_file("qml/main.qml");
        engine.exec();
        engine.quit();
    }
}
