#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser, Debug)]
struct Cli {
    /// The parrern to look for
    pattern: String,
    /// The path to the fule to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("Hello, world!");
    println!("{:?}", args);
}
