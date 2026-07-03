pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> usize {
    input.iter().map(score).sum()
}

pub fn part2(input: &[&str]) -> usize {
    input.iter().map(score).filter(|score| score.is_multiple_of(2)).sum()
}

pub fn part3(input: &[&str]) -> usize {
    input.iter().filter(|line| !line.contains("ne")).map(score).sum()
}

fn score(line: &&str) -> usize {
    line.matches("ba").count() + line.matches("na").count() + line.matches("ne").count()
}
