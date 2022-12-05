use std::{fs::File, io::{BufRead, BufReader, Seek}};

use clap::Parser;
use std::collections::HashSet;
use itertools::Itertools;

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

struct ElfGroup {
    one: String,
    two: String,
    three: String,
}

impl ElfGroup {
    fn common_element(&self) -> Vec<char> {
        let mut retval = Vec::new();
        let mut set_one: HashSet<char> = HashSet::from_iter(self.one.chars());
        let set_two: HashSet<char> = HashSet::from_iter(self.two.chars());
        let set_three: HashSet<char> = HashSet::from_iter(self.three.chars());

        // Dude this sucks, but it seems to work.
        let intersection = &mut set_one;
        intersection.retain(|e| set_two.contains(e));
        let sames : Vec<&char> = intersection.intersection(&set_three).collect();
        for ch in sames {
            retval.push(*ch);
        }
        retval
    }
}

struct Rucksack {
    one: String,
    two: String,
}

impl Rucksack {
    fn common_elements(&self) -> Vec<char> {
        let mut retval = Vec::new();
        let set_one: HashSet<char> = HashSet::from_iter(self.one.chars());
        let set_two: HashSet<char> = HashSet::from_iter(self.two.chars());
        let sames : Vec<&char> = set_one.intersection(&set_two).collect();
        for ch in sames {
            retval.push(*ch);
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

fn parseElfGroup<T>(input_buffer: T) -> Result<Vec<ElfGroup>, ParseError> where T: BufRead {
    let mut groups = Vec::new();
    let lines = BufReader::new(input_buffer).lines();
    for chunk in &lines.into_iter().chunks(3) {
        if let Some((a,b,c)) = chunk.collect_tuple() {
            groups.push(ElfGroup{one:a.unwrap(), two:b.unwrap(), three:c.unwrap()})
        }
    }
    Ok(groups)
}

fn score_char(input: char) -> usize {
    let uch = input as u8 as usize;
    if (65..=90).contains(&uch) {
        uch - 38
    } else if (97..=122).contains(&uch) {
        uch - 96
    } else {
        0
    }

}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(&args.input).unwrap();
    let rucksacks = parse(BufReader::new(input_file))?;

    let mut score: usize = 0;

    for sack in rucksacks {
        let common = sack.common_elements();
        for ch in common {
            score += score_char(ch);
        }
    }
    println!("Value = {}", score);

    let input_file = File::open(args.input).unwrap();
    let groups = parseElfGroup(BufReader::new(input_file));

    let mut group_score = 0;

    for group in groups.unwrap() {
        for ch in group.common_element() {
            group_score += score_char(ch);
        }
    }

    println!("Group Score: {}", group_score);

    Ok(())
}
