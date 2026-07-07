pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    most_heated(positions(input.bytes().map(delta)))
}

pub fn part2(input: &str) -> usize {
    let robot = positions(input.bytes().map(delta));
    let wall = positions(input.bytes().rev().map(delta));
    robot.zip(wall).filter(|(r, w)| r == w).count()
}

pub fn part3(input: &str) -> usize {
    let deltas = input.bytes().zip(input.bytes().rev()).map(|(f, r)| delta(f) - delta(r));
    most_heated(positions(deltas))
}

fn delta(movement: u8) -> i32 {
    if movement == b'>' { 1 } else { -1 }
}

fn positions<I: Iterator<Item = i32>>(deltas: I) -> impl Iterator<Item = usize> {
    deltas.scan(0, |position, delta| {
        *position = (*position + delta).rem_euclid(100);
        Some(*position as usize)
    })
}

fn most_heated<I: Iterator<Item = usize>>(positions: I) -> usize {
    let mut wall = [0; 100];
    positions.for_each(|position| wall[position] += 1);

    let (segment, temperature) = wall.iter().enumerate().rev().max_by_key(|&(_, t)| t).unwrap();
    (segment + 1) * temperature
}
