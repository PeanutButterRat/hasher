mod sha;

use std::{fs, path::PathBuf};
use std::process::exit;
use clap::Parser;
use sha::{sha256, sha384, sha512};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct App {
    /// Hashing algorithm to use
    algorithm: String,

    /// File to hash
    file: PathBuf,

    /// Hash the string passed as the file argument instead of opening a file
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    immediate: bool,
}

#[allow(unused)]
fn main() {
    let cli = App::parse();

    let algorithm = match cli.algorithm.to_ascii_lowercase().as_str() {
        "sha256" => sha256::hash,
        "sha384" => sha384::hash,
        "sha512" => sha512::hash,
        _ => {
            println!("unknown algorithm specified ({})", cli.algorithm);
            exit(1);
        }
    };

    let message;

    if cli.immediate {
        message = cli.file.as_os_str()
            .to_str()
            .expect("failed to convert string")
            .as_bytes()
            .to_vec();
    } else {
        message = fs::read(cli.file).expect("failed to open file");
    }

    let result = algorithm(message);

    println!("{}", hex::encode(result));
}