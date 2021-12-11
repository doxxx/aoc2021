use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;

    part1(&input);

    Ok(())
}

fn read_input() -> Result<Vec<Entry>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    Ok(input.lines().map(parse_line).collect())
}

fn parse_line(s: &str) -> Entry {
    let parts = s.split('|').take(2).collect::<Vec<&str>>();
    let patterns = parts[0].split_whitespace().map(str::to_string).collect();
    let output = parts[1].split_whitespace().map(str::to_string).collect();
    Entry { patterns, output }
}

fn sort_pattern(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    String::from_iter(chars)
}

struct Entry {
    patterns: Vec<String>,
    output: Vec<String>,
}

fn part1(input: &[Entry]) {
    let mut count = 0;
    for entry in input {
        count += count_unique(&entry.output);
    }
    
    println!("part1: result = {}", count);
}

fn count_unique(patterns: &[String]) -> usize {
    let mut count = 0;
    for pattern in patterns {
        let len = pattern.len();
        if len == 2 || len == 4 || len == 3 || len == 7 {
            count += 1;
        }
    }
    count
}
