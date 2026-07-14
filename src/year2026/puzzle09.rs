use crate::util::grid::*;
use crate::util::heap::*;
use crate::util::point::*;
use std::collections::VecDeque;

type Input = (Grid<u8>, Point, Point);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();
    (grid, start, end)
}

pub fn part1(input: &Input) -> u32 {
    let (grid, start, end) = input;
    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with(false);

    todo.push_back((*start, 0));
    seen[*start] = true;

    while let Some((position, steps)) = todo.pop_front() {
        if position == *end {
            return steps;
        }

        for next in ORTHOGONAL.map(|o| position + o) {
            if grid[next] != b'#' && !seen[next] {
                todo.push_back((next, steps + 1));
                seen[next] = true;
            }
        }
    }

    unreachable!()
}

pub fn part2(input: &Input) -> u32 {
    let (grid, start, end) = input;
    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with(false);

    todo.push_back((*start, 0));
    seen[*start] = true;

    while let Some((position, steps)) = todo.pop_front() {
        if position == *end {
            return steps;
        }

        for direction in ORTHOGONAL {
            let mut portal = position;

            while grid[portal + direction] != b'#' {
                portal += direction;
            }

            for next in [position + direction, portal] {
                if grid[next] != b'#' && !seen[next] {
                    todo.push_back((next, steps + 1));
                    seen[next] = true;
                }
            }
        }
    }

    unreachable!()
}

pub fn part3(input: &Input) -> u32 {
    let (grid, start, end) = input;
    let mut todo = MinHeap::new();
    let mut cost = grid.same_size_with(u32::MAX);

    todo.push(0, (*start, false));
    cost[*start] = 0;

    while let Some((steps, (position, portal))) = todo.pop() {
        if position == *end {
            return steps;
        }

        for next in ORTHOGONAL.map(|o| position + o) {
            if grid[next] != b'#' && cost[next] > steps + 1 {
                todo.push(steps + 1, (next, false));
                cost[next] = steps + 1;
            }
        }

        if ORTHOGONAL.iter().any(|&o| grid[position + o] == b'#') {
            let next_steps = steps + if portal { 2 } else { 3 };

            for direction in ORTHOGONAL {
                let mut next = position;

                while grid[next + direction] != b'#' {
                    next += direction;
                }

                if cost[next] > next_steps {
                    todo.push(next_steps, (next, true));
                    cost[next] = next_steps;
                }
            }
        }
    }

    unreachable!()
}
