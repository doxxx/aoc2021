use shared::Grid;
use std::fmt;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;

    part1(&input);

    Ok(())
}

fn read_input() -> Result<Vec<LineSegment>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    Ok(input.lines().map(LineSegment::parse).collect())
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn parse(s: &str) -> Point {
        s.split(',')
            .map(|s| s.trim().parse().expect("invalid number"))
            .collect::<Vec<isize>>()
            .into()
    }
}

impl From<Vec<isize>> for Point {
    fn from(v: Vec<isize>) -> Self {
        assert!(v.len() == 2);
        Point { x: v[0], y: v[1] }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct LineSegment(Point, Point);

impl LineSegment {
    fn parse(s: &str) -> LineSegment {
        s.split("->")
            .map(Point::parse)
            .collect::<Vec<Point>>()
            .into()
    }

    fn is_horizontal(&self) -> bool {
        self.0.y == self.1.y
    }

    fn is_vertical(&self) -> bool {
        self.0.x == self.1.x
    }

    fn x_dir(&self) -> isize {
        if self.0.x < self.1.x {
            1
        } else if self.0.x > self.1.x {
            -1
        } else {
            0
        }
    }

    fn y_dir(&self) -> isize {
        if self.0.y < self.1.y {
            1
        } else if self.0.y > self.1.y {
            -1
        } else {
            0
        }
    }

    fn apply_to_grid<T, F>(&self, grid: &mut Grid<T>, f: F)
    where
        T: Clone,
        F: Fn(isize, isize, &T) -> T,
    {
        if self.is_horizontal() {
            let x_dir = self.x_dir();
            let y = self.0.y;
            let mut x = self.0.x;
            while x != self.1.x + x_dir {
                let coords = (x as usize, y as usize);
                let v = &grid[coords];
                grid[coords] = f(x, y, v);
                x += x_dir;
            }
        } else if self.is_vertical() {
            let y_dir = self.y_dir();
            let x = self.0.x;
            let mut y = self.0.y;
            while y != self.1.y + y_dir {
                let coords = (x as usize, y as usize);
                let v = &grid[coords];
                grid[coords] = f(x, y, v);
                y += y_dir;
            }
        } else {
            let x_dir = self.x_dir();
            let y_dir = self.y_dir();
            let mut x = self.0.x;
            let mut y = self.0.y;
            while x != self.1.x + x_dir && y != self.1.y + y_dir {
                let coords = (x as usize, y as usize);
                let v = &grid[coords];
                grid[coords] = f(x, y, v);
                x += x_dir;
                y += y_dir;
            }
        }
    }
}

impl From<Vec<Point>> for LineSegment {
    fn from(v: Vec<Point>) -> Self {
        assert!(v.len() == 2);
        LineSegment(v[0], v[1])
    }
}

struct OceanFloor(Grid<isize>);

impl OceanFloor {
    fn new(size: usize) -> OceanFloor {
        OceanFloor(Grid::new_square(size, 0))
    }

    fn count_overlaps(&self, min_overlap: isize) -> usize {
        self.0.iter().filter(|(_, _, &v)| v >= min_overlap).count()
    }
}

impl fmt::Display for OceanFloor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (x, y, &v) in self.0.iter() {
            if x == 0 && y > 0 {
                write!(f, "\n")?;
            }
            let c = (if v == 0 { b'.' } else { b'0' + (v as u8) }) as char;
            write!(f, "{} ", c)?;
        }

        Ok(())
    }
}

fn part1(lines: &[LineSegment]) {
    let max_coord = lines
        .iter()
        .flat_map(|l| vec![l.0.x, l.0.y, l.1.x, l.1.y])
        .max()
        .expect("no lines!") as usize;

    let mut ocean_floor = OceanFloor::new(max_coord + 1);

    for line in lines.iter() {
        line.apply_to_grid(&mut ocean_floor.0, |_, _, v| v + 1);
    }

    // println!("{}", ocean_floor);

    let result = ocean_floor.count_overlaps(2);

    println!("part1: result = {}", result);
}
