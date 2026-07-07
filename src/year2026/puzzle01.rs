use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u32> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[u32]) -> u32 {
    input.iter().map(|&from| heat(from, 60)).sum()
}

pub fn part2(input: &[u32]) -> u32 {
    input.iter().map(|&from| heat(from, 60) + cool(from, 60)).sum()
}

pub fn part3(input: &[u32]) -> u32 {
    let (first, second) = input.split_at(input.len() / 2);
    first.iter().zip(second).map(|(&from, &to)| heat(from, to) + cool(from, to)).sum()
}

fn heat(from: u32, to: u32) -> u32 {
    to.saturating_sub(from)
}

fn cool(from: u32, to: u32) -> u32 {
    5 * from.saturating_sub(to)
}
