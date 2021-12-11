use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn read_input() -> Result<Vec<isize>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    Ok(input
        .split(',')
        .map(|s| s.trim().parse().expect("not a number!"))
        .collect())
}

fn part1(input: &[isize]) {
    let best_fuel = find_best_fuel(input, calc_fuel_simple);
    println!("part1: result = {}", best_fuel);
}

fn part2(input: &[isize]) {
    let best_fuel = find_best_fuel(input, calc_fuel_cumulative);

    println!("part2: result = {}", best_fuel);
}

fn find_best_fuel<F>(input: &[isize], calc_fuel_fn: F) -> isize
where
    F: Fn(isize, isize) -> isize + Copy,
{
    let &min_pos = input.iter().min().unwrap();
    let &max_pos = input.iter().max().unwrap();

    let mut best_fuel = isize::MAX;

    for pos in min_pos..=max_pos {
        let fuel = calc_total_fuel(input, pos, calc_fuel_fn);
        if fuel < best_fuel {
            best_fuel = fuel;
        }
    }

    best_fuel
}

fn calc_total_fuel<F>(sources: &[isize], target: isize, calc: F) -> isize
where
    F: Fn(isize, isize) -> isize,
{
    let mut total_fuel = 0;

    for &source in sources {
        total_fuel += calc(source, target);
    }

    total_fuel
}

fn calc_fuel_simple(source: isize, target: isize) -> isize {
    (target - source).abs()
}

fn calc_fuel_cumulative(source: isize, target: isize) -> isize {
    (0..(target - source).abs()).map(|i| i + 1).sum()
}
