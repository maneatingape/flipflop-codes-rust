use crate::util::grid::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;

type Input = (Vec<Point>, Vec<Point>);

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();

    let positions = prefix
        .bytes()
        .scan(ORIGIN, |position, movement| {
            *position += match movement {
                b'^' => DOWN,
                b'v' => UP,
                b'>' => RIGHT,
                b'<' => LEFT,
                _ => unreachable!(),
            };
            Some(*position)
        })
        .collect();
    let sushi = suffix.iter_signed().chunk::<2>().map(|[x, y]| Point::new(x, y)).collect();

    (positions, sushi)
}

pub fn part1(input: &Input) -> usize {
    let (positions, sushi) = input;

    positions[..positions.len() / 2]
        .iter()
        .fold(0, |eaten, &position| eaten + usize::from(position == sushi[eaten]))
}

pub fn part2(input: &Input) -> usize {
    let (positions, sushi) = input;
    let mut grid = Grid::new(30, 30, 0);
    let mut eaten = 0;

    for (i, &position) in positions.iter().enumerate() {
        if grid[position] > i - eaten {
            return eaten + 1;
        }

        grid[position] = i;
        eaten += usize::from(position == sushi[eaten]);
    }

    unreachable!()
}

pub fn part3(input: &Input) -> usize {
    let (positions, sushi) = input;
    let mut grid = Grid::new(30, 30, 0);
    let mut eaten = 0;
    let mut hits = 0;
    let mut size = 0;

    for (i, &position) in positions.iter().enumerate() {
        if grid[position] > i - size {
            hits += 1;
            size = i - grid[position] - 1;
        }

        grid[position] = i;

        if sushi.get(eaten) == Some(&position) {
            eaten += 1;
            size += 1;
        }
    }

    hits * size
}
