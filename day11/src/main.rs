use shared::Grid;
use std::env;
use std::fs::File;
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
    if let Some(filename) = env::args().skip(1).nth(0) {
        let mut r = File::open(filename)?;
        r.read_to_string(&mut input)?;
    } else {
        std::io::stdin().read_to_string(&mut input)?;
    }

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

fn reset_flashed(g: &mut Grid<u8>) {
    for y in 0..g.height() {
        for x in 0..g.width() {
            if g[(x, y)] >= 10 {
                g[(x, y)] = 0;
            }
        }
    }
}

fn increment(g: &mut Grid<u8>, x: usize, y: usize) -> usize {
    let mut flashes = 0;

    g[(x, y)] += 1;

    if g[(x, y)] == 10 {
        flashes += 1;

        let x = x as isize;
        let y = y as isize;
        for yy in (y - 1)..=(y + 1) {
            for xx in (x - 1)..=(x + 1) {
                if xx >= 0 && (xx as usize) < g.width() && yy >= 0 && (yy as usize) < g.height() {
                    flashes += increment(g, xx as usize, yy as usize);
                }
            }
        }
    }

    flashes
}

fn dump(g: &Grid<u8>) {
    for y in 0..g.height() {
        for x in 0..g.width() {
            let v = g[(x, y)];
            if v >= 10 {
                print!("*");
            } else {
                print!("{}", v)
            }
        }
        println!("")
    }
}

fn part1(input: &Grid<u8>) {
    let mut g = input.clone();
    let mut total_flashes = 0;

    let step_count = 100;

    for step in 1..=step_count {
        // println!("\nstep {} start:", i);
        // dump(input);

        for y in 0..g.height() {
            for x in 0..g.width() {
                total_flashes += increment(&mut g, x, y);
            }
        }

        reset_flashed(&mut g);

        // println!("\nstep {} end:", i);
        // dump(&g);
    }

    println!("\npart1: result = {}", total_flashes);
}

fn part2(input: &Grid<u8>) {
    let mut g = input.clone();

    let mut step_count = 1;

    loop {
        // println!("\nstep {} start:", i);
        // dump(input);

        for y in 0..g.height() {
            for x in 0..g.width() {
                increment(&mut g, x, y);
            }
        }

        if g.iter().all(|(_, _, &v)| v >= 10) {
            break;
        }

        reset_flashed(&mut g);

        // println!("\nstep {} end:", i);
        // dump(&g);

        step_count += 1;
    }

    println!("\npart1: result = {}", step_count);
}
