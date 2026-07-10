use flipflop_codes::year2026::puzzle05::*;

const EXAMPLE: &str = "\
>>>>vvv>vv
>^>>>>>>>v
>>^<>vvv>v
^^^>>vv>^v
^<<<v>vv>v
^<>^<>v>v<
>^^<<<<<vv
^^^<^v<<>v
^v<^<<vvvv
^<^<<<<<<<";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 45);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 49);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 66);
}
