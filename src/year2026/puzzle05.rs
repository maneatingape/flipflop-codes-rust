use crate::util::grid::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(input: &Grid<u8>) -> usize {
    drive(input, 0)
}

pub fn part2(input: &Grid<u8>) -> usize {
    modify(input, 0)
}

pub fn part3(input: &Grid<u8>) -> usize {
    modify(input, 3)
}

fn modify(input: &Grid<u8>, limit: u32) -> usize {
    let mut grid = input.clone();
    let mut result = 0;

    for point in input.points().filter(|&point| interior(input, point)) {
        for &direction in b"^v<>" {
            if direction != input[point] {
                grid[point] = direction;
                result = result.max(drive(&grid, limit));
            }
        }

        grid[point] = input[point];
    }

    result
}

fn drive(grid: &Grid<u8>, limit: u32) -> usize {
    let mut unvisited = grid.same_size_with(true);
    let mut position = ORIGIN;
    let mut distinct = 0;
    let mut right_turns = 0;

    while right_turns <= limit {
        let street = Point::from(grid[position]);

        position += if unvisited[position] {
            unvisited[position] = false;
            distinct += 1;
            street
        } else if interior(grid, position) {
            right_turns += 1;
            street.clockwise()
        } else {
            break;
        };
    }

    distinct
}

fn interior(grid: &Grid<u8>, point: Point) -> bool {
    point.x > 0 && point.y > 0 && point.x < grid.width - 1 && point.y < grid.height - 1
}
