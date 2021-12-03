use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn read_input() -> Result<Vec<Command>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input.lines()
        .map(|line| parse_command(line).expect("invalid command"))
        .collect())
}

fn parse_command(s: &str) -> Result<Command> {
    let mut parts = s.split_whitespace();
    let word = parts.next().unwrap();
    let num = parts.next().unwrap().parse::<u32>().expect("invalid number");
    match word {
        "forward" => Ok(Command::Forward(num)),
        "down" => Ok(Command::Down(num)),
        "up" => Ok(Command::Up(num)),
        _ => Err("invalid command word".into())
    }
}

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn part1(input: &[Command]) {
    let mut depth = 0;
    let mut pos = 0;

    for command in input {
        match &command {
            Command::Forward(n) => pos += n,
            Command::Down(n) => depth += n,
            Command::Up(n) => depth -= n,
        }
    }

    let result = depth * pos;

    println!("part1: {}", result);
}

fn part2(input: &[Command]) {
    let mut aim = 0;
    let mut depth = 0;
    let mut pos = 0;

    for command in input {
        match &command {
            Command::Forward(n) => {
                pos += n;
                depth += aim * n;
            },
            Command::Down(n) => aim += n,
            Command::Up(n) => aim -= n,
        }
    }

    let result = depth * pos;

    println!("part2: {}", result);
}
