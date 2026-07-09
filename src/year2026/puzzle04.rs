pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> usize {
    input.iter().rev().skip(401).filter(|line| line.contains('o')).count()
}

pub fn part2(input: &[&str]) -> usize {
    runs(input).len() - 1
}

pub fn part3(input: &[&str]) -> usize {
    let mut runs = runs(input);
    let mut result = 0;

    while let Some(&min) = runs.iter().min() {
        result += min;

        let shrunk: Vec<_> = runs
            .iter()
            .enumerate()
            .filter(|&(_, &run)| run > min)
            .map(|(index, &run)| (index % 2, run - min))
            .collect();

        runs = shrunk
            .chunk_by(|a, b| a.0 == b.0)
            .map(|side| side.iter().map(|&(_, run)| run).sum())
            .collect();
    }

    result
}

fn runs(input: &[&str]) -> Vec<usize> {
    let leaves: Vec<_> = input.iter().filter(|line| line.contains('o')).collect();
    leaves.chunk_by(|a, b| a == b).map(<[_]>::len).collect()
}
