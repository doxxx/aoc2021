use std::io::prelude::*;
use std::iter::IntoIterator;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn read_input() -> Result<Vec<u16>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input.lines()
        .map(|line| line.parse::<u16>().expect("invalid number"))
        .collect())
}

fn part1(input: &[u16]) {
    let num_increases = count_increases(input.iter().cloned());

    println!("part1: {}", num_increases);
}

fn part2(input: &[u16]) {
    let window_sums = input.windows(3).map(|w| w[0] + w[1] + w[2]);
    let num_increases = count_increases(window_sums);

    println!("part2: {}", num_increases);
}

fn count_increases(input: impl IntoIterator<Item=u16>) -> u32 {
    let mut iter = input.into_iter();
    let mut last_depth = iter.next().unwrap();
    let mut num_increases = 0;

    for depth in iter {
        if depth > last_depth {
            num_increases += 1;
        }
        last_depth = depth;
    }

    num_increases
}
