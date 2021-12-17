use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn read_input() -> Result<Vec<String>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    Ok(input.lines().map(|l| l.to_string()).collect())
}

const PAIRS: [&str; 4] = ["()", "[]", "{}", "<>"];

fn is_opener(c: char) -> bool {
    PAIRS.iter().any(|s| s.chars().nth(0).unwrap() == c)
}

fn find_opener(c: char) -> char {
    PAIRS
        .iter()
        .find(|s| s.chars().nth(1).unwrap() == c)
        .map(|s| s.chars().nth(0).unwrap())
        .unwrap()
}

fn find_closer(c: char) -> char {
    PAIRS
        .iter()
        .find(|s| s.chars().nth(0).unwrap() == c)
        .map(|s| s.chars().nth(1).unwrap())
        .unwrap()
}

enum ParseResult {
    Complete,
    Incomplete(Vec<char>),
    Corrupt(char),
}

fn parse_line(line: &str) -> ParseResult {
    let mut stack = Vec::new();
    for c in line.chars() {
        if is_opener(c) {
            stack.push(c);
        } else {
            let opener = find_opener(c);
            if *stack.last().expect("stack is empty!") != opener {
                return ParseResult::Corrupt(c);
            } else {
                stack.pop();
            }
        }
    }
    if stack.is_empty() {
        ParseResult::Complete
    } else {
        ParseResult::Incomplete(stack)
    }
}

fn part1(input: &[String]) {
    let mut score = 0;
    for line in input.iter() {
        if let ParseResult::Corrupt(c) = parse_line(line) {
            score += match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("unrecognized character: {}", c),
            }
        }
    }

    println!("part1: score = {}", score);
}

fn part2(input: &[String]) {
    let incomplete_lines: Vec<Vec<char>> = input
        .iter()
        .map(|line| parse_line(line))
        .map(|r| match r {
            ParseResult::Incomplete(stack) => Some(stack),
            _ => None,
        })
        .flatten()
        .collect();

    let mut line_scores = Vec::new();

    for mut stack in incomplete_lines {
        let mut score = 0u64;
        while let Some(c) = stack.pop() {
            let closer = find_closer(c);
            score *= 5;
            score += match closer {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("unrecognized character: {}", closer),
            }
        }
        line_scores.push(score);
    }

    line_scores.sort();

    let result = line_scores[line_scores.len()/2];

    println!("part2: result = {}", result);
}
