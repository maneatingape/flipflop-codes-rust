use flipflop_codes::year2026::puzzle06::*;

const EXAMPLE: &str = "\
;&%,&/<.%~&|-!-;-+`.
=#######:@#=*3333,%!
@*+;|.|####.!3,33A&^
-<a|*!~#`!#~`3*-3|/;
S##########*@/!`-|`-
,,|@:#./,@#,,@B=@!%@
<3C!*#`~=*#;./333*.@
%3@&/#*:`~#^|/+3<!&=
|33*><:b###<<c333*~|
<&&@/:!|``:/:&:&&`,&";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 4);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 1195);
}

#[test]
fn part3_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part3(&input), 148);
}
