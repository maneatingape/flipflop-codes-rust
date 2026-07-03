type Input = Vec<(u8, usize)>;

pub fn parse(input: &str) -> Input {
    input.as_bytes().chunk_by(|a, b| a == b).map(|sequence| (sequence[0], sequence.len())).collect()
}

pub fn part1(input: &Input) -> i32 {
    solve(input, |n| n)
}

pub fn part2(input: &Input) -> i32 {
    solve(input, |n| n * (n + 1) / 2)
}

pub fn part3(input: &Input) -> i32 {
    let mut fibonacci = vec![0, 1];
    solve(input, |n| {
        for i in fibonacci.len()..=n {
            fibonacci.push(fibonacci[i - 1] + fibonacci[i - 2]);
        }
        fibonacci[n]
    })
}

fn solve<F>(input: &Input, mut adjust: F) -> i32
where
    F: FnMut(usize) -> usize,
{
    input
        .iter()
        .scan(0, |position, &(movement, amount)| {
            let adjusted = adjust(amount) as i32;
            *position += if movement == b'^' { adjusted } else { -adjusted };
            Some(*position)
        })
        .max()
        .unwrap()
}
