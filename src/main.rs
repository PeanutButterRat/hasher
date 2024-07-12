pub mod sha256;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct App {
    /// Hashing algorithm to use
    algorithm: String,

    /// File to hash
    file: PathBuf,

    /// Hash the string passed as the file argument instead of opening a file
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    immediate: u8,
}

fn main() {
    let _cli = App::parse();
}