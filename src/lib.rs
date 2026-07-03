macro_rules! library {
    ($year:tt $($day:tt),*) => {
        pub mod $year {$(pub mod $day;)*}
    }
}

library!(util
    ansi, grid, heap, integer, iter, math, parse, point
);

library!(year2025
    puzzle01, puzzle02, puzzle03, puzzle04, puzzle05, puzzle06, puzzle07
);
