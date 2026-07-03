use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;
use std::iter::once;

pub fn parse(input: &str) -> Vec<Point> {
    once(ORIGIN).chain(input.iter_signed().chunk::<2>().map(|[x, y]| Point::new(x, y))).collect()
}

pub fn part1(input: &[Point]) -> i32 {
    pickup(input, Point::manhattan)
}

pub fn part2(input: &[Point]) -> i32 {
    pickup(input, diagonal)
}

pub fn part3(input: &[Point]) -> i32 {
    let mut sorted = input.to_vec();
    sorted.sort_by_key(|p| p.manhattan(ORIGIN));
    pickup(&sorted, diagonal)
}

fn pickup(input: &[Point], distance: fn(Point, Point) -> i32) -> i32 {
    input.array_windows::<2>().map(|&[a, b]| distance(a, b)).sum()
}

fn diagonal(a: Point, b: Point) -> i32 {
    let Point { x, y } = a - b;
    x.abs().max(y.abs())
}
