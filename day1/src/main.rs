use std::{fs::File, io::{BufRead, BufReader}, collections::BinaryHeap};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to read
    #[arg(short, long)]
    input: String,
}

#[derive(Debug)]
enum ParseError {
    Error
}

fn parse<T>(input_buffer: T) -> Result<Vec<Vec<u32>>, ParseError> where T: BufRead {
    let lines = BufReader::new(input_buffer).lines();

    let mut elves : Vec<Vec<u32>> = Vec::new();
    let mut elf: Vec<u32> = Vec::new();

    for line in lines {
        match line {
            Ok(s) => {
                match s.parse::<u32>() {
                    Ok(v) => elf.push(v),
                    Err(_) => {
                        elves.push(elf);
                        elf = Vec::new();
                    }
                }
            }
            Err(_) => return Err(ParseError::Error)
        }
    }
    Ok(elves)
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;

    let elf_cals : Vec<u32> = input_ranges.iter().map(|e| e.iter().sum()).collect();

    println!("Max value: {}", elf_cals.iter().max().unwrap());

    // part two
    let mut elf_heap: BinaryHeap<u32> = BinaryHeap::new();

    elf_cals.iter().for_each(|e| elf_heap.push(*e));

    let top_3_sum : u32 = elf_heap.into_sorted_vec().iter().rev().take(3).sum();

    println!("Top 3 sum: {}", top_3_sum);

    Ok(())
}
