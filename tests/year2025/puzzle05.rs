use flipflop_codes::year2025::puzzle05::*;

const EXAMPLE: &str = "ABccksiPiBAksP";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 38);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), "Bc");
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), -6);
}
