use std::{fs, collections::HashSet};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to read
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    let input = fs::read_to_string(args.input).unwrap();

    let width = 4;

    let starting = input.as_bytes().windows(width).position(|pos| {
        println!("{:?}", pos);
        pos.iter().collect::<HashSet<_>>().len() == width
    }).map(|p|{
        println!("map: {}", p);
        p + width
    } );

    println!("Starting point: {}", starting.unwrap());
}
