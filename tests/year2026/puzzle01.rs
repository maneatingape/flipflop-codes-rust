use flipflop_codes::year2026::puzzle01::*;

const EXAMPLE: &str = "\
41
87
93
104
46
102
65
105
81
36
66
46
60
65
64
64
61
73
55
69";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 76);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 1371);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 1141);
}
