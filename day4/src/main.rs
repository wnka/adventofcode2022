use std::{fs::File, io::{BufRead, BufReader}, fmt};

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

#[derive(Debug)]
struct Groups {
    group1: Schedule,
    group2: Schedule,
}

#[derive(Debug)]
struct Schedule {
    start: usize,
    end: usize,
}

impl Schedule {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

impl fmt::Display for Groups {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Group 1: {}, Group 2: {}", self.group1, self.group2)
    }
}

impl fmt::Display for Schedule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Start: {}, End: {}", self.start, self.end)
    }
}

fn parse<T>(input_buffer: T) -> Result<Vec<Groups>, ParseError> where T: BufRead {
    let mut ranges = Vec::new();
    let lines = BufReader::new(input_buffer).lines();
    for line in lines {
        match line {
            Ok(s) => {
                let elements : Vec<&str> = s.split(',').collect();
                let group1times : Vec<usize> = elements[0].split('-').map(|v| v.parse::<usize>().unwrap()).collect();
                let group2times : Vec<usize> = elements[1].split('-').map(|v| v.parse::<usize>().unwrap()).collect();
                ranges.push(Groups{
                    group1: Schedule{start: group1times[0], end: group1times[1]},
                    group2: Schedule{start: group2times[0], end: group2times[1]},
                });
            },
            Err(_) => return Err(ParseError::Error)
        }
    }
    Ok(ranges)
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;

    let contains_count = input_ranges.iter().filter(|x| x.group1.contains(&x.group2) || x.group2.contains(&x.group1)).count();

    println!("Contains count = {}", contains_count);

    Ok(())
}
