use flipflop_codes::year2026::puzzle03::*;

const EXAMPLE: &str = "\
aaaaa111
Ks3SDblu
eowcdredkcasdblu
de333333
7dedlblu
o3klll
8ebluered
DkoGreenD7
green037";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(input), "DkoGreenD7");
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(input), "de333333");
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(input), 1453);
}
