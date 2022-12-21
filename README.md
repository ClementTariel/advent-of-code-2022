# advent-of-code-2022

I started to learn rust on the 1st of December so the quality of the code is not great especially for the first days.


To test the solution of the last day, run `cargo run`.

To test the solution of a specific day, run `AOC_DAY=XX cargo run` where "XX" is the day you want to test.

For example `AOC_DAY=7 cargo run` or `AOC_DAY=05 cargo run` or `AOC_DAY=18 cargo run` 

You can specify the part you want to run with the --features flag for example `cargo run --features "part1"` to run only the first part.

You can also use the "small" feature to use the small input, for example `AOC_DAY=14 cargo run --features "part2 small"` to solve only the part2 of day 14 with the small input.