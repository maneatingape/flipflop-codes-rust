use crate::util::iter::*;
use crate::util::parse::*;
use std::collections::HashMap;

type Input = Vec<[u8; 3]>;

pub fn parse(input: &str) -> Input {
    input.iter_unsigned().chunk::<3>().collect()
}

pub fn part1(input: &Input) -> String {
    let mut frequency = HashMap::new();

    for &bush in input {
        *frequency.entry(bush).or_insert(0) += 1;
    }

    let ([r, g, b], _) = frequency.into_iter().max_by_key(|&(_, v)| v).unwrap();
    format!("{r},{g},{b}")
}

pub fn part2(input: &Input) -> usize {
    input.iter().filter(|&&[r, g, b]| label_color(g, r, b)).count()
}

pub fn part3(input: &Input) -> u32 {
    input
        .iter()
        .map(|&[r, g, b]| {
            if label_color(r, g, b) {
                5
            } else if label_color(g, r, b) {
                2
            } else if label_color(b, r, g) {
                4
            } else {
                10
            }
        })
        .sum()
}

fn label_color(x: u8, y: u8, z: u8) -> bool {
    x > y && x > z && y != z
}
