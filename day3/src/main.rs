use std::{fs::File, io::{BufRead, BufReader}};

use clap::Parser;
use sets::{Set,MutSetOps};

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
    Error,
    NotEven
}

struct Rucksack {
    one: String,
    two: String,
}

impl Rucksack {
    fn common_elements(&self) -> Vec<char> {
        let mut retval = Vec::new();
        let set_one = Set::new_ordered(self.one.as_bytes(), true);
        let set_two = Set::new_ordered(self.two.as_bytes(), true);
        let sames = set_one.intersection(&set_two);
        for item in sames.data {
            retval.push(char::from(item));
        }
        retval
    }
}

fn parse<T>(input_buffer: T) -> Result<Vec<Rucksack>, ParseError> where T: BufRead {
    let mut ranges = Vec::new();
    let lines = BufReader::new(input_buffer).lines();
    for line in lines {
        match line {
            Ok(s) => {
                if s.len() % 2 != 0 { return Err(ParseError::NotEven) }
                let size = s.len()/2;
                let one = &s[0..size];
                let two = &s[size..s.len()];
                ranges.push(Rucksack{one:one.to_string(), two:two.to_string()});
            }
            Err(_) => return Err(ParseError::Error)
        }
    }
    Ok(ranges)
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let rucksacks = parse(BufReader::new(input_file))?;

    let mut score: usize = 0;

    for sack in rucksacks {
        let common = sack.common_elements();
        println!("{},{} = {:?}", sack.one, sack.two, common);
        for ch in common {
            let uch = ch as u8 as usize;
            if (65..=90).contains(&uch) {
                score += uch - 38;
            } else if (97..=122).contains(&uch) {
                score += uch - 96;
            }
            println!("Value = {}", score);
        }
    }

    // initial guess was 8966 ... ohhh I didn't dedupe

    Ok(())
}
