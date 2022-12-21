#[macro_export]
macro_rules! include_day{
    ($n:expr) => {
        pub fn get_current_day() -> i32{
            let n:i32 = stringify!($n).parse().unwrap();
            return n;
        }
        include!(concat!("challenges/day",stringify!($n),"/day",stringify!($n),".rs"));
        
    };
    () => {
        pub fn get_current_day() -> i32{
            let n:i32 = env!("AOC_DAY").parse().unwrap();
            return n;
        }
        include!(concat!("challenges/day",env!("AOC_DAY"),"/day",env!("AOC_DAY"),".rs"));
    }
    ;
}

include_day!();

fn main() {
    println!("{}",env!("CARGO_MANIFEST_DIR"));
    println!("solving day{}",get_current_day());
    let mut run_part1 = true;
    #[cfg(feature = "part2")]{
        #[cfg(not(feature = "part1"))]{
            run_part1 = false;
        }
    }
    if run_part1{
        println!("part1");
        part1();
    }
    let mut run_part2 = true;
    #[cfg(feature = "part1")]{
        #[cfg(not(feature = "part2"))]{
            run_part2 = false;
        }
    }
    if run_part2{
        println!("part2");
        part2();
    } 
}


