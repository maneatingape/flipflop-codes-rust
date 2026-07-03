use crate::util::iter::*;
use crate::util::parse::*;
use std::collections::HashMap;

type Input = Vec<[u8; 2]>;
type Dimension = [u8; 8];
type Cache = HashMap<Dimension, u64>;

pub fn parse(input: &str) -> Input {
    input.iter_unsigned().chunk::<2>().collect()
}

pub fn part1(input: &Input) -> u64 {
    solve(input, |x, y| [x, y, 1, 1, 1, 1, 1, 1])
}

pub fn part2(input: &Input) -> u64 {
    solve(input, |x, y| [x, y, x, 1, 1, 1, 1, 1])
}

pub fn part3(input: &Input) -> u64 {
    solve(input, |x, y| {
        let mut dimensions = [1; 8];
        dimensions[..x as usize].fill(y);
        dimensions
    })
}

fn solve(input: &Input, f: fn(u8, u8) -> Dimension) -> u64 {
    let mut cache = HashMap::new();
    input.iter().map(|&[x, y]| paths(&mut cache, f(x, y))).sum()
}

fn paths(cache: &mut Cache, mut dimensions: Dimension) -> u64 {
    dimensions.sort_unstable();

    if dimensions[6] == 1 {
        return 1;
    }
    if let Some(&seen) = cache.get(&dimensions) {
        return seen;
    }

    let result = (0..8)
        .filter(|&i| dimensions[i] > 1)
        .map(|i| {
            let mut next = dimensions;
            next[i] -= 1;
            paths(cache, next)
        })
        .sum();
    cache.insert(dimensions, result);
    result
}
