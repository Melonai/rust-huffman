extern crate clap;

use clap::{App, SubCommand, Arg};

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod bst;
mod decode;
mod encode;
mod utils;

fn main() {
    let cli_input = App::new("RHE")
        .about("Rust Huffman Encoder - Encodes and decodes files using Huffman coding.")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            SubCommand::with_name("encode")
                .about("Encodes the given file 'example.txt' into 'example.huff'.")
                .arg(
                    Arg::with_name("FILE")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            SubCommand::with_name("decode")
                .about("Decodes the given file 'example.huff' into 'example.huff_decoded'.")
                .arg(
                    Arg::with_name("FILE")
                        .required(true)
                        .index(1)
                )
        )
        .get_matches();

    match cli_input.subcommand() {
        ("decode", Some(matched_file)) => {
            let file_path = matched_file.value_of("FILE").unwrap();
            match decode::decode(utils::file_path_to_buffer(file_path)) {
                Ok(decoded) => write_output(file_path, "huff_decoded", decoded).expect("Failed to write."),
                Err(_) => error(),
            };
        }
        ("encode", Some(matched_file)) => {
            let file_path = matched_file.value_of("FILE").unwrap();
            match encode::encode(&utils::file_path_to_buffer(file_path)) {
                Ok(encoded) => write_output(file_path, "huff", encoded).expect("Failed to write."),
                Err(_) => error(),
            };
        }
        _ => {}
    }
}

fn error() {
    println!("An error occurred.");
}

fn write_output(
    old_path: &str,
    new_extension: &str,
    output: Vec<u8>,
) -> Result<(), std::io::Error> {
    let mut output_file = File::create(format!(
        "{}.{}",
        Path::new(old_path).file_stem().unwrap().to_str().unwrap(),
        new_extension
    ))?;
    output_file.write_all(output.as_slice())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::decode;
    use crate::encode;
    use std::collections::BTreeMap;

    #[test]
    fn decoding_simple() {
        let test_data = "Hello!".as_bytes();
        let encoded = encode::encode(test_data).unwrap();
        let decoded = decode::decode(encoded.clone()).unwrap();
        assert_eq!(decoded.as_slice(), test_data);
    }

    #[test]
    fn decoding_emoji() {
        let test_data = "♣️ 💄 📭 👻 😰 🔱 🚚 ☯ 🔋 🗓".as_bytes();
        let encoded = encode::encode(test_data).unwrap();
        let decoded = decode::decode(encoded.clone()).unwrap();
        assert_eq!(decoded.as_slice(), test_data);
    }

    #[test]
    fn decoding_utf() {
        let test_data = "Οἱ δὲ Φοίνιϰες οὗτοι οἱ σὺν Κάδμῳ ἀπιϰόμενοι.. ἐσήγαγον διδασϰάλια ἐς τοὺς ῞Ελληνας ϰαὶ δὴ ϰαὶ γράμματα.".as_bytes();
        let encoded = encode::encode(test_data).unwrap();
        let decoded = decode::decode(encoded.clone()).unwrap();
        assert_eq!(decoded.as_slice(), test_data);
    }

    #[test]
    fn bst_transformation() {
        let occurrence = encode::count_occurrence("Hello!".as_bytes());
        let indices_from_tree = encode::unwrap_bst_to_indices(encode::create_bst(&occurrence));
        let indices_to_tree =
            encode::unwrap_bst_to_indices(decode::indices_to_tree(&indices_from_tree));
        assert_eq!(indices_to_tree, indices_from_tree);
    }

    #[test]
    fn map_reading() {
        let indices = get_indices();
        let mut map = encode::create_map(indices.clone());
        let (map_length, payload) = map.split_first_mut().unwrap();
        let (indices_from_map, ..) = decode::read_map_and_separate(*map_length, payload);
        assert_eq!(indices, indices_from_map);
    }

    fn get_indices() -> BTreeMap<u8, Vec<bool>> {
        let occurrence = encode::count_occurrence("Hello!".as_bytes());
        encode::unwrap_bst_to_indices(encode::create_bst(&occurrence))
    }
}
