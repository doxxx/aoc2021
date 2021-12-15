use shared::Grid;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn read_input() -> Result<Grid<u8>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let width = lines[0].len();
    let height = lines.len();
    let mut g = Grid::new(width, height, 0u8);

    for (row, line) in lines.into_iter().enumerate() {
        for (col, c) in line.char_indices() {
            g[(col, row)] = (c as u8) - ('0' as u8);
        }
    }

    Ok(g)
}

fn is_low_point(g: &Grid<u8>, x: usize, y: usize) -> bool {
    let center = g[(x, y)];
    let x = x as isize;
    let y = y as isize;
    let surrounding = vec![
        g.try_get(x, y - 1),
        g.try_get(x, y + 1),
        g.try_get(x - 1, y),
        g.try_get(x + 1, y),
    ];

    surrounding.into_iter().flatten().all(|&v| center < v)
}

fn part1(input: &Grid<u8>) {
    let low_points = input.iter().filter(|&(x, y, _)| is_low_point(input, x, y));
    let risk_levels = low_points.into_iter().map(|(_, _, v)| (1 + v) as u32);
    let sum_risk_levels: u32 = risk_levels.into_iter().sum();

    println!("part1: result = {}", sum_risk_levels);
}

fn discover_basin_size(g: &Grid<u8>, counted: &mut Grid<bool>, x: usize, y: usize) -> usize {
    counted[(x, y)] = true;

    let center = g[(x, y)];
    let x = x as isize;
    let y = y as isize;
    let surrounding = vec![
        g.try_get(x, y - 1).map(|&v| (x, y - 1, v)),
        g.try_get(x, y + 1).map(|&v| (x, y + 1, v)),
        g.try_get(x - 1, y).map(|&v| (x - 1, y, v)),
        g.try_get(x + 1, y).map(|&v| (x + 1, y, v)),
    ];

    let surrounding: Vec<(usize, usize, u8)> = surrounding
        .into_iter()
        .flatten()
        .map(|(x, y, v)| (x as usize, y as usize, v))
        .filter(|&(x, y, v)| !counted[(x, y)] && v < 9 && v > center)
        .collect();

    for &(x, y, _) in surrounding.iter() {
        counted[(x, y)] = true;
    }

    1usize
        + surrounding
            .into_iter()
            .map(|(x, y, _)| discover_basin_size(g, counted, x as usize, y as usize))
            .sum::<usize>()
}

fn part2(input: &Grid<u8>) {
    let mut counted = Grid::new(input.width(), input.height(), false);
    let low_points = input.iter().filter(|&(x, y, _)| is_low_point(input, x, y));
    let mut basin_sizes = low_points
        .into_iter()
        .map(|(x, y, _)| discover_basin_size(input, &mut counted, x, y))
        .collect::<Vec<usize>>();

    basin_sizes.sort();
    basin_sizes.reverse();

    let result = basin_sizes
        .iter()
        .take(3)
        .fold(1usize, |acc, size| acc * size);

    println!("part2: result = {}", result);
}
