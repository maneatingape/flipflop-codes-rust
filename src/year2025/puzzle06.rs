use crate::util::iter::*;
use crate::util::parse::*;

type Input = Vec<[i64; 2]>;

pub fn parse(input: &str) -> Input {
    input.iter_signed().chunk::<2>().collect()
}

pub fn part1(input: &Input) -> usize {
    picture(input, 100)
}

pub fn part2(input: &Input) -> usize {
    (1..=1000).map(|n| picture(input, 3600 * n)).sum()
}

pub fn part3(input: &Input) -> usize {
    (1..=1000).map(|n| picture(input, 31556926 * n)).sum()
}

fn picture(input: &Input, time: i64) -> usize {
    let visible = |n: i64| (250..750).contains(&(n * time).rem_euclid(1000));
    input.iter().filter(|&&[x, y]| visible(x) && visible(y)).count()
}
