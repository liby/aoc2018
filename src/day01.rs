use std::collections::HashSet;
use std::io;

use crate::error::Error;

pub fn run<R>(mut input: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;

    let total_one = part_one(&buffer);
    let total_two = part_two(&buffer);

    Ok((total_one.to_string(), total_two.to_string()))
}

fn part_one(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let change = line.parse::<i32>().unwrap();
        total += change;
    }
    total
}

fn part_two(input: &str) -> i32 {
    let mut freq = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    loop {
        for line in input.lines() {
            let change: i32 = line.parse().unwrap();
            freq += change;
            if seen.contains(&freq) {
                return freq;
            }
            seen.insert(freq);
        }
    }
}
