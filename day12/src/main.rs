use std::collections::{BTreeMap, BTreeSet};
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

fn read_input() -> Result<CaveSystem> {
    let mut input = String::new();
    if let Some(filename) = env::args().skip(1).nth(0) {
        let mut r = File::open(filename)?;
        r.read_to_string(&mut input)?;
    } else {
        std::io::stdin().read_to_string(&mut input)?;
    }

    let mut cs = CaveSystem::default();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        let first = parts[0];
        let second = parts[1];
        {
            let cave = cs
                .caves
                .entry(first.to_string())
                .or_insert_with(|| Cave::new(&first));
            cave.connections.insert(second.to_string());
        }
        {
            let cave = cs
                .caves
                .entry(second.to_string())
                .or_insert_with(|| Cave::new(&second));
            cave.connections.insert(first.to_string());
        }
    }

    Ok(cs)
}

struct CaveSystem {
    caves: BTreeMap<String, Cave>,
}

impl Default for CaveSystem {
    fn default() -> Self {
        Self {
            caves: BTreeMap::new(),
        }
    }
}

struct Cave {
    name: String,
    connections: BTreeSet<String>,
}

impl Cave {
    fn new(name: &str) -> Cave {
        Self {
            name: name.to_string(),
            connections: BTreeSet::new(),
        }
    }

    fn is_big(name: &str) -> bool {
        name.chars().all(|c| c.is_uppercase())
    }

    fn is_small(name: &str) -> bool {
        !Cave::is_big(name)
    }
}

fn discover_path(
    cs: &CaveSystem,
    current_cave: &Cave,
    paths: &mut Vec<Vec<String>>,
    current_path: &mut Vec<String>,
) {
    if Cave::is_small(&current_cave.name) && current_path.contains(&current_cave.name) {
        return;
    }

    current_path.push(current_cave.name.clone());

    if current_cave.name == "end" {
        paths.push(current_path.clone());
        // println!("{}", current_path.join(","));
    } else {
        for connection in current_cave.connections.iter() {
            let connected_cave = cs
                .caves
                .get(connection)
                .expect(&format!("no cave called {}", connection));
            discover_path(cs, connected_cave, paths, current_path);
        }
    }

    current_path.pop();
}

fn already_visited_small_cave_twice(path: &[String]) -> bool {
    let mut visited = BTreeSet::new();
    for cave in path.iter().filter(|c| Cave::is_small(c)) {
        if visited.contains(cave) {
            return true;
        } else {
            visited.insert(cave);
        }
    }
    false
}

fn discover_path2(
    cs: &CaveSystem,
    current_cave: &Cave,
    paths: &mut Vec<Vec<String>>,
    current_path: &mut Vec<String>,
) {
    if current_cave.name == "start" && !current_path.is_empty() {
        return;
    }
    if Cave::is_small(&current_cave.name)
        && current_path.contains(&current_cave.name)
        && already_visited_small_cave_twice(&current_path)
    {
        return;
    }

    current_path.push(current_cave.name.clone());

    if current_cave.name == "end" {
        paths.push(current_path.clone());
        // println!("{}", current_path.join(","));
    } else {
        for connection in current_cave.connections.iter() {
            let connected_cave = cs
                .caves
                .get(connection)
                .expect(&format!("no cave called {}", connection));
            discover_path2(cs, connected_cave, paths, current_path);
        }
    }

    current_path.pop();
}

fn part1(input: &CaveSystem) {
    let mut paths = Vec::new();
    let mut path = Vec::new();
    let start = input.caves.get("start").expect("no start cave!");

    discover_path(input, start, &mut paths, &mut path);

    println!("part1: result = {}", paths.len());
}

fn part2(input: &CaveSystem) {
    let mut paths = Vec::new();
    let mut path = Vec::new();
    let start = input.caves.get("start").expect("no start cave!");

    discover_path2(input, start, &mut paths, &mut path);

    println!("part2: result = {}", paths.len());
}
