use flipflop_codes::year2025::puzzle01::*;

const EXAMPLE: &str = "\
banana
banenanana
bananana
bananananana
bananananana";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 24);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 16);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 19);
}
