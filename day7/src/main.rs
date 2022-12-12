use std::{fs::File, io::{BufRead, BufReader}};
use clap::Parser;
use camino::Utf8PathBuf;
use nom::{bytes::complete::{take_while1, tag}, combinator::{map, all_consuming}, IResult, sequence::{preceded, separated_pair}, branch::alt, Finish};

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

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
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls(Ls),
    Cd(Cd),
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (cmd, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Command::Ls), map(parse_cd, Command::Cd)))(cmd)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );

    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_dir, parse_file))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry)
    ))(i)
}

fn parse<T>(input_buffer: T) -> Result<Vec<Line>, ParseError> where T: BufRead {
    let lines = BufReader::new(input_buffer).lines();
//    let lines = include_str!("../input.txt").lines();
    let plines = lines.map(|l| {
        let l_temp = l.unwrap();
        let result = all_consuming(parse_line)(l_temp.as_str()).finish().unwrap().1;
        result
    }).collect();
    Ok(plines)
}

fn main() -> Result<(), ParseError> {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let input_ranges = parse(BufReader::new(input_file))?;

    for range in input_ranges {
        println!("{:?}", range);
    }

    Ok(())
}
