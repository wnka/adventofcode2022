use std::{fs::File, io::{BufRead, BufReader}, collections::HashMap};

use clap::Parser;

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
struct Command {
    times: u8,
    from: u8,
    to: u8,
}

fn parse<T>(input_buffer: T) -> Result<Vec<Command>, ParseError> where T: BufRead {
    let mut ranges = Vec::new();
    let lines = BufReader::new(input_buffer).lines();
    for line in lines {
        match line {
            Ok(s) => {
                let mut iter = s.splitn(3, ' ');
                ranges.push(Command {
                    times: iter.next().unwrap().parse::<u8>().unwrap(),
                    from: iter.next().unwrap().parse::<u8>().unwrap(),
                    to: iter.next().unwrap().parse::<u8>().unwrap()
                });
            }
            Err(_) => return Err(ParseError::Error)
        }
    }
    Ok(ranges)
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let input_commands = parse(BufReader::new(input_file))?;

    let mut containers : HashMap<u8, Vec<char>> = HashMap::new();

    let v = vec!['R','N','P','G'];
    containers.insert(1, v);
    let v = vec!['T', 'J', 'B', 'L', 'C', 'S', 'V', 'H'];
    containers.insert(2, v);
    let v = vec!['T', 'D', 'B', 'M', 'N', 'L'];
    containers.insert(3, v);
    containers.insert(4, vec!['R', 'V', 'P', 'S', 'B']);
    containers.insert(5, vec!['G', 'C', 'Q', 'S', 'W', 'M', 'V', 'H']);
    containers.insert(6, vec!['W', 'Q', 'S', 'C', 'D', 'B', 'J']);
    containers.insert(7, vec!['F', 'Q', 'L']);
    containers.insert(8, vec!['W', 'M', 'H', 'T', 'D', 'L', 'F', 'V']);
    containers.insert(9, vec!['L', 'P', 'B', 'V', 'M', 'J', 'F']);

    println!("{:?}", containers);

    for command in input_commands {
        println!("{:?}", command);
        let mut temp_q: Vec<char> = Vec::new();
        for _i in 0..command.times {
            let container = containers.get_mut(&command.from).unwrap().pop();
            match container {
                Some(c) => temp_q.push(c),
                None => panic!("Couldn't get char from container!"),
            }
        }
        for i in temp_q.iter_mut().rev() {
            containers.get_mut(&command.to).unwrap().push(*i);
        }
    }

    println!("{:?}", containers);

    for i in 1..=9 {
        print!("{}", containers.get_mut(&i).unwrap().pop().unwrap());
    }

    Ok(())
}
