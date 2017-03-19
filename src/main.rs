#[macro_use]
extern crate clap;
mod crypto;
use crypto::key_generator;

fn main() {
    let matches = clap_app!(bletchley =>
        (version: "0.1")
        (author: "Paweł Świątkowski")
        (@subcommand gen =>
            (about: "Generates keys")
            (@arg output: -o --output +takes_value "Base of the filenames of created keys (default: \"id\")")
        )
    ).get_matches();


    if let Some(matches) = matches.subcommand_matches("gen") {
        let name = matches.value_of("output").unwrap_or("id");

        let pair = key_generator::generate_pair("rsa");
        match pair {
            Some(p) => p.write_keys(name),
            None => print!("Could not generate keys")
        }
    }
}
