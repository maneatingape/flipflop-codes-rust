use flipflop_codes::year2026::puzzle08::*;

const EXAMPLE: &str = "\
A A C
A B C
A C B
B B A B A
B C B A
C C B B";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(input), 4102);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(input), 433);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(input), 117302657);
}
