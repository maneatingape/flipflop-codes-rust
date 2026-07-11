use crate::util::grid::*;
use crate::util::point::*;
use std::mem::replace;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(input: &Grid<u8>) -> u64 {
    machine(input, false, true)
}

pub fn part2(input: &Grid<u8>) -> u64 {
    machine(input, true, true)
}

pub fn part3(input: &Grid<u8>) -> u64 {
    machine(input, true, false)
}

fn machine(input: &Grid<u8>, follow_connections: bool, allow_primes: bool) -> u64 {
    let mut grid = input.clone();
    let mut letters = vec![(b's', true)];
    let mut valid_lights = Vec::new();

    while let Some((letter, parity)) = letters.pop() {
        let start = grid.find(letter.to_ascii_uppercase()).unwrap();

        let mut todo = vec![(start, parity)];
        let mut lights = Vec::new();
        let mut connections = Vec::new();
        let mut gears = 0;

        while let Some((point, parity)) = todo.pop() {
            for next in ORTHOGONAL.map(|o| point + o) {
                match replace(&mut grid[next], b' ') {
                    b'#' | b'3' => {
                        todo.push((next, !parity));
                        gears += 1;
                    }
                    b'*' => lights.push((next, !parity)),
                    from if from.is_ascii_lowercase() => connections.push((from, parity)),
                    _ => (),
                }
            }
        }

        if letter == b's' || allow_primes || is_composite(gears) {
            if follow_connections {
                letters.extend(connections);
            }
            valid_lights.extend(lights);
        }
    }

    valid_lights.sort_unstable_by_key(|&(point, _)| (point.y, point.x));
    valid_lights.into_iter().fold(0, |n, (_, parity)| (n << 1) | u64::from(parity))
}

fn is_composite(n: u32) -> bool {
    (2..n).take_while(|&f| f * f <= n).any(|f| n.is_multiple_of(f))
}
