use std::env;

mod encode;
mod decode;
mod bst;

fn main() {
    let args: Vec<String> = env::args().collect();
    let flag = args[1].as_str();
    let file = args[2].as_str();

    match flag {
        "-e" => {
            encode::encode(file);
        }
        "-d" => {
            decode::decode(file);
        }
        _ => {
            help();
        }
    }
}

fn help() {
    println!("Wrong arguments used.");
}