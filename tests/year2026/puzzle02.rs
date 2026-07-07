use flipflop_codes::year2026::puzzle02::*;

const EXAMPLE: &str = "><>><<>><<<>>>>><><><><><>>>>>";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(input), 12);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(input), 3);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(input), 1358);
}
