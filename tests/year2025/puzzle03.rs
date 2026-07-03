use flipflop_codes::year2025::puzzle03::*;

const EXAMPLE: &str = "\
10,20,30
20,10,30
30,20,10
10,50,10
50,10,50
10,20,30";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), "10,20,30");
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 0);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 37);
}
