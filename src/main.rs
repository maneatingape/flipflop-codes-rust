use flipflop_codes::util::ansi::*;
use flipflop_codes::util::parse::*;
use std::env::args;
use std::fs::read_to_string;

fn main() {
    // Parse command line options
    let mut iter = args().flat_map(|arg| arg.iter_unsigned().collect::<Vec<u32>>());
    let (year, puzzle) = (iter.next(), iter.next());

    let solutions = [year2025(), year2026()];

    // Filter solutions then pretty print output.
    solutions
        .into_iter()
        .flatten()
        .filter(|s| year.is_none_or(|y| y == s.year))
        .filter(|s| puzzle.is_none_or(|p| p == s.puzzle))
        .for_each(|Solution { year, puzzle, wrapper }| {
            let path = format!("input/year{year}/puzzle{puzzle:02}.txt");

            if let Ok(data) = read_to_string(&path) {
                let (part1, part2, part3) = wrapper(&data);

                println!("{YELLOW}Year {year} Puzzle {puzzle}{RESET}");
                println!("    Part 1: {BOLD}{WHITE}{part1}{RESET}");
                println!("    Part 2: {BOLD}{WHITE}{part2}{RESET}");
                println!("    Part 3: {BOLD}{WHITE}{part3}{RESET}");
            } else {
                eprintln!("{BOLD}{RED}Year {year} Puzzle {puzzle}{RESET}");
                eprintln!("    Missing input!");
                eprintln!("    Place input file in {BOLD}{WHITE}{path}{RESET}");
            }
        });
}

struct Solution {
    year: u32,
    puzzle: u32,
    wrapper: fn(&str) -> (String, String, String),
}

macro_rules! run {
    ($year:tt $($puzzle:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$(
                Solution {
                    year: stringify!($year).unsigned(),
                    puzzle: stringify!($puzzle).unsigned(),
                    wrapper: |data: &str| {
                        use flipflop_codes::$year::$puzzle::*;

                        let input = parse(data);
                        let part1 = part1(&input).to_string();
                        let part2 = part2(&input).to_string();
                        let part3 = part3(&input).to_string();

                        (part1, part2, part3)
                    }
                }
            ,)*]
        }
    }
}

run!(year2025
    puzzle01, puzzle02, puzzle03, puzzle04, puzzle05, puzzle06, puzzle07
);

run!(year2026
    puzzle01, puzzle02, puzzle03, puzzle04, puzzle05, puzzle06, puzzle07, puzzle08
);
