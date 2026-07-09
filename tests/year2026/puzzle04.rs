use flipflop_codes::year2026::puzzle04::*;

const EXAMPLE: &str = r"
 -@-
 /|\
  |-o
o-|
o-|
  |-o
  |
o-|
  |-o
  |-o
o-|
o-|
  |
o-|
o-|
  |
o-|
  |-o
#####";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 0);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 6);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 5);
}
