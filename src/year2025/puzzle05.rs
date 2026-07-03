use std::mem::replace;

type Input = (i32, String, i32);

pub fn parse(input: &str) -> Input {
    let tunnels = input.as_bytes();
    let mut pairs = [0; 128];

    for (i, tunnel) in input.bytes().enumerate() {
        pairs[tunnel as usize] ^= i;
    }

    let mut from = 0;
    let mut part_one = 0;
    let mut part_three = 0;
    let mut visited = [false; 128];

    while from < tunnels.len() {
        let tunnel = tunnels[from];
        let to = from ^ pairs[tunnel as usize];
        let delta = from.abs_diff(to) as i32;

        part_one += delta;
        part_three += if tunnel.is_ascii_uppercase() { -delta } else { delta };

        visited[tunnel as usize] = true;
        from = to + 1;
    }

    let part_two =
        input.chars().filter(|&tunnel| !replace(&mut visited[tunnel as usize], true)).collect();

    (part_one, part_two, part_three)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> &str {
    &input.1
}

pub fn part3(input: &Input) -> i32 {
    input.2
}
