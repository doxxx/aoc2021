use std::env;
use std::fs::File;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;

    // TODO: process input

    Ok(())
}

fn read_input() -> Result<Vec<String>> {
    let mut input = String::new();
    if let Some(filename) = env::args().skip(1).nth(0) {
        let mut r = File::open(filename)?;
        r.read_to_string(&mut input)?;
    } else {
        std::io::stdin().read_to_string(&mut input)?;
    }

    Ok(input.lines().map(|l| l.to_string()).collect())
}
