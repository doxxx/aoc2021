use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn read_input() -> Result<Vec<Vec<u16>>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input.lines()
        .map(|line| line.bytes().map(|b| (b - b'0') as u16).collect::<Vec<u16>>())
        .collect())
}

const NUM_BITS: usize = 12;

fn part1(input: &[Vec<u16>]) {
    let gamma = calc_gamma(&input);
    let epsilon = !gamma & ((1 << NUM_BITS) - 1);
    let result = gamma as u32 * epsilon as u32;

    println!("part1: {}", result);
}

fn part2(input: &[Vec<u16>]) {
    let mut oxygen_input = input.to_owned();
    
    for bit in 0..NUM_BITS {
        if oxygen_input.len() == 1 {
            break;
        }
        
        let [count_0, count_1] = count_bits(&oxygen_input, bit);
        let most_common = {
            if count_1 > count_0 { 
                1
            }
            else if count_0 > count_1 {
                0
            }
            else {
                1
            }
        };
        oxygen_input = oxygen_input.into_iter()
                                   .filter(|bits| bits[bit] == most_common)
                                   .collect();
    }

    let oxygen = bits_to_int(oxygen_input.first().unwrap());
    println!("part2: oxygen = {}", oxygen);

    let mut co2_input = input.to_owned();
    
    for bit in 0..NUM_BITS {
        if co2_input.len() == 1 {
            break;
        }
        
        let [count_0, count_1] = count_bits(&co2_input, bit);
        let least_common = {
            if count_1 < count_0 { 
                1
            }
            else if count_0 < count_1 {
                0
            }
            else {
                0
            }
        };
        co2_input = co2_input.into_iter()
                                   .filter(|bits| bits[bit] == least_common)
                                   .collect();
    }

    let co2 = bits_to_int(co2_input.first().unwrap());
    println!("part2: co2 = {}", co2);

    let result = oxygen as u32 * co2 as u32;

    println!("part2: {}", result);
}

fn calc_gamma(input: &[Vec<u16>]) -> u16 {
    let mut gamma: [u16; NUM_BITS] = [0; NUM_BITS];

    for bit in 0..NUM_BITS {
        let [count_0, count_1] = count_bits(input, bit);
        if count_1 > count_0 {
            gamma[bit] = 1
        }
    }

    bits_to_int(&gamma)
}

fn count_bits(input: &[Vec<u16>], pos: usize) -> [u16; 2] {
    let mut counts: [u16; 2] = [0; 2];
    
    for num in input {
        counts[num[pos] as usize] += 1;
    }

    counts
}

fn bits_to_int(bits: &[u16]) -> u16 {
    let len = bits.len();
    bits.iter()
        .enumerate()
        .fold(0, |acc, (i, bit)| acc | (bit << (len - i - 1)))
}
