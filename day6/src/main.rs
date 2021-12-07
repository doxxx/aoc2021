use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn read_input() -> Result<Vec<usize>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    Ok(input
        .split(',')
        .map(|s| s.trim().parse().expect("not a number!"))
        .collect())
}

fn calculate(initial_timers: &[usize], num_days: usize) -> usize {
    let mut counts = [0; 9];
    
    for &timer in initial_timers {
        counts[timer] += 1
    }

    for _ in 0..num_days {
        let spawning = counts[0];
        for i in 0..8 {
            counts[i] = counts[i+1];
        }
        counts[6] += spawning;
        counts[8] = spawning;
    }

    counts.iter().sum()
}

fn part1(input: &[usize]) {
    let num_fish = calculate(input, 80);
    println!("part1: num fish = {}", num_fish);
}

fn part2(input: &[usize]) {
    let num_fish = calculate(input, 256);
    println!("part2: num fish = {}", num_fish);
}
