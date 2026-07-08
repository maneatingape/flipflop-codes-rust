const COLORS: [&str; 3] = ["red", "green", "blue"];

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> &str {
    input.lines().max_by_key(|password| score(password, false)).unwrap()
}

pub fn part2(input: &str) -> &str {
    input.lines().max_by_key(|password| score(password, true)).unwrap()
}

pub fn part3(input: &str) -> usize {
    ('a'..='z')
        .chain('A'..='Z')
        .chain('0'..='9')
        .map(|last| input.lines().map(|password| score(&format!("{password}{last}"), true)).sum())
        .max()
        .unwrap()
}

fn score(password: &str, additional: bool) -> usize {
    let count = |predicate: fn(&u8) -> bool| password.bytes().filter(predicate).count();
    let lower = count(u8::is_ascii_lowercase);
    let upper = count(u8::is_ascii_uppercase);
    let digits = count(u8::is_ascii_digit);
    let sevens = count(|&b| b == b'7');

    let variety = lower.min(1) + upper.min(1) + digits.min(1);

    let sevens = if additional && digits > 0 && digits == sevens { 7 } else { 0 };

    let run = password.as_bytes().chunk_by(|a, b| a == b).map(<[_]>::len).max().unwrap();
    let run = if additional && run >= 3 { run * run } else { 0 };

    let color = if additional && COLORS.iter().any(|c| password.contains(c)) { 3 } else { 1 };

    (variety + sevens + run) * color * password.len()
}
