use flipflop_codes::year2026::puzzle09::*;

const EXAMPLE: &str = "\
###########
#S#.#...#.#
#.#.#.###.#
#.#.......#
#.#.#.#####
#...#.#..E#
###.#.#.###
#...#.#...#
#.###.#.#.#
#.#.....#.#
###########";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 24);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 10);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 19);
}
