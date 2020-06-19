use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

mod bst;
mod decode;
mod encode;

fn main() {
    let args: Vec<String> = env::args().collect();
    let flag = args[1].as_str();
    let file_path = args[2].as_str();

    let mut file = File::open(file_path).expect("Failed to open file.");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file.");

    match flag {
        "-e" => {
            match encode::encode(&buffer) {
                Ok(encoded) => write_output(file_path, "huff", encoded).expect("Failed to write."),
                Err(_) => error(),
            };
        }
        "-d" => {
            decode::decode(buffer);
        }
        _ => {
            help();
        }
    }
}

fn help() {
    println!("Wrong arguments used.");
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
