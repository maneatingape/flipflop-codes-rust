use flipflop_codes::year2026::puzzle07::*;

const EXAMPLE: &str = "\
>>>^>>v<^<^>>>>v<^^>vv>^<^^^^^<^<vv>^^^>

3,0
5,1
5,0
3,2
7,1
6,1
7,3
7,1
7,6
5,8
5,7
6,6
7,9";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 7);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 7);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 18);
}
